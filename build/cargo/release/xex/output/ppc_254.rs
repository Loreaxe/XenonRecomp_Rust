pub fn sub_8327F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F098 size=64
    let mut pc: u32 = 0x8327F098;
    'dispatch: loop {
        match pc {
            0x8327F098 => {
    //   block [0x8327F098..0x8327F0D8)
	// 8327F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F0A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F0AC: 388BF0D4  addi r4, r11, -0xf2c
	ctx.r[4].s64 = ctx.r[11].s64 + -3884;
	// 8327F0B0: 386ADDE4  addi r3, r10, -0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + -8732;
	// 8327F0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F0B8: 4AFADE19  bl 0x8222ced0
	ctx.lr = 0x8327F0BC;
	sub_8222CED0(ctx, base);
	// 8327F0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F0C0: 38690AA8  addi r3, r9, 0xaa8
	ctx.r[3].s64 = ctx.r[9].s64 + 2728;
	// 8327F0C4: 4BA2AE5D  bl 0x82ca9f20
	ctx.lr = 0x8327F0C8;
	sub_82CA9F20(ctx, base);
	// 8327F0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F0D8 size=64
    let mut pc: u32 = 0x8327F0D8;
    'dispatch: loop {
        match pc {
            0x8327F0D8 => {
    //   block [0x8327F0D8..0x8327F118)
	// 8327F0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F0E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F0E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F0EC: 388BF0E0  addi r4, r11, -0xf20
	ctx.r[4].s64 = ctx.r[11].s64 + -3872;
	// 8327F0F0: 386ADDE8  addi r3, r10, -0x2218
	ctx.r[3].s64 = ctx.r[10].s64 + -8728;
	// 8327F0F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F0F8: 4AFADDD9  bl 0x8222ced0
	ctx.lr = 0x8327F0FC;
	sub_8222CED0(ctx, base);
	// 8327F0FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F100: 38690AB8  addi r3, r9, 0xab8
	ctx.r[3].s64 = ctx.r[9].s64 + 2744;
	// 8327F104: 4BA2AE1D  bl 0x82ca9f20
	ctx.lr = 0x8327F108;
	sub_82CA9F20(ctx, base);
	// 8327F108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F118 size=64
    let mut pc: u32 = 0x8327F118;
    'dispatch: loop {
        match pc {
            0x8327F118 => {
    //   block [0x8327F118..0x8327F158)
	// 8327F118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F124: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F12C: 388BF0F8  addi r4, r11, -0xf08
	ctx.r[4].s64 = ctx.r[11].s64 + -3848;
	// 8327F130: 386ADDEC  addi r3, r10, -0x2214
	ctx.r[3].s64 = ctx.r[10].s64 + -8724;
	// 8327F134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F138: 4AFADD99  bl 0x8222ced0
	ctx.lr = 0x8327F13C;
	sub_8222CED0(ctx, base);
	// 8327F13C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F140: 38690AC8  addi r3, r9, 0xac8
	ctx.r[3].s64 = ctx.r[9].s64 + 2760;
	// 8327F144: 4BA2ADDD  bl 0x82ca9f20
	ctx.lr = 0x8327F148;
	sub_82CA9F20(ctx, base);
	// 8327F148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F158 size=64
    let mut pc: u32 = 0x8327F158;
    'dispatch: loop {
        match pc {
            0x8327F158 => {
    //   block [0x8327F158..0x8327F198)
	// 8327F158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F164: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F16C: 388BF10C  addi r4, r11, -0xef4
	ctx.r[4].s64 = ctx.r[11].s64 + -3828;
	// 8327F170: 386ADDF0  addi r3, r10, -0x2210
	ctx.r[3].s64 = ctx.r[10].s64 + -8720;
	// 8327F174: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F178: 4AFADD59  bl 0x8222ced0
	ctx.lr = 0x8327F17C;
	sub_8222CED0(ctx, base);
	// 8327F17C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F180: 38690AD8  addi r3, r9, 0xad8
	ctx.r[3].s64 = ctx.r[9].s64 + 2776;
	// 8327F184: 4BA2AD9D  bl 0x82ca9f20
	ctx.lr = 0x8327F188;
	sub_82CA9F20(ctx, base);
	// 8327F188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F198 size=64
    let mut pc: u32 = 0x8327F198;
    'dispatch: loop {
        match pc {
            0x8327F198 => {
    //   block [0x8327F198..0x8327F1D8)
	// 8327F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F1A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F1A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F1A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F1AC: 388BF120  addi r4, r11, -0xee0
	ctx.r[4].s64 = ctx.r[11].s64 + -3808;
	// 8327F1B0: 386ADDF4  addi r3, r10, -0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + -8716;
	// 8327F1B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F1B8: 4AFADD19  bl 0x8222ced0
	ctx.lr = 0x8327F1BC;
	sub_8222CED0(ctx, base);
	// 8327F1BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F1C0: 38690AE8  addi r3, r9, 0xae8
	ctx.r[3].s64 = ctx.r[9].s64 + 2792;
	// 8327F1C4: 4BA2AD5D  bl 0x82ca9f20
	ctx.lr = 0x8327F1C8;
	sub_82CA9F20(ctx, base);
	// 8327F1C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F1D8 size=64
    let mut pc: u32 = 0x8327F1D8;
    'dispatch: loop {
        match pc {
            0x8327F1D8 => {
    //   block [0x8327F1D8..0x8327F218)
	// 8327F1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F1E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F1EC: 388BF134  addi r4, r11, -0xecc
	ctx.r[4].s64 = ctx.r[11].s64 + -3788;
	// 8327F1F0: 386ADDF8  addi r3, r10, -0x2208
	ctx.r[3].s64 = ctx.r[10].s64 + -8712;
	// 8327F1F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F1F8: 4AFADCD9  bl 0x8222ced0
	ctx.lr = 0x8327F1FC;
	sub_8222CED0(ctx, base);
	// 8327F1FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F200: 38690AF8  addi r3, r9, 0xaf8
	ctx.r[3].s64 = ctx.r[9].s64 + 2808;
	// 8327F204: 4BA2AD1D  bl 0x82ca9f20
	ctx.lr = 0x8327F208;
	sub_82CA9F20(ctx, base);
	// 8327F208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F218 size=64
    let mut pc: u32 = 0x8327F218;
    'dispatch: loop {
        match pc {
            0x8327F218 => {
    //   block [0x8327F218..0x8327F258)
	// 8327F218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F224: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F22C: 388BF148  addi r4, r11, -0xeb8
	ctx.r[4].s64 = ctx.r[11].s64 + -3768;
	// 8327F230: 386ADDFC  addi r3, r10, -0x2204
	ctx.r[3].s64 = ctx.r[10].s64 + -8708;
	// 8327F234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F238: 4AFADC99  bl 0x8222ced0
	ctx.lr = 0x8327F23C;
	sub_8222CED0(ctx, base);
	// 8327F23C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F240: 38690B08  addi r3, r9, 0xb08
	ctx.r[3].s64 = ctx.r[9].s64 + 2824;
	// 8327F244: 4BA2ACDD  bl 0x82ca9f20
	ctx.lr = 0x8327F248;
	sub_82CA9F20(ctx, base);
	// 8327F248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F258 size=64
    let mut pc: u32 = 0x8327F258;
    'dispatch: loop {
        match pc {
            0x8327F258 => {
    //   block [0x8327F258..0x8327F298)
	// 8327F258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F264: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F26C: 388BF160  addi r4, r11, -0xea0
	ctx.r[4].s64 = ctx.r[11].s64 + -3744;
	// 8327F270: 386ADE00  addi r3, r10, -0x2200
	ctx.r[3].s64 = ctx.r[10].s64 + -8704;
	// 8327F274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F278: 4AFADC59  bl 0x8222ced0
	ctx.lr = 0x8327F27C;
	sub_8222CED0(ctx, base);
	// 8327F27C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F280: 38690B18  addi r3, r9, 0xb18
	ctx.r[3].s64 = ctx.r[9].s64 + 2840;
	// 8327F284: 4BA2AC9D  bl 0x82ca9f20
	ctx.lr = 0x8327F288;
	sub_82CA9F20(ctx, base);
	// 8327F288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F298 size=64
    let mut pc: u32 = 0x8327F298;
    'dispatch: loop {
        match pc {
            0x8327F298 => {
    //   block [0x8327F298..0x8327F2D8)
	// 8327F298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F2A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F2A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F2AC: 388BF174  addi r4, r11, -0xe8c
	ctx.r[4].s64 = ctx.r[11].s64 + -3724;
	// 8327F2B0: 386ADE04  addi r3, r10, -0x21fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8700;
	// 8327F2B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F2B8: 4AFADC19  bl 0x8222ced0
	ctx.lr = 0x8327F2BC;
	sub_8222CED0(ctx, base);
	// 8327F2BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F2C0: 38690B28  addi r3, r9, 0xb28
	ctx.r[3].s64 = ctx.r[9].s64 + 2856;
	// 8327F2C4: 4BA2AC5D  bl 0x82ca9f20
	ctx.lr = 0x8327F2C8;
	sub_82CA9F20(ctx, base);
	// 8327F2C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F2D8 size=64
    let mut pc: u32 = 0x8327F2D8;
    'dispatch: loop {
        match pc {
            0x8327F2D8 => {
    //   block [0x8327F2D8..0x8327F318)
	// 8327F2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F2E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F2E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F2E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F2EC: 388BF190  addi r4, r11, -0xe70
	ctx.r[4].s64 = ctx.r[11].s64 + -3696;
	// 8327F2F0: 386ADE08  addi r3, r10, -0x21f8
	ctx.r[3].s64 = ctx.r[10].s64 + -8696;
	// 8327F2F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F2F8: 4AFADBD9  bl 0x8222ced0
	ctx.lr = 0x8327F2FC;
	sub_8222CED0(ctx, base);
	// 8327F2FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F300: 38690B38  addi r3, r9, 0xb38
	ctx.r[3].s64 = ctx.r[9].s64 + 2872;
	// 8327F304: 4BA2AC1D  bl 0x82ca9f20
	ctx.lr = 0x8327F308;
	sub_82CA9F20(ctx, base);
	// 8327F308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F318 size=64
    let mut pc: u32 = 0x8327F318;
    'dispatch: loop {
        match pc {
            0x8327F318 => {
    //   block [0x8327F318..0x8327F358)
	// 8327F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F324: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F32C: 388BF1AC  addi r4, r11, -0xe54
	ctx.r[4].s64 = ctx.r[11].s64 + -3668;
	// 8327F330: 386ADE0C  addi r3, r10, -0x21f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8692;
	// 8327F334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F338: 4AFADB99  bl 0x8222ced0
	ctx.lr = 0x8327F33C;
	sub_8222CED0(ctx, base);
	// 8327F33C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F340: 38690B48  addi r3, r9, 0xb48
	ctx.r[3].s64 = ctx.r[9].s64 + 2888;
	// 8327F344: 4BA2ABDD  bl 0x82ca9f20
	ctx.lr = 0x8327F348;
	sub_82CA9F20(ctx, base);
	// 8327F348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F358 size=64
    let mut pc: u32 = 0x8327F358;
    'dispatch: loop {
        match pc {
            0x8327F358 => {
    //   block [0x8327F358..0x8327F398)
	// 8327F358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F364: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F36C: 388BF1C0  addi r4, r11, -0xe40
	ctx.r[4].s64 = ctx.r[11].s64 + -3648;
	// 8327F370: 386ADE10  addi r3, r10, -0x21f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8688;
	// 8327F374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F378: 4AFADB59  bl 0x8222ced0
	ctx.lr = 0x8327F37C;
	sub_8222CED0(ctx, base);
	// 8327F37C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F380: 38690B58  addi r3, r9, 0xb58
	ctx.r[3].s64 = ctx.r[9].s64 + 2904;
	// 8327F384: 4BA2AB9D  bl 0x82ca9f20
	ctx.lr = 0x8327F388;
	sub_82CA9F20(ctx, base);
	// 8327F388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F398 size=64
    let mut pc: u32 = 0x8327F398;
    'dispatch: loop {
        match pc {
            0x8327F398 => {
    //   block [0x8327F398..0x8327F3D8)
	// 8327F398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F3A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F3AC: 388BF1D4  addi r4, r11, -0xe2c
	ctx.r[4].s64 = ctx.r[11].s64 + -3628;
	// 8327F3B0: 386ADE14  addi r3, r10, -0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8684;
	// 8327F3B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F3B8: 4AFADB19  bl 0x8222ced0
	ctx.lr = 0x8327F3BC;
	sub_8222CED0(ctx, base);
	// 8327F3BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F3C0: 38690B68  addi r3, r9, 0xb68
	ctx.r[3].s64 = ctx.r[9].s64 + 2920;
	// 8327F3C4: 4BA2AB5D  bl 0x82ca9f20
	ctx.lr = 0x8327F3C8;
	sub_82CA9F20(ctx, base);
	// 8327F3C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F3D8 size=64
    let mut pc: u32 = 0x8327F3D8;
    'dispatch: loop {
        match pc {
            0x8327F3D8 => {
    //   block [0x8327F3D8..0x8327F418)
	// 8327F3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F3E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F3E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F3E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F3EC: 388BF1EC  addi r4, r11, -0xe14
	ctx.r[4].s64 = ctx.r[11].s64 + -3604;
	// 8327F3F0: 386ADE18  addi r3, r10, -0x21e8
	ctx.r[3].s64 = ctx.r[10].s64 + -8680;
	// 8327F3F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F3F8: 4AFADAD9  bl 0x8222ced0
	ctx.lr = 0x8327F3FC;
	sub_8222CED0(ctx, base);
	// 8327F3FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F400: 38690B78  addi r3, r9, 0xb78
	ctx.r[3].s64 = ctx.r[9].s64 + 2936;
	// 8327F404: 4BA2AB1D  bl 0x82ca9f20
	ctx.lr = 0x8327F408;
	sub_82CA9F20(ctx, base);
	// 8327F408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F418 size=64
    let mut pc: u32 = 0x8327F418;
    'dispatch: loop {
        match pc {
            0x8327F418 => {
    //   block [0x8327F418..0x8327F458)
	// 8327F418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F42C: 388BF204  addi r4, r11, -0xdfc
	ctx.r[4].s64 = ctx.r[11].s64 + -3580;
	// 8327F430: 386ADE1C  addi r3, r10, -0x21e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8676;
	// 8327F434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F438: 4AFADA99  bl 0x8222ced0
	ctx.lr = 0x8327F43C;
	sub_8222CED0(ctx, base);
	// 8327F43C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F440: 38690B88  addi r3, r9, 0xb88
	ctx.r[3].s64 = ctx.r[9].s64 + 2952;
	// 8327F444: 4BA2AADD  bl 0x82ca9f20
	ctx.lr = 0x8327F448;
	sub_82CA9F20(ctx, base);
	// 8327F448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F458 size=64
    let mut pc: u32 = 0x8327F458;
    'dispatch: loop {
        match pc {
            0x8327F458 => {
    //   block [0x8327F458..0x8327F498)
	// 8327F458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F46C: 388BF21C  addi r4, r11, -0xde4
	ctx.r[4].s64 = ctx.r[11].s64 + -3556;
	// 8327F470: 386ADE20  addi r3, r10, -0x21e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8672;
	// 8327F474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F478: 4AFADA59  bl 0x8222ced0
	ctx.lr = 0x8327F47C;
	sub_8222CED0(ctx, base);
	// 8327F47C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F480: 38690B98  addi r3, r9, 0xb98
	ctx.r[3].s64 = ctx.r[9].s64 + 2968;
	// 8327F484: 4BA2AA9D  bl 0x82ca9f20
	ctx.lr = 0x8327F488;
	sub_82CA9F20(ctx, base);
	// 8327F488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F498 size=64
    let mut pc: u32 = 0x8327F498;
    'dispatch: loop {
        match pc {
            0x8327F498 => {
    //   block [0x8327F498..0x8327F4D8)
	// 8327F498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F4A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F4A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F4AC: 388BF238  addi r4, r11, -0xdc8
	ctx.r[4].s64 = ctx.r[11].s64 + -3528;
	// 8327F4B0: 386ADE24  addi r3, r10, -0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8668;
	// 8327F4B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F4B8: 4AFADA19  bl 0x8222ced0
	ctx.lr = 0x8327F4BC;
	sub_8222CED0(ctx, base);
	// 8327F4BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F4C0: 38690BA8  addi r3, r9, 0xba8
	ctx.r[3].s64 = ctx.r[9].s64 + 2984;
	// 8327F4C4: 4BA2AA5D  bl 0x82ca9f20
	ctx.lr = 0x8327F4C8;
	sub_82CA9F20(ctx, base);
	// 8327F4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F4D8 size=64
    let mut pc: u32 = 0x8327F4D8;
    'dispatch: loop {
        match pc {
            0x8327F4D8 => {
    //   block [0x8327F4D8..0x8327F518)
	// 8327F4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F4E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F4E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F4EC: 388BF24C  addi r4, r11, -0xdb4
	ctx.r[4].s64 = ctx.r[11].s64 + -3508;
	// 8327F4F0: 386ADE28  addi r3, r10, -0x21d8
	ctx.r[3].s64 = ctx.r[10].s64 + -8664;
	// 8327F4F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F4F8: 4AFAD9D9  bl 0x8222ced0
	ctx.lr = 0x8327F4FC;
	sub_8222CED0(ctx, base);
	// 8327F4FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F500: 38690BB8  addi r3, r9, 0xbb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3000;
	// 8327F504: 4BA2AA1D  bl 0x82ca9f20
	ctx.lr = 0x8327F508;
	sub_82CA9F20(ctx, base);
	// 8327F508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F518 size=64
    let mut pc: u32 = 0x8327F518;
    'dispatch: loop {
        match pc {
            0x8327F518 => {
    //   block [0x8327F518..0x8327F558)
	// 8327F518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F524: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F52C: 388BF260  addi r4, r11, -0xda0
	ctx.r[4].s64 = ctx.r[11].s64 + -3488;
	// 8327F530: 386ADE2C  addi r3, r10, -0x21d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8660;
	// 8327F534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F538: 4AFAD999  bl 0x8222ced0
	ctx.lr = 0x8327F53C;
	sub_8222CED0(ctx, base);
	// 8327F53C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F540: 38690BC8  addi r3, r9, 0xbc8
	ctx.r[3].s64 = ctx.r[9].s64 + 3016;
	// 8327F544: 4BA2A9DD  bl 0x82ca9f20
	ctx.lr = 0x8327F548;
	sub_82CA9F20(ctx, base);
	// 8327F548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F558 size=64
    let mut pc: u32 = 0x8327F558;
    'dispatch: loop {
        match pc {
            0x8327F558 => {
    //   block [0x8327F558..0x8327F598)
	// 8327F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F564: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F56C: 388BF274  addi r4, r11, -0xd8c
	ctx.r[4].s64 = ctx.r[11].s64 + -3468;
	// 8327F570: 386ADE30  addi r3, r10, -0x21d0
	ctx.r[3].s64 = ctx.r[10].s64 + -8656;
	// 8327F574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F578: 4AFAD959  bl 0x8222ced0
	ctx.lr = 0x8327F57C;
	sub_8222CED0(ctx, base);
	// 8327F57C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F580: 38690BD8  addi r3, r9, 0xbd8
	ctx.r[3].s64 = ctx.r[9].s64 + 3032;
	// 8327F584: 4BA2A99D  bl 0x82ca9f20
	ctx.lr = 0x8327F588;
	sub_82CA9F20(ctx, base);
	// 8327F588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F598 size=64
    let mut pc: u32 = 0x8327F598;
    'dispatch: loop {
        match pc {
            0x8327F598 => {
    //   block [0x8327F598..0x8327F5D8)
	// 8327F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F5A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F5A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F5AC: 388BF290  addi r4, r11, -0xd70
	ctx.r[4].s64 = ctx.r[11].s64 + -3440;
	// 8327F5B0: 386ADE34  addi r3, r10, -0x21cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8652;
	// 8327F5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F5B8: 4AFAD919  bl 0x8222ced0
	ctx.lr = 0x8327F5BC;
	sub_8222CED0(ctx, base);
	// 8327F5BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F5C0: 38690BE8  addi r3, r9, 0xbe8
	ctx.r[3].s64 = ctx.r[9].s64 + 3048;
	// 8327F5C4: 4BA2A95D  bl 0x82ca9f20
	ctx.lr = 0x8327F5C8;
	sub_82CA9F20(ctx, base);
	// 8327F5C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F5D8 size=64
    let mut pc: u32 = 0x8327F5D8;
    'dispatch: loop {
        match pc {
            0x8327F5D8 => {
    //   block [0x8327F5D8..0x8327F618)
	// 8327F5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F5E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F5EC: 388BF2AC  addi r4, r11, -0xd54
	ctx.r[4].s64 = ctx.r[11].s64 + -3412;
	// 8327F5F0: 386ADE38  addi r3, r10, -0x21c8
	ctx.r[3].s64 = ctx.r[10].s64 + -8648;
	// 8327F5F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F5F8: 4AFAD8D9  bl 0x8222ced0
	ctx.lr = 0x8327F5FC;
	sub_8222CED0(ctx, base);
	// 8327F5FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F600: 38690BF8  addi r3, r9, 0xbf8
	ctx.r[3].s64 = ctx.r[9].s64 + 3064;
	// 8327F604: 4BA2A91D  bl 0x82ca9f20
	ctx.lr = 0x8327F608;
	sub_82CA9F20(ctx, base);
	// 8327F608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F618 size=64
    let mut pc: u32 = 0x8327F618;
    'dispatch: loop {
        match pc {
            0x8327F618 => {
    //   block [0x8327F618..0x8327F658)
	// 8327F618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F624: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F62C: 388BF2CC  addi r4, r11, -0xd34
	ctx.r[4].s64 = ctx.r[11].s64 + -3380;
	// 8327F630: 386ADE3C  addi r3, r10, -0x21c4
	ctx.r[3].s64 = ctx.r[10].s64 + -8644;
	// 8327F634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F638: 4AFAD899  bl 0x8222ced0
	ctx.lr = 0x8327F63C;
	sub_8222CED0(ctx, base);
	// 8327F63C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F640: 38690C08  addi r3, r9, 0xc08
	ctx.r[3].s64 = ctx.r[9].s64 + 3080;
	// 8327F644: 4BA2A8DD  bl 0x82ca9f20
	ctx.lr = 0x8327F648;
	sub_82CA9F20(ctx, base);
	// 8327F648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F658 size=64
    let mut pc: u32 = 0x8327F658;
    'dispatch: loop {
        match pc {
            0x8327F658 => {
    //   block [0x8327F658..0x8327F698)
	// 8327F658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F664: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F66C: 388BF2E4  addi r4, r11, -0xd1c
	ctx.r[4].s64 = ctx.r[11].s64 + -3356;
	// 8327F670: 386ADE40  addi r3, r10, -0x21c0
	ctx.r[3].s64 = ctx.r[10].s64 + -8640;
	// 8327F674: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F678: 4AFAD859  bl 0x8222ced0
	ctx.lr = 0x8327F67C;
	sub_8222CED0(ctx, base);
	// 8327F67C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F680: 38690C18  addi r3, r9, 0xc18
	ctx.r[3].s64 = ctx.r[9].s64 + 3096;
	// 8327F684: 4BA2A89D  bl 0x82ca9f20
	ctx.lr = 0x8327F688;
	sub_82CA9F20(ctx, base);
	// 8327F688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F698 size=64
    let mut pc: u32 = 0x8327F698;
    'dispatch: loop {
        match pc {
            0x8327F698 => {
    //   block [0x8327F698..0x8327F6D8)
	// 8327F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F6A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F6A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F6A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F6AC: 388BF2FC  addi r4, r11, -0xd04
	ctx.r[4].s64 = ctx.r[11].s64 + -3332;
	// 8327F6B0: 386ADE44  addi r3, r10, -0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8636;
	// 8327F6B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F6B8: 4AFAD819  bl 0x8222ced0
	ctx.lr = 0x8327F6BC;
	sub_8222CED0(ctx, base);
	// 8327F6BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F6C0: 38690C28  addi r3, r9, 0xc28
	ctx.r[3].s64 = ctx.r[9].s64 + 3112;
	// 8327F6C4: 4BA2A85D  bl 0x82ca9f20
	ctx.lr = 0x8327F6C8;
	sub_82CA9F20(ctx, base);
	// 8327F6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F6D8 size=64
    let mut pc: u32 = 0x8327F6D8;
    'dispatch: loop {
        match pc {
            0x8327F6D8 => {
    //   block [0x8327F6D8..0x8327F718)
	// 8327F6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F6E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F6E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F6E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F6EC: 388BF314  addi r4, r11, -0xcec
	ctx.r[4].s64 = ctx.r[11].s64 + -3308;
	// 8327F6F0: 386ADE48  addi r3, r10, -0x21b8
	ctx.r[3].s64 = ctx.r[10].s64 + -8632;
	// 8327F6F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F6F8: 4AFAD7D9  bl 0x8222ced0
	ctx.lr = 0x8327F6FC;
	sub_8222CED0(ctx, base);
	// 8327F6FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F700: 38690C38  addi r3, r9, 0xc38
	ctx.r[3].s64 = ctx.r[9].s64 + 3128;
	// 8327F704: 4BA2A81D  bl 0x82ca9f20
	ctx.lr = 0x8327F708;
	sub_82CA9F20(ctx, base);
	// 8327F708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F718 size=64
    let mut pc: u32 = 0x8327F718;
    'dispatch: loop {
        match pc {
            0x8327F718 => {
    //   block [0x8327F718..0x8327F758)
	// 8327F718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F724: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F72C: 388BF330  addi r4, r11, -0xcd0
	ctx.r[4].s64 = ctx.r[11].s64 + -3280;
	// 8327F730: 386ADE4C  addi r3, r10, -0x21b4
	ctx.r[3].s64 = ctx.r[10].s64 + -8628;
	// 8327F734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F738: 4AFAD799  bl 0x8222ced0
	ctx.lr = 0x8327F73C;
	sub_8222CED0(ctx, base);
	// 8327F73C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F740: 38690C48  addi r3, r9, 0xc48
	ctx.r[3].s64 = ctx.r[9].s64 + 3144;
	// 8327F744: 4BA2A7DD  bl 0x82ca9f20
	ctx.lr = 0x8327F748;
	sub_82CA9F20(ctx, base);
	// 8327F748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F758 size=64
    let mut pc: u32 = 0x8327F758;
    'dispatch: loop {
        match pc {
            0x8327F758 => {
    //   block [0x8327F758..0x8327F798)
	// 8327F758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F764: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F76C: 388BF34C  addi r4, r11, -0xcb4
	ctx.r[4].s64 = ctx.r[11].s64 + -3252;
	// 8327F770: 386ADE50  addi r3, r10, -0x21b0
	ctx.r[3].s64 = ctx.r[10].s64 + -8624;
	// 8327F774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F778: 4AFAD759  bl 0x8222ced0
	ctx.lr = 0x8327F77C;
	sub_8222CED0(ctx, base);
	// 8327F77C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F780: 38690C58  addi r3, r9, 0xc58
	ctx.r[3].s64 = ctx.r[9].s64 + 3160;
	// 8327F784: 4BA2A79D  bl 0x82ca9f20
	ctx.lr = 0x8327F788;
	sub_82CA9F20(ctx, base);
	// 8327F788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F798 size=64
    let mut pc: u32 = 0x8327F798;
    'dispatch: loop {
        match pc {
            0x8327F798 => {
    //   block [0x8327F798..0x8327F7D8)
	// 8327F798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F7A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F7A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F7AC: 388BF36C  addi r4, r11, -0xc94
	ctx.r[4].s64 = ctx.r[11].s64 + -3220;
	// 8327F7B0: 386ADE54  addi r3, r10, -0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8620;
	// 8327F7B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F7B8: 4AFAD719  bl 0x8222ced0
	ctx.lr = 0x8327F7BC;
	sub_8222CED0(ctx, base);
	// 8327F7BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F7C0: 38690C68  addi r3, r9, 0xc68
	ctx.r[3].s64 = ctx.r[9].s64 + 3176;
	// 8327F7C4: 4BA2A75D  bl 0x82ca9f20
	ctx.lr = 0x8327F7C8;
	sub_82CA9F20(ctx, base);
	// 8327F7C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F7D8 size=64
    let mut pc: u32 = 0x8327F7D8;
    'dispatch: loop {
        match pc {
            0x8327F7D8 => {
    //   block [0x8327F7D8..0x8327F818)
	// 8327F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F7E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F7E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F7EC: 388BF388  addi r4, r11, -0xc78
	ctx.r[4].s64 = ctx.r[11].s64 + -3192;
	// 8327F7F0: 386ADE58  addi r3, r10, -0x21a8
	ctx.r[3].s64 = ctx.r[10].s64 + -8616;
	// 8327F7F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F7F8: 4AFAD6D9  bl 0x8222ced0
	ctx.lr = 0x8327F7FC;
	sub_8222CED0(ctx, base);
	// 8327F7FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F800: 38690C78  addi r3, r9, 0xc78
	ctx.r[3].s64 = ctx.r[9].s64 + 3192;
	// 8327F804: 4BA2A71D  bl 0x82ca9f20
	ctx.lr = 0x8327F808;
	sub_82CA9F20(ctx, base);
	// 8327F808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F818 size=64
    let mut pc: u32 = 0x8327F818;
    'dispatch: loop {
        match pc {
            0x8327F818 => {
    //   block [0x8327F818..0x8327F858)
	// 8327F818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F824: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F828: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F82C: 388BF3A0  addi r4, r11, -0xc60
	ctx.r[4].s64 = ctx.r[11].s64 + -3168;
	// 8327F830: 386ADE5C  addi r3, r10, -0x21a4
	ctx.r[3].s64 = ctx.r[10].s64 + -8612;
	// 8327F834: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F838: 4AFAD699  bl 0x8222ced0
	ctx.lr = 0x8327F83C;
	sub_8222CED0(ctx, base);
	// 8327F83C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F840: 38690C88  addi r3, r9, 0xc88
	ctx.r[3].s64 = ctx.r[9].s64 + 3208;
	// 8327F844: 4BA2A6DD  bl 0x82ca9f20
	ctx.lr = 0x8327F848;
	sub_82CA9F20(ctx, base);
	// 8327F848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F858 size=64
    let mut pc: u32 = 0x8327F858;
    'dispatch: loop {
        match pc {
            0x8327F858 => {
    //   block [0x8327F858..0x8327F898)
	// 8327F858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F864: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F868: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F86C: 388BF3B4  addi r4, r11, -0xc4c
	ctx.r[4].s64 = ctx.r[11].s64 + -3148;
	// 8327F870: 386ADE60  addi r3, r10, -0x21a0
	ctx.r[3].s64 = ctx.r[10].s64 + -8608;
	// 8327F874: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F878: 4AFAD659  bl 0x8222ced0
	ctx.lr = 0x8327F87C;
	sub_8222CED0(ctx, base);
	// 8327F87C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F880: 38690C98  addi r3, r9, 0xc98
	ctx.r[3].s64 = ctx.r[9].s64 + 3224;
	// 8327F884: 4BA2A69D  bl 0x82ca9f20
	ctx.lr = 0x8327F888;
	sub_82CA9F20(ctx, base);
	// 8327F888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F898 size=64
    let mut pc: u32 = 0x8327F898;
    'dispatch: loop {
        match pc {
            0x8327F898 => {
    //   block [0x8327F898..0x8327F8D8)
	// 8327F898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F8A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F8A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F8A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F8AC: 388BF3CC  addi r4, r11, -0xc34
	ctx.r[4].s64 = ctx.r[11].s64 + -3124;
	// 8327F8B0: 386ADE64  addi r3, r10, -0x219c
	ctx.r[3].s64 = ctx.r[10].s64 + -8604;
	// 8327F8B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F8B8: 4AFAD619  bl 0x8222ced0
	ctx.lr = 0x8327F8BC;
	sub_8222CED0(ctx, base);
	// 8327F8BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F8C0: 38690CA8  addi r3, r9, 0xca8
	ctx.r[3].s64 = ctx.r[9].s64 + 3240;
	// 8327F8C4: 4BA2A65D  bl 0x82ca9f20
	ctx.lr = 0x8327F8C8;
	sub_82CA9F20(ctx, base);
	// 8327F8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F8D8 size=64
    let mut pc: u32 = 0x8327F8D8;
    'dispatch: loop {
        match pc {
            0x8327F8D8 => {
    //   block [0x8327F8D8..0x8327F918)
	// 8327F8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F8E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F8E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F8EC: 388BF3E8  addi r4, r11, -0xc18
	ctx.r[4].s64 = ctx.r[11].s64 + -3096;
	// 8327F8F0: 386ADE68  addi r3, r10, -0x2198
	ctx.r[3].s64 = ctx.r[10].s64 + -8600;
	// 8327F8F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F8F8: 4AFAD5D9  bl 0x8222ced0
	ctx.lr = 0x8327F8FC;
	sub_8222CED0(ctx, base);
	// 8327F8FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F900: 38690CB8  addi r3, r9, 0xcb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3256;
	// 8327F904: 4BA2A61D  bl 0x82ca9f20
	ctx.lr = 0x8327F908;
	sub_82CA9F20(ctx, base);
	// 8327F908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F918 size=64
    let mut pc: u32 = 0x8327F918;
    'dispatch: loop {
        match pc {
            0x8327F918 => {
    //   block [0x8327F918..0x8327F958)
	// 8327F918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F924: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F92C: 388BF408  addi r4, r11, -0xbf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3064;
	// 8327F930: 386ADE6C  addi r3, r10, -0x2194
	ctx.r[3].s64 = ctx.r[10].s64 + -8596;
	// 8327F934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F938: 4AFAD599  bl 0x8222ced0
	ctx.lr = 0x8327F93C;
	sub_8222CED0(ctx, base);
	// 8327F93C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F940: 38690CC8  addi r3, r9, 0xcc8
	ctx.r[3].s64 = ctx.r[9].s64 + 3272;
	// 8327F944: 4BA2A5DD  bl 0x82ca9f20
	ctx.lr = 0x8327F948;
	sub_82CA9F20(ctx, base);
	// 8327F948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F958 size=64
    let mut pc: u32 = 0x8327F958;
    'dispatch: loop {
        match pc {
            0x8327F958 => {
    //   block [0x8327F958..0x8327F998)
	// 8327F958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F964: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F96C: 388BF428  addi r4, r11, -0xbd8
	ctx.r[4].s64 = ctx.r[11].s64 + -3032;
	// 8327F970: 386ADE70  addi r3, r10, -0x2190
	ctx.r[3].s64 = ctx.r[10].s64 + -8592;
	// 8327F974: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F978: 4AFAD559  bl 0x8222ced0
	ctx.lr = 0x8327F97C;
	sub_8222CED0(ctx, base);
	// 8327F97C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F980: 38690CD8  addi r3, r9, 0xcd8
	ctx.r[3].s64 = ctx.r[9].s64 + 3288;
	// 8327F984: 4BA2A59D  bl 0x82ca9f20
	ctx.lr = 0x8327F988;
	sub_82CA9F20(ctx, base);
	// 8327F988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F998 size=64
    let mut pc: u32 = 0x8327F998;
    'dispatch: loop {
        match pc {
            0x8327F998 => {
    //   block [0x8327F998..0x8327F9D8)
	// 8327F998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F9A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F9A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F9A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F9AC: 388BF440  addi r4, r11, -0xbc0
	ctx.r[4].s64 = ctx.r[11].s64 + -3008;
	// 8327F9B0: 386ADE74  addi r3, r10, -0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + -8588;
	// 8327F9B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F9B8: 4AFAD519  bl 0x8222ced0
	ctx.lr = 0x8327F9BC;
	sub_8222CED0(ctx, base);
	// 8327F9BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F9C0: 38690CE8  addi r3, r9, 0xce8
	ctx.r[3].s64 = ctx.r[9].s64 + 3304;
	// 8327F9C4: 4BA2A55D  bl 0x82ca9f20
	ctx.lr = 0x8327F9C8;
	sub_82CA9F20(ctx, base);
	// 8327F9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F9D8 size=64
    let mut pc: u32 = 0x8327F9D8;
    'dispatch: loop {
        match pc {
            0x8327F9D8 => {
    //   block [0x8327F9D8..0x8327FA18)
	// 8327F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F9E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F9E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F9E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F9EC: 388BF454  addi r4, r11, -0xbac
	ctx.r[4].s64 = ctx.r[11].s64 + -2988;
	// 8327F9F0: 386ADE78  addi r3, r10, -0x2188
	ctx.r[3].s64 = ctx.r[10].s64 + -8584;
	// 8327F9F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F9F8: 4AFAD4D9  bl 0x8222ced0
	ctx.lr = 0x8327F9FC;
	sub_8222CED0(ctx, base);
	// 8327F9FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FA00: 38690CF8  addi r3, r9, 0xcf8
	ctx.r[3].s64 = ctx.r[9].s64 + 3320;
	// 8327FA04: 4BA2A51D  bl 0x82ca9f20
	ctx.lr = 0x8327FA08;
	sub_82CA9F20(ctx, base);
	// 8327FA08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FA18 size=64
    let mut pc: u32 = 0x8327FA18;
    'dispatch: loop {
        match pc {
            0x8327FA18 => {
    //   block [0x8327FA18..0x8327FA58)
	// 8327FA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FA20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FA24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FA28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FA2C: 388BF470  addi r4, r11, -0xb90
	ctx.r[4].s64 = ctx.r[11].s64 + -2960;
	// 8327FA30: 386ADE7C  addi r3, r10, -0x2184
	ctx.r[3].s64 = ctx.r[10].s64 + -8580;
	// 8327FA34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FA38: 4AFAD499  bl 0x8222ced0
	ctx.lr = 0x8327FA3C;
	sub_8222CED0(ctx, base);
	// 8327FA3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FA40: 38690D08  addi r3, r9, 0xd08
	ctx.r[3].s64 = ctx.r[9].s64 + 3336;
	// 8327FA44: 4BA2A4DD  bl 0x82ca9f20
	ctx.lr = 0x8327FA48;
	sub_82CA9F20(ctx, base);
	// 8327FA48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FA58 size=64
    let mut pc: u32 = 0x8327FA58;
    'dispatch: loop {
        match pc {
            0x8327FA58 => {
    //   block [0x8327FA58..0x8327FA98)
	// 8327FA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FA60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FA64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FA68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FA6C: 388BF488  addi r4, r11, -0xb78
	ctx.r[4].s64 = ctx.r[11].s64 + -2936;
	// 8327FA70: 386ADE80  addi r3, r10, -0x2180
	ctx.r[3].s64 = ctx.r[10].s64 + -8576;
	// 8327FA74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FA78: 4AFAD459  bl 0x8222ced0
	ctx.lr = 0x8327FA7C;
	sub_8222CED0(ctx, base);
	// 8327FA7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FA80: 38690D18  addi r3, r9, 0xd18
	ctx.r[3].s64 = ctx.r[9].s64 + 3352;
	// 8327FA84: 4BA2A49D  bl 0x82ca9f20
	ctx.lr = 0x8327FA88;
	sub_82CA9F20(ctx, base);
	// 8327FA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FA98 size=64
    let mut pc: u32 = 0x8327FA98;
    'dispatch: loop {
        match pc {
            0x8327FA98 => {
    //   block [0x8327FA98..0x8327FAD8)
	// 8327FA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FAA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FAA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FAAC: 388BF4A0  addi r4, r11, -0xb60
	ctx.r[4].s64 = ctx.r[11].s64 + -2912;
	// 8327FAB0: 386ADE84  addi r3, r10, -0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + -8572;
	// 8327FAB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FAB8: 4AFAD419  bl 0x8222ced0
	ctx.lr = 0x8327FABC;
	sub_8222CED0(ctx, base);
	// 8327FABC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FAC0: 38690D28  addi r3, r9, 0xd28
	ctx.r[3].s64 = ctx.r[9].s64 + 3368;
	// 8327FAC4: 4BA2A45D  bl 0x82ca9f20
	ctx.lr = 0x8327FAC8;
	sub_82CA9F20(ctx, base);
	// 8327FAC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FAD8 size=64
    let mut pc: u32 = 0x8327FAD8;
    'dispatch: loop {
        match pc {
            0x8327FAD8 => {
    //   block [0x8327FAD8..0x8327FB18)
	// 8327FAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FAE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FAE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FAE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FAEC: 388BF4BC  addi r4, r11, -0xb44
	ctx.r[4].s64 = ctx.r[11].s64 + -2884;
	// 8327FAF0: 386ADE88  addi r3, r10, -0x2178
	ctx.r[3].s64 = ctx.r[10].s64 + -8568;
	// 8327FAF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FAF8: 4AFAD3D9  bl 0x8222ced0
	ctx.lr = 0x8327FAFC;
	sub_8222CED0(ctx, base);
	// 8327FAFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FB00: 38690D38  addi r3, r9, 0xd38
	ctx.r[3].s64 = ctx.r[9].s64 + 3384;
	// 8327FB04: 4BA2A41D  bl 0x82ca9f20
	ctx.lr = 0x8327FB08;
	sub_82CA9F20(ctx, base);
	// 8327FB08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FB18 size=64
    let mut pc: u32 = 0x8327FB18;
    'dispatch: loop {
        match pc {
            0x8327FB18 => {
    //   block [0x8327FB18..0x8327FB58)
	// 8327FB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FB20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FB24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FB2C: 388BF4D8  addi r4, r11, -0xb28
	ctx.r[4].s64 = ctx.r[11].s64 + -2856;
	// 8327FB30: 386ADE8C  addi r3, r10, -0x2174
	ctx.r[3].s64 = ctx.r[10].s64 + -8564;
	// 8327FB34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FB38: 4AFAD399  bl 0x8222ced0
	ctx.lr = 0x8327FB3C;
	sub_8222CED0(ctx, base);
	// 8327FB3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FB40: 38690D48  addi r3, r9, 0xd48
	ctx.r[3].s64 = ctx.r[9].s64 + 3400;
	// 8327FB44: 4BA2A3DD  bl 0x82ca9f20
	ctx.lr = 0x8327FB48;
	sub_82CA9F20(ctx, base);
	// 8327FB48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FB58 size=64
    let mut pc: u32 = 0x8327FB58;
    'dispatch: loop {
        match pc {
            0x8327FB58 => {
    //   block [0x8327FB58..0x8327FB98)
	// 8327FB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FB60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FB64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FB68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FB6C: 388BF4F0  addi r4, r11, -0xb10
	ctx.r[4].s64 = ctx.r[11].s64 + -2832;
	// 8327FB70: 386ADE90  addi r3, r10, -0x2170
	ctx.r[3].s64 = ctx.r[10].s64 + -8560;
	// 8327FB74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FB78: 4AFAD359  bl 0x8222ced0
	ctx.lr = 0x8327FB7C;
	sub_8222CED0(ctx, base);
	// 8327FB7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FB80: 38690D58  addi r3, r9, 0xd58
	ctx.r[3].s64 = ctx.r[9].s64 + 3416;
	// 8327FB84: 4BA2A39D  bl 0x82ca9f20
	ctx.lr = 0x8327FB88;
	sub_82CA9F20(ctx, base);
	// 8327FB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FB98 size=64
    let mut pc: u32 = 0x8327FB98;
    'dispatch: loop {
        match pc {
            0x8327FB98 => {
    //   block [0x8327FB98..0x8327FBD8)
	// 8327FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FBA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FBA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FBAC: 388BF510  addi r4, r11, -0xaf0
	ctx.r[4].s64 = ctx.r[11].s64 + -2800;
	// 8327FBB0: 386ADE94  addi r3, r10, -0x216c
	ctx.r[3].s64 = ctx.r[10].s64 + -8556;
	// 8327FBB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FBB8: 4AFAD319  bl 0x8222ced0
	ctx.lr = 0x8327FBBC;
	sub_8222CED0(ctx, base);
	// 8327FBBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FBC0: 38690D68  addi r3, r9, 0xd68
	ctx.r[3].s64 = ctx.r[9].s64 + 3432;
	// 8327FBC4: 4BA2A35D  bl 0x82ca9f20
	ctx.lr = 0x8327FBC8;
	sub_82CA9F20(ctx, base);
	// 8327FBC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FBD8 size=64
    let mut pc: u32 = 0x8327FBD8;
    'dispatch: loop {
        match pc {
            0x8327FBD8 => {
    //   block [0x8327FBD8..0x8327FC18)
	// 8327FBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FBE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FBE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FBE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FBEC: 388BF52C  addi r4, r11, -0xad4
	ctx.r[4].s64 = ctx.r[11].s64 + -2772;
	// 8327FBF0: 386ADE98  addi r3, r10, -0x2168
	ctx.r[3].s64 = ctx.r[10].s64 + -8552;
	// 8327FBF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FBF8: 4AFAD2D9  bl 0x8222ced0
	ctx.lr = 0x8327FBFC;
	sub_8222CED0(ctx, base);
	// 8327FBFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FC00: 38690D78  addi r3, r9, 0xd78
	ctx.r[3].s64 = ctx.r[9].s64 + 3448;
	// 8327FC04: 4BA2A31D  bl 0x82ca9f20
	ctx.lr = 0x8327FC08;
	sub_82CA9F20(ctx, base);
	// 8327FC08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FC18 size=64
    let mut pc: u32 = 0x8327FC18;
    'dispatch: loop {
        match pc {
            0x8327FC18 => {
    //   block [0x8327FC18..0x8327FC58)
	// 8327FC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FC24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FC2C: 388BF548  addi r4, r11, -0xab8
	ctx.r[4].s64 = ctx.r[11].s64 + -2744;
	// 8327FC30: 386ADE9C  addi r3, r10, -0x2164
	ctx.r[3].s64 = ctx.r[10].s64 + -8548;
	// 8327FC34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FC38: 4AFAD299  bl 0x8222ced0
	ctx.lr = 0x8327FC3C;
	sub_8222CED0(ctx, base);
	// 8327FC3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FC40: 38690D88  addi r3, r9, 0xd88
	ctx.r[3].s64 = ctx.r[9].s64 + 3464;
	// 8327FC44: 4BA2A2DD  bl 0x82ca9f20
	ctx.lr = 0x8327FC48;
	sub_82CA9F20(ctx, base);
	// 8327FC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FC58 size=64
    let mut pc: u32 = 0x8327FC58;
    'dispatch: loop {
        match pc {
            0x8327FC58 => {
    //   block [0x8327FC58..0x8327FC98)
	// 8327FC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FC64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FC68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FC6C: 388BF568  addi r4, r11, -0xa98
	ctx.r[4].s64 = ctx.r[11].s64 + -2712;
	// 8327FC70: 386ADEA0  addi r3, r10, -0x2160
	ctx.r[3].s64 = ctx.r[10].s64 + -8544;
	// 8327FC74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FC78: 4AFAD259  bl 0x8222ced0
	ctx.lr = 0x8327FC7C;
	sub_8222CED0(ctx, base);
	// 8327FC7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FC80: 38690D98  addi r3, r9, 0xd98
	ctx.r[3].s64 = ctx.r[9].s64 + 3480;
	// 8327FC84: 4BA2A29D  bl 0x82ca9f20
	ctx.lr = 0x8327FC88;
	sub_82CA9F20(ctx, base);
	// 8327FC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FC98 size=64
    let mut pc: u32 = 0x8327FC98;
    'dispatch: loop {
        match pc {
            0x8327FC98 => {
    //   block [0x8327FC98..0x8327FCD8)
	// 8327FC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FCA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FCA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FCA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FCAC: 388BF588  addi r4, r11, -0xa78
	ctx.r[4].s64 = ctx.r[11].s64 + -2680;
	// 8327FCB0: 386ADEA4  addi r3, r10, -0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + -8540;
	// 8327FCB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FCB8: 4AFAD219  bl 0x8222ced0
	ctx.lr = 0x8327FCBC;
	sub_8222CED0(ctx, base);
	// 8327FCBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FCC0: 38690DA8  addi r3, r9, 0xda8
	ctx.r[3].s64 = ctx.r[9].s64 + 3496;
	// 8327FCC4: 4BA2A25D  bl 0x82ca9f20
	ctx.lr = 0x8327FCC8;
	sub_82CA9F20(ctx, base);
	// 8327FCC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FCD8 size=64
    let mut pc: u32 = 0x8327FCD8;
    'dispatch: loop {
        match pc {
            0x8327FCD8 => {
    //   block [0x8327FCD8..0x8327FD18)
	// 8327FCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FCE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FCE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FCE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FCEC: 388BF5A4  addi r4, r11, -0xa5c
	ctx.r[4].s64 = ctx.r[11].s64 + -2652;
	// 8327FCF0: 386ADEA8  addi r3, r10, -0x2158
	ctx.r[3].s64 = ctx.r[10].s64 + -8536;
	// 8327FCF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FCF8: 4AFAD1D9  bl 0x8222ced0
	ctx.lr = 0x8327FCFC;
	sub_8222CED0(ctx, base);
	// 8327FCFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FD00: 38690DB8  addi r3, r9, 0xdb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3512;
	// 8327FD04: 4BA2A21D  bl 0x82ca9f20
	ctx.lr = 0x8327FD08;
	sub_82CA9F20(ctx, base);
	// 8327FD08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FD18 size=64
    let mut pc: u32 = 0x8327FD18;
    'dispatch: loop {
        match pc {
            0x8327FD18 => {
    //   block [0x8327FD18..0x8327FD58)
	// 8327FD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FD20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FD24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FD28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FD2C: 388BF5C0  addi r4, r11, -0xa40
	ctx.r[4].s64 = ctx.r[11].s64 + -2624;
	// 8327FD30: 386ADEAC  addi r3, r10, -0x2154
	ctx.r[3].s64 = ctx.r[10].s64 + -8532;
	// 8327FD34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FD38: 4AFAD199  bl 0x8222ced0
	ctx.lr = 0x8327FD3C;
	sub_8222CED0(ctx, base);
	// 8327FD3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FD40: 38690DC8  addi r3, r9, 0xdc8
	ctx.r[3].s64 = ctx.r[9].s64 + 3528;
	// 8327FD44: 4BA2A1DD  bl 0x82ca9f20
	ctx.lr = 0x8327FD48;
	sub_82CA9F20(ctx, base);
	// 8327FD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FD58 size=64
    let mut pc: u32 = 0x8327FD58;
    'dispatch: loop {
        match pc {
            0x8327FD58 => {
    //   block [0x8327FD58..0x8327FD98)
	// 8327FD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FD64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FD68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FD6C: 388BF5DC  addi r4, r11, -0xa24
	ctx.r[4].s64 = ctx.r[11].s64 + -2596;
	// 8327FD70: 386ADEB0  addi r3, r10, -0x2150
	ctx.r[3].s64 = ctx.r[10].s64 + -8528;
	// 8327FD74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FD78: 4AFAD159  bl 0x8222ced0
	ctx.lr = 0x8327FD7C;
	sub_8222CED0(ctx, base);
	// 8327FD7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FD80: 38690DD8  addi r3, r9, 0xdd8
	ctx.r[3].s64 = ctx.r[9].s64 + 3544;
	// 8327FD84: 4BA2A19D  bl 0x82ca9f20
	ctx.lr = 0x8327FD88;
	sub_82CA9F20(ctx, base);
	// 8327FD88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FD98 size=64
    let mut pc: u32 = 0x8327FD98;
    'dispatch: loop {
        match pc {
            0x8327FD98 => {
    //   block [0x8327FD98..0x8327FDD8)
	// 8327FD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FDA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FDAC: 388BF5FC  addi r4, r11, -0xa04
	ctx.r[4].s64 = ctx.r[11].s64 + -2564;
	// 8327FDB0: 386ADEB4  addi r3, r10, -0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + -8524;
	// 8327FDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FDB8: 4AFAD119  bl 0x8222ced0
	ctx.lr = 0x8327FDBC;
	sub_8222CED0(ctx, base);
	// 8327FDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FDC0: 38690DE8  addi r3, r9, 0xde8
	ctx.r[3].s64 = ctx.r[9].s64 + 3560;
	// 8327FDC4: 4BA2A15D  bl 0x82ca9f20
	ctx.lr = 0x8327FDC8;
	sub_82CA9F20(ctx, base);
	// 8327FDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FDD8 size=64
    let mut pc: u32 = 0x8327FDD8;
    'dispatch: loop {
        match pc {
            0x8327FDD8 => {
    //   block [0x8327FDD8..0x8327FE18)
	// 8327FDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FDE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FDEC: 388BF620  addi r4, r11, -0x9e0
	ctx.r[4].s64 = ctx.r[11].s64 + -2528;
	// 8327FDF0: 386ADEB8  addi r3, r10, -0x2148
	ctx.r[3].s64 = ctx.r[10].s64 + -8520;
	// 8327FDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FDF8: 4AFAD0D9  bl 0x8222ced0
	ctx.lr = 0x8327FDFC;
	sub_8222CED0(ctx, base);
	// 8327FDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FE00: 38690DF8  addi r3, r9, 0xdf8
	ctx.r[3].s64 = ctx.r[9].s64 + 3576;
	// 8327FE04: 4BA2A11D  bl 0x82ca9f20
	ctx.lr = 0x8327FE08;
	sub_82CA9F20(ctx, base);
	// 8327FE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FE18 size=64
    let mut pc: u32 = 0x8327FE18;
    'dispatch: loop {
        match pc {
            0x8327FE18 => {
    //   block [0x8327FE18..0x8327FE58)
	// 8327FE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FE24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FE2C: 388BF634  addi r4, r11, -0x9cc
	ctx.r[4].s64 = ctx.r[11].s64 + -2508;
	// 8327FE30: 386ADEBC  addi r3, r10, -0x2144
	ctx.r[3].s64 = ctx.r[10].s64 + -8516;
	// 8327FE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FE38: 4AFAD099  bl 0x8222ced0
	ctx.lr = 0x8327FE3C;
	sub_8222CED0(ctx, base);
	// 8327FE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FE40: 38690E08  addi r3, r9, 0xe08
	ctx.r[3].s64 = ctx.r[9].s64 + 3592;
	// 8327FE44: 4BA2A0DD  bl 0x82ca9f20
	ctx.lr = 0x8327FE48;
	sub_82CA9F20(ctx, base);
	// 8327FE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FE58 size=64
    let mut pc: u32 = 0x8327FE58;
    'dispatch: loop {
        match pc {
            0x8327FE58 => {
    //   block [0x8327FE58..0x8327FE98)
	// 8327FE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FE64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FE6C: 388BF648  addi r4, r11, -0x9b8
	ctx.r[4].s64 = ctx.r[11].s64 + -2488;
	// 8327FE70: 386ADEC0  addi r3, r10, -0x2140
	ctx.r[3].s64 = ctx.r[10].s64 + -8512;
	// 8327FE74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FE78: 4AFAD059  bl 0x8222ced0
	ctx.lr = 0x8327FE7C;
	sub_8222CED0(ctx, base);
	// 8327FE7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FE80: 38690E18  addi r3, r9, 0xe18
	ctx.r[3].s64 = ctx.r[9].s64 + 3608;
	// 8327FE84: 4BA2A09D  bl 0x82ca9f20
	ctx.lr = 0x8327FE88;
	sub_82CA9F20(ctx, base);
	// 8327FE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FE98 size=64
    let mut pc: u32 = 0x8327FE98;
    'dispatch: loop {
        match pc {
            0x8327FE98 => {
    //   block [0x8327FE98..0x8327FED8)
	// 8327FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FEA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FEA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FEAC: 388BF65C  addi r4, r11, -0x9a4
	ctx.r[4].s64 = ctx.r[11].s64 + -2468;
	// 8327FEB0: 386ADEC4  addi r3, r10, -0x213c
	ctx.r[3].s64 = ctx.r[10].s64 + -8508;
	// 8327FEB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FEB8: 4AFAD019  bl 0x8222ced0
	ctx.lr = 0x8327FEBC;
	sub_8222CED0(ctx, base);
	// 8327FEBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FEC0: 38690E28  addi r3, r9, 0xe28
	ctx.r[3].s64 = ctx.r[9].s64 + 3624;
	// 8327FEC4: 4BA2A05D  bl 0x82ca9f20
	ctx.lr = 0x8327FEC8;
	sub_82CA9F20(ctx, base);
	// 8327FEC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FED8 size=64
    let mut pc: u32 = 0x8327FED8;
    'dispatch: loop {
        match pc {
            0x8327FED8 => {
    //   block [0x8327FED8..0x8327FF18)
	// 8327FED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FEE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FEE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FEE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FEEC: 388BF670  addi r4, r11, -0x990
	ctx.r[4].s64 = ctx.r[11].s64 + -2448;
	// 8327FEF0: 386ADEC8  addi r3, r10, -0x2138
	ctx.r[3].s64 = ctx.r[10].s64 + -8504;
	// 8327FEF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FEF8: 4AFACFD9  bl 0x8222ced0
	ctx.lr = 0x8327FEFC;
	sub_8222CED0(ctx, base);
	// 8327FEFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FF00: 38690E38  addi r3, r9, 0xe38
	ctx.r[3].s64 = ctx.r[9].s64 + 3640;
	// 8327FF04: 4BA2A01D  bl 0x82ca9f20
	ctx.lr = 0x8327FF08;
	sub_82CA9F20(ctx, base);
	// 8327FF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FF18 size=64
    let mut pc: u32 = 0x8327FF18;
    'dispatch: loop {
        match pc {
            0x8327FF18 => {
    //   block [0x8327FF18..0x8327FF58)
	// 8327FF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FF24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FF2C: 388BF688  addi r4, r11, -0x978
	ctx.r[4].s64 = ctx.r[11].s64 + -2424;
	// 8327FF30: 386ADECC  addi r3, r10, -0x2134
	ctx.r[3].s64 = ctx.r[10].s64 + -8500;
	// 8327FF34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FF38: 4AFACF99  bl 0x8222ced0
	ctx.lr = 0x8327FF3C;
	sub_8222CED0(ctx, base);
	// 8327FF3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FF40: 38690E48  addi r3, r9, 0xe48
	ctx.r[3].s64 = ctx.r[9].s64 + 3656;
	// 8327FF44: 4BA29FDD  bl 0x82ca9f20
	ctx.lr = 0x8327FF48;
	sub_82CA9F20(ctx, base);
	// 8327FF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FF58 size=64
    let mut pc: u32 = 0x8327FF58;
    'dispatch: loop {
        match pc {
            0x8327FF58 => {
    //   block [0x8327FF58..0x8327FF98)
	// 8327FF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FF64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FF68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FF6C: 388BF6A0  addi r4, r11, -0x960
	ctx.r[4].s64 = ctx.r[11].s64 + -2400;
	// 8327FF70: 386ADED0  addi r3, r10, -0x2130
	ctx.r[3].s64 = ctx.r[10].s64 + -8496;
	// 8327FF74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FF78: 4AFACF59  bl 0x8222ced0
	ctx.lr = 0x8327FF7C;
	sub_8222CED0(ctx, base);
	// 8327FF7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FF80: 38690E58  addi r3, r9, 0xe58
	ctx.r[3].s64 = ctx.r[9].s64 + 3672;
	// 8327FF84: 4BA29F9D  bl 0x82ca9f20
	ctx.lr = 0x8327FF88;
	sub_82CA9F20(ctx, base);
	// 8327FF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FF98 size=64
    let mut pc: u32 = 0x8327FF98;
    'dispatch: loop {
        match pc {
            0x8327FF98 => {
    //   block [0x8327FF98..0x8327FFD8)
	// 8327FF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FFA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FFA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FFAC: 388BF6B8  addi r4, r11, -0x948
	ctx.r[4].s64 = ctx.r[11].s64 + -2376;
	// 8327FFB0: 386ADED4  addi r3, r10, -0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + -8492;
	// 8327FFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FFB8: 4AFACF19  bl 0x8222ced0
	ctx.lr = 0x8327FFBC;
	sub_8222CED0(ctx, base);
	// 8327FFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FFC0: 38690E68  addi r3, r9, 0xe68
	ctx.r[3].s64 = ctx.r[9].s64 + 3688;
	// 8327FFC4: 4BA29F5D  bl 0x82ca9f20
	ctx.lr = 0x8327FFC8;
	sub_82CA9F20(ctx, base);
	// 8327FFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FFD8 size=64
    let mut pc: u32 = 0x8327FFD8;
    'dispatch: loop {
        match pc {
            0x8327FFD8 => {
    //   block [0x8327FFD8..0x83280018)
	// 8327FFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FFE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FFEC: 388BF6D0  addi r4, r11, -0x930
	ctx.r[4].s64 = ctx.r[11].s64 + -2352;
	// 8327FFF0: 386ADED8  addi r3, r10, -0x2128
	ctx.r[3].s64 = ctx.r[10].s64 + -8488;
	// 8327FFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FFF8: 4AFACED9  bl 0x8222ced0
	ctx.lr = 0x8327FFFC;
	sub_8222CED0(ctx, base);
	// 8327FFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280000: 38690E78  addi r3, r9, 0xe78
	ctx.r[3].s64 = ctx.r[9].s64 + 3704;
	// 83280004: 4BA29F1D  bl 0x82ca9f20
	ctx.lr = 0x83280008;
	sub_82CA9F20(ctx, base);
	// 83280008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328000C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280018 size=64
    let mut pc: u32 = 0x83280018;
    'dispatch: loop {
        match pc {
            0x83280018 => {
    //   block [0x83280018..0x83280058)
	// 83280018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328001C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280024: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328002C: 388BF6E8  addi r4, r11, -0x918
	ctx.r[4].s64 = ctx.r[11].s64 + -2328;
	// 83280030: 386ADEDC  addi r3, r10, -0x2124
	ctx.r[3].s64 = ctx.r[10].s64 + -8484;
	// 83280034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280038: 4AFACE99  bl 0x8222ced0
	ctx.lr = 0x8328003C;
	sub_8222CED0(ctx, base);
	// 8328003C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280040: 38690E88  addi r3, r9, 0xe88
	ctx.r[3].s64 = ctx.r[9].s64 + 3720;
	// 83280044: 4BA29EDD  bl 0x82ca9f20
	ctx.lr = 0x83280048;
	sub_82CA9F20(ctx, base);
	// 83280048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328004C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280058 size=64
    let mut pc: u32 = 0x83280058;
    'dispatch: loop {
        match pc {
            0x83280058 => {
    //   block [0x83280058..0x83280098)
	// 83280058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328005C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280064: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328006C: 388BF704  addi r4, r11, -0x8fc
	ctx.r[4].s64 = ctx.r[11].s64 + -2300;
	// 83280070: 386ADEE0  addi r3, r10, -0x2120
	ctx.r[3].s64 = ctx.r[10].s64 + -8480;
	// 83280074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280078: 4AFACE59  bl 0x8222ced0
	ctx.lr = 0x8328007C;
	sub_8222CED0(ctx, base);
	// 8328007C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280080: 38690E98  addi r3, r9, 0xe98
	ctx.r[3].s64 = ctx.r[9].s64 + 3736;
	// 83280084: 4BA29E9D  bl 0x82ca9f20
	ctx.lr = 0x83280088;
	sub_82CA9F20(ctx, base);
	// 83280088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328008C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280098 size=64
    let mut pc: u32 = 0x83280098;
    'dispatch: loop {
        match pc {
            0x83280098 => {
    //   block [0x83280098..0x832800D8)
	// 83280098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328009C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832800A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832800A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832800A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832800AC: 388BF71C  addi r4, r11, -0x8e4
	ctx.r[4].s64 = ctx.r[11].s64 + -2276;
	// 832800B0: 386ADEE4  addi r3, r10, -0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + -8476;
	// 832800B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832800B8: 4AFACE19  bl 0x8222ced0
	ctx.lr = 0x832800BC;
	sub_8222CED0(ctx, base);
	// 832800BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832800C0: 38690EA8  addi r3, r9, 0xea8
	ctx.r[3].s64 = ctx.r[9].s64 + 3752;
	// 832800C4: 4BA29E5D  bl 0x82ca9f20
	ctx.lr = 0x832800C8;
	sub_82CA9F20(ctx, base);
	// 832800C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832800CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832800D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832800D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832800D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832800D8 size=64
    let mut pc: u32 = 0x832800D8;
    'dispatch: loop {
        match pc {
            0x832800D8 => {
    //   block [0x832800D8..0x83280118)
	// 832800D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832800DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832800E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832800E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832800E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832800EC: 388BF73C  addi r4, r11, -0x8c4
	ctx.r[4].s64 = ctx.r[11].s64 + -2244;
	// 832800F0: 386ADEE8  addi r3, r10, -0x2118
	ctx.r[3].s64 = ctx.r[10].s64 + -8472;
	// 832800F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832800F8: 4AFACDD9  bl 0x8222ced0
	ctx.lr = 0x832800FC;
	sub_8222CED0(ctx, base);
	// 832800FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280100: 38690EB8  addi r3, r9, 0xeb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3768;
	// 83280104: 4BA29E1D  bl 0x82ca9f20
	ctx.lr = 0x83280108;
	sub_82CA9F20(ctx, base);
	// 83280108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328010C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280118 size=64
    let mut pc: u32 = 0x83280118;
    'dispatch: loop {
        match pc {
            0x83280118 => {
    //   block [0x83280118..0x83280158)
	// 83280118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328011C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280124: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328012C: 388BF75C  addi r4, r11, -0x8a4
	ctx.r[4].s64 = ctx.r[11].s64 + -2212;
	// 83280130: 386ADEEC  addi r3, r10, -0x2114
	ctx.r[3].s64 = ctx.r[10].s64 + -8468;
	// 83280134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280138: 4AFACD99  bl 0x8222ced0
	ctx.lr = 0x8328013C;
	sub_8222CED0(ctx, base);
	// 8328013C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280140: 38690EC8  addi r3, r9, 0xec8
	ctx.r[3].s64 = ctx.r[9].s64 + 3784;
	// 83280144: 4BA29DDD  bl 0x82ca9f20
	ctx.lr = 0x83280148;
	sub_82CA9F20(ctx, base);
	// 83280148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280158 size=64
    let mut pc: u32 = 0x83280158;
    'dispatch: loop {
        match pc {
            0x83280158 => {
    //   block [0x83280158..0x83280198)
	// 83280158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328015C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280164: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328016C: 388BF774  addi r4, r11, -0x88c
	ctx.r[4].s64 = ctx.r[11].s64 + -2188;
	// 83280170: 386ADEF0  addi r3, r10, -0x2110
	ctx.r[3].s64 = ctx.r[10].s64 + -8464;
	// 83280174: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280178: 4AFACD59  bl 0x8222ced0
	ctx.lr = 0x8328017C;
	sub_8222CED0(ctx, base);
	// 8328017C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280180: 38690ED8  addi r3, r9, 0xed8
	ctx.r[3].s64 = ctx.r[9].s64 + 3800;
	// 83280184: 4BA29D9D  bl 0x82ca9f20
	ctx.lr = 0x83280188;
	sub_82CA9F20(ctx, base);
	// 83280188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328018C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280198 size=64
    let mut pc: u32 = 0x83280198;
    'dispatch: loop {
        match pc {
            0x83280198 => {
    //   block [0x83280198..0x832801D8)
	// 83280198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328019C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832801A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832801A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832801A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832801AC: 388BF78C  addi r4, r11, -0x874
	ctx.r[4].s64 = ctx.r[11].s64 + -2164;
	// 832801B0: 386ADEF4  addi r3, r10, -0x210c
	ctx.r[3].s64 = ctx.r[10].s64 + -8460;
	// 832801B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832801B8: 4AFACD19  bl 0x8222ced0
	ctx.lr = 0x832801BC;
	sub_8222CED0(ctx, base);
	// 832801BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832801C0: 38690EE8  addi r3, r9, 0xee8
	ctx.r[3].s64 = ctx.r[9].s64 + 3816;
	// 832801C4: 4BA29D5D  bl 0x82ca9f20
	ctx.lr = 0x832801C8;
	sub_82CA9F20(ctx, base);
	// 832801C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832801CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832801D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832801D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832801D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832801D8 size=64
    let mut pc: u32 = 0x832801D8;
    'dispatch: loop {
        match pc {
            0x832801D8 => {
    //   block [0x832801D8..0x83280218)
	// 832801D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832801DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832801E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832801E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832801E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832801EC: 388BF7A4  addi r4, r11, -0x85c
	ctx.r[4].s64 = ctx.r[11].s64 + -2140;
	// 832801F0: 386ADEF8  addi r3, r10, -0x2108
	ctx.r[3].s64 = ctx.r[10].s64 + -8456;
	// 832801F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832801F8: 4AFACCD9  bl 0x8222ced0
	ctx.lr = 0x832801FC;
	sub_8222CED0(ctx, base);
	// 832801FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280200: 38690EF8  addi r3, r9, 0xef8
	ctx.r[3].s64 = ctx.r[9].s64 + 3832;
	// 83280204: 4BA29D1D  bl 0x82ca9f20
	ctx.lr = 0x83280208;
	sub_82CA9F20(ctx, base);
	// 83280208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280218 size=64
    let mut pc: u32 = 0x83280218;
    'dispatch: loop {
        match pc {
            0x83280218 => {
    //   block [0x83280218..0x83280258)
	// 83280218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328021C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280224: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328022C: 388BF7CC  addi r4, r11, -0x834
	ctx.r[4].s64 = ctx.r[11].s64 + -2100;
	// 83280230: 386ADEFC  addi r3, r10, -0x2104
	ctx.r[3].s64 = ctx.r[10].s64 + -8452;
	// 83280234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280238: 4AFACC99  bl 0x8222ced0
	ctx.lr = 0x8328023C;
	sub_8222CED0(ctx, base);
	// 8328023C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280240: 38690F08  addi r3, r9, 0xf08
	ctx.r[3].s64 = ctx.r[9].s64 + 3848;
	// 83280244: 4BA29CDD  bl 0x82ca9f20
	ctx.lr = 0x83280248;
	sub_82CA9F20(ctx, base);
	// 83280248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328024C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280258 size=64
    let mut pc: u32 = 0x83280258;
    'dispatch: loop {
        match pc {
            0x83280258 => {
    //   block [0x83280258..0x83280298)
	// 83280258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280264: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328026C: 388BF7EC  addi r4, r11, -0x814
	ctx.r[4].s64 = ctx.r[11].s64 + -2068;
	// 83280270: 386ADF00  addi r3, r10, -0x2100
	ctx.r[3].s64 = ctx.r[10].s64 + -8448;
	// 83280274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280278: 4AFACC59  bl 0x8222ced0
	ctx.lr = 0x8328027C;
	sub_8222CED0(ctx, base);
	// 8328027C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280280: 38690F18  addi r3, r9, 0xf18
	ctx.r[3].s64 = ctx.r[9].s64 + 3864;
	// 83280284: 4BA29C9D  bl 0x82ca9f20
	ctx.lr = 0x83280288;
	sub_82CA9F20(ctx, base);
	// 83280288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328028C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280298 size=64
    let mut pc: u32 = 0x83280298;
    'dispatch: loop {
        match pc {
            0x83280298 => {
    //   block [0x83280298..0x832802D8)
	// 83280298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328029C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832802A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832802A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832802A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832802AC: 388BF808  addi r4, r11, -0x7f8
	ctx.r[4].s64 = ctx.r[11].s64 + -2040;
	// 832802B0: 386ADF04  addi r3, r10, -0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8444;
	// 832802B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832802B8: 4AFACC19  bl 0x8222ced0
	ctx.lr = 0x832802BC;
	sub_8222CED0(ctx, base);
	// 832802BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832802C0: 38690F28  addi r3, r9, 0xf28
	ctx.r[3].s64 = ctx.r[9].s64 + 3880;
	// 832802C4: 4BA29C5D  bl 0x82ca9f20
	ctx.lr = 0x832802C8;
	sub_82CA9F20(ctx, base);
	// 832802C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832802CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832802D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832802D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832802D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832802D8 size=64
    let mut pc: u32 = 0x832802D8;
    'dispatch: loop {
        match pc {
            0x832802D8 => {
    //   block [0x832802D8..0x83280318)
	// 832802D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832802DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832802E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832802E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832802E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832802EC: 388BF824  addi r4, r11, -0x7dc
	ctx.r[4].s64 = ctx.r[11].s64 + -2012;
	// 832802F0: 386ADF08  addi r3, r10, -0x20f8
	ctx.r[3].s64 = ctx.r[10].s64 + -8440;
	// 832802F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832802F8: 4AFACBD9  bl 0x8222ced0
	ctx.lr = 0x832802FC;
	sub_8222CED0(ctx, base);
	// 832802FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280300: 38690F38  addi r3, r9, 0xf38
	ctx.r[3].s64 = ctx.r[9].s64 + 3896;
	// 83280304: 4BA29C1D  bl 0x82ca9f20
	ctx.lr = 0x83280308;
	sub_82CA9F20(ctx, base);
	// 83280308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328030C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280318 size=64
    let mut pc: u32 = 0x83280318;
    'dispatch: loop {
        match pc {
            0x83280318 => {
    //   block [0x83280318..0x83280358)
	// 83280318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328031C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280324: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328032C: 388BF83C  addi r4, r11, -0x7c4
	ctx.r[4].s64 = ctx.r[11].s64 + -1988;
	// 83280330: 386ADF0C  addi r3, r10, -0x20f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8436;
	// 83280334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280338: 4AFACB99  bl 0x8222ced0
	ctx.lr = 0x8328033C;
	sub_8222CED0(ctx, base);
	// 8328033C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280340: 38690F48  addi r3, r9, 0xf48
	ctx.r[3].s64 = ctx.r[9].s64 + 3912;
	// 83280344: 4BA29BDD  bl 0x82ca9f20
	ctx.lr = 0x83280348;
	sub_82CA9F20(ctx, base);
	// 83280348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328034C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280358 size=64
    let mut pc: u32 = 0x83280358;
    'dispatch: loop {
        match pc {
            0x83280358 => {
    //   block [0x83280358..0x83280398)
	// 83280358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328035C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280364: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328036C: 388BF850  addi r4, r11, -0x7b0
	ctx.r[4].s64 = ctx.r[11].s64 + -1968;
	// 83280370: 386ADF10  addi r3, r10, -0x20f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8432;
	// 83280374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280378: 4AFACB59  bl 0x8222ced0
	ctx.lr = 0x8328037C;
	sub_8222CED0(ctx, base);
	// 8328037C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280380: 38690F58  addi r3, r9, 0xf58
	ctx.r[3].s64 = ctx.r[9].s64 + 3928;
	// 83280384: 4BA29B9D  bl 0x82ca9f20
	ctx.lr = 0x83280388;
	sub_82CA9F20(ctx, base);
	// 83280388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328038C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280398 size=64
    let mut pc: u32 = 0x83280398;
    'dispatch: loop {
        match pc {
            0x83280398 => {
    //   block [0x83280398..0x832803D8)
	// 83280398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328039C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832803A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832803A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832803A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832803AC: 388BF86C  addi r4, r11, -0x794
	ctx.r[4].s64 = ctx.r[11].s64 + -1940;
	// 832803B0: 386ADF14  addi r3, r10, -0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8428;
	// 832803B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832803B8: 4AFACB19  bl 0x8222ced0
	ctx.lr = 0x832803BC;
	sub_8222CED0(ctx, base);
	// 832803BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832803C0: 38690F68  addi r3, r9, 0xf68
	ctx.r[3].s64 = ctx.r[9].s64 + 3944;
	// 832803C4: 4BA29B5D  bl 0x82ca9f20
	ctx.lr = 0x832803C8;
	sub_82CA9F20(ctx, base);
	// 832803C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832803CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832803D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832803D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832803D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832803D8 size=64
    let mut pc: u32 = 0x832803D8;
    'dispatch: loop {
        match pc {
            0x832803D8 => {
    //   block [0x832803D8..0x83280418)
	// 832803D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832803DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832803E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832803E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832803E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832803EC: 388BF884  addi r4, r11, -0x77c
	ctx.r[4].s64 = ctx.r[11].s64 + -1916;
	// 832803F0: 386ADF18  addi r3, r10, -0x20e8
	ctx.r[3].s64 = ctx.r[10].s64 + -8424;
	// 832803F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832803F8: 4AFACAD9  bl 0x8222ced0
	ctx.lr = 0x832803FC;
	sub_8222CED0(ctx, base);
	// 832803FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280400: 38690F78  addi r3, r9, 0xf78
	ctx.r[3].s64 = ctx.r[9].s64 + 3960;
	// 83280404: 4BA29B1D  bl 0x82ca9f20
	ctx.lr = 0x83280408;
	sub_82CA9F20(ctx, base);
	// 83280408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328040C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280418 size=64
    let mut pc: u32 = 0x83280418;
    'dispatch: loop {
        match pc {
            0x83280418 => {
    //   block [0x83280418..0x83280458)
	// 83280418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328042C: 388BF8A0  addi r4, r11, -0x760
	ctx.r[4].s64 = ctx.r[11].s64 + -1888;
	// 83280430: 386ADF1C  addi r3, r10, -0x20e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8420;
	// 83280434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280438: 4AFACA99  bl 0x8222ced0
	ctx.lr = 0x8328043C;
	sub_8222CED0(ctx, base);
	// 8328043C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280440: 38690F88  addi r3, r9, 0xf88
	ctx.r[3].s64 = ctx.r[9].s64 + 3976;
	// 83280444: 4BA29ADD  bl 0x82ca9f20
	ctx.lr = 0x83280448;
	sub_82CA9F20(ctx, base);
	// 83280448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328044C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280458 size=64
    let mut pc: u32 = 0x83280458;
    'dispatch: loop {
        match pc {
            0x83280458 => {
    //   block [0x83280458..0x83280498)
	// 83280458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328045C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328046C: 388BF8B4  addi r4, r11, -0x74c
	ctx.r[4].s64 = ctx.r[11].s64 + -1868;
	// 83280470: 386ADF20  addi r3, r10, -0x20e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8416;
	// 83280474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280478: 4AFACA59  bl 0x8222ced0
	ctx.lr = 0x8328047C;
	sub_8222CED0(ctx, base);
	// 8328047C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280480: 38690F98  addi r3, r9, 0xf98
	ctx.r[3].s64 = ctx.r[9].s64 + 3992;
	// 83280484: 4BA29A9D  bl 0x82ca9f20
	ctx.lr = 0x83280488;
	sub_82CA9F20(ctx, base);
	// 83280488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328048C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280498 size=64
    let mut pc: u32 = 0x83280498;
    'dispatch: loop {
        match pc {
            0x83280498 => {
    //   block [0x83280498..0x832804D8)
	// 83280498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328049C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832804A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832804A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832804A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832804AC: 388BF8C0  addi r4, r11, -0x740
	ctx.r[4].s64 = ctx.r[11].s64 + -1856;
	// 832804B0: 386ADF24  addi r3, r10, -0x20dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8412;
	// 832804B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832804B8: 4AFACA19  bl 0x8222ced0
	ctx.lr = 0x832804BC;
	sub_8222CED0(ctx, base);
	// 832804BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832804C0: 38690FA8  addi r3, r9, 0xfa8
	ctx.r[3].s64 = ctx.r[9].s64 + 4008;
	// 832804C4: 4BA29A5D  bl 0x82ca9f20
	ctx.lr = 0x832804C8;
	sub_82CA9F20(ctx, base);
	// 832804C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832804CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832804D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832804D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832804D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832804D8 size=64
    let mut pc: u32 = 0x832804D8;
    'dispatch: loop {
        match pc {
            0x832804D8 => {
    //   block [0x832804D8..0x83280518)
	// 832804D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832804DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832804E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832804E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832804E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832804EC: 388BF8D4  addi r4, r11, -0x72c
	ctx.r[4].s64 = ctx.r[11].s64 + -1836;
	// 832804F0: 386ADF28  addi r3, r10, -0x20d8
	ctx.r[3].s64 = ctx.r[10].s64 + -8408;
	// 832804F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832804F8: 4AFAC9D9  bl 0x8222ced0
	ctx.lr = 0x832804FC;
	sub_8222CED0(ctx, base);
	// 832804FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280500: 38690FB8  addi r3, r9, 0xfb8
	ctx.r[3].s64 = ctx.r[9].s64 + 4024;
	// 83280504: 4BA29A1D  bl 0x82ca9f20
	ctx.lr = 0x83280508;
	sub_82CA9F20(ctx, base);
	// 83280508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328050C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280518 size=56
    let mut pc: u32 = 0x83280518;
    'dispatch: loop {
        match pc {
            0x83280518 => {
    //   block [0x83280518..0x83280550)
	// 83280518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328051C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280524: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328052C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83280530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280534: 4AF73825  bl 0x821f3d58
	ctx.lr = 0x83280538;
	sub_821F3D58(ctx, base);
	// 83280538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328053C: 906ADF2C  stw r3, -0x20d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8404 as u32), ctx.r[3].u32 ) };
	// 83280540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328054C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280550 size=56
    let mut pc: u32 = 0x83280550;
    'dispatch: loop {
        match pc {
            0x83280550 => {
    //   block [0x83280550..0x83280588)
	// 83280550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328055C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280560: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280564: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83280568: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328056C: 4AF737ED  bl 0x821f3d58
	ctx.lr = 0x83280570;
	sub_821F3D58(ctx, base);
	// 83280570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280574: 906ADF30  stw r3, -0x20d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8400 as u32), ctx.r[3].u32 ) };
	// 83280578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328057C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280588 size=56
    let mut pc: u32 = 0x83280588;
    'dispatch: loop {
        match pc {
            0x83280588 => {
    //   block [0x83280588..0x832805C0)
	// 83280588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328058C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280594: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280598: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328059C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832805A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832805A4: 4AF737B5  bl 0x821f3d58
	ctx.lr = 0x832805A8;
	sub_821F3D58(ctx, base);
	// 832805A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832805AC: 906ADF34  stw r3, -0x20cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8396 as u32), ctx.r[3].u32 ) };
	// 832805B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832805B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832805B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832805BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832805C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832805C0 size=56
    let mut pc: u32 = 0x832805C0;
    'dispatch: loop {
        match pc {
            0x832805C0 => {
    //   block [0x832805C0..0x832805F8)
	// 832805C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832805C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832805C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832805CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832805D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832805D4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832805D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832805DC: 4AF7377D  bl 0x821f3d58
	ctx.lr = 0x832805E0;
	sub_821F3D58(ctx, base);
	// 832805E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832805E4: 906ADF38  stw r3, -0x20c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8392 as u32), ctx.r[3].u32 ) };
	// 832805E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832805EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832805F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832805F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832805F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832805F8 size=56
    let mut pc: u32 = 0x832805F8;
    'dispatch: loop {
        match pc {
            0x832805F8 => {
    //   block [0x832805F8..0x83280630)
	// 832805F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832805FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280608: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328060C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83280610: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280614: 4AF73745  bl 0x821f3d58
	ctx.lr = 0x83280618;
	sub_821F3D58(ctx, base);
	// 83280618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328061C: 906ADF3C  stw r3, -0x20c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8388 as u32), ctx.r[3].u32 ) };
	// 83280620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328062C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280630 size=56
    let mut pc: u32 = 0x83280630;
    'dispatch: loop {
        match pc {
            0x83280630 => {
    //   block [0x83280630..0x83280668)
	// 83280630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328063C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280640: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280644: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83280648: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328064C: 4AF7370D  bl 0x821f3d58
	ctx.lr = 0x83280650;
	sub_821F3D58(ctx, base);
	// 83280650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280654: 906ADF40  stw r3, -0x20c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8384 as u32), ctx.r[3].u32 ) };
	// 83280658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328065C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280668 size=56
    let mut pc: u32 = 0x83280668;
    'dispatch: loop {
        match pc {
            0x83280668 => {
    //   block [0x83280668..0x832806A0)
	// 83280668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328066C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280674: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280678: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328067C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83280680: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280684: 4AF736D5  bl 0x821f3d58
	ctx.lr = 0x83280688;
	sub_821F3D58(ctx, base);
	// 83280688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328068C: 906ADF44  stw r3, -0x20bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8380 as u32), ctx.r[3].u32 ) };
	// 83280690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328069C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832806A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832806A0 size=56
    let mut pc: u32 = 0x832806A0;
    'dispatch: loop {
        match pc {
            0x832806A0 => {
    //   block [0x832806A0..0x832806D8)
	// 832806A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832806A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832806A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832806AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832806B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832806B4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832806B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832806BC: 4AF7369D  bl 0x821f3d58
	ctx.lr = 0x832806C0;
	sub_821F3D58(ctx, base);
	// 832806C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832806C4: 906ADF48  stw r3, -0x20b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8376 as u32), ctx.r[3].u32 ) };
	// 832806C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832806CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832806D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832806D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832806D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832806D8 size=56
    let mut pc: u32 = 0x832806D8;
    'dispatch: loop {
        match pc {
            0x832806D8 => {
    //   block [0x832806D8..0x83280710)
	// 832806D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832806DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832806E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832806E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832806E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832806EC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832806F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832806F4: 4AF73665  bl 0x821f3d58
	ctx.lr = 0x832806F8;
	sub_821F3D58(ctx, base);
	// 832806F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832806FC: 906ADF4C  stw r3, -0x20b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8372 as u32), ctx.r[3].u32 ) };
	// 83280700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328070C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280710 size=56
    let mut pc: u32 = 0x83280710;
    'dispatch: loop {
        match pc {
            0x83280710 => {
    //   block [0x83280710..0x83280748)
	// 83280710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328071C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280724: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83280728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328072C: 4AF7362D  bl 0x821f3d58
	ctx.lr = 0x83280730;
	sub_821F3D58(ctx, base);
	// 83280730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280734: 906ADF50  stw r3, -0x20b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8368 as u32), ctx.r[3].u32 ) };
	// 83280738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328073C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280748 size=56
    let mut pc: u32 = 0x83280748;
    'dispatch: loop {
        match pc {
            0x83280748 => {
    //   block [0x83280748..0x83280780)
	// 83280748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328074C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328075C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83280760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280764: 4AF735F5  bl 0x821f3d58
	ctx.lr = 0x83280768;
	sub_821F3D58(ctx, base);
	// 83280768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328076C: 906ADF54  stw r3, -0x20ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8364 as u32), ctx.r[3].u32 ) };
	// 83280770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328077C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280780 size=56
    let mut pc: u32 = 0x83280780;
    'dispatch: loop {
        match pc {
            0x83280780 => {
    //   block [0x83280780..0x832807B8)
	// 83280780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328078C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280794: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83280798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328079C: 4AF735BD  bl 0x821f3d58
	ctx.lr = 0x832807A0;
	sub_821F3D58(ctx, base);
	// 832807A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832807A4: 906ADF58  stw r3, -0x20a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8360 as u32), ctx.r[3].u32 ) };
	// 832807A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832807AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832807B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832807B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832807B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832807B8 size=56
    let mut pc: u32 = 0x832807B8;
    'dispatch: loop {
        match pc {
            0x832807B8 => {
    //   block [0x832807B8..0x832807F0)
	// 832807B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832807BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832807C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832807C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832807C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832807CC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832807D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832807D4: 4AF73585  bl 0x821f3d58
	ctx.lr = 0x832807D8;
	sub_821F3D58(ctx, base);
	// 832807D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832807DC: 906ADF5C  stw r3, -0x20a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8356 as u32), ctx.r[3].u32 ) };
	// 832807E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832807E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832807E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832807EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832807F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832807F0 size=56
    let mut pc: u32 = 0x832807F0;
    'dispatch: loop {
        match pc {
            0x832807F0 => {
    //   block [0x832807F0..0x83280828)
	// 832807F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832807F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832807F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832807FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280804: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83280808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328080C: 4AF7354D  bl 0x821f3d58
	ctx.lr = 0x83280810;
	sub_821F3D58(ctx, base);
	// 83280810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280814: 906ADF60  stw r3, -0x20a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8352 as u32), ctx.r[3].u32 ) };
	// 83280818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328081C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280828 size=56
    let mut pc: u32 = 0x83280828;
    'dispatch: loop {
        match pc {
            0x83280828 => {
    //   block [0x83280828..0x83280860)
	// 83280828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328082C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328083C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83280840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280844: 4AF73515  bl 0x821f3d58
	ctx.lr = 0x83280848;
	sub_821F3D58(ctx, base);
	// 83280848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328084C: 906ADF64  stw r3, -0x209c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8348 as u32), ctx.r[3].u32 ) };
	// 83280850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328085C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280860 size=56
    let mut pc: u32 = 0x83280860;
    'dispatch: loop {
        match pc {
            0x83280860 => {
    //   block [0x83280860..0x83280898)
	// 83280860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328086C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280874: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83280878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328087C: 4AF734DD  bl 0x821f3d58
	ctx.lr = 0x83280880;
	sub_821F3D58(ctx, base);
	// 83280880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280884: 906ADF68  stw r3, -0x2098(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8344 as u32), ctx.r[3].u32 ) };
	// 83280888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328088C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280898 size=56
    let mut pc: u32 = 0x83280898;
    'dispatch: loop {
        match pc {
            0x83280898 => {
    //   block [0x83280898..0x832808D0)
	// 83280898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328089C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832808A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832808A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832808A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832808AC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832808B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832808B4: 4AF734A5  bl 0x821f3d58
	ctx.lr = 0x832808B8;
	sub_821F3D58(ctx, base);
	// 832808B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832808BC: 906ADF6C  stw r3, -0x2094(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8340 as u32), ctx.r[3].u32 ) };
	// 832808C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832808C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832808C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832808CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832808D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832808D0 size=56
    let mut pc: u32 = 0x832808D0;
    'dispatch: loop {
        match pc {
            0x832808D0 => {
    //   block [0x832808D0..0x83280908)
	// 832808D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832808D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832808D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832808DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832808E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832808E4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832808E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832808EC: 4AF7346D  bl 0x821f3d58
	ctx.lr = 0x832808F0;
	sub_821F3D58(ctx, base);
	// 832808F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832808F4: 906ADF70  stw r3, -0x2090(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8336 as u32), ctx.r[3].u32 ) };
	// 832808F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832808FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280908 size=56
    let mut pc: u32 = 0x83280908;
    'dispatch: loop {
        match pc {
            0x83280908 => {
    //   block [0x83280908..0x83280940)
	// 83280908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328091C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83280920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280924: 4AF73435  bl 0x821f3d58
	ctx.lr = 0x83280928;
	sub_821F3D58(ctx, base);
	// 83280928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328092C: 906ADF74  stw r3, -0x208c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8332 as u32), ctx.r[3].u32 ) };
	// 83280930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328093C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280940 size=56
    let mut pc: u32 = 0x83280940;
    'dispatch: loop {
        match pc {
            0x83280940 => {
    //   block [0x83280940..0x83280978)
	// 83280940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328094C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280954: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83280958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328095C: 4AF733FD  bl 0x821f3d58
	ctx.lr = 0x83280960;
	sub_821F3D58(ctx, base);
	// 83280960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280964: 906ADF78  stw r3, -0x2088(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8328 as u32), ctx.r[3].u32 ) };
	// 83280968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328096C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280978 size=56
    let mut pc: u32 = 0x83280978;
    'dispatch: loop {
        match pc {
            0x83280978 => {
    //   block [0x83280978..0x832809B0)
	// 83280978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328098C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83280990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280994: 4AF733C5  bl 0x821f3d58
	ctx.lr = 0x83280998;
	sub_821F3D58(ctx, base);
	// 83280998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328099C: 906ADF7C  stw r3, -0x2084(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8324 as u32), ctx.r[3].u32 ) };
	// 832809A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832809A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832809A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832809AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832809B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832809B0 size=64
    let mut pc: u32 = 0x832809B0;
    'dispatch: loop {
        match pc {
            0x832809B0 => {
    //   block [0x832809B0..0x832809F0)
	// 832809B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832809B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832809B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832809BC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832809C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832809C4: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 832809C8: 386ADF80  addi r3, r10, -0x2080
	ctx.r[3].s64 = ctx.r[10].s64 + -8320;
	// 832809CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832809D0: 4AFAC501  bl 0x8222ced0
	ctx.lr = 0x832809D4;
	sub_8222CED0(ctx, base);
	// 832809D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832809D8: 38690FC8  addi r3, r9, 0xfc8
	ctx.r[3].s64 = ctx.r[9].s64 + 4040;
	// 832809DC: 4BA29545  bl 0x82ca9f20
	ctx.lr = 0x832809E0;
	sub_82CA9F20(ctx, base);
	// 832809E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832809E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832809E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832809EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832809F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832809F0 size=64
    let mut pc: u32 = 0x832809F0;
    'dispatch: loop {
        match pc {
            0x832809F0 => {
    //   block [0x832809F0..0x83280A30)
	// 832809F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832809F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832809F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832809FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280A04: 388BF934  addi r4, r11, -0x6cc
	ctx.r[4].s64 = ctx.r[11].s64 + -1740;
	// 83280A08: 386ADF84  addi r3, r10, -0x207c
	ctx.r[3].s64 = ctx.r[10].s64 + -8316;
	// 83280A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280A10: 4AFAC4C1  bl 0x8222ced0
	ctx.lr = 0x83280A14;
	sub_8222CED0(ctx, base);
	// 83280A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280A18: 38690FD8  addi r3, r9, 0xfd8
	ctx.r[3].s64 = ctx.r[9].s64 + 4056;
	// 83280A1C: 4BA29505  bl 0x82ca9f20
	ctx.lr = 0x83280A20;
	sub_82CA9F20(ctx, base);
	// 83280A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280A30 size=64
    let mut pc: u32 = 0x83280A30;
    'dispatch: loop {
        match pc {
            0x83280A30 => {
    //   block [0x83280A30..0x83280A70)
	// 83280A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280A3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280A44: 388BF968  addi r4, r11, -0x698
	ctx.r[4].s64 = ctx.r[11].s64 + -1688;
	// 83280A48: 386ADF88  addi r3, r10, -0x2078
	ctx.r[3].s64 = ctx.r[10].s64 + -8312;
	// 83280A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280A50: 4AFAC481  bl 0x8222ced0
	ctx.lr = 0x83280A54;
	sub_8222CED0(ctx, base);
	// 83280A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280A58: 38690FE8  addi r3, r9, 0xfe8
	ctx.r[3].s64 = ctx.r[9].s64 + 4072;
	// 83280A5C: 4BA294C5  bl 0x82ca9f20
	ctx.lr = 0x83280A60;
	sub_82CA9F20(ctx, base);
	// 83280A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280A70 size=64
    let mut pc: u32 = 0x83280A70;
    'dispatch: loop {
        match pc {
            0x83280A70 => {
    //   block [0x83280A70..0x83280AB0)
	// 83280A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280A7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280A84: 388BF9A0  addi r4, r11, -0x660
	ctx.r[4].s64 = ctx.r[11].s64 + -1632;
	// 83280A88: 386ADF8C  addi r3, r10, -0x2074
	ctx.r[3].s64 = ctx.r[10].s64 + -8308;
	// 83280A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280A90: 4AFAC441  bl 0x8222ced0
	ctx.lr = 0x83280A94;
	sub_8222CED0(ctx, base);
	// 83280A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280A98: 38690FF8  addi r3, r9, 0xff8
	ctx.r[3].s64 = ctx.r[9].s64 + 4088;
	// 83280A9C: 4BA29485  bl 0x82ca9f20
	ctx.lr = 0x83280AA0;
	sub_82CA9F20(ctx, base);
	// 83280AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280AB0 size=64
    let mut pc: u32 = 0x83280AB0;
    'dispatch: loop {
        match pc {
            0x83280AB0 => {
    //   block [0x83280AB0..0x83280AF0)
	// 83280AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280ABC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280AC4: 388BF9B4  addi r4, r11, -0x64c
	ctx.r[4].s64 = ctx.r[11].s64 + -1612;
	// 83280AC8: 386ADF90  addi r3, r10, -0x2070
	ctx.r[3].s64 = ctx.r[10].s64 + -8304;
	// 83280ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280AD0: 4AFAC401  bl 0x8222ced0
	ctx.lr = 0x83280AD4;
	sub_8222CED0(ctx, base);
	// 83280AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280AD8: 38691008  addi r3, r9, 0x1008
	ctx.r[3].s64 = ctx.r[9].s64 + 4104;
	// 83280ADC: 4BA29445  bl 0x82ca9f20
	ctx.lr = 0x83280AE0;
	sub_82CA9F20(ctx, base);
	// 83280AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280AF0 size=64
    let mut pc: u32 = 0x83280AF0;
    'dispatch: loop {
        match pc {
            0x83280AF0 => {
    //   block [0x83280AF0..0x83280B30)
	// 83280AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280AFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280B04: 388BF9C4  addi r4, r11, -0x63c
	ctx.r[4].s64 = ctx.r[11].s64 + -1596;
	// 83280B08: 386ADF94  addi r3, r10, -0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + -8300;
	// 83280B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280B10: 4AFAC3C1  bl 0x8222ced0
	ctx.lr = 0x83280B14;
	sub_8222CED0(ctx, base);
	// 83280B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280B18: 38691018  addi r3, r9, 0x1018
	ctx.r[3].s64 = ctx.r[9].s64 + 4120;
	// 83280B1C: 4BA29405  bl 0x82ca9f20
	ctx.lr = 0x83280B20;
	sub_82CA9F20(ctx, base);
	// 83280B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280B30 size=64
    let mut pc: u32 = 0x83280B30;
    'dispatch: loop {
        match pc {
            0x83280B30 => {
    //   block [0x83280B30..0x83280B70)
	// 83280B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280B3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280B44: 388BF9D4  addi r4, r11, -0x62c
	ctx.r[4].s64 = ctx.r[11].s64 + -1580;
	// 83280B48: 386ADF98  addi r3, r10, -0x2068
	ctx.r[3].s64 = ctx.r[10].s64 + -8296;
	// 83280B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280B50: 4AFAC381  bl 0x8222ced0
	ctx.lr = 0x83280B54;
	sub_8222CED0(ctx, base);
	// 83280B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280B58: 38691028  addi r3, r9, 0x1028
	ctx.r[3].s64 = ctx.r[9].s64 + 4136;
	// 83280B5C: 4BA293C5  bl 0x82ca9f20
	ctx.lr = 0x83280B60;
	sub_82CA9F20(ctx, base);
	// 83280B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280B70 size=56
    let mut pc: u32 = 0x83280B70;
    'dispatch: loop {
        match pc {
            0x83280B70 => {
    //   block [0x83280B70..0x83280BA8)
	// 83280B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280B80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280B84: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83280B88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280B8C: 4AF731CD  bl 0x821f3d58
	ctx.lr = 0x83280B90;
	sub_821F3D58(ctx, base);
	// 83280B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280B94: 906ADF9C  stw r3, -0x2064(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8292 as u32), ctx.r[3].u32 ) };
	// 83280B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280BA8 size=56
    let mut pc: u32 = 0x83280BA8;
    'dispatch: loop {
        match pc {
            0x83280BA8 => {
    //   block [0x83280BA8..0x83280BE0)
	// 83280BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280BB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280BB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280BBC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83280BC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280BC4: 4AF73195  bl 0x821f3d58
	ctx.lr = 0x83280BC8;
	sub_821F3D58(ctx, base);
	// 83280BC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280BCC: 906ADFA0  stw r3, -0x2060(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8288 as u32), ctx.r[3].u32 ) };
	// 83280BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280BE0 size=56
    let mut pc: u32 = 0x83280BE0;
    'dispatch: loop {
        match pc {
            0x83280BE0 => {
    //   block [0x83280BE0..0x83280C18)
	// 83280BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280BEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280BF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280BF4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83280BF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280BFC: 4AF7315D  bl 0x821f3d58
	ctx.lr = 0x83280C00;
	sub_821F3D58(ctx, base);
	// 83280C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280C04: 906ADFA4  stw r3, -0x205c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8284 as u32), ctx.r[3].u32 ) };
	// 83280C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280C18 size=56
    let mut pc: u32 = 0x83280C18;
    'dispatch: loop {
        match pc {
            0x83280C18 => {
    //   block [0x83280C18..0x83280C50)
	// 83280C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280C24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280C28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280C2C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83280C30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280C34: 4AF73125  bl 0x821f3d58
	ctx.lr = 0x83280C38;
	sub_821F3D58(ctx, base);
	// 83280C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280C3C: 906ADFA8  stw r3, -0x2058(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8280 as u32), ctx.r[3].u32 ) };
	// 83280C40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280C50 size=56
    let mut pc: u32 = 0x83280C50;
    'dispatch: loop {
        match pc {
            0x83280C50 => {
    //   block [0x83280C50..0x83280C88)
	// 83280C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280C5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280C60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280C64: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83280C68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280C6C: 4AF730ED  bl 0x821f3d58
	ctx.lr = 0x83280C70;
	sub_821F3D58(ctx, base);
	// 83280C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280C74: 906ADFAC  stw r3, -0x2054(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8276 as u32), ctx.r[3].u32 ) };
	// 83280C78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280C88 size=56
    let mut pc: u32 = 0x83280C88;
    'dispatch: loop {
        match pc {
            0x83280C88 => {
    //   block [0x83280C88..0x83280CC0)
	// 83280C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280C90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280C94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280C98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280C9C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83280CA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280CA4: 4AF730B5  bl 0x821f3d58
	ctx.lr = 0x83280CA8;
	sub_821F3D58(ctx, base);
	// 83280CA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280CAC: 906ADFB0  stw r3, -0x2050(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8272 as u32), ctx.r[3].u32 ) };
	// 83280CB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280CC0 size=56
    let mut pc: u32 = 0x83280CC0;
    'dispatch: loop {
        match pc {
            0x83280CC0 => {
    //   block [0x83280CC0..0x83280CF8)
	// 83280CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280CCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280CD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280CD4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83280CD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280CDC: 4AF7307D  bl 0x821f3d58
	ctx.lr = 0x83280CE0;
	sub_821F3D58(ctx, base);
	// 83280CE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280CE4: 906ADFB4  stw r3, -0x204c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8268 as u32), ctx.r[3].u32 ) };
	// 83280CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280CF8 size=56
    let mut pc: u32 = 0x83280CF8;
    'dispatch: loop {
        match pc {
            0x83280CF8 => {
    //   block [0x83280CF8..0x83280D30)
	// 83280CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280D04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280D08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280D0C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83280D10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280D14: 4AF73045  bl 0x821f3d58
	ctx.lr = 0x83280D18;
	sub_821F3D58(ctx, base);
	// 83280D18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280D1C: 906ADFB8  stw r3, -0x2048(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8264 as u32), ctx.r[3].u32 ) };
	// 83280D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280D30 size=56
    let mut pc: u32 = 0x83280D30;
    'dispatch: loop {
        match pc {
            0x83280D30 => {
    //   block [0x83280D30..0x83280D68)
	// 83280D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280D3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280D40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280D44: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83280D48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280D4C: 4AF7300D  bl 0x821f3d58
	ctx.lr = 0x83280D50;
	sub_821F3D58(ctx, base);
	// 83280D50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280D54: 906ADFBC  stw r3, -0x2044(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8260 as u32), ctx.r[3].u32 ) };
	// 83280D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280D68 size=56
    let mut pc: u32 = 0x83280D68;
    'dispatch: loop {
        match pc {
            0x83280D68 => {
    //   block [0x83280D68..0x83280DA0)
	// 83280D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280D74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280D78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280D7C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83280D80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280D84: 4AF72FD5  bl 0x821f3d58
	ctx.lr = 0x83280D88;
	sub_821F3D58(ctx, base);
	// 83280D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280D8C: 906ADFC0  stw r3, -0x2040(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8256 as u32), ctx.r[3].u32 ) };
	// 83280D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280DA0 size=56
    let mut pc: u32 = 0x83280DA0;
    'dispatch: loop {
        match pc {
            0x83280DA0 => {
    //   block [0x83280DA0..0x83280DD8)
	// 83280DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280DAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280DB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280DB4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83280DB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280DBC: 4AF72F9D  bl 0x821f3d58
	ctx.lr = 0x83280DC0;
	sub_821F3D58(ctx, base);
	// 83280DC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280DC4: 906ADFC4  stw r3, -0x203c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8252 as u32), ctx.r[3].u32 ) };
	// 83280DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280DD8 size=56
    let mut pc: u32 = 0x83280DD8;
    'dispatch: loop {
        match pc {
            0x83280DD8 => {
    //   block [0x83280DD8..0x83280E10)
	// 83280DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280DE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280DE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280DEC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83280DF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280DF4: 4AF72F65  bl 0x821f3d58
	ctx.lr = 0x83280DF8;
	sub_821F3D58(ctx, base);
	// 83280DF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280DFC: 906ADFC8  stw r3, -0x2038(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8248 as u32), ctx.r[3].u32 ) };
	// 83280E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280E10 size=56
    let mut pc: u32 = 0x83280E10;
    'dispatch: loop {
        match pc {
            0x83280E10 => {
    //   block [0x83280E10..0x83280E48)
	// 83280E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280E1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280E20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280E24: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83280E28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280E2C: 4AF72F2D  bl 0x821f3d58
	ctx.lr = 0x83280E30;
	sub_821F3D58(ctx, base);
	// 83280E30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280E34: 906ADFCC  stw r3, -0x2034(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8244 as u32), ctx.r[3].u32 ) };
	// 83280E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280E48 size=56
    let mut pc: u32 = 0x83280E48;
    'dispatch: loop {
        match pc {
            0x83280E48 => {
    //   block [0x83280E48..0x83280E80)
	// 83280E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280E54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280E58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280E5C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83280E60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280E64: 4AF72EF5  bl 0x821f3d58
	ctx.lr = 0x83280E68;
	sub_821F3D58(ctx, base);
	// 83280E68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280E6C: 906ADFD0  stw r3, -0x2030(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8240 as u32), ctx.r[3].u32 ) };
	// 83280E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280E80 size=56
    let mut pc: u32 = 0x83280E80;
    'dispatch: loop {
        match pc {
            0x83280E80 => {
    //   block [0x83280E80..0x83280EB8)
	// 83280E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280E8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280E90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280E94: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83280E98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280E9C: 4AF72EBD  bl 0x821f3d58
	ctx.lr = 0x83280EA0;
	sub_821F3D58(ctx, base);
	// 83280EA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280EA4: 906ADFD4  stw r3, -0x202c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8236 as u32), ctx.r[3].u32 ) };
	// 83280EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280EB8 size=56
    let mut pc: u32 = 0x83280EB8;
    'dispatch: loop {
        match pc {
            0x83280EB8 => {
    //   block [0x83280EB8..0x83280EF0)
	// 83280EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280EC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280EC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280ECC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83280ED0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280ED4: 4AF72E85  bl 0x821f3d58
	ctx.lr = 0x83280ED8;
	sub_821F3D58(ctx, base);
	// 83280ED8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280EDC: 906ADFD8  stw r3, -0x2028(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8232 as u32), ctx.r[3].u32 ) };
	// 83280EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280EF0 size=56
    let mut pc: u32 = 0x83280EF0;
    'dispatch: loop {
        match pc {
            0x83280EF0 => {
    //   block [0x83280EF0..0x83280F28)
	// 83280EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280EFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280F00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280F04: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83280F08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280F0C: 4AF72E4D  bl 0x821f3d58
	ctx.lr = 0x83280F10;
	sub_821F3D58(ctx, base);
	// 83280F10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280F14: 906ADFDC  stw r3, -0x2024(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8228 as u32), ctx.r[3].u32 ) };
	// 83280F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280F28 size=56
    let mut pc: u32 = 0x83280F28;
    'dispatch: loop {
        match pc {
            0x83280F28 => {
    //   block [0x83280F28..0x83280F60)
	// 83280F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280F34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280F38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280F3C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83280F40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280F44: 4AF72E15  bl 0x821f3d58
	ctx.lr = 0x83280F48;
	sub_821F3D58(ctx, base);
	// 83280F48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280F4C: 906ADFE0  stw r3, -0x2020(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8224 as u32), ctx.r[3].u32 ) };
	// 83280F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280F60 size=56
    let mut pc: u32 = 0x83280F60;
    'dispatch: loop {
        match pc {
            0x83280F60 => {
    //   block [0x83280F60..0x83280F98)
	// 83280F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280F6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280F70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280F74: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83280F78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280F7C: 4AF72DDD  bl 0x821f3d58
	ctx.lr = 0x83280F80;
	sub_821F3D58(ctx, base);
	// 83280F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280F84: 906ADFE4  stw r3, -0x201c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8220 as u32), ctx.r[3].u32 ) };
	// 83280F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280F98 size=56
    let mut pc: u32 = 0x83280F98;
    'dispatch: loop {
        match pc {
            0x83280F98 => {
    //   block [0x83280F98..0x83280FD0)
	// 83280F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280FA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280FA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280FAC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83280FB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280FB4: 4AF72DA5  bl 0x821f3d58
	ctx.lr = 0x83280FB8;
	sub_821F3D58(ctx, base);
	// 83280FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280FBC: 906ADFE8  stw r3, -0x2018(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8216 as u32), ctx.r[3].u32 ) };
	// 83280FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280FD0 size=56
    let mut pc: u32 = 0x83280FD0;
    'dispatch: loop {
        match pc {
            0x83280FD0 => {
    //   block [0x83280FD0..0x83281008)
	// 83280FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280FDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280FE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280FE4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83280FE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280FEC: 4AF72D6D  bl 0x821f3d58
	ctx.lr = 0x83280FF0;
	sub_821F3D58(ctx, base);
	// 83280FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280FF4: 906ADFEC  stw r3, -0x2014(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8212 as u32), ctx.r[3].u32 ) };
	// 83280FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281008 size=64
    let mut pc: u32 = 0x83281008;
    'dispatch: loop {
        match pc {
            0x83281008 => {
    //   block [0x83281008..0x83281048)
	// 83281008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328100C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281014: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328101C: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 83281020: 386ADFF0  addi r3, r10, -0x2010
	ctx.r[3].s64 = ctx.r[10].s64 + -8208;
	// 83281024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281028: 4AFABEA9  bl 0x8222ced0
	ctx.lr = 0x8328102C;
	sub_8222CED0(ctx, base);
	// 8328102C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281030: 38691038  addi r3, r9, 0x1038
	ctx.r[3].s64 = ctx.r[9].s64 + 4152;
	// 83281034: 4BA28EED  bl 0x82ca9f20
	ctx.lr = 0x83281038;
	sub_82CA9F20(ctx, base);
	// 83281038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328103C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281048 size=64
    let mut pc: u32 = 0x83281048;
    'dispatch: loop {
        match pc {
            0x83281048 => {
    //   block [0x83281048..0x83281088)
	// 83281048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328104C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281054: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281058: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328105C: 388BFC54  addi r4, r11, -0x3ac
	ctx.r[4].s64 = ctx.r[11].s64 + -940;
	// 83281060: 386ADFF4  addi r3, r10, -0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + -8204;
	// 83281064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281068: 4AFABE69  bl 0x8222ced0
	ctx.lr = 0x8328106C;
	sub_8222CED0(ctx, base);
	// 8328106C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281070: 38691048  addi r3, r9, 0x1048
	ctx.r[3].s64 = ctx.r[9].s64 + 4168;
	// 83281074: 4BA28EAD  bl 0x82ca9f20
	ctx.lr = 0x83281078;
	sub_82CA9F20(ctx, base);
	// 83281078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328107C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281088 size=64
    let mut pc: u32 = 0x83281088;
    'dispatch: loop {
        match pc {
            0x83281088 => {
    //   block [0x83281088..0x832810C8)
	// 83281088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328108C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281094: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281098: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328109C: 388BFC84  addi r4, r11, -0x37c
	ctx.r[4].s64 = ctx.r[11].s64 + -892;
	// 832810A0: 386ADFF8  addi r3, r10, -0x2008
	ctx.r[3].s64 = ctx.r[10].s64 + -8200;
	// 832810A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832810A8: 4AFABE29  bl 0x8222ced0
	ctx.lr = 0x832810AC;
	sub_8222CED0(ctx, base);
	// 832810AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832810B0: 38691058  addi r3, r9, 0x1058
	ctx.r[3].s64 = ctx.r[9].s64 + 4184;
	// 832810B4: 4BA28E6D  bl 0x82ca9f20
	ctx.lr = 0x832810B8;
	sub_82CA9F20(ctx, base);
	// 832810B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832810BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832810C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832810C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832810C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832810C8 size=64
    let mut pc: u32 = 0x832810C8;
    'dispatch: loop {
        match pc {
            0x832810C8 => {
    //   block [0x832810C8..0x83281108)
	// 832810C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832810CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832810D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832810D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832810D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832810DC: 388BFCBC  addi r4, r11, -0x344
	ctx.r[4].s64 = ctx.r[11].s64 + -836;
	// 832810E0: 386ADFFC  addi r3, r10, -0x2004
	ctx.r[3].s64 = ctx.r[10].s64 + -8196;
	// 832810E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832810E8: 4AFABDE9  bl 0x8222ced0
	ctx.lr = 0x832810EC;
	sub_8222CED0(ctx, base);
	// 832810EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832810F0: 38691068  addi r3, r9, 0x1068
	ctx.r[3].s64 = ctx.r[9].s64 + 4200;
	// 832810F4: 4BA28E2D  bl 0x82ca9f20
	ctx.lr = 0x832810F8;
	sub_82CA9F20(ctx, base);
	// 832810F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832810FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281108 size=64
    let mut pc: u32 = 0x83281108;
    'dispatch: loop {
        match pc {
            0x83281108 => {
    //   block [0x83281108..0x83281148)
	// 83281108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281114: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281118: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328111C: 388BFCE8  addi r4, r11, -0x318
	ctx.r[4].s64 = ctx.r[11].s64 + -792;
	// 83281120: 386AE000  addi r3, r10, -0x2000
	ctx.r[3].s64 = ctx.r[10].s64 + -8192;
	// 83281124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281128: 4AFABDA9  bl 0x8222ced0
	ctx.lr = 0x8328112C;
	sub_8222CED0(ctx, base);
	// 8328112C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281130: 38691078  addi r3, r9, 0x1078
	ctx.r[3].s64 = ctx.r[9].s64 + 4216;
	// 83281134: 4BA28DED  bl 0x82ca9f20
	ctx.lr = 0x83281138;
	sub_82CA9F20(ctx, base);
	// 83281138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328113C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281148 size=64
    let mut pc: u32 = 0x83281148;
    'dispatch: loop {
        match pc {
            0x83281148 => {
    //   block [0x83281148..0x83281188)
	// 83281148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281154: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328115C: 388BFD1C  addi r4, r11, -0x2e4
	ctx.r[4].s64 = ctx.r[11].s64 + -740;
	// 83281160: 386AE004  addi r3, r10, -0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -8188;
	// 83281164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281168: 4AFABD69  bl 0x8222ced0
	ctx.lr = 0x8328116C;
	sub_8222CED0(ctx, base);
	// 8328116C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281170: 38691088  addi r3, r9, 0x1088
	ctx.r[3].s64 = ctx.r[9].s64 + 4232;
	// 83281174: 4BA28DAD  bl 0x82ca9f20
	ctx.lr = 0x83281178;
	sub_82CA9F20(ctx, base);
	// 83281178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328117C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281188 size=64
    let mut pc: u32 = 0x83281188;
    'dispatch: loop {
        match pc {
            0x83281188 => {
    //   block [0x83281188..0x832811C8)
	// 83281188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328118C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281194: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328119C: 388BFD58  addi r4, r11, -0x2a8
	ctx.r[4].s64 = ctx.r[11].s64 + -680;
	// 832811A0: 386AE008  addi r3, r10, -0x1ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -8184;
	// 832811A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832811A8: 4AFABD29  bl 0x8222ced0
	ctx.lr = 0x832811AC;
	sub_8222CED0(ctx, base);
	// 832811AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832811B0: 38691098  addi r3, r9, 0x1098
	ctx.r[3].s64 = ctx.r[9].s64 + 4248;
	// 832811B4: 4BA28D6D  bl 0x82ca9f20
	ctx.lr = 0x832811B8;
	sub_82CA9F20(ctx, base);
	// 832811B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832811BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832811C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832811C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832811C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832811C8 size=64
    let mut pc: u32 = 0x832811C8;
    'dispatch: loop {
        match pc {
            0x832811C8 => {
    //   block [0x832811C8..0x83281208)
	// 832811C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832811CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832811D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832811D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832811D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832811DC: 388BFD98  addi r4, r11, -0x268
	ctx.r[4].s64 = ctx.r[11].s64 + -616;
	// 832811E0: 386AE00C  addi r3, r10, -0x1ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -8180;
	// 832811E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832811E8: 4AFABCE9  bl 0x8222ced0
	ctx.lr = 0x832811EC;
	sub_8222CED0(ctx, base);
	// 832811EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832811F0: 386910A8  addi r3, r9, 0x10a8
	ctx.r[3].s64 = ctx.r[9].s64 + 4264;
	// 832811F4: 4BA28D2D  bl 0x82ca9f20
	ctx.lr = 0x832811F8;
	sub_82CA9F20(ctx, base);
	// 832811F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832811FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281208 size=64
    let mut pc: u32 = 0x83281208;
    'dispatch: loop {
        match pc {
            0x83281208 => {
    //   block [0x83281208..0x83281248)
	// 83281208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328120C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281214: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328121C: 388BFDD4  addi r4, r11, -0x22c
	ctx.r[4].s64 = ctx.r[11].s64 + -556;
	// 83281220: 386AE010  addi r3, r10, -0x1ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -8176;
	// 83281224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281228: 4AFABCA9  bl 0x8222ced0
	ctx.lr = 0x8328122C;
	sub_8222CED0(ctx, base);
	// 8328122C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281230: 386910B8  addi r3, r9, 0x10b8
	ctx.r[3].s64 = ctx.r[9].s64 + 4280;
	// 83281234: 4BA28CED  bl 0x82ca9f20
	ctx.lr = 0x83281238;
	sub_82CA9F20(ctx, base);
	// 83281238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328123C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281248 size=56
    let mut pc: u32 = 0x83281248;
    'dispatch: loop {
        match pc {
            0x83281248 => {
    //   block [0x83281248..0x83281280)
	// 83281248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328124C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281254: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281258: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328125C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83281260: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281264: 4AF72AF5  bl 0x821f3d58
	ctx.lr = 0x83281268;
	sub_821F3D58(ctx, base);
	// 83281268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328126C: 906AE014  stw r3, -0x1fec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8172 as u32), ctx.r[3].u32 ) };
	// 83281270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328127C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281280 size=56
    let mut pc: u32 = 0x83281280;
    'dispatch: loop {
        match pc {
            0x83281280 => {
    //   block [0x83281280..0x832812B8)
	// 83281280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328128C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281290: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281294: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83281298: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328129C: 4AF72ABD  bl 0x821f3d58
	ctx.lr = 0x832812A0;
	sub_821F3D58(ctx, base);
	// 832812A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832812A4: 906AE018  stw r3, -0x1fe8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8168 as u32), ctx.r[3].u32 ) };
	// 832812A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832812AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832812B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832812B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832812B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832812B8 size=56
    let mut pc: u32 = 0x832812B8;
    'dispatch: loop {
        match pc {
            0x832812B8 => {
    //   block [0x832812B8..0x832812F0)
	// 832812B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832812BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832812C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832812C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832812C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832812CC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832812D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832812D4: 4AF72A85  bl 0x821f3d58
	ctx.lr = 0x832812D8;
	sub_821F3D58(ctx, base);
	// 832812D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832812DC: 906AE01C  stw r3, -0x1fe4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8164 as u32), ctx.r[3].u32 ) };
	// 832812E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832812E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832812E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832812EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832812F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832812F0 size=56
    let mut pc: u32 = 0x832812F0;
    'dispatch: loop {
        match pc {
            0x832812F0 => {
    //   block [0x832812F0..0x83281328)
	// 832812F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832812F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832812F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832812FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281300: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281304: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83281308: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328130C: 4AF72A4D  bl 0x821f3d58
	ctx.lr = 0x83281310;
	sub_821F3D58(ctx, base);
	// 83281310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281314: 906AE020  stw r3, -0x1fe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8160 as u32), ctx.r[3].u32 ) };
	// 83281318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328131C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281328 size=56
    let mut pc: u32 = 0x83281328;
    'dispatch: loop {
        match pc {
            0x83281328 => {
    //   block [0x83281328..0x83281360)
	// 83281328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328132C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281334: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281338: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328133C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83281340: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281344: 4AF72A15  bl 0x821f3d58
	ctx.lr = 0x83281348;
	sub_821F3D58(ctx, base);
	// 83281348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328134C: 906AE024  stw r3, -0x1fdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8156 as u32), ctx.r[3].u32 ) };
	// 83281350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328135C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281360 size=56
    let mut pc: u32 = 0x83281360;
    'dispatch: loop {
        match pc {
            0x83281360 => {
    //   block [0x83281360..0x83281398)
	// 83281360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328136C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281370: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281374: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83281378: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328137C: 4AF729DD  bl 0x821f3d58
	ctx.lr = 0x83281380;
	sub_821F3D58(ctx, base);
	// 83281380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281384: 906AE028  stw r3, -0x1fd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8152 as u32), ctx.r[3].u32 ) };
	// 83281388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328138C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281398 size=56
    let mut pc: u32 = 0x83281398;
    'dispatch: loop {
        match pc {
            0x83281398 => {
    //   block [0x83281398..0x832813D0)
	// 83281398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328139C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832813A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832813A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832813A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832813AC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832813B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832813B4: 4AF729A5  bl 0x821f3d58
	ctx.lr = 0x832813B8;
	sub_821F3D58(ctx, base);
	// 832813B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832813BC: 906AE02C  stw r3, -0x1fd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8148 as u32), ctx.r[3].u32 ) };
	// 832813C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832813C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832813C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832813CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832813D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832813D0 size=56
    let mut pc: u32 = 0x832813D0;
    'dispatch: loop {
        match pc {
            0x832813D0 => {
    //   block [0x832813D0..0x83281408)
	// 832813D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832813D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832813D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832813DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832813E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832813E4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832813E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832813EC: 4AF7296D  bl 0x821f3d58
	ctx.lr = 0x832813F0;
	sub_821F3D58(ctx, base);
	// 832813F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832813F4: 906AE030  stw r3, -0x1fd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8144 as u32), ctx.r[3].u32 ) };
	// 832813F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832813FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281408 size=56
    let mut pc: u32 = 0x83281408;
    'dispatch: loop {
        match pc {
            0x83281408 => {
    //   block [0x83281408..0x83281440)
	// 83281408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328140C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281414: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328141C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83281420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281424: 4AF72935  bl 0x821f3d58
	ctx.lr = 0x83281428;
	sub_821F3D58(ctx, base);
	// 83281428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328142C: 906AE034  stw r3, -0x1fcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8140 as u32), ctx.r[3].u32 ) };
	// 83281430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328143C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281440 size=56
    let mut pc: u32 = 0x83281440;
    'dispatch: loop {
        match pc {
            0x83281440 => {
    //   block [0x83281440..0x83281478)
	// 83281440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328144C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281454: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83281458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328145C: 4AF728FD  bl 0x821f3d58
	ctx.lr = 0x83281460;
	sub_821F3D58(ctx, base);
	// 83281460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281464: 906AE038  stw r3, -0x1fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8136 as u32), ctx.r[3].u32 ) };
	// 83281468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328146C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281478 size=56
    let mut pc: u32 = 0x83281478;
    'dispatch: loop {
        match pc {
            0x83281478 => {
    //   block [0x83281478..0x832814B0)
	// 83281478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281484: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328148C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83281490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281494: 4AF728C5  bl 0x821f3d58
	ctx.lr = 0x83281498;
	sub_821F3D58(ctx, base);
	// 83281498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328149C: 906AE03C  stw r3, -0x1fc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8132 as u32), ctx.r[3].u32 ) };
	// 832814A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832814A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832814A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832814AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832814B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832814B0 size=56
    let mut pc: u32 = 0x832814B0;
    'dispatch: loop {
        match pc {
            0x832814B0 => {
    //   block [0x832814B0..0x832814E8)
	// 832814B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832814B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832814B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832814BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832814C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832814C4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832814C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832814CC: 4AF7288D  bl 0x821f3d58
	ctx.lr = 0x832814D0;
	sub_821F3D58(ctx, base);
	// 832814D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832814D4: 906AE040  stw r3, -0x1fc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8128 as u32), ctx.r[3].u32 ) };
	// 832814D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832814DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832814E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832814E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832814E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832814E8 size=56
    let mut pc: u32 = 0x832814E8;
    'dispatch: loop {
        match pc {
            0x832814E8 => {
    //   block [0x832814E8..0x83281520)
	// 832814E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832814EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832814F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832814F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832814F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832814FC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83281500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281504: 4AF72855  bl 0x821f3d58
	ctx.lr = 0x83281508;
	sub_821F3D58(ctx, base);
	// 83281508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328150C: 906AE044  stw r3, -0x1fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8124 as u32), ctx.r[3].u32 ) };
	// 83281510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281520 size=56
    let mut pc: u32 = 0x83281520;
    'dispatch: loop {
        match pc {
            0x83281520 => {
    //   block [0x83281520..0x83281558)
	// 83281520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328152C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281534: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83281538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328153C: 4AF7281D  bl 0x821f3d58
	ctx.lr = 0x83281540;
	sub_821F3D58(ctx, base);
	// 83281540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281544: 906AE048  stw r3, -0x1fb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8120 as u32), ctx.r[3].u32 ) };
	// 83281548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328154C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281558 size=56
    let mut pc: u32 = 0x83281558;
    'dispatch: loop {
        match pc {
            0x83281558 => {
    //   block [0x83281558..0x83281590)
	// 83281558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281564: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328156C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83281570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281574: 4AF727E5  bl 0x821f3d58
	ctx.lr = 0x83281578;
	sub_821F3D58(ctx, base);
	// 83281578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328157C: 906AE04C  stw r3, -0x1fb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8116 as u32), ctx.r[3].u32 ) };
	// 83281580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328158C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281590 size=56
    let mut pc: u32 = 0x83281590;
    'dispatch: loop {
        match pc {
            0x83281590 => {
    //   block [0x83281590..0x832815C8)
	// 83281590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328159C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832815A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832815A4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832815A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832815AC: 4AF727AD  bl 0x821f3d58
	ctx.lr = 0x832815B0;
	sub_821F3D58(ctx, base);
	// 832815B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832815B4: 906AE050  stw r3, -0x1fb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8112 as u32), ctx.r[3].u32 ) };
	// 832815B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832815BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832815C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832815C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832815C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832815C8 size=56
    let mut pc: u32 = 0x832815C8;
    'dispatch: loop {
        match pc {
            0x832815C8 => {
    //   block [0x832815C8..0x83281600)
	// 832815C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832815CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832815D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832815D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832815D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832815DC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832815E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832815E4: 4AF72775  bl 0x821f3d58
	ctx.lr = 0x832815E8;
	sub_821F3D58(ctx, base);
	// 832815E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832815EC: 906AE054  stw r3, -0x1fac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8108 as u32), ctx.r[3].u32 ) };
	// 832815F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832815F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832815F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832815FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281600 size=56
    let mut pc: u32 = 0x83281600;
    'dispatch: loop {
        match pc {
            0x83281600 => {
    //   block [0x83281600..0x83281638)
	// 83281600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328160C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281614: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83281618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328161C: 4AF7273D  bl 0x821f3d58
	ctx.lr = 0x83281620;
	sub_821F3D58(ctx, base);
	// 83281620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281624: 906AE058  stw r3, -0x1fa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8104 as u32), ctx.r[3].u32 ) };
	// 83281628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328162C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281638 size=56
    let mut pc: u32 = 0x83281638;
    'dispatch: loop {
        match pc {
            0x83281638 => {
    //   block [0x83281638..0x83281670)
	// 83281638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328164C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83281650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281654: 4AF72705  bl 0x821f3d58
	ctx.lr = 0x83281658;
	sub_821F3D58(ctx, base);
	// 83281658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328165C: 906AE05C  stw r3, -0x1fa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8100 as u32), ctx.r[3].u32 ) };
	// 83281660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328166C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281670 size=56
    let mut pc: u32 = 0x83281670;
    'dispatch: loop {
        match pc {
            0x83281670 => {
    //   block [0x83281670..0x832816A8)
	// 83281670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328167C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281684: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83281688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328168C: 4AF726CD  bl 0x821f3d58
	ctx.lr = 0x83281690;
	sub_821F3D58(ctx, base);
	// 83281690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281694: 906AE060  stw r3, -0x1fa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8096 as u32), ctx.r[3].u32 ) };
	// 83281698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832816A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832816A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832816A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832816A8 size=56
    let mut pc: u32 = 0x832816A8;
    'dispatch: loop {
        match pc {
            0x832816A8 => {
    //   block [0x832816A8..0x832816E0)
	// 832816A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832816AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832816B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832816B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832816B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832816BC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832816C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832816C4: 4AF72695  bl 0x821f3d58
	ctx.lr = 0x832816C8;
	sub_821F3D58(ctx, base);
	// 832816C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832816CC: 906AE064  stw r3, -0x1f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8092 as u32), ctx.r[3].u32 ) };
	// 832816D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832816D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832816D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832816DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832816E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832816E0 size=64
    let mut pc: u32 = 0x832816E0;
    'dispatch: loop {
        match pc {
            0x832816E0 => {
    //   block [0x832816E0..0x83281720)
	// 832816E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832816E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832816E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832816EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832816F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832816F4: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 832816F8: 386AE068  addi r3, r10, -0x1f98
	ctx.r[3].s64 = ctx.r[10].s64 + -8088;
	// 832816FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281700: 4AFAB7D1  bl 0x8222ced0
	ctx.lr = 0x83281704;
	sub_8222CED0(ctx, base);
	// 83281704: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281708: 386910C8  addi r3, r9, 0x10c8
	ctx.r[3].s64 = ctx.r[9].s64 + 4296;
	// 8328170C: 4BA28815  bl 0x82ca9f20
	ctx.lr = 0x83281710;
	sub_82CA9F20(ctx, base);
	// 83281710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328171C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281720 size=64
    let mut pc: u32 = 0x83281720;
    'dispatch: loop {
        match pc {
            0x83281720 => {
    //   block [0x83281720..0x83281760)
	// 83281720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328172C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281734: 388BFE70  addi r4, r11, -0x190
	ctx.r[4].s64 = ctx.r[11].s64 + -400;
	// 83281738: 386AE06C  addi r3, r10, -0x1f94
	ctx.r[3].s64 = ctx.r[10].s64 + -8084;
	// 8328173C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281740: 4AFAB791  bl 0x8222ced0
	ctx.lr = 0x83281744;
	sub_8222CED0(ctx, base);
	// 83281744: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281748: 386910D8  addi r3, r9, 0x10d8
	ctx.r[3].s64 = ctx.r[9].s64 + 4312;
	// 8328174C: 4BA287D5  bl 0x82ca9f20
	ctx.lr = 0x83281750;
	sub_82CA9F20(ctx, base);
	// 83281750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328175C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281760 size=64
    let mut pc: u32 = 0x83281760;
    'dispatch: loop {
        match pc {
            0x83281760 => {
    //   block [0x83281760..0x832817A0)
	// 83281760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328176C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281774: 388BFEAC  addi r4, r11, -0x154
	ctx.r[4].s64 = ctx.r[11].s64 + -340;
	// 83281778: 386AE070  addi r3, r10, -0x1f90
	ctx.r[3].s64 = ctx.r[10].s64 + -8080;
	// 8328177C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281780: 4AFAB751  bl 0x8222ced0
	ctx.lr = 0x83281784;
	sub_8222CED0(ctx, base);
	// 83281784: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281788: 386910E8  addi r3, r9, 0x10e8
	ctx.r[3].s64 = ctx.r[9].s64 + 4328;
	// 8328178C: 4BA28795  bl 0x82ca9f20
	ctx.lr = 0x83281790;
	sub_82CA9F20(ctx, base);
	// 83281790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328179C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832817A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832817A0 size=64
    let mut pc: u32 = 0x832817A0;
    'dispatch: loop {
        match pc {
            0x832817A0 => {
    //   block [0x832817A0..0x832817E0)
	// 832817A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832817A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832817A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832817AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832817B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832817B4: 388BFEE8  addi r4, r11, -0x118
	ctx.r[4].s64 = ctx.r[11].s64 + -280;
	// 832817B8: 386AE074  addi r3, r10, -0x1f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -8076;
	// 832817BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832817C0: 4AFAB711  bl 0x8222ced0
	ctx.lr = 0x832817C4;
	sub_8222CED0(ctx, base);
	// 832817C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832817C8: 386910F8  addi r3, r9, 0x10f8
	ctx.r[3].s64 = ctx.r[9].s64 + 4344;
	// 832817CC: 4BA28755  bl 0x82ca9f20
	ctx.lr = 0x832817D0;
	sub_82CA9F20(ctx, base);
	// 832817D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832817D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832817D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832817DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832817E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832817E0 size=64
    let mut pc: u32 = 0x832817E0;
    'dispatch: loop {
        match pc {
            0x832817E0 => {
    //   block [0x832817E0..0x83281820)
	// 832817E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832817E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832817E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832817EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832817F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832817F4: 388BFF34  addi r4, r11, -0xcc
	ctx.r[4].s64 = ctx.r[11].s64 + -204;
	// 832817F8: 386AE078  addi r3, r10, -0x1f88
	ctx.r[3].s64 = ctx.r[10].s64 + -8072;
	// 832817FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281800: 4AFAB6D1  bl 0x8222ced0
	ctx.lr = 0x83281804;
	sub_8222CED0(ctx, base);
	// 83281804: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281808: 38691108  addi r3, r9, 0x1108
	ctx.r[3].s64 = ctx.r[9].s64 + 4360;
	// 8328180C: 4BA28715  bl 0x82ca9f20
	ctx.lr = 0x83281810;
	sub_82CA9F20(ctx, base);
	// 83281810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328181C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281820 size=376
    let mut pc: u32 = 0x83281820;
    'dispatch: loop {
        match pc {
            0x83281820 => {
    //   block [0x83281820..0x83281998)
	// 83281820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328182C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281830: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83281834: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83281838: 3BEBE080  addi r31, r11, -0x1f80
	ctx.r[31].s64 = ctx.r[11].s64 + -8064;
	// 8328183C: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 83281840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83281844: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281848: 4AFAB689  bl 0x8222ced0
	ctx.lr = 0x8328184C;
	sub_8222CED0(ctx, base);
	// 8328184C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83281850: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281854: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 83281858: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8328185C: 4AFAB675  bl 0x8222ced0
	ctx.lr = 0x83281860;
	sub_8222CED0(ctx, base);
	// 83281860: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83281864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281868: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 8328186C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83281870: 4AFAB661  bl 0x8222ced0
	ctx.lr = 0x83281874;
	sub_8222CED0(ctx, base);
	// 83281874: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83281878: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328187C: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 83281880: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83281884: 4AFAB64D  bl 0x8222ced0
	ctx.lr = 0x83281888;
	sub_8222CED0(ctx, base);
	// 83281888: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8328188C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281890: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 83281894: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83281898: 4AFAB639  bl 0x8222ced0
	ctx.lr = 0x8328189C;
	sub_8222CED0(ctx, base);
	// 8328189C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832818A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818A4: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 832818A8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832818AC: 4AFAB625  bl 0x8222ced0
	ctx.lr = 0x832818B0;
	sub_8222CED0(ctx, base);
	// 832818B0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832818B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818B8: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 832818BC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832818C0: 4AFAB611  bl 0x8222ced0
	ctx.lr = 0x832818C4;
	sub_8222CED0(ctx, base);
	// 832818C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832818C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818CC: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 832818D0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832818D4: 4AFAB5FD  bl 0x8222ced0
	ctx.lr = 0x832818D8;
	sub_8222CED0(ctx, base);
	// 832818D8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832818DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818E0: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 832818E4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832818E8: 4AFAB5E9  bl 0x8222ced0
	ctx.lr = 0x832818EC;
	sub_8222CED0(ctx, base);
	// 832818EC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832818F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818F4: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 832818F8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832818FC: 4AFAB5D5  bl 0x8222ced0
	ctx.lr = 0x83281900;
	sub_8222CED0(ctx, base);
	// 83281900: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83281904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281908: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 8328190C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83281910: 4AFAB5C1  bl 0x8222ced0
	ctx.lr = 0x83281914;
	sub_8222CED0(ctx, base);
	// 83281914: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83281918: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328191C: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 83281920: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83281924: 4AFAB5AD  bl 0x8222ced0
	ctx.lr = 0x83281928;
	sub_8222CED0(ctx, base);
	// 83281928: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8328192C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281930: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 83281934: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83281938: 4AFAB599  bl 0x8222ced0
	ctx.lr = 0x8328193C;
	sub_8222CED0(ctx, base);
	// 8328193C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83281940: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281944: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 83281948: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8328194C: 4AFAB585  bl 0x8222ced0
	ctx.lr = 0x83281950;
	sub_8222CED0(ctx, base);
	// 83281950: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83281954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281958: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 8328195C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83281960: 4AFAB571  bl 0x8222ced0
	ctx.lr = 0x83281964;
	sub_8222CED0(ctx, base);
	// 83281964: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83281968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328196C: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 83281970: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83281974: 4AFAB55D  bl 0x8222ced0
	ctx.lr = 0x83281978;
	sub_8222CED0(ctx, base);
	// 83281978: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328197C: 386A1118  addi r3, r10, 0x1118
	ctx.r[3].s64 = ctx.r[10].s64 + 4376;
	// 83281980: 4BA285A1  bl 0x82ca9f20
	ctx.lr = 0x83281984;
	sub_82CA9F20(ctx, base);
	// 83281984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328198C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83281994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281998 size=376
    let mut pc: u32 = 0x83281998;
    'dispatch: loop {
        match pc {
            0x83281998 => {
    //   block [0x83281998..0x83281B10)
	// 83281998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832819A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832819A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832819A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832819AC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832819B0: 3BEBE0C0  addi r31, r11, -0x1f40
	ctx.r[31].s64 = ctx.r[11].s64 + -8000;
	// 832819B4: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 832819B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832819BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819C0: 4AFAB511  bl 0x8222ced0
	ctx.lr = 0x832819C4;
	sub_8222CED0(ctx, base);
	// 832819C4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832819C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819CC: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 832819D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832819D4: 4AFAB4FD  bl 0x8222ced0
	ctx.lr = 0x832819D8;
	sub_8222CED0(ctx, base);
	// 832819D8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832819DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819E0: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 832819E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832819E8: 4AFAB4E9  bl 0x8222ced0
	ctx.lr = 0x832819EC;
	sub_8222CED0(ctx, base);
	// 832819EC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832819F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819F4: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 832819F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832819FC: 4AFAB4D5  bl 0x8222ced0
	ctx.lr = 0x83281A00;
	sub_8222CED0(ctx, base);
	// 83281A00: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83281A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A08: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 83281A0C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83281A10: 4AFAB4C1  bl 0x8222ced0
	ctx.lr = 0x83281A14;
	sub_8222CED0(ctx, base);
	// 83281A14: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83281A18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A1C: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 83281A20: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83281A24: 4AFAB4AD  bl 0x8222ced0
	ctx.lr = 0x83281A28;
	sub_8222CED0(ctx, base);
	// 83281A28: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83281A2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A30: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 83281A34: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83281A38: 4AFAB499  bl 0x8222ced0
	ctx.lr = 0x83281A3C;
	sub_8222CED0(ctx, base);
	// 83281A3C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83281A40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A44: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 83281A48: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83281A4C: 4AFAB485  bl 0x8222ced0
	ctx.lr = 0x83281A50;
	sub_8222CED0(ctx, base);
	// 83281A50: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83281A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A58: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 83281A5C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83281A60: 4AFAB471  bl 0x8222ced0
	ctx.lr = 0x83281A64;
	sub_8222CED0(ctx, base);
	// 83281A64: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83281A68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A6C: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 83281A70: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83281A74: 4AFAB45D  bl 0x8222ced0
	ctx.lr = 0x83281A78;
	sub_8222CED0(ctx, base);
	// 83281A78: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83281A7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A80: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 83281A84: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83281A88: 4AFAB449  bl 0x8222ced0
	ctx.lr = 0x83281A8C;
	sub_8222CED0(ctx, base);
	// 83281A8C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83281A90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A94: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 83281A98: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83281A9C: 4AFAB435  bl 0x8222ced0
	ctx.lr = 0x83281AA0;
	sub_8222CED0(ctx, base);
	// 83281AA0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83281AA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281AA8: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 83281AAC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83281AB0: 4AFAB421  bl 0x8222ced0
	ctx.lr = 0x83281AB4;
	sub_8222CED0(ctx, base);
	// 83281AB4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83281AB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281ABC: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 83281AC0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83281AC4: 4AFAB40D  bl 0x8222ced0
	ctx.lr = 0x83281AC8;
	sub_8222CED0(ctx, base);
	// 83281AC8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83281ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281AD0: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 83281AD4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83281AD8: 4AFAB3F9  bl 0x8222ced0
	ctx.lr = 0x83281ADC;
	sub_8222CED0(ctx, base);
	// 83281ADC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83281AE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281AE4: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 83281AE8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83281AEC: 4AFAB3E5  bl 0x8222ced0
	ctx.lr = 0x83281AF0;
	sub_8222CED0(ctx, base);
	// 83281AF0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83281AF4: 386A1180  addi r3, r10, 0x1180
	ctx.r[3].s64 = ctx.r[10].s64 + 4480;
	// 83281AF8: 4BA28429  bl 0x82ca9f20
	ctx.lr = 0x83281AFC;
	sub_82CA9F20(ctx, base);
	// 83281AFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281B08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83281B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281B10 size=56
    let mut pc: u32 = 0x83281B10;
    'dispatch: loop {
        match pc {
            0x83281B10 => {
    //   block [0x83281B10..0x83281B48)
	// 83281B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281B1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281B20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281B24: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83281B28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281B2C: 4AF7222D  bl 0x821f3d58
	ctx.lr = 0x83281B30;
	sub_821F3D58(ctx, base);
	// 83281B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281B34: 906AE07C  stw r3, -0x1f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8068 as u32), ctx.r[3].u32 ) };
	// 83281B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281B48 size=56
    let mut pc: u32 = 0x83281B48;
    'dispatch: loop {
        match pc {
            0x83281B48 => {
    //   block [0x83281B48..0x83281B80)
	// 83281B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281B54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281B58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281B5C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83281B60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281B64: 4AF721F5  bl 0x821f3d58
	ctx.lr = 0x83281B68;
	sub_821F3D58(ctx, base);
	// 83281B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281B6C: 906AE100  stw r3, -0x1f00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7936 as u32), ctx.r[3].u32 ) };
	// 83281B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281B80 size=56
    let mut pc: u32 = 0x83281B80;
    'dispatch: loop {
        match pc {
            0x83281B80 => {
    //   block [0x83281B80..0x83281BB8)
	// 83281B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281B8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281B90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281B94: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83281B98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281B9C: 4AF721BD  bl 0x821f3d58
	ctx.lr = 0x83281BA0;
	sub_821F3D58(ctx, base);
	// 83281BA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281BA4: 906AE104  stw r3, -0x1efc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7932 as u32), ctx.r[3].u32 ) };
	// 83281BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281BB8 size=56
    let mut pc: u32 = 0x83281BB8;
    'dispatch: loop {
        match pc {
            0x83281BB8 => {
    //   block [0x83281BB8..0x83281BF0)
	// 83281BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281BC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281BCC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83281BD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281BD4: 4AF72185  bl 0x821f3d58
	ctx.lr = 0x83281BD8;
	sub_821F3D58(ctx, base);
	// 83281BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281BDC: 906AE108  stw r3, -0x1ef8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7928 as u32), ctx.r[3].u32 ) };
	// 83281BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281BF0 size=56
    let mut pc: u32 = 0x83281BF0;
    'dispatch: loop {
        match pc {
            0x83281BF0 => {
    //   block [0x83281BF0..0x83281C28)
	// 83281BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281C00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281C04: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83281C08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281C0C: 4AF7214D  bl 0x821f3d58
	ctx.lr = 0x83281C10;
	sub_821F3D58(ctx, base);
	// 83281C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281C14: 906AE10C  stw r3, -0x1ef4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7924 as u32), ctx.r[3].u32 ) };
	// 83281C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281C28 size=56
    let mut pc: u32 = 0x83281C28;
    'dispatch: loop {
        match pc {
            0x83281C28 => {
    //   block [0x83281C28..0x83281C60)
	// 83281C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281C34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281C38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281C3C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83281C40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281C44: 4AF72115  bl 0x821f3d58
	ctx.lr = 0x83281C48;
	sub_821F3D58(ctx, base);
	// 83281C48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281C4C: 906AE110  stw r3, -0x1ef0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7920 as u32), ctx.r[3].u32 ) };
	// 83281C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281C60 size=56
    let mut pc: u32 = 0x83281C60;
    'dispatch: loop {
        match pc {
            0x83281C60 => {
    //   block [0x83281C60..0x83281C98)
	// 83281C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281C6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281C70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281C74: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83281C78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281C7C: 4AF720DD  bl 0x821f3d58
	ctx.lr = 0x83281C80;
	sub_821F3D58(ctx, base);
	// 83281C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281C84: 906AE114  stw r3, -0x1eec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7916 as u32), ctx.r[3].u32 ) };
	// 83281C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281C98 size=56
    let mut pc: u32 = 0x83281C98;
    'dispatch: loop {
        match pc {
            0x83281C98 => {
    //   block [0x83281C98..0x83281CD0)
	// 83281C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281CA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281CA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281CAC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83281CB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281CB4: 4AF720A5  bl 0x821f3d58
	ctx.lr = 0x83281CB8;
	sub_821F3D58(ctx, base);
	// 83281CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281CBC: 906AE118  stw r3, -0x1ee8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7912 as u32), ctx.r[3].u32 ) };
	// 83281CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281CD0 size=56
    let mut pc: u32 = 0x83281CD0;
    'dispatch: loop {
        match pc {
            0x83281CD0 => {
    //   block [0x83281CD0..0x83281D08)
	// 83281CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281CDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281CE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281CE4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83281CE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281CEC: 4AF7206D  bl 0x821f3d58
	ctx.lr = 0x83281CF0;
	sub_821F3D58(ctx, base);
	// 83281CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281CF4: 906AE11C  stw r3, -0x1ee4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7908 as u32), ctx.r[3].u32 ) };
	// 83281CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281D08 size=56
    let mut pc: u32 = 0x83281D08;
    'dispatch: loop {
        match pc {
            0x83281D08 => {
    //   block [0x83281D08..0x83281D40)
	// 83281D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281D14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281D18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281D1C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83281D20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281D24: 4AF72035  bl 0x821f3d58
	ctx.lr = 0x83281D28;
	sub_821F3D58(ctx, base);
	// 83281D28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281D2C: 906AE120  stw r3, -0x1ee0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7904 as u32), ctx.r[3].u32 ) };
	// 83281D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281D40 size=56
    let mut pc: u32 = 0x83281D40;
    'dispatch: loop {
        match pc {
            0x83281D40 => {
    //   block [0x83281D40..0x83281D78)
	// 83281D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281D4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281D50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281D54: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83281D58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281D5C: 4AF71FFD  bl 0x821f3d58
	ctx.lr = 0x83281D60;
	sub_821F3D58(ctx, base);
	// 83281D60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281D64: 906AE124  stw r3, -0x1edc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7900 as u32), ctx.r[3].u32 ) };
	// 83281D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281D78 size=56
    let mut pc: u32 = 0x83281D78;
    'dispatch: loop {
        match pc {
            0x83281D78 => {
    //   block [0x83281D78..0x83281DB0)
	// 83281D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281D88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281D8C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83281D90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281D94: 4AF71FC5  bl 0x821f3d58
	ctx.lr = 0x83281D98;
	sub_821F3D58(ctx, base);
	// 83281D98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281D9C: 906AE128  stw r3, -0x1ed8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7896 as u32), ctx.r[3].u32 ) };
	// 83281DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281DB0 size=56
    let mut pc: u32 = 0x83281DB0;
    'dispatch: loop {
        match pc {
            0x83281DB0 => {
    //   block [0x83281DB0..0x83281DE8)
	// 83281DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281DBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281DC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281DC4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83281DC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281DCC: 4AF71F8D  bl 0x821f3d58
	ctx.lr = 0x83281DD0;
	sub_821F3D58(ctx, base);
	// 83281DD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281DD4: 906AE12C  stw r3, -0x1ed4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7892 as u32), ctx.r[3].u32 ) };
	// 83281DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281DE8 size=56
    let mut pc: u32 = 0x83281DE8;
    'dispatch: loop {
        match pc {
            0x83281DE8 => {
    //   block [0x83281DE8..0x83281E20)
	// 83281DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281DF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281DF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281DFC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83281E00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281E04: 4AF71F55  bl 0x821f3d58
	ctx.lr = 0x83281E08;
	sub_821F3D58(ctx, base);
	// 83281E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281E0C: 906AE130  stw r3, -0x1ed0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7888 as u32), ctx.r[3].u32 ) };
	// 83281E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281E20 size=56
    let mut pc: u32 = 0x83281E20;
    'dispatch: loop {
        match pc {
            0x83281E20 => {
    //   block [0x83281E20..0x83281E58)
	// 83281E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281E2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281E30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281E34: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83281E38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281E3C: 4AF71F1D  bl 0x821f3d58
	ctx.lr = 0x83281E40;
	sub_821F3D58(ctx, base);
	// 83281E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281E44: 906AE134  stw r3, -0x1ecc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7884 as u32), ctx.r[3].u32 ) };
	// 83281E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281E58 size=56
    let mut pc: u32 = 0x83281E58;
    'dispatch: loop {
        match pc {
            0x83281E58 => {
    //   block [0x83281E58..0x83281E90)
	// 83281E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281E64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281E68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281E6C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83281E70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281E74: 4AF71EE5  bl 0x821f3d58
	ctx.lr = 0x83281E78;
	sub_821F3D58(ctx, base);
	// 83281E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281E7C: 906AE138  stw r3, -0x1ec8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7880 as u32), ctx.r[3].u32 ) };
	// 83281E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281E90 size=56
    let mut pc: u32 = 0x83281E90;
    'dispatch: loop {
        match pc {
            0x83281E90 => {
    //   block [0x83281E90..0x83281EC8)
	// 83281E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281E9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281EA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281EA4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83281EA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281EAC: 4AF71EAD  bl 0x821f3d58
	ctx.lr = 0x83281EB0;
	sub_821F3D58(ctx, base);
	// 83281EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281EB4: 906AE13C  stw r3, -0x1ec4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7876 as u32), ctx.r[3].u32 ) };
	// 83281EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281EC8 size=56
    let mut pc: u32 = 0x83281EC8;
    'dispatch: loop {
        match pc {
            0x83281EC8 => {
    //   block [0x83281EC8..0x83281F00)
	// 83281EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281ED4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281ED8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281EDC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83281EE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281EE4: 4AF71E75  bl 0x821f3d58
	ctx.lr = 0x83281EE8;
	sub_821F3D58(ctx, base);
	// 83281EE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281EEC: 906AE140  stw r3, -0x1ec0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7872 as u32), ctx.r[3].u32 ) };
	// 83281EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281F00 size=56
    let mut pc: u32 = 0x83281F00;
    'dispatch: loop {
        match pc {
            0x83281F00 => {
    //   block [0x83281F00..0x83281F38)
	// 83281F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281F0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281F10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281F14: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83281F18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281F1C: 4AF71E3D  bl 0x821f3d58
	ctx.lr = 0x83281F20;
	sub_821F3D58(ctx, base);
	// 83281F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281F24: 906AE144  stw r3, -0x1ebc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7868 as u32), ctx.r[3].u32 ) };
	// 83281F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281F38 size=56
    let mut pc: u32 = 0x83281F38;
    'dispatch: loop {
        match pc {
            0x83281F38 => {
    //   block [0x83281F38..0x83281F70)
	// 83281F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281F44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281F48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281F4C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83281F50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281F54: 4AF71E05  bl 0x821f3d58
	ctx.lr = 0x83281F58;
	sub_821F3D58(ctx, base);
	// 83281F58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281F5C: 906AE148  stw r3, -0x1eb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7864 as u32), ctx.r[3].u32 ) };
	// 83281F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281F70 size=56
    let mut pc: u32 = 0x83281F70;
    'dispatch: loop {
        match pc {
            0x83281F70 => {
    //   block [0x83281F70..0x83281FA8)
	// 83281F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281F7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281F80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281F84: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83281F88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281F8C: 4AF71DCD  bl 0x821f3d58
	ctx.lr = 0x83281F90;
	sub_821F3D58(ctx, base);
	// 83281F90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281F94: 906AE14C  stw r3, -0x1eb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7860 as u32), ctx.r[3].u32 ) };
	// 83281F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281FA8 size=64
    let mut pc: u32 = 0x83281FA8;
    'dispatch: loop {
        match pc {
            0x83281FA8 => {
    //   block [0x83281FA8..0x83281FE8)
	// 83281FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281FB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281FBC: 388BFFC8  addi r4, r11, -0x38
	ctx.r[4].s64 = ctx.r[11].s64 + -56;
	// 83281FC0: 386AE150  addi r3, r10, -0x1eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -7856;
	// 83281FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281FC8: 4AFAAF09  bl 0x8222ced0
	ctx.lr = 0x83281FCC;
	sub_8222CED0(ctx, base);
	// 83281FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281FD0: 386911E8  addi r3, r9, 0x11e8
	ctx.r[3].s64 = ctx.r[9].s64 + 4584;
	// 83281FD4: 4BA27F4D  bl 0x82ca9f20
	ctx.lr = 0x83281FD8;
	sub_82CA9F20(ctx, base);
	// 83281FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281FE8 size=64
    let mut pc: u32 = 0x83281FE8;
    'dispatch: loop {
        match pc {
            0x83281FE8 => {
    //   block [0x83281FE8..0x83282028)
	// 83281FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281FF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281FFC: 388B0000  addi r4, r11, 0
	ctx.r[4].s64 = ctx.r[11].s64 + 0;
	// 83282000: 386AE154  addi r3, r10, -0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + -7852;
	// 83282004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282008: 4AFAAEC9  bl 0x8222ced0
	ctx.lr = 0x8328200C;
	sub_8222CED0(ctx, base);
	// 8328200C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282010: 386911F8  addi r3, r9, 0x11f8
	ctx.r[3].s64 = ctx.r[9].s64 + 4600;
	// 83282014: 4BA27F0D  bl 0x82ca9f20
	ctx.lr = 0x83282018;
	sub_82CA9F20(ctx, base);
	// 83282018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328201C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282028 size=64
    let mut pc: u32 = 0x83282028;
    'dispatch: loop {
        match pc {
            0x83282028 => {
    //   block [0x83282028..0x83282068)
	// 83282028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282034: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328203C: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 83282040: 386AE158  addi r3, r10, -0x1ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -7848;
	// 83282044: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282048: 4AFAAE89  bl 0x8222ced0
	ctx.lr = 0x8328204C;
	sub_8222CED0(ctx, base);
	// 8328204C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282050: 38691208  addi r3, r9, 0x1208
	ctx.r[3].s64 = ctx.r[9].s64 + 4616;
	// 83282054: 4BA27ECD  bl 0x82ca9f20
	ctx.lr = 0x83282058;
	sub_82CA9F20(ctx, base);
	// 83282058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282068 size=64
    let mut pc: u32 = 0x83282068;
    'dispatch: loop {
        match pc {
            0x83282068 => {
    //   block [0x83282068..0x832820A8)
	// 83282068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282074: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328207C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 83282080: 386AE15C  addi r3, r10, -0x1ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -7844;
	// 83282084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282088: 4AFAAE49  bl 0x8222ced0
	ctx.lr = 0x8328208C;
	sub_8222CED0(ctx, base);
	// 8328208C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282090: 38691218  addi r3, r9, 0x1218
	ctx.r[3].s64 = ctx.r[9].s64 + 4632;
	// 83282094: 4BA27E8D  bl 0x82ca9f20
	ctx.lr = 0x83282098;
	sub_82CA9F20(ctx, base);
	// 83282098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832820A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832820A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832820A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832820A8 size=56
    let mut pc: u32 = 0x832820A8;
    'dispatch: loop {
        match pc {
            0x832820A8 => {
    //   block [0x832820A8..0x832820E0)
	// 832820A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832820AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832820B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832820B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832820B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832820BC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832820C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832820C4: 4AF71C95  bl 0x821f3d58
	ctx.lr = 0x832820C8;
	sub_821F3D58(ctx, base);
	// 832820C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832820CC: 906AE160  stw r3, -0x1ea0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7840 as u32), ctx.r[3].u32 ) };
	// 832820D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832820D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832820D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832820DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832820E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832820E0 size=56
    let mut pc: u32 = 0x832820E0;
    'dispatch: loop {
        match pc {
            0x832820E0 => {
    //   block [0x832820E0..0x83282118)
	// 832820E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832820E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832820E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832820EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832820F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832820F4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832820F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832820FC: 4AF71C5D  bl 0x821f3d58
	ctx.lr = 0x83282100;
	sub_821F3D58(ctx, base);
	// 83282100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282104: 906AE164  stw r3, -0x1e9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7836 as u32), ctx.r[3].u32 ) };
	// 83282108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328210C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282118 size=56
    let mut pc: u32 = 0x83282118;
    'dispatch: loop {
        match pc {
            0x83282118 => {
    //   block [0x83282118..0x83282150)
	// 83282118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328212C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83282130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282134: 4AF71C25  bl 0x821f3d58
	ctx.lr = 0x83282138;
	sub_821F3D58(ctx, base);
	// 83282138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328213C: 906AE168  stw r3, -0x1e98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7832 as u32), ctx.r[3].u32 ) };
	// 83282140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328214C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282150 size=56
    let mut pc: u32 = 0x83282150;
    'dispatch: loop {
        match pc {
            0x83282150 => {
    //   block [0x83282150..0x83282188)
	// 83282150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328215C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282164: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83282168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328216C: 4AF71BED  bl 0x821f3d58
	ctx.lr = 0x83282170;
	sub_821F3D58(ctx, base);
	// 83282170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282174: 906AE16C  stw r3, -0x1e94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7828 as u32), ctx.r[3].u32 ) };
	// 83282178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328217C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282188 size=56
    let mut pc: u32 = 0x83282188;
    'dispatch: loop {
        match pc {
            0x83282188 => {
    //   block [0x83282188..0x832821C0)
	// 83282188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282194: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282198: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328219C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832821A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832821A4: 4AF71BB5  bl 0x821f3d58
	ctx.lr = 0x832821A8;
	sub_821F3D58(ctx, base);
	// 832821A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832821AC: 906AE170  stw r3, -0x1e90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7824 as u32), ctx.r[3].u32 ) };
	// 832821B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832821B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832821B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832821BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832821C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832821C0 size=56
    let mut pc: u32 = 0x832821C0;
    'dispatch: loop {
        match pc {
            0x832821C0 => {
    //   block [0x832821C0..0x832821F8)
	// 832821C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832821C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832821C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832821CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832821D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832821D4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832821D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832821DC: 4AF71B7D  bl 0x821f3d58
	ctx.lr = 0x832821E0;
	sub_821F3D58(ctx, base);
	// 832821E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832821E4: 906AE174  stw r3, -0x1e8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7820 as u32), ctx.r[3].u32 ) };
	// 832821E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832821EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832821F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832821F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832821F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832821F8 size=56
    let mut pc: u32 = 0x832821F8;
    'dispatch: loop {
        match pc {
            0x832821F8 => {
    //   block [0x832821F8..0x83282230)
	// 832821F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832821FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282208: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328220C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83282210: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282214: 4AF71B45  bl 0x821f3d58
	ctx.lr = 0x83282218;
	sub_821F3D58(ctx, base);
	// 83282218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328221C: 906AE178  stw r3, -0x1e88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7816 as u32), ctx.r[3].u32 ) };
	// 83282220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328222C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282230 size=56
    let mut pc: u32 = 0x83282230;
    'dispatch: loop {
        match pc {
            0x83282230 => {
    //   block [0x83282230..0x83282268)
	// 83282230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328223C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282240: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282244: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83282248: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328224C: 4AF71B0D  bl 0x821f3d58
	ctx.lr = 0x83282250;
	sub_821F3D58(ctx, base);
	// 83282250: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282254: 906AE17C  stw r3, -0x1e84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7812 as u32), ctx.r[3].u32 ) };
	// 83282258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282268 size=56
    let mut pc: u32 = 0x83282268;
    'dispatch: loop {
        match pc {
            0x83282268 => {
    //   block [0x83282268..0x832822A0)
	// 83282268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282274: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282278: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328227C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83282280: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282284: 4AF71AD5  bl 0x821f3d58
	ctx.lr = 0x83282288;
	sub_821F3D58(ctx, base);
	// 83282288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328228C: 906AE180  stw r3, -0x1e80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7808 as u32), ctx.r[3].u32 ) };
	// 83282290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328229C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832822A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832822A0 size=56
    let mut pc: u32 = 0x832822A0;
    'dispatch: loop {
        match pc {
            0x832822A0 => {
    //   block [0x832822A0..0x832822D8)
	// 832822A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832822A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832822A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832822AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832822B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832822B4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832822B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832822BC: 4AF71A9D  bl 0x821f3d58
	ctx.lr = 0x832822C0;
	sub_821F3D58(ctx, base);
	// 832822C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832822C4: 906AE184  stw r3, -0x1e7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7804 as u32), ctx.r[3].u32 ) };
	// 832822C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832822CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832822D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832822D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832822D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832822D8 size=56
    let mut pc: u32 = 0x832822D8;
    'dispatch: loop {
        match pc {
            0x832822D8 => {
    //   block [0x832822D8..0x83282310)
	// 832822D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832822DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832822E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832822E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832822E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832822EC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832822F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832822F4: 4AF71A65  bl 0x821f3d58
	ctx.lr = 0x832822F8;
	sub_821F3D58(ctx, base);
	// 832822F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832822FC: 906AE188  stw r3, -0x1e78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7800 as u32), ctx.r[3].u32 ) };
	// 83282300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328230C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282310 size=56
    let mut pc: u32 = 0x83282310;
    'dispatch: loop {
        match pc {
            0x83282310 => {
    //   block [0x83282310..0x83282348)
	// 83282310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328231C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282320: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282324: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83282328: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328232C: 4AF71A2D  bl 0x821f3d58
	ctx.lr = 0x83282330;
	sub_821F3D58(ctx, base);
	// 83282330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282334: 906AE18C  stw r3, -0x1e74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7796 as u32), ctx.r[3].u32 ) };
	// 83282338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328233C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282348 size=56
    let mut pc: u32 = 0x83282348;
    'dispatch: loop {
        match pc {
            0x83282348 => {
    //   block [0x83282348..0x83282380)
	// 83282348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282354: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282358: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328235C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83282360: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282364: 4AF719F5  bl 0x821f3d58
	ctx.lr = 0x83282368;
	sub_821F3D58(ctx, base);
	// 83282368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328236C: 906AE190  stw r3, -0x1e70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7792 as u32), ctx.r[3].u32 ) };
	// 83282370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328237C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282380 size=56
    let mut pc: u32 = 0x83282380;
    'dispatch: loop {
        match pc {
            0x83282380 => {
    //   block [0x83282380..0x832823B8)
	// 83282380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328238C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282390: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282394: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83282398: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328239C: 4AF719BD  bl 0x821f3d58
	ctx.lr = 0x832823A0;
	sub_821F3D58(ctx, base);
	// 832823A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832823A4: 906AE194  stw r3, -0x1e6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7788 as u32), ctx.r[3].u32 ) };
	// 832823A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832823AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832823B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832823B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832823B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832823B8 size=56
    let mut pc: u32 = 0x832823B8;
    'dispatch: loop {
        match pc {
            0x832823B8 => {
    //   block [0x832823B8..0x832823F0)
	// 832823B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832823BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832823C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832823C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832823C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832823CC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832823D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832823D4: 4AF71985  bl 0x821f3d58
	ctx.lr = 0x832823D8;
	sub_821F3D58(ctx, base);
	// 832823D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832823DC: 906AE198  stw r3, -0x1e68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7784 as u32), ctx.r[3].u32 ) };
	// 832823E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832823E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832823E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832823EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832823F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832823F0 size=56
    let mut pc: u32 = 0x832823F0;
    'dispatch: loop {
        match pc {
            0x832823F0 => {
    //   block [0x832823F0..0x83282428)
	// 832823F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832823F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832823F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832823FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282404: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83282408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328240C: 4AF7194D  bl 0x821f3d58
	ctx.lr = 0x83282410;
	sub_821F3D58(ctx, base);
	// 83282410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282414: 906AE19C  stw r3, -0x1e64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7780 as u32), ctx.r[3].u32 ) };
	// 83282418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328241C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282428 size=56
    let mut pc: u32 = 0x83282428;
    'dispatch: loop {
        match pc {
            0x83282428 => {
    //   block [0x83282428..0x83282460)
	// 83282428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328242C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282434: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328243C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83282440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282444: 4AF71915  bl 0x821f3d58
	ctx.lr = 0x83282448;
	sub_821F3D58(ctx, base);
	// 83282448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328244C: 906AE1A0  stw r3, -0x1e60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7776 as u32), ctx.r[3].u32 ) };
	// 83282450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328245C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282460 size=56
    let mut pc: u32 = 0x83282460;
    'dispatch: loop {
        match pc {
            0x83282460 => {
    //   block [0x83282460..0x83282498)
	// 83282460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328246C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282474: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83282478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328247C: 4AF718DD  bl 0x821f3d58
	ctx.lr = 0x83282480;
	sub_821F3D58(ctx, base);
	// 83282480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282484: 906AE1A4  stw r3, -0x1e5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7772 as u32), ctx.r[3].u32 ) };
	// 83282488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328248C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282498 size=56
    let mut pc: u32 = 0x83282498;
    'dispatch: loop {
        match pc {
            0x83282498 => {
    //   block [0x83282498..0x832824D0)
	// 83282498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328249C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832824A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832824A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832824A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832824AC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832824B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832824B4: 4AF718A5  bl 0x821f3d58
	ctx.lr = 0x832824B8;
	sub_821F3D58(ctx, base);
	// 832824B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832824BC: 906AE1A8  stw r3, -0x1e58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7768 as u32), ctx.r[3].u32 ) };
	// 832824C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832824C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832824C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832824CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832824D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832824D0 size=56
    let mut pc: u32 = 0x832824D0;
    'dispatch: loop {
        match pc {
            0x832824D0 => {
    //   block [0x832824D0..0x83282508)
	// 832824D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832824D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832824D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832824DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832824E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832824E4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832824E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832824EC: 4AF7186D  bl 0x821f3d58
	ctx.lr = 0x832824F0;
	sub_821F3D58(ctx, base);
	// 832824F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832824F4: 906AE1AC  stw r3, -0x1e54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7764 as u32), ctx.r[3].u32 ) };
	// 832824F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832824FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282508 size=56
    let mut pc: u32 = 0x83282508;
    'dispatch: loop {
        match pc {
            0x83282508 => {
    //   block [0x83282508..0x83282540)
	// 83282508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282514: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328251C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83282520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282524: 4AF71835  bl 0x821f3d58
	ctx.lr = 0x83282528;
	sub_821F3D58(ctx, base);
	// 83282528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328252C: 906AE1B0  stw r3, -0x1e50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7760 as u32), ctx.r[3].u32 ) };
	// 83282530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328253C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282540 size=64
    let mut pc: u32 = 0x83282540;
    'dispatch: loop {
        match pc {
            0x83282540 => {
    //   block [0x83282540..0x83282580)
	// 83282540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328254C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282554: 388B00B8  addi r4, r11, 0xb8
	ctx.r[4].s64 = ctx.r[11].s64 + 184;
	// 83282558: 386AE1B4  addi r3, r10, -0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7756;
	// 8328255C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282560: 4AFAA971  bl 0x8222ced0
	ctx.lr = 0x83282564;
	sub_8222CED0(ctx, base);
	// 83282564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282568: 38691228  addi r3, r9, 0x1228
	ctx.r[3].s64 = ctx.r[9].s64 + 4648;
	// 8328256C: 4BA279B5  bl 0x82ca9f20
	ctx.lr = 0x83282570;
	sub_82CA9F20(ctx, base);
	// 83282570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328257C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282580 size=64
    let mut pc: u32 = 0x83282580;
    'dispatch: loop {
        match pc {
            0x83282580 => {
    //   block [0x83282580..0x832825C0)
	// 83282580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328258C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282594: 388B00B8  addi r4, r11, 0xb8
	ctx.r[4].s64 = ctx.r[11].s64 + 184;
	// 83282598: 386AE1B8  addi r3, r10, -0x1e48
	ctx.r[3].s64 = ctx.r[10].s64 + -7752;
	// 8328259C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832825A0: 4AFAA931  bl 0x8222ced0
	ctx.lr = 0x832825A4;
	sub_8222CED0(ctx, base);
	// 832825A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832825A8: 38691238  addi r3, r9, 0x1238
	ctx.r[3].s64 = ctx.r[9].s64 + 4664;
	// 832825AC: 4BA27975  bl 0x82ca9f20
	ctx.lr = 0x832825B0;
	sub_82CA9F20(ctx, base);
	// 832825B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832825B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832825B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832825BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832825C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832825C0 size=64
    let mut pc: u32 = 0x832825C0;
    'dispatch: loop {
        match pc {
            0x832825C0 => {
    //   block [0x832825C0..0x83282600)
	// 832825C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832825C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832825C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832825CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832825D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832825D4: 388B00F8  addi r4, r11, 0xf8
	ctx.r[4].s64 = ctx.r[11].s64 + 248;
	// 832825D8: 386AE1BC  addi r3, r10, -0x1e44
	ctx.r[3].s64 = ctx.r[10].s64 + -7748;
	// 832825DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832825E0: 4AFAA8F1  bl 0x8222ced0
	ctx.lr = 0x832825E4;
	sub_8222CED0(ctx, base);
	// 832825E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832825E8: 38691248  addi r3, r9, 0x1248
	ctx.r[3].s64 = ctx.r[9].s64 + 4680;
	// 832825EC: 4BA27935  bl 0x82ca9f20
	ctx.lr = 0x832825F0;
	sub_82CA9F20(ctx, base);
	// 832825F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832825F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832825F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832825FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282600 size=64
    let mut pc: u32 = 0x83282600;
    'dispatch: loop {
        match pc {
            0x83282600 => {
    //   block [0x83282600..0x83282640)
	// 83282600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328260C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282614: 388B013C  addi r4, r11, 0x13c
	ctx.r[4].s64 = ctx.r[11].s64 + 316;
	// 83282618: 386AE1C0  addi r3, r10, -0x1e40
	ctx.r[3].s64 = ctx.r[10].s64 + -7744;
	// 8328261C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282620: 4AFAA8B1  bl 0x8222ced0
	ctx.lr = 0x83282624;
	sub_8222CED0(ctx, base);
	// 83282624: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282628: 38691258  addi r3, r9, 0x1258
	ctx.r[3].s64 = ctx.r[9].s64 + 4696;
	// 8328262C: 4BA278F5  bl 0x82ca9f20
	ctx.lr = 0x83282630;
	sub_82CA9F20(ctx, base);
	// 83282630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328263C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282640 size=64
    let mut pc: u32 = 0x83282640;
    'dispatch: loop {
        match pc {
            0x83282640 => {
    //   block [0x83282640..0x83282680)
	// 83282640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328264C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282654: 388B0178  addi r4, r11, 0x178
	ctx.r[4].s64 = ctx.r[11].s64 + 376;
	// 83282658: 386AE1C4  addi r3, r10, -0x1e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7740;
	// 8328265C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282660: 4AFAA871  bl 0x8222ced0
	ctx.lr = 0x83282664;
	sub_8222CED0(ctx, base);
	// 83282664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282668: 38691268  addi r3, r9, 0x1268
	ctx.r[3].s64 = ctx.r[9].s64 + 4712;
	// 8328266C: 4BA278B5  bl 0x82ca9f20
	ctx.lr = 0x83282670;
	sub_82CA9F20(ctx, base);
	// 83282670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328267C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282680 size=64
    let mut pc: u32 = 0x83282680;
    'dispatch: loop {
        match pc {
            0x83282680 => {
    //   block [0x83282680..0x832826C0)
	// 83282680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328268C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282694: 388B01BC  addi r4, r11, 0x1bc
	ctx.r[4].s64 = ctx.r[11].s64 + 444;
	// 83282698: 386AE1C8  addi r3, r10, -0x1e38
	ctx.r[3].s64 = ctx.r[10].s64 + -7736;
	// 8328269C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832826A0: 4AFAA831  bl 0x8222ced0
	ctx.lr = 0x832826A4;
	sub_8222CED0(ctx, base);
	// 832826A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832826A8: 38691278  addi r3, r9, 0x1278
	ctx.r[3].s64 = ctx.r[9].s64 + 4728;
	// 832826AC: 4BA27875  bl 0x82ca9f20
	ctx.lr = 0x832826B0;
	sub_82CA9F20(ctx, base);
	// 832826B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832826B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832826B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832826BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832826C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832826C0 size=64
    let mut pc: u32 = 0x832826C0;
    'dispatch: loop {
        match pc {
            0x832826C0 => {
    //   block [0x832826C0..0x83282700)
	// 832826C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832826C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832826C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832826CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832826D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832826D4: 388B01F8  addi r4, r11, 0x1f8
	ctx.r[4].s64 = ctx.r[11].s64 + 504;
	// 832826D8: 386AE1CC  addi r3, r10, -0x1e34
	ctx.r[3].s64 = ctx.r[10].s64 + -7732;
	// 832826DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832826E0: 4AFAA7F1  bl 0x8222ced0
	ctx.lr = 0x832826E4;
	sub_8222CED0(ctx, base);
	// 832826E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832826E8: 38691288  addi r3, r9, 0x1288
	ctx.r[3].s64 = ctx.r[9].s64 + 4744;
	// 832826EC: 4BA27835  bl 0x82ca9f20
	ctx.lr = 0x832826F0;
	sub_82CA9F20(ctx, base);
	// 832826F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832826F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832826F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832826FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282700 size=64
    let mut pc: u32 = 0x83282700;
    'dispatch: loop {
        match pc {
            0x83282700 => {
    //   block [0x83282700..0x83282740)
	// 83282700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328270C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282714: 388B0240  addi r4, r11, 0x240
	ctx.r[4].s64 = ctx.r[11].s64 + 576;
	// 83282718: 386AE1D0  addi r3, r10, -0x1e30
	ctx.r[3].s64 = ctx.r[10].s64 + -7728;
	// 8328271C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282720: 4AFAA7B1  bl 0x8222ced0
	ctx.lr = 0x83282724;
	sub_8222CED0(ctx, base);
	// 83282724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282728: 38691298  addi r3, r9, 0x1298
	ctx.r[3].s64 = ctx.r[9].s64 + 4760;
	// 8328272C: 4BA277F5  bl 0x82ca9f20
	ctx.lr = 0x83282730;
	sub_82CA9F20(ctx, base);
	// 83282730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328273C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282740 size=64
    let mut pc: u32 = 0x83282740;
    'dispatch: loop {
        match pc {
            0x83282740 => {
    //   block [0x83282740..0x83282780)
	// 83282740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328274C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282754: 388B0280  addi r4, r11, 0x280
	ctx.r[4].s64 = ctx.r[11].s64 + 640;
	// 83282758: 386AE1D4  addi r3, r10, -0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7724;
	// 8328275C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282760: 4AFAA771  bl 0x8222ced0
	ctx.lr = 0x83282764;
	sub_8222CED0(ctx, base);
	// 83282764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282768: 386912A8  addi r3, r9, 0x12a8
	ctx.r[3].s64 = ctx.r[9].s64 + 4776;
	// 8328276C: 4BA277B5  bl 0x82ca9f20
	ctx.lr = 0x83282770;
	sub_82CA9F20(ctx, base);
	// 83282770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282780 size=64
    let mut pc: u32 = 0x83282780;
    'dispatch: loop {
        match pc {
            0x83282780 => {
    //   block [0x83282780..0x832827C0)
	// 83282780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328278C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282794: 388B02C4  addi r4, r11, 0x2c4
	ctx.r[4].s64 = ctx.r[11].s64 + 708;
	// 83282798: 386AE1D8  addi r3, r10, -0x1e28
	ctx.r[3].s64 = ctx.r[10].s64 + -7720;
	// 8328279C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832827A0: 4AFAA731  bl 0x8222ced0
	ctx.lr = 0x832827A4;
	sub_8222CED0(ctx, base);
	// 832827A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832827A8: 386912B8  addi r3, r9, 0x12b8
	ctx.r[3].s64 = ctx.r[9].s64 + 4792;
	// 832827AC: 4BA27775  bl 0x82ca9f20
	ctx.lr = 0x832827B0;
	sub_82CA9F20(ctx, base);
	// 832827B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832827B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832827B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832827BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832827C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832827C0 size=64
    let mut pc: u32 = 0x832827C0;
    'dispatch: loop {
        match pc {
            0x832827C0 => {
    //   block [0x832827C0..0x83282800)
	// 832827C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832827C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832827C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832827CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832827D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832827D4: 388B0300  addi r4, r11, 0x300
	ctx.r[4].s64 = ctx.r[11].s64 + 768;
	// 832827D8: 386AE1DC  addi r3, r10, -0x1e24
	ctx.r[3].s64 = ctx.r[10].s64 + -7716;
	// 832827DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832827E0: 4AFAA6F1  bl 0x8222ced0
	ctx.lr = 0x832827E4;
	sub_8222CED0(ctx, base);
	// 832827E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832827E8: 386912C8  addi r3, r9, 0x12c8
	ctx.r[3].s64 = ctx.r[9].s64 + 4808;
	// 832827EC: 4BA27735  bl 0x82ca9f20
	ctx.lr = 0x832827F0;
	sub_82CA9F20(ctx, base);
	// 832827F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832827F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832827F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832827FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282800 size=156
    let mut pc: u32 = 0x83282800;
    'dispatch: loop {
        match pc {
            0x83282800 => {
    //   block [0x83282800..0x8328289C)
	// 83282800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328280C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282810: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83282814: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83282818: 3BEBE1E0  addi r31, r11, -0x1e20
	ctx.r[31].s64 = ctx.r[11].s64 + -7712;
	// 8328281C: 388A0398  addi r4, r10, 0x398
	ctx.r[4].s64 = ctx.r[10].s64 + 920;
	// 83282820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83282824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282828: 4AFAA6A9  bl 0x8222ced0
	ctx.lr = 0x8328282C;
	sub_8222CED0(ctx, base);
	// 8328282C: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83282830: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282834: 38890380  addi r4, r9, 0x380
	ctx.r[4].s64 = ctx.r[9].s64 + 896;
	// 83282838: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8328283C: 4AFAA695  bl 0x8222ced0
	ctx.lr = 0x83282840;
	sub_8222CED0(ctx, base);
	// 83282840: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83282844: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282848: 38880364  addi r4, r8, 0x364
	ctx.r[4].s64 = ctx.r[8].s64 + 868;
	// 8328284C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83282850: 4AFAA681  bl 0x8222ced0
	ctx.lr = 0x83282854;
	sub_8222CED0(ctx, base);
	// 83282854: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83282858: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328285C: 3887034C  addi r4, r7, 0x34c
	ctx.r[4].s64 = ctx.r[7].s64 + 844;
	// 83282860: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83282864: 4AFAA66D  bl 0x8222ced0
	ctx.lr = 0x83282868;
	sub_8222CED0(ctx, base);
	// 83282868: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328286C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282870: 38860330  addi r4, r6, 0x330
	ctx.r[4].s64 = ctx.r[6].s64 + 816;
	// 83282874: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83282878: 4AFAA659  bl 0x8222ced0
	ctx.lr = 0x8328287C;
	sub_8222CED0(ctx, base);
	// 8328287C: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83282880: 386512D8  addi r3, r5, 0x12d8
	ctx.r[3].s64 = ctx.r[5].s64 + 4824;
	// 83282884: 4BA2769D  bl 0x82ca9f20
	ctx.lr = 0x83282888;
	sub_82CA9F20(ctx, base);
	// 83282888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328288C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282894: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83282898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832828A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832828A0 size=56
    let mut pc: u32 = 0x832828A0;
    'dispatch: loop {
        match pc {
            0x832828A0 => {
    //   block [0x832828A0..0x832828D8)
	// 832828A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832828A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832828A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832828AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832828B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832828B4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832828B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832828BC: 4AF7149D  bl 0x821f3d58
	ctx.lr = 0x832828C0;
	sub_821F3D58(ctx, base);
	// 832828C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832828C4: 906AE1F4  stw r3, -0x1e0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7692 as u32), ctx.r[3].u32 ) };
	// 832828C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832828CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832828D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832828D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832828D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832828D8 size=56
    let mut pc: u32 = 0x832828D8;
    'dispatch: loop {
        match pc {
            0x832828D8 => {
    //   block [0x832828D8..0x83282910)
	// 832828D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832828DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832828E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832828E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832828E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832828EC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832828F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832828F4: 4AF71465  bl 0x821f3d58
	ctx.lr = 0x832828F8;
	sub_821F3D58(ctx, base);
	// 832828F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832828FC: 906AE1F8  stw r3, -0x1e08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7688 as u32), ctx.r[3].u32 ) };
	// 83282900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328290C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282910 size=56
    let mut pc: u32 = 0x83282910;
    'dispatch: loop {
        match pc {
            0x83282910 => {
    //   block [0x83282910..0x83282948)
	// 83282910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328291C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282924: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83282928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328292C: 4AF7142D  bl 0x821f3d58
	ctx.lr = 0x83282930;
	sub_821F3D58(ctx, base);
	// 83282930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282934: 906AE1FC  stw r3, -0x1e04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7684 as u32), ctx.r[3].u32 ) };
	// 83282938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328293C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282948 size=56
    let mut pc: u32 = 0x83282948;
    'dispatch: loop {
        match pc {
            0x83282948 => {
    //   block [0x83282948..0x83282980)
	// 83282948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328295C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83282960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282964: 4AF713F5  bl 0x821f3d58
	ctx.lr = 0x83282968;
	sub_821F3D58(ctx, base);
	// 83282968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328296C: 906AE200  stw r3, -0x1e00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7680 as u32), ctx.r[3].u32 ) };
	// 83282970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328297C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282980 size=56
    let mut pc: u32 = 0x83282980;
    'dispatch: loop {
        match pc {
            0x83282980 => {
    //   block [0x83282980..0x832829B8)
	// 83282980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328298C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282994: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83282998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328299C: 4AF713BD  bl 0x821f3d58
	ctx.lr = 0x832829A0;
	sub_821F3D58(ctx, base);
	// 832829A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832829A4: 906AE204  stw r3, -0x1dfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7676 as u32), ctx.r[3].u32 ) };
	// 832829A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832829AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832829B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832829B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832829B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832829B8 size=56
    let mut pc: u32 = 0x832829B8;
    'dispatch: loop {
        match pc {
            0x832829B8 => {
    //   block [0x832829B8..0x832829F0)
	// 832829B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832829BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832829C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832829C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832829C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832829CC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832829D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832829D4: 4AF71385  bl 0x821f3d58
	ctx.lr = 0x832829D8;
	sub_821F3D58(ctx, base);
	// 832829D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832829DC: 906AE208  stw r3, -0x1df8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7672 as u32), ctx.r[3].u32 ) };
	// 832829E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832829E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832829E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832829EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832829F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832829F0 size=56
    let mut pc: u32 = 0x832829F0;
    'dispatch: loop {
        match pc {
            0x832829F0 => {
    //   block [0x832829F0..0x83282A28)
	// 832829F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832829F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832829F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832829FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282A00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282A04: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83282A08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282A0C: 4AF7134D  bl 0x821f3d58
	ctx.lr = 0x83282A10;
	sub_821F3D58(ctx, base);
	// 83282A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282A14: 906AE20C  stw r3, -0x1df4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7668 as u32), ctx.r[3].u32 ) };
	// 83282A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282A28 size=56
    let mut pc: u32 = 0x83282A28;
    'dispatch: loop {
        match pc {
            0x83282A28 => {
    //   block [0x83282A28..0x83282A60)
	// 83282A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282A34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282A38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282A3C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83282A40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282A44: 4AF71315  bl 0x821f3d58
	ctx.lr = 0x83282A48;
	sub_821F3D58(ctx, base);
	// 83282A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282A4C: 906AE210  stw r3, -0x1df0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7664 as u32), ctx.r[3].u32 ) };
	// 83282A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282A60 size=56
    let mut pc: u32 = 0x83282A60;
    'dispatch: loop {
        match pc {
            0x83282A60 => {
    //   block [0x83282A60..0x83282A98)
	// 83282A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282A6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282A70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282A74: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83282A78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282A7C: 4AF712DD  bl 0x821f3d58
	ctx.lr = 0x83282A80;
	sub_821F3D58(ctx, base);
	// 83282A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282A84: 906AE214  stw r3, -0x1dec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7660 as u32), ctx.r[3].u32 ) };
	// 83282A88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282A98 size=56
    let mut pc: u32 = 0x83282A98;
    'dispatch: loop {
        match pc {
            0x83282A98 => {
    //   block [0x83282A98..0x83282AD0)
	// 83282A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282AA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282AA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282AAC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83282AB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282AB4: 4AF712A5  bl 0x821f3d58
	ctx.lr = 0x83282AB8;
	sub_821F3D58(ctx, base);
	// 83282AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282ABC: 906AE218  stw r3, -0x1de8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7656 as u32), ctx.r[3].u32 ) };
	// 83282AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282AD0 size=56
    let mut pc: u32 = 0x83282AD0;
    'dispatch: loop {
        match pc {
            0x83282AD0 => {
    //   block [0x83282AD0..0x83282B08)
	// 83282AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282ADC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282AE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282AE4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83282AE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282AEC: 4AF7126D  bl 0x821f3d58
	ctx.lr = 0x83282AF0;
	sub_821F3D58(ctx, base);
	// 83282AF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282AF4: 906AE21C  stw r3, -0x1de4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7652 as u32), ctx.r[3].u32 ) };
	// 83282AF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282B08 size=56
    let mut pc: u32 = 0x83282B08;
    'dispatch: loop {
        match pc {
            0x83282B08 => {
    //   block [0x83282B08..0x83282B40)
	// 83282B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282B10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282B14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282B18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282B1C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83282B20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282B24: 4AF71235  bl 0x821f3d58
	ctx.lr = 0x83282B28;
	sub_821F3D58(ctx, base);
	// 83282B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282B2C: 906AE220  stw r3, -0x1de0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7648 as u32), ctx.r[3].u32 ) };
	// 83282B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282B40 size=56
    let mut pc: u32 = 0x83282B40;
    'dispatch: loop {
        match pc {
            0x83282B40 => {
    //   block [0x83282B40..0x83282B78)
	// 83282B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282B4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282B50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282B54: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83282B58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282B5C: 4AF711FD  bl 0x821f3d58
	ctx.lr = 0x83282B60;
	sub_821F3D58(ctx, base);
	// 83282B60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282B64: 906AE224  stw r3, -0x1ddc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7644 as u32), ctx.r[3].u32 ) };
	// 83282B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282B78 size=56
    let mut pc: u32 = 0x83282B78;
    'dispatch: loop {
        match pc {
            0x83282B78 => {
    //   block [0x83282B78..0x83282BB0)
	// 83282B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282B88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282B8C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83282B90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282B94: 4AF711C5  bl 0x821f3d58
	ctx.lr = 0x83282B98;
	sub_821F3D58(ctx, base);
	// 83282B98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282B9C: 906AE228  stw r3, -0x1dd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7640 as u32), ctx.r[3].u32 ) };
	// 83282BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282BB0 size=56
    let mut pc: u32 = 0x83282BB0;
    'dispatch: loop {
        match pc {
            0x83282BB0 => {
    //   block [0x83282BB0..0x83282BE8)
	// 83282BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282BBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282BC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282BC4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83282BC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282BCC: 4AF7118D  bl 0x821f3d58
	ctx.lr = 0x83282BD0;
	sub_821F3D58(ctx, base);
	// 83282BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282BD4: 906AE22C  stw r3, -0x1dd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7636 as u32), ctx.r[3].u32 ) };
	// 83282BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282BE8 size=56
    let mut pc: u32 = 0x83282BE8;
    'dispatch: loop {
        match pc {
            0x83282BE8 => {
    //   block [0x83282BE8..0x83282C20)
	// 83282BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282BF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282BFC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83282C00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282C04: 4AF71155  bl 0x821f3d58
	ctx.lr = 0x83282C08;
	sub_821F3D58(ctx, base);
	// 83282C08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282C0C: 906AE230  stw r3, -0x1dd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7632 as u32), ctx.r[3].u32 ) };
	// 83282C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282C20 size=56
    let mut pc: u32 = 0x83282C20;
    'dispatch: loop {
        match pc {
            0x83282C20 => {
    //   block [0x83282C20..0x83282C58)
	// 83282C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282C30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282C34: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83282C38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282C3C: 4AF7111D  bl 0x821f3d58
	ctx.lr = 0x83282C40;
	sub_821F3D58(ctx, base);
	// 83282C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282C44: 906AE234  stw r3, -0x1dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7628 as u32), ctx.r[3].u32 ) };
	// 83282C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282C58 size=56
    let mut pc: u32 = 0x83282C58;
    'dispatch: loop {
        match pc {
            0x83282C58 => {
    //   block [0x83282C58..0x83282C90)
	// 83282C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282C64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282C68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282C6C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83282C70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282C74: 4AF710E5  bl 0x821f3d58
	ctx.lr = 0x83282C78;
	sub_821F3D58(ctx, base);
	// 83282C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282C7C: 906AE238  stw r3, -0x1dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7624 as u32), ctx.r[3].u32 ) };
	// 83282C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282C90 size=56
    let mut pc: u32 = 0x83282C90;
    'dispatch: loop {
        match pc {
            0x83282C90 => {
    //   block [0x83282C90..0x83282CC8)
	// 83282C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282C9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282CA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282CA4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83282CA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282CAC: 4AF710AD  bl 0x821f3d58
	ctx.lr = 0x83282CB0;
	sub_821F3D58(ctx, base);
	// 83282CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282CB4: 906AE23C  stw r3, -0x1dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7620 as u32), ctx.r[3].u32 ) };
	// 83282CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282CC8 size=56
    let mut pc: u32 = 0x83282CC8;
    'dispatch: loop {
        match pc {
            0x83282CC8 => {
    //   block [0x83282CC8..0x83282D00)
	// 83282CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282CD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282CD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282CDC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83282CE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282CE4: 4AF71075  bl 0x821f3d58
	ctx.lr = 0x83282CE8;
	sub_821F3D58(ctx, base);
	// 83282CE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282CEC: 906AE240  stw r3, -0x1dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7616 as u32), ctx.r[3].u32 ) };
	// 83282CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282D00 size=56
    let mut pc: u32 = 0x83282D00;
    'dispatch: loop {
        match pc {
            0x83282D00 => {
    //   block [0x83282D00..0x83282D38)
	// 83282D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282D0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282D10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282D14: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83282D18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282D1C: 4AF7103D  bl 0x821f3d58
	ctx.lr = 0x83282D20;
	sub_821F3D58(ctx, base);
	// 83282D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282D24: 906AE244  stw r3, -0x1dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7612 as u32), ctx.r[3].u32 ) };
	// 83282D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282D38 size=64
    let mut pc: u32 = 0x83282D38;
    'dispatch: loop {
        match pc {
            0x83282D38 => {
    //   block [0x83282D38..0x83282D78)
	// 83282D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282D44: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282D4C: 388B0410  addi r4, r11, 0x410
	ctx.r[4].s64 = ctx.r[11].s64 + 1040;
	// 83282D50: 386AE248  addi r3, r10, -0x1db8
	ctx.r[3].s64 = ctx.r[10].s64 + -7608;
	// 83282D54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282D58: 4AFAA179  bl 0x8222ced0
	ctx.lr = 0x83282D5C;
	sub_8222CED0(ctx, base);
	// 83282D5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282D60: 38691340  addi r3, r9, 0x1340
	ctx.r[3].s64 = ctx.r[9].s64 + 4928;
	// 83282D64: 4BA271BD  bl 0x82ca9f20
	ctx.lr = 0x83282D68;
	sub_82CA9F20(ctx, base);
	// 83282D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282D78 size=56
    let mut pc: u32 = 0x83282D78;
    'dispatch: loop {
        match pc {
            0x83282D78 => {
    //   block [0x83282D78..0x83282DB0)
	// 83282D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282D88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282D8C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83282D90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282D94: 4AF70FC5  bl 0x821f3d58
	ctx.lr = 0x83282D98;
	sub_821F3D58(ctx, base);
	// 83282D98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282D9C: 906AE24C  stw r3, -0x1db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7604 as u32), ctx.r[3].u32 ) };
	// 83282DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282DB0 size=56
    let mut pc: u32 = 0x83282DB0;
    'dispatch: loop {
        match pc {
            0x83282DB0 => {
    //   block [0x83282DB0..0x83282DE8)
	// 83282DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282DBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282DC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282DC4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83282DC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282DCC: 4AF70F8D  bl 0x821f3d58
	ctx.lr = 0x83282DD0;
	sub_821F3D58(ctx, base);
	// 83282DD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282DD4: 906AE250  stw r3, -0x1db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7600 as u32), ctx.r[3].u32 ) };
	// 83282DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


