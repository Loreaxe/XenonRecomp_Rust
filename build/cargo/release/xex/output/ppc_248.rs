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
    //   block [0x83268C48..0x83268CD8)
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
    //   block [0x832699D8..0x83269A68)
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
    //   block [0x83269BE8..0x83269C48)
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
    //   block [0x83269C48..0x83269CA8)
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
    //   block [0x8326A168..0x8326A1A8)
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


pub fn sub_8326A690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A690 size=64
    let mut pc: u32 = 0x8326A690;
    'dispatch: loop {
        match pc {
            0x8326A690 => {
    //   block [0x8326A690..0x8326A6D0)
	// 8326A690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A69C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A6A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A6A4: 388BD958  addi r4, r11, -0x26a8
	ctx.r[4].s64 = ctx.r[11].s64 + -9896;
	// 8326A6A8: 386AC1A0  addi r3, r10, -0x3e60
	ctx.r[3].s64 = ctx.r[10].s64 + -15968;
	// 8326A6AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A6B0: 4AFC2821  bl 0x8222ced0
	ctx.lr = 0x8326A6B4;
	sub_8222CED0(ctx, base);
	// 8326A6B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A6B8: 3869E320  addi r3, r9, -0x1ce0
	ctx.r[3].s64 = ctx.r[9].s64 + -7392;
	// 8326A6BC: 4BA3F865  bl 0x82ca9f20
	ctx.lr = 0x8326A6C0;
	sub_82CA9F20(ctx, base);
	// 8326A6C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A6C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A6C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A6D0 size=64
    let mut pc: u32 = 0x8326A6D0;
    'dispatch: loop {
        match pc {
            0x8326A6D0 => {
    //   block [0x8326A6D0..0x8326A710)
	// 8326A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A6D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A6DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A6E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A6E4: 388BD96C  addi r4, r11, -0x2694
	ctx.r[4].s64 = ctx.r[11].s64 + -9876;
	// 8326A6E8: 386AC1A4  addi r3, r10, -0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15964;
	// 8326A6EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A6F0: 4AFC27E1  bl 0x8222ced0
	ctx.lr = 0x8326A6F4;
	sub_8222CED0(ctx, base);
	// 8326A6F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A6F8: 3869E330  addi r3, r9, -0x1cd0
	ctx.r[3].s64 = ctx.r[9].s64 + -7376;
	// 8326A6FC: 4BA3F825  bl 0x82ca9f20
	ctx.lr = 0x8326A700;
	sub_82CA9F20(ctx, base);
	// 8326A700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A710 size=64
    let mut pc: u32 = 0x8326A710;
    'dispatch: loop {
        match pc {
            0x8326A710 => {
    //   block [0x8326A710..0x8326A750)
	// 8326A710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A71C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A720: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A724: 388BD98C  addi r4, r11, -0x2674
	ctx.r[4].s64 = ctx.r[11].s64 + -9844;
	// 8326A728: 386AC1A8  addi r3, r10, -0x3e58
	ctx.r[3].s64 = ctx.r[10].s64 + -15960;
	// 8326A72C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A730: 4AFC27A1  bl 0x8222ced0
	ctx.lr = 0x8326A734;
	sub_8222CED0(ctx, base);
	// 8326A734: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A738: 3869E340  addi r3, r9, -0x1cc0
	ctx.r[3].s64 = ctx.r[9].s64 + -7360;
	// 8326A73C: 4BA3F7E5  bl 0x82ca9f20
	ctx.lr = 0x8326A740;
	sub_82CA9F20(ctx, base);
	// 8326A740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A750 size=64
    let mut pc: u32 = 0x8326A750;
    'dispatch: loop {
        match pc {
            0x8326A750 => {
    //   block [0x8326A750..0x8326A790)
	// 8326A750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A75C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A760: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A764: 388BD9A8  addi r4, r11, -0x2658
	ctx.r[4].s64 = ctx.r[11].s64 + -9816;
	// 8326A768: 386AC1AC  addi r3, r10, -0x3e54
	ctx.r[3].s64 = ctx.r[10].s64 + -15956;
	// 8326A76C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A770: 4AFC2761  bl 0x8222ced0
	ctx.lr = 0x8326A774;
	sub_8222CED0(ctx, base);
	// 8326A774: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A778: 3869E350  addi r3, r9, -0x1cb0
	ctx.r[3].s64 = ctx.r[9].s64 + -7344;
	// 8326A77C: 4BA3F7A5  bl 0x82ca9f20
	ctx.lr = 0x8326A780;
	sub_82CA9F20(ctx, base);
	// 8326A780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A790 size=64
    let mut pc: u32 = 0x8326A790;
    'dispatch: loop {
        match pc {
            0x8326A790 => {
    //   block [0x8326A790..0x8326A7D0)
	// 8326A790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A79C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A7A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A7A4: 388BD9C4  addi r4, r11, -0x263c
	ctx.r[4].s64 = ctx.r[11].s64 + -9788;
	// 8326A7A8: 386AC1B0  addi r3, r10, -0x3e50
	ctx.r[3].s64 = ctx.r[10].s64 + -15952;
	// 8326A7AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A7B0: 4AFC2721  bl 0x8222ced0
	ctx.lr = 0x8326A7B4;
	sub_8222CED0(ctx, base);
	// 8326A7B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A7B8: 3869E360  addi r3, r9, -0x1ca0
	ctx.r[3].s64 = ctx.r[9].s64 + -7328;
	// 8326A7BC: 4BA3F765  bl 0x82ca9f20
	ctx.lr = 0x8326A7C0;
	sub_82CA9F20(ctx, base);
	// 8326A7C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A7D0 size=64
    let mut pc: u32 = 0x8326A7D0;
    'dispatch: loop {
        match pc {
            0x8326A7D0 => {
    //   block [0x8326A7D0..0x8326A810)
	// 8326A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A7D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A7DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A7E4: 388BD9D8  addi r4, r11, -0x2628
	ctx.r[4].s64 = ctx.r[11].s64 + -9768;
	// 8326A7E8: 386AC1B4  addi r3, r10, -0x3e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15948;
	// 8326A7EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A7F0: 4AFC26E1  bl 0x8222ced0
	ctx.lr = 0x8326A7F4;
	sub_8222CED0(ctx, base);
	// 8326A7F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A7F8: 3869E370  addi r3, r9, -0x1c90
	ctx.r[3].s64 = ctx.r[9].s64 + -7312;
	// 8326A7FC: 4BA3F725  bl 0x82ca9f20
	ctx.lr = 0x8326A800;
	sub_82CA9F20(ctx, base);
	// 8326A800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A810 size=64
    let mut pc: u32 = 0x8326A810;
    'dispatch: loop {
        match pc {
            0x8326A810 => {
    //   block [0x8326A810..0x8326A850)
	// 8326A810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A81C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A824: 388BD9F4  addi r4, r11, -0x260c
	ctx.r[4].s64 = ctx.r[11].s64 + -9740;
	// 8326A828: 386AC1B8  addi r3, r10, -0x3e48
	ctx.r[3].s64 = ctx.r[10].s64 + -15944;
	// 8326A82C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A830: 4AFC26A1  bl 0x8222ced0
	ctx.lr = 0x8326A834;
	sub_8222CED0(ctx, base);
	// 8326A834: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A838: 3869E380  addi r3, r9, -0x1c80
	ctx.r[3].s64 = ctx.r[9].s64 + -7296;
	// 8326A83C: 4BA3F6E5  bl 0x82ca9f20
	ctx.lr = 0x8326A840;
	sub_82CA9F20(ctx, base);
	// 8326A840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A850 size=64
    let mut pc: u32 = 0x8326A850;
    'dispatch: loop {
        match pc {
            0x8326A850 => {
    //   block [0x8326A850..0x8326A890)
	// 8326A850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A85C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A864: 388BDA18  addi r4, r11, -0x25e8
	ctx.r[4].s64 = ctx.r[11].s64 + -9704;
	// 8326A868: 386AC1BC  addi r3, r10, -0x3e44
	ctx.r[3].s64 = ctx.r[10].s64 + -15940;
	// 8326A86C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A870: 4AFC2661  bl 0x8222ced0
	ctx.lr = 0x8326A874;
	sub_8222CED0(ctx, base);
	// 8326A874: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A878: 3869E390  addi r3, r9, -0x1c70
	ctx.r[3].s64 = ctx.r[9].s64 + -7280;
	// 8326A87C: 4BA3F6A5  bl 0x82ca9f20
	ctx.lr = 0x8326A880;
	sub_82CA9F20(ctx, base);
	// 8326A880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A890 size=64
    let mut pc: u32 = 0x8326A890;
    'dispatch: loop {
        match pc {
            0x8326A890 => {
    //   block [0x8326A890..0x8326A8D0)
	// 8326A890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A89C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A8A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A8A4: 388BDA2C  addi r4, r11, -0x25d4
	ctx.r[4].s64 = ctx.r[11].s64 + -9684;
	// 8326A8A8: 386AC1C0  addi r3, r10, -0x3e40
	ctx.r[3].s64 = ctx.r[10].s64 + -15936;
	// 8326A8AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A8B0: 4AFC2621  bl 0x8222ced0
	ctx.lr = 0x8326A8B4;
	sub_8222CED0(ctx, base);
	// 8326A8B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A8B8: 3869E3A0  addi r3, r9, -0x1c60
	ctx.r[3].s64 = ctx.r[9].s64 + -7264;
	// 8326A8BC: 4BA3F665  bl 0x82ca9f20
	ctx.lr = 0x8326A8C0;
	sub_82CA9F20(ctx, base);
	// 8326A8C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A8D0 size=64
    let mut pc: u32 = 0x8326A8D0;
    'dispatch: loop {
        match pc {
            0x8326A8D0 => {
    //   block [0x8326A8D0..0x8326A910)
	// 8326A8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A8D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A8DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A8E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A8E4: 388BDA48  addi r4, r11, -0x25b8
	ctx.r[4].s64 = ctx.r[11].s64 + -9656;
	// 8326A8E8: 386AC1C4  addi r3, r10, -0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15932;
	// 8326A8EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A8F0: 4AFC25E1  bl 0x8222ced0
	ctx.lr = 0x8326A8F4;
	sub_8222CED0(ctx, base);
	// 8326A8F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A8F8: 3869E3B0  addi r3, r9, -0x1c50
	ctx.r[3].s64 = ctx.r[9].s64 + -7248;
	// 8326A8FC: 4BA3F625  bl 0x82ca9f20
	ctx.lr = 0x8326A900;
	sub_82CA9F20(ctx, base);
	// 8326A900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A910 size=64
    let mut pc: u32 = 0x8326A910;
    'dispatch: loop {
        match pc {
            0x8326A910 => {
    //   block [0x8326A910..0x8326A950)
	// 8326A910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A91C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A924: 388BDA58  addi r4, r11, -0x25a8
	ctx.r[4].s64 = ctx.r[11].s64 + -9640;
	// 8326A928: 386AC1C8  addi r3, r10, -0x3e38
	ctx.r[3].s64 = ctx.r[10].s64 + -15928;
	// 8326A92C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A930: 4AFC25A1  bl 0x8222ced0
	ctx.lr = 0x8326A934;
	sub_8222CED0(ctx, base);
	// 8326A934: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A938: 3869E3C0  addi r3, r9, -0x1c40
	ctx.r[3].s64 = ctx.r[9].s64 + -7232;
	// 8326A93C: 4BA3F5E5  bl 0x82ca9f20
	ctx.lr = 0x8326A940;
	sub_82CA9F20(ctx, base);
	// 8326A940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A950 size=64
    let mut pc: u32 = 0x8326A950;
    'dispatch: loop {
        match pc {
            0x8326A950 => {
    //   block [0x8326A950..0x8326A990)
	// 8326A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A95C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A964: 388BDA64  addi r4, r11, -0x259c
	ctx.r[4].s64 = ctx.r[11].s64 + -9628;
	// 8326A968: 386AC1CC  addi r3, r10, -0x3e34
	ctx.r[3].s64 = ctx.r[10].s64 + -15924;
	// 8326A96C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A970: 4AFC2561  bl 0x8222ced0
	ctx.lr = 0x8326A974;
	sub_8222CED0(ctx, base);
	// 8326A974: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A978: 3869E3D0  addi r3, r9, -0x1c30
	ctx.r[3].s64 = ctx.r[9].s64 + -7216;
	// 8326A97C: 4BA3F5A5  bl 0x82ca9f20
	ctx.lr = 0x8326A980;
	sub_82CA9F20(ctx, base);
	// 8326A980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A990 size=64
    let mut pc: u32 = 0x8326A990;
    'dispatch: loop {
        match pc {
            0x8326A990 => {
    //   block [0x8326A990..0x8326A9D0)
	// 8326A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A99C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A9A4: 388BDA7C  addi r4, r11, -0x2584
	ctx.r[4].s64 = ctx.r[11].s64 + -9604;
	// 8326A9A8: 386AC1D0  addi r3, r10, -0x3e30
	ctx.r[3].s64 = ctx.r[10].s64 + -15920;
	// 8326A9AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A9B0: 4AFC2521  bl 0x8222ced0
	ctx.lr = 0x8326A9B4;
	sub_8222CED0(ctx, base);
	// 8326A9B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A9B8: 3869E3E0  addi r3, r9, -0x1c20
	ctx.r[3].s64 = ctx.r[9].s64 + -7200;
	// 8326A9BC: 4BA3F565  bl 0x82ca9f20
	ctx.lr = 0x8326A9C0;
	sub_82CA9F20(ctx, base);
	// 8326A9C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326A9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326A9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326A9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326A9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A9D0 size=64
    let mut pc: u32 = 0x8326A9D0;
    'dispatch: loop {
        match pc {
            0x8326A9D0 => {
    //   block [0x8326A9D0..0x8326AA10)
	// 8326A9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A9D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326A9DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326A9E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326A9E4: 388BDA98  addi r4, r11, -0x2568
	ctx.r[4].s64 = ctx.r[11].s64 + -9576;
	// 8326A9E8: 386AC1D4  addi r3, r10, -0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15916;
	// 8326A9EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326A9F0: 4AFC24E1  bl 0x8222ced0
	ctx.lr = 0x8326A9F4;
	sub_8222CED0(ctx, base);
	// 8326A9F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326A9F8: 3869E3F0  addi r3, r9, -0x1c10
	ctx.r[3].s64 = ctx.r[9].s64 + -7184;
	// 8326A9FC: 4BA3F525  bl 0x82ca9f20
	ctx.lr = 0x8326AA00;
	sub_82CA9F20(ctx, base);
	// 8326AA00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA10 size=64
    let mut pc: u32 = 0x8326AA10;
    'dispatch: loop {
        match pc {
            0x8326AA10 => {
    //   block [0x8326AA10..0x8326AA50)
	// 8326AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AA1C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AA20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AA24: 388BDAAC  addi r4, r11, -0x2554
	ctx.r[4].s64 = ctx.r[11].s64 + -9556;
	// 8326AA28: 386AC1D8  addi r3, r10, -0x3e28
	ctx.r[3].s64 = ctx.r[10].s64 + -15912;
	// 8326AA2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AA30: 4AFC24A1  bl 0x8222ced0
	ctx.lr = 0x8326AA34;
	sub_8222CED0(ctx, base);
	// 8326AA34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AA38: 3869E400  addi r3, r9, -0x1c00
	ctx.r[3].s64 = ctx.r[9].s64 + -7168;
	// 8326AA3C: 4BA3F4E5  bl 0x82ca9f20
	ctx.lr = 0x8326AA40;
	sub_82CA9F20(ctx, base);
	// 8326AA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA50 size=64
    let mut pc: u32 = 0x8326AA50;
    'dispatch: loop {
        match pc {
            0x8326AA50 => {
    //   block [0x8326AA50..0x8326AA90)
	// 8326AA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AA5C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AA60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AA64: 388BDAC4  addi r4, r11, -0x253c
	ctx.r[4].s64 = ctx.r[11].s64 + -9532;
	// 8326AA68: 386AC1DC  addi r3, r10, -0x3e24
	ctx.r[3].s64 = ctx.r[10].s64 + -15908;
	// 8326AA6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AA70: 4AFC2461  bl 0x8222ced0
	ctx.lr = 0x8326AA74;
	sub_8222CED0(ctx, base);
	// 8326AA74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AA78: 3869E410  addi r3, r9, -0x1bf0
	ctx.r[3].s64 = ctx.r[9].s64 + -7152;
	// 8326AA7C: 4BA3F4A5  bl 0x82ca9f20
	ctx.lr = 0x8326AA80;
	sub_82CA9F20(ctx, base);
	// 8326AA80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA90 size=64
    let mut pc: u32 = 0x8326AA90;
    'dispatch: loop {
        match pc {
            0x8326AA90 => {
    //   block [0x8326AA90..0x8326AAD0)
	// 8326AA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AA9C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AAA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AAA4: 388BDAD4  addi r4, r11, -0x252c
	ctx.r[4].s64 = ctx.r[11].s64 + -9516;
	// 8326AAA8: 386AC1E0  addi r3, r10, -0x3e20
	ctx.r[3].s64 = ctx.r[10].s64 + -15904;
	// 8326AAAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AAB0: 4AFC2421  bl 0x8222ced0
	ctx.lr = 0x8326AAB4;
	sub_8222CED0(ctx, base);
	// 8326AAB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AAB8: 3869E420  addi r3, r9, -0x1be0
	ctx.r[3].s64 = ctx.r[9].s64 + -7136;
	// 8326AABC: 4BA3F465  bl 0x82ca9f20
	ctx.lr = 0x8326AAC0;
	sub_82CA9F20(ctx, base);
	// 8326AAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AAD0 size=64
    let mut pc: u32 = 0x8326AAD0;
    'dispatch: loop {
        match pc {
            0x8326AAD0 => {
    //   block [0x8326AAD0..0x8326AB10)
	// 8326AAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AADC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AAE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AAE4: 388BDAE8  addi r4, r11, -0x2518
	ctx.r[4].s64 = ctx.r[11].s64 + -9496;
	// 8326AAE8: 386AC1E4  addi r3, r10, -0x3e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -15900;
	// 8326AAEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AAF0: 4AFC23E1  bl 0x8222ced0
	ctx.lr = 0x8326AAF4;
	sub_8222CED0(ctx, base);
	// 8326AAF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AAF8: 3869E430  addi r3, r9, -0x1bd0
	ctx.r[3].s64 = ctx.r[9].s64 + -7120;
	// 8326AAFC: 4BA3F425  bl 0x82ca9f20
	ctx.lr = 0x8326AB00;
	sub_82CA9F20(ctx, base);
	// 8326AB00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB10 size=56
    let mut pc: u32 = 0x8326AB10;
    'dispatch: loop {
        match pc {
            0x8326AB10 => {
    //   block [0x8326AB10..0x8326AB48)
	// 8326AB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AB18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AB1C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AB20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326AB24: 386BDAFC  addi r3, r11, -0x2504
	ctx.r[3].s64 = ctx.r[11].s64 + -9476;
	// 8326AB28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326AB2C: 4AF8922D  bl 0x821f3d58
	ctx.lr = 0x8326AB30;
	sub_821F3D58(ctx, base);
	// 8326AB30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AB34: 906AC1E8  stw r3, -0x3e18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15896 as u32), ctx.r[3].u32 ) };
	// 8326AB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB48 size=56
    let mut pc: u32 = 0x8326AB48;
    'dispatch: loop {
        match pc {
            0x8326AB48 => {
    //   block [0x8326AB48..0x8326AB80)
	// 8326AB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AB50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AB54: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AB58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326AB5C: 386BDB10  addi r3, r11, -0x24f0
	ctx.r[3].s64 = ctx.r[11].s64 + -9456;
	// 8326AB60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326AB64: 4AF891F5  bl 0x821f3d58
	ctx.lr = 0x8326AB68;
	sub_821F3D58(ctx, base);
	// 8326AB68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AB6C: 906AC1EC  stw r3, -0x3e14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15892 as u32), ctx.r[3].u32 ) };
	// 8326AB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB80 size=56
    let mut pc: u32 = 0x8326AB80;
    'dispatch: loop {
        match pc {
            0x8326AB80 => {
    //   block [0x8326AB80..0x8326ABB8)
	// 8326AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AB8C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AB90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326AB94: 386BDB24  addi r3, r11, -0x24dc
	ctx.r[3].s64 = ctx.r[11].s64 + -9436;
	// 8326AB98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326AB9C: 4AF891BD  bl 0x821f3d58
	ctx.lr = 0x8326ABA0;
	sub_821F3D58(ctx, base);
	// 8326ABA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ABA4: 906AC1F0  stw r3, -0x3e10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15888 as u32), ctx.r[3].u32 ) };
	// 8326ABA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ABAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ABB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ABB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ABB8 size=56
    let mut pc: u32 = 0x8326ABB8;
    'dispatch: loop {
        match pc {
            0x8326ABB8 => {
    //   block [0x8326ABB8..0x8326ABF0)
	// 8326ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ABBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ABC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ABC4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326ABC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326ABCC: 386BDB38  addi r3, r11, -0x24c8
	ctx.r[3].s64 = ctx.r[11].s64 + -9416;
	// 8326ABD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326ABD4: 4AF89185  bl 0x821f3d58
	ctx.lr = 0x8326ABD8;
	sub_821F3D58(ctx, base);
	// 8326ABD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ABDC: 906AC1F4  stw r3, -0x3e0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15884 as u32), ctx.r[3].u32 ) };
	// 8326ABE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ABE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ABE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ABEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ABF0 size=64
    let mut pc: u32 = 0x8326ABF0;
    'dispatch: loop {
        match pc {
            0x8326ABF0 => {
    //   block [0x8326ABF0..0x8326AC30)
	// 8326ABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ABF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ABF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ABFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AC00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AC04: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8326AC08: 386AC1F8  addi r3, r10, -0x3e08
	ctx.r[3].s64 = ctx.r[10].s64 + -15880;
	// 8326AC0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AC10: 4AFC22C1  bl 0x8222ced0
	ctx.lr = 0x8326AC14;
	sub_8222CED0(ctx, base);
	// 8326AC14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AC18: 3869E440  addi r3, r9, -0x1bc0
	ctx.r[3].s64 = ctx.r[9].s64 + -7104;
	// 8326AC1C: 4BA3F305  bl 0x82ca9f20
	ctx.lr = 0x8326AC20;
	sub_82CA9F20(ctx, base);
	// 8326AC20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AC30 size=64
    let mut pc: u32 = 0x8326AC30;
    'dispatch: loop {
        match pc {
            0x8326AC30 => {
    //   block [0x8326AC30..0x8326AC70)
	// 8326AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AC38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AC3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AC44: 388BE14C  addi r4, r11, -0x1eb4
	ctx.r[4].s64 = ctx.r[11].s64 + -7860;
	// 8326AC48: 386AC1FC  addi r3, r10, -0x3e04
	ctx.r[3].s64 = ctx.r[10].s64 + -15876;
	// 8326AC4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AC50: 4AFC2281  bl 0x8222ced0
	ctx.lr = 0x8326AC54;
	sub_8222CED0(ctx, base);
	// 8326AC54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AC58: 3869E450  addi r3, r9, -0x1bb0
	ctx.r[3].s64 = ctx.r[9].s64 + -7088;
	// 8326AC5C: 4BA3F2C5  bl 0x82ca9f20
	ctx.lr = 0x8326AC60;
	sub_82CA9F20(ctx, base);
	// 8326AC60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AC70 size=64
    let mut pc: u32 = 0x8326AC70;
    'dispatch: loop {
        match pc {
            0x8326AC70 => {
    //   block [0x8326AC70..0x8326ACB0)
	// 8326AC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AC7C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AC80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AC84: 388BE160  addi r4, r11, -0x1ea0
	ctx.r[4].s64 = ctx.r[11].s64 + -7840;
	// 8326AC88: 386AC200  addi r3, r10, -0x3e00
	ctx.r[3].s64 = ctx.r[10].s64 + -15872;
	// 8326AC8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AC90: 4AFC2241  bl 0x8222ced0
	ctx.lr = 0x8326AC94;
	sub_8222CED0(ctx, base);
	// 8326AC94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AC98: 3869E460  addi r3, r9, -0x1ba0
	ctx.r[3].s64 = ctx.r[9].s64 + -7072;
	// 8326AC9C: 4BA3F285  bl 0x82ca9f20
	ctx.lr = 0x8326ACA0;
	sub_82CA9F20(ctx, base);
	// 8326ACA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ACA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ACA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ACAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ACB0 size=64
    let mut pc: u32 = 0x8326ACB0;
    'dispatch: loop {
        match pc {
            0x8326ACB0 => {
    //   block [0x8326ACB0..0x8326ACF0)
	// 8326ACB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ACB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ACB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ACBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326ACC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ACC4: 388BE184  addi r4, r11, -0x1e7c
	ctx.r[4].s64 = ctx.r[11].s64 + -7804;
	// 8326ACC8: 386AC204  addi r3, r10, -0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -15868;
	// 8326ACCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ACD0: 4AFC2201  bl 0x8222ced0
	ctx.lr = 0x8326ACD4;
	sub_8222CED0(ctx, base);
	// 8326ACD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ACD8: 3869E470  addi r3, r9, -0x1b90
	ctx.r[3].s64 = ctx.r[9].s64 + -7056;
	// 8326ACDC: 4BA3F245  bl 0x82ca9f20
	ctx.lr = 0x8326ACE0;
	sub_82CA9F20(ctx, base);
	// 8326ACE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ACE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ACE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ACEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ACF0 size=64
    let mut pc: u32 = 0x8326ACF0;
    'dispatch: loop {
        match pc {
            0x8326ACF0 => {
    //   block [0x8326ACF0..0x8326AD30)
	// 8326ACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ACF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ACFC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AD00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AD04: 388BE1A0  addi r4, r11, -0x1e60
	ctx.r[4].s64 = ctx.r[11].s64 + -7776;
	// 8326AD08: 386AC208  addi r3, r10, -0x3df8
	ctx.r[3].s64 = ctx.r[10].s64 + -15864;
	// 8326AD0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AD10: 4AFC21C1  bl 0x8222ced0
	ctx.lr = 0x8326AD14;
	sub_8222CED0(ctx, base);
	// 8326AD14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AD18: 3869E480  addi r3, r9, -0x1b80
	ctx.r[3].s64 = ctx.r[9].s64 + -7040;
	// 8326AD1C: 4BA3F205  bl 0x82ca9f20
	ctx.lr = 0x8326AD20;
	sub_82CA9F20(ctx, base);
	// 8326AD20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AD30 size=64
    let mut pc: u32 = 0x8326AD30;
    'dispatch: loop {
        match pc {
            0x8326AD30 => {
    //   block [0x8326AD30..0x8326AD70)
	// 8326AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AD38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AD3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AD44: 388BE1BC  addi r4, r11, -0x1e44
	ctx.r[4].s64 = ctx.r[11].s64 + -7748;
	// 8326AD48: 386AC20C  addi r3, r10, -0x3df4
	ctx.r[3].s64 = ctx.r[10].s64 + -15860;
	// 8326AD4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AD50: 4AFC2181  bl 0x8222ced0
	ctx.lr = 0x8326AD54;
	sub_8222CED0(ctx, base);
	// 8326AD54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AD58: 3869E490  addi r3, r9, -0x1b70
	ctx.r[3].s64 = ctx.r[9].s64 + -7024;
	// 8326AD5C: 4BA3F1C5  bl 0x82ca9f20
	ctx.lr = 0x8326AD60;
	sub_82CA9F20(ctx, base);
	// 8326AD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AD70 size=64
    let mut pc: u32 = 0x8326AD70;
    'dispatch: loop {
        match pc {
            0x8326AD70 => {
    //   block [0x8326AD70..0x8326ADB0)
	// 8326AD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AD7C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AD80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AD84: 388BA398  addi r4, r11, -0x5c68
	ctx.r[4].s64 = ctx.r[11].s64 + -23656;
	// 8326AD88: 386AC210  addi r3, r10, -0x3df0
	ctx.r[3].s64 = ctx.r[10].s64 + -15856;
	// 8326AD8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AD90: 4AFC2141  bl 0x8222ced0
	ctx.lr = 0x8326AD94;
	sub_8222CED0(ctx, base);
	// 8326AD94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AD98: 3869E4A0  addi r3, r9, -0x1b60
	ctx.r[3].s64 = ctx.r[9].s64 + -7008;
	// 8326AD9C: 4BA3F185  bl 0x82ca9f20
	ctx.lr = 0x8326ADA0;
	sub_82CA9F20(ctx, base);
	// 8326ADA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ADA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ADA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ADAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ADB0 size=64
    let mut pc: u32 = 0x8326ADB0;
    'dispatch: loop {
        match pc {
            0x8326ADB0 => {
    //   block [0x8326ADB0..0x8326ADF0)
	// 8326ADB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ADB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ADB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ADBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326ADC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ADC4: 388BE1DC  addi r4, r11, -0x1e24
	ctx.r[4].s64 = ctx.r[11].s64 + -7716;
	// 8326ADC8: 386AC214  addi r3, r10, -0x3dec
	ctx.r[3].s64 = ctx.r[10].s64 + -15852;
	// 8326ADCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ADD0: 4AFC2101  bl 0x8222ced0
	ctx.lr = 0x8326ADD4;
	sub_8222CED0(ctx, base);
	// 8326ADD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ADD8: 3869E4B0  addi r3, r9, -0x1b50
	ctx.r[3].s64 = ctx.r[9].s64 + -6992;
	// 8326ADDC: 4BA3F145  bl 0x82ca9f20
	ctx.lr = 0x8326ADE0;
	sub_82CA9F20(ctx, base);
	// 8326ADE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ADE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ADE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ADEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ADF0 size=64
    let mut pc: u32 = 0x8326ADF0;
    'dispatch: loop {
        match pc {
            0x8326ADF0 => {
    //   block [0x8326ADF0..0x8326AE30)
	// 8326ADF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ADF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ADF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ADFC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AE00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AE04: 388BE1FC  addi r4, r11, -0x1e04
	ctx.r[4].s64 = ctx.r[11].s64 + -7684;
	// 8326AE08: 386AC218  addi r3, r10, -0x3de8
	ctx.r[3].s64 = ctx.r[10].s64 + -15848;
	// 8326AE0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AE10: 4AFC20C1  bl 0x8222ced0
	ctx.lr = 0x8326AE14;
	sub_8222CED0(ctx, base);
	// 8326AE14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AE18: 3869E4C0  addi r3, r9, -0x1b40
	ctx.r[3].s64 = ctx.r[9].s64 + -6976;
	// 8326AE1C: 4BA3F105  bl 0x82ca9f20
	ctx.lr = 0x8326AE20;
	sub_82CA9F20(ctx, base);
	// 8326AE20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AE30 size=64
    let mut pc: u32 = 0x8326AE30;
    'dispatch: loop {
        match pc {
            0x8326AE30 => {
    //   block [0x8326AE30..0x8326AE70)
	// 8326AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AE3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AE40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AE44: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326AE48: 386AC21C  addi r3, r10, -0x3de4
	ctx.r[3].s64 = ctx.r[10].s64 + -15844;
	// 8326AE4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AE50: 4AFC2081  bl 0x8222ced0
	ctx.lr = 0x8326AE54;
	sub_8222CED0(ctx, base);
	// 8326AE54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AE58: 3869E4D0  addi r3, r9, -0x1b30
	ctx.r[3].s64 = ctx.r[9].s64 + -6960;
	// 8326AE5C: 4BA3F0C5  bl 0x82ca9f20
	ctx.lr = 0x8326AE60;
	sub_82CA9F20(ctx, base);
	// 8326AE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AE70 size=64
    let mut pc: u32 = 0x8326AE70;
    'dispatch: loop {
        match pc {
            0x8326AE70 => {
    //   block [0x8326AE70..0x8326AEB0)
	// 8326AE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AE7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AE80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AE84: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326AE88: 386AC220  addi r3, r10, -0x3de0
	ctx.r[3].s64 = ctx.r[10].s64 + -15840;
	// 8326AE8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AE90: 4AFC2041  bl 0x8222ced0
	ctx.lr = 0x8326AE94;
	sub_8222CED0(ctx, base);
	// 8326AE94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AE98: 3869E4E0  addi r3, r9, -0x1b20
	ctx.r[3].s64 = ctx.r[9].s64 + -6944;
	// 8326AE9C: 4BA3F085  bl 0x82ca9f20
	ctx.lr = 0x8326AEA0;
	sub_82CA9F20(ctx, base);
	// 8326AEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AEB0 size=64
    let mut pc: u32 = 0x8326AEB0;
    'dispatch: loop {
        match pc {
            0x8326AEB0 => {
    //   block [0x8326AEB0..0x8326AEF0)
	// 8326AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AEBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326AEC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AEC4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326AEC8: 386AC224  addi r3, r10, -0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -15836;
	// 8326AECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AED0: 4AFC2001  bl 0x8222ced0
	ctx.lr = 0x8326AED4;
	sub_8222CED0(ctx, base);
	// 8326AED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326AED8: 3869E4F0  addi r3, r9, -0x1b10
	ctx.r[3].s64 = ctx.r[9].s64 + -6928;
	// 8326AEDC: 4BA3F045  bl 0x82ca9f20
	ctx.lr = 0x8326AEE0;
	sub_82CA9F20(ctx, base);
	// 8326AEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326AEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AEF0 size=192
    let mut pc: u32 = 0x8326AEF0;
    'dispatch: loop {
        match pc {
            0x8326AEF0 => {
    //   block [0x8326AEF0..0x8326AFB0)
	// 8326AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326AEFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AF00: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AF04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AF08: 388BE364  addi r4, r11, -0x1c9c
	ctx.r[4].s64 = ctx.r[11].s64 + -7324;
	// 8326AF0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326AF10: 4AFC1FC1  bl 0x8222ced0
	ctx.lr = 0x8326AF14;
	sub_8222CED0(ctx, base);
	// 8326AF14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326AF18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AF1C: 4AF83C1D  bl 0x821eeb38
	ctx.lr = 0x8326AF20;
	sub_821EEB38(ctx, base);
	// 8326AF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AF24: 4B9988CD  bl 0x82c037f0
	ctx.lr = 0x8326AF28;
	sub_82C037F0(ctx, base);
	// 8326AF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AF2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326AF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AF34: 916AC228  stw r11, -0x3dd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15832 as u32), ctx.r[11].u32 ) };
	// 8326AF38: 4AF5B831  bl 0x821c6768
	ctx.lr = 0x8326AF3C;
	sub_821C6768(ctx, base);
	// 8326AF3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326AF40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326AF44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8326AF48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326AF4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326AF54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326AF58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326AF5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF60: 4082FFE8  bne 0x8326af48
	if !ctx.cr[0].eq {
	pc = 0x8326AF48; continue 'dispatch;
	}
	// 8326AF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326AF68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326AF6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326AF70: 4AF5B7F9  bl 0x821c6768
	ctx.lr = 0x8326AF74;
	sub_821C6768(ctx, base);
	// 8326AF74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326AF78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326AF80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326AF84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326AF88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326AF8C: 4082FFE8  bne 0x8326af74
	if !ctx.cr[0].eq {
	pc = 0x8326AF74; continue 'dispatch;
	}
	// 8326AF90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326AF94: 386BE500  addi r3, r11, -0x1b00
	ctx.r[3].s64 = ctx.r[11].s64 + -6912;
	// 8326AF98: 4BA3EF89  bl 0x82ca9f20
	ctx.lr = 0x8326AF9C;
	sub_82CA9F20(ctx, base);
	// 8326AF9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326AFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326AFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326AFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326AFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326AFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AFB0 size=192
    let mut pc: u32 = 0x8326AFB0;
    'dispatch: loop {
        match pc {
            0x8326AFB0 => {
    //   block [0x8326AFB0..0x8326B070)
	// 8326AFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326AFBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326AFC0: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326AFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326AFC8: 388BE394  addi r4, r11, -0x1c6c
	ctx.r[4].s64 = ctx.r[11].s64 + -7276;
	// 8326AFCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326AFD0: 4AFC1F01  bl 0x8222ced0
	ctx.lr = 0x8326AFD4;
	sub_8222CED0(ctx, base);
	// 8326AFD4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326AFD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AFDC: 4AF83B5D  bl 0x821eeb38
	ctx.lr = 0x8326AFE0;
	sub_821EEB38(ctx, base);
	// 8326AFE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AFE4: 4B99880D  bl 0x82c037f0
	ctx.lr = 0x8326AFE8;
	sub_82C037F0(ctx, base);
	// 8326AFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326AFEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326AFF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326AFF4: 916AC22C  stw r11, -0x3dd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15828 as u32), ctx.r[11].u32 ) };
	// 8326AFF8: 4AF5B771  bl 0x821c6768
	ctx.lr = 0x8326AFFC;
	sub_821C6768(ctx, base);
	// 8326AFFC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B000: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B004: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8326B008: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B00C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B010: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B014: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B018: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B01C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B020: 4082FFE8  bne 0x8326b008
	if !ctx.cr[0].eq {
	pc = 0x8326B008; continue 'dispatch;
	}
	// 8326B024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B028: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B02C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B030: 4AF5B739  bl 0x821c6768
	ctx.lr = 0x8326B034;
	sub_821C6768(ctx, base);
	// 8326B034: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B038: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B03C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B040: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B044: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B048: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B04C: 4082FFE8  bne 0x8326b034
	if !ctx.cr[0].eq {
	pc = 0x8326B034; continue 'dispatch;
	}
	// 8326B050: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B054: 386BE508  addi r3, r11, -0x1af8
	ctx.r[3].s64 = ctx.r[11].s64 + -6904;
	// 8326B058: 4BA3EEC9  bl 0x82ca9f20
	ctx.lr = 0x8326B05C;
	sub_82CA9F20(ctx, base);
	// 8326B05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B070 size=192
    let mut pc: u32 = 0x8326B070;
    'dispatch: loop {
        match pc {
            0x8326B070 => {
    //   block [0x8326B070..0x8326B130)
	// 8326B070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326B07C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B080: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B088: 388BE3C0  addi r4, r11, -0x1c40
	ctx.r[4].s64 = ctx.r[11].s64 + -7232;
	// 8326B08C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B090: 4AFC1E41  bl 0x8222ced0
	ctx.lr = 0x8326B094;
	sub_8222CED0(ctx, base);
	// 8326B094: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326B098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B09C: 4AF83A9D  bl 0x821eeb38
	ctx.lr = 0x8326B0A0;
	sub_821EEB38(ctx, base);
	// 8326B0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B0A4: 4B99874D  bl 0x82c037f0
	ctx.lr = 0x8326B0A8;
	sub_82C037F0(ctx, base);
	// 8326B0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B0AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326B0B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B0B4: 916AC230  stw r11, -0x3dd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15824 as u32), ctx.r[11].u32 ) };
	// 8326B0B8: 4AF5B6B1  bl 0x821c6768
	ctx.lr = 0x8326B0BC;
	sub_821C6768(ctx, base);
	// 8326B0BC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B0C0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B0C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8326B0C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B0CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B0D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B0D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B0D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B0DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B0E0: 4082FFE8  bne 0x8326b0c8
	if !ctx.cr[0].eq {
	pc = 0x8326B0C8; continue 'dispatch;
	}
	// 8326B0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B0E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B0EC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B0F0: 4AF5B679  bl 0x821c6768
	ctx.lr = 0x8326B0F4;
	sub_821C6768(ctx, base);
	// 8326B0F4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B0F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B0FC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B100: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B104: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B108: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B10C: 4082FFE8  bne 0x8326b0f4
	if !ctx.cr[0].eq {
	pc = 0x8326B0F4; continue 'dispatch;
	}
	// 8326B110: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B114: 386BE510  addi r3, r11, -0x1af0
	ctx.r[3].s64 = ctx.r[11].s64 + -6896;
	// 8326B118: 4BA3EE09  bl 0x82ca9f20
	ctx.lr = 0x8326B11C;
	sub_82CA9F20(ctx, base);
	// 8326B11C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B130 size=192
    let mut pc: u32 = 0x8326B130;
    'dispatch: loop {
        match pc {
            0x8326B130 => {
    //   block [0x8326B130..0x8326B1F0)
	// 8326B130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326B13C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B140: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B148: 388BE3F0  addi r4, r11, -0x1c10
	ctx.r[4].s64 = ctx.r[11].s64 + -7184;
	// 8326B14C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B150: 4AFC1D81  bl 0x8222ced0
	ctx.lr = 0x8326B154;
	sub_8222CED0(ctx, base);
	// 8326B154: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326B158: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B15C: 4AF839DD  bl 0x821eeb38
	ctx.lr = 0x8326B160;
	sub_821EEB38(ctx, base);
	// 8326B160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B164: 4B99868D  bl 0x82c037f0
	ctx.lr = 0x8326B168;
	sub_82C037F0(ctx, base);
	// 8326B168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B16C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326B170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B174: 916AC234  stw r11, -0x3dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15820 as u32), ctx.r[11].u32 ) };
	// 8326B178: 4AF5B5F1  bl 0x821c6768
	ctx.lr = 0x8326B17C;
	sub_821C6768(ctx, base);
	// 8326B17C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B180: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B184: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8326B188: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B18C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B190: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B194: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B198: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B19C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B1A0: 4082FFE8  bne 0x8326b188
	if !ctx.cr[0].eq {
	pc = 0x8326B188; continue 'dispatch;
	}
	// 8326B1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B1A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B1AC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B1B0: 4AF5B5B9  bl 0x821c6768
	ctx.lr = 0x8326B1B4;
	sub_821C6768(ctx, base);
	// 8326B1B4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B1B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B1BC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B1C0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B1C4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B1C8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B1CC: 4082FFE8  bne 0x8326b1b4
	if !ctx.cr[0].eq {
	pc = 0x8326B1B4; continue 'dispatch;
	}
	// 8326B1D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B1D4: 386BE518  addi r3, r11, -0x1ae8
	ctx.r[3].s64 = ctx.r[11].s64 + -6888;
	// 8326B1D8: 4BA3ED49  bl 0x82ca9f20
	ctx.lr = 0x8326B1DC;
	sub_82CA9F20(ctx, base);
	// 8326B1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B1E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B1F0 size=192
    let mut pc: u32 = 0x8326B1F0;
    'dispatch: loop {
        match pc {
            0x8326B1F0 => {
    //   block [0x8326B1F0..0x8326B2B0)
	// 8326B1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326B1FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B200: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B208: 388BE420  addi r4, r11, -0x1be0
	ctx.r[4].s64 = ctx.r[11].s64 + -7136;
	// 8326B20C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B210: 4AFC1CC1  bl 0x8222ced0
	ctx.lr = 0x8326B214;
	sub_8222CED0(ctx, base);
	// 8326B214: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8326B218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B21C: 4AF8391D  bl 0x821eeb38
	ctx.lr = 0x8326B220;
	sub_821EEB38(ctx, base);
	// 8326B220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B224: 4B9985CD  bl 0x82c037f0
	ctx.lr = 0x8326B228;
	sub_82C037F0(ctx, base);
	// 8326B228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B22C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326B230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326B234: 916AC238  stw r11, -0x3dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15816 as u32), ctx.r[11].u32 ) };
	// 8326B238: 4AF5B531  bl 0x821c6768
	ctx.lr = 0x8326B23C;
	sub_821C6768(ctx, base);
	// 8326B23C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8326B240: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326B244: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8326B248: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8326B24C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B250: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8326B254: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8326B258: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B25C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B260: 4082FFE8  bne 0x8326b248
	if !ctx.cr[0].eq {
	pc = 0x8326B248; continue 'dispatch;
	}
	// 8326B264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8326B268: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326B26C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8326B270: 4AF5B4F9  bl 0x821c6768
	ctx.lr = 0x8326B274;
	sub_821C6768(ctx, base);
	// 8326B274: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326B278: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B27C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326B280: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326B284: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326B288: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326B28C: 4082FFE8  bne 0x8326b274
	if !ctx.cr[0].eq {
	pc = 0x8326B274; continue 'dispatch;
	}
	// 8326B290: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326B294: 386BE520  addi r3, r11, -0x1ae0
	ctx.r[3].s64 = ctx.r[11].s64 + -6880;
	// 8326B298: 4BA3EC89  bl 0x82ca9f20
	ctx.lr = 0x8326B29C;
	sub_82CA9F20(ctx, base);
	// 8326B29C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326B2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B2A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326B2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B2B0 size=64
    let mut pc: u32 = 0x8326B2B0;
    'dispatch: loop {
        match pc {
            0x8326B2B0 => {
    //   block [0x8326B2B0..0x8326B2F0)
	// 8326B2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B2BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B2C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B2C4: 388B37B0  addi r4, r11, 0x37b0
	ctx.r[4].s64 = ctx.r[11].s64 + 14256;
	// 8326B2C8: 386AC23C  addi r3, r10, -0x3dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15812;
	// 8326B2CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B2D0: 4AFC1C01  bl 0x8222ced0
	ctx.lr = 0x8326B2D4;
	sub_8222CED0(ctx, base);
	// 8326B2D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B2D8: 3869E548  addi r3, r9, -0x1ab8
	ctx.r[3].s64 = ctx.r[9].s64 + -6840;
	// 8326B2DC: 4BA3EC45  bl 0x82ca9f20
	ctx.lr = 0x8326B2E0;
	sub_82CA9F20(ctx, base);
	// 8326B2E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B2F0 size=64
    let mut pc: u32 = 0x8326B2F0;
    'dispatch: loop {
        match pc {
            0x8326B2F0 => {
    //   block [0x8326B2F0..0x8326B330)
	// 8326B2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B2F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B2FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B304: 388BEB24  addi r4, r11, -0x14dc
	ctx.r[4].s64 = ctx.r[11].s64 + -5340;
	// 8326B308: 386AC240  addi r3, r10, -0x3dc0
	ctx.r[3].s64 = ctx.r[10].s64 + -15808;
	// 8326B30C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B310: 4AFC1BC1  bl 0x8222ced0
	ctx.lr = 0x8326B314;
	sub_8222CED0(ctx, base);
	// 8326B314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B318: 3869E558  addi r3, r9, -0x1aa8
	ctx.r[3].s64 = ctx.r[9].s64 + -6824;
	// 8326B31C: 4BA3EC05  bl 0x82ca9f20
	ctx.lr = 0x8326B320;
	sub_82CA9F20(ctx, base);
	// 8326B320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B330 size=64
    let mut pc: u32 = 0x8326B330;
    'dispatch: loop {
        match pc {
            0x8326B330 => {
    //   block [0x8326B330..0x8326B370)
	// 8326B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B33C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B344: 388BEB30  addi r4, r11, -0x14d0
	ctx.r[4].s64 = ctx.r[11].s64 + -5328;
	// 8326B348: 386AC244  addi r3, r10, -0x3dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15804;
	// 8326B34C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B350: 4AFC1B81  bl 0x8222ced0
	ctx.lr = 0x8326B354;
	sub_8222CED0(ctx, base);
	// 8326B354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B358: 3869E568  addi r3, r9, -0x1a98
	ctx.r[3].s64 = ctx.r[9].s64 + -6808;
	// 8326B35C: 4BA3EBC5  bl 0x82ca9f20
	ctx.lr = 0x8326B360;
	sub_82CA9F20(ctx, base);
	// 8326B360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B370 size=64
    let mut pc: u32 = 0x8326B370;
    'dispatch: loop {
        match pc {
            0x8326B370 => {
    //   block [0x8326B370..0x8326B3B0)
	// 8326B370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B37C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B384: 388BEB40  addi r4, r11, -0x14c0
	ctx.r[4].s64 = ctx.r[11].s64 + -5312;
	// 8326B388: 386AC248  addi r3, r10, -0x3db8
	ctx.r[3].s64 = ctx.r[10].s64 + -15800;
	// 8326B38C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B390: 4AFC1B41  bl 0x8222ced0
	ctx.lr = 0x8326B394;
	sub_8222CED0(ctx, base);
	// 8326B394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B398: 3869E578  addi r3, r9, -0x1a88
	ctx.r[3].s64 = ctx.r[9].s64 + -6792;
	// 8326B39C: 4BA3EB85  bl 0x82ca9f20
	ctx.lr = 0x8326B3A0;
	sub_82CA9F20(ctx, base);
	// 8326B3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B3B0 size=64
    let mut pc: u32 = 0x8326B3B0;
    'dispatch: loop {
        match pc {
            0x8326B3B0 => {
    //   block [0x8326B3B0..0x8326B3F0)
	// 8326B3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B3BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B3C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B3C4: 388BEB50  addi r4, r11, -0x14b0
	ctx.r[4].s64 = ctx.r[11].s64 + -5296;
	// 8326B3C8: 386AC24C  addi r3, r10, -0x3db4
	ctx.r[3].s64 = ctx.r[10].s64 + -15796;
	// 8326B3CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B3D0: 4AFC1B01  bl 0x8222ced0
	ctx.lr = 0x8326B3D4;
	sub_8222CED0(ctx, base);
	// 8326B3D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B3D8: 3869E588  addi r3, r9, -0x1a78
	ctx.r[3].s64 = ctx.r[9].s64 + -6776;
	// 8326B3DC: 4BA3EB45  bl 0x82ca9f20
	ctx.lr = 0x8326B3E0;
	sub_82CA9F20(ctx, base);
	// 8326B3E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B3F0 size=64
    let mut pc: u32 = 0x8326B3F0;
    'dispatch: loop {
        match pc {
            0x8326B3F0 => {
    //   block [0x8326B3F0..0x8326B430)
	// 8326B3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B3F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B3FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B404: 388B0840  addi r4, r11, 0x840
	ctx.r[4].s64 = ctx.r[11].s64 + 2112;
	// 8326B408: 386AC250  addi r3, r10, -0x3db0
	ctx.r[3].s64 = ctx.r[10].s64 + -15792;
	// 8326B40C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B410: 4AFC1AC1  bl 0x8222ced0
	ctx.lr = 0x8326B414;
	sub_8222CED0(ctx, base);
	// 8326B414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B418: 3869E598  addi r3, r9, -0x1a68
	ctx.r[3].s64 = ctx.r[9].s64 + -6760;
	// 8326B41C: 4BA3EB05  bl 0x82ca9f20
	ctx.lr = 0x8326B420;
	sub_82CA9F20(ctx, base);
	// 8326B420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B430 size=64
    let mut pc: u32 = 0x8326B430;
    'dispatch: loop {
        match pc {
            0x8326B430 => {
    //   block [0x8326B430..0x8326B470)
	// 8326B430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B43C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B444: 388B0754  addi r4, r11, 0x754
	ctx.r[4].s64 = ctx.r[11].s64 + 1876;
	// 8326B448: 386AC254  addi r3, r10, -0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + -15788;
	// 8326B44C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B450: 4AFC1A81  bl 0x8222ced0
	ctx.lr = 0x8326B454;
	sub_8222CED0(ctx, base);
	// 8326B454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B458: 3869E5A8  addi r3, r9, -0x1a58
	ctx.r[3].s64 = ctx.r[9].s64 + -6744;
	// 8326B45C: 4BA3EAC5  bl 0x82ca9f20
	ctx.lr = 0x8326B460;
	sub_82CA9F20(ctx, base);
	// 8326B460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B470 size=64
    let mut pc: u32 = 0x8326B470;
    'dispatch: loop {
        match pc {
            0x8326B470 => {
    //   block [0x8326B470..0x8326B4B0)
	// 8326B470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B47C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B484: 388B0768  addi r4, r11, 0x768
	ctx.r[4].s64 = ctx.r[11].s64 + 1896;
	// 8326B488: 386AC258  addi r3, r10, -0x3da8
	ctx.r[3].s64 = ctx.r[10].s64 + -15784;
	// 8326B48C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B490: 4AFC1A41  bl 0x8222ced0
	ctx.lr = 0x8326B494;
	sub_8222CED0(ctx, base);
	// 8326B494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B498: 3869E5B8  addi r3, r9, -0x1a48
	ctx.r[3].s64 = ctx.r[9].s64 + -6728;
	// 8326B49C: 4BA3EA85  bl 0x82ca9f20
	ctx.lr = 0x8326B4A0;
	sub_82CA9F20(ctx, base);
	// 8326B4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B4B0 size=64
    let mut pc: u32 = 0x8326B4B0;
    'dispatch: loop {
        match pc {
            0x8326B4B0 => {
    //   block [0x8326B4B0..0x8326B4F0)
	// 8326B4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B4BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326B4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B4C4: 388B37BC  addi r4, r11, 0x37bc
	ctx.r[4].s64 = ctx.r[11].s64 + 14268;
	// 8326B4C8: 386AC25C  addi r3, r10, -0x3da4
	ctx.r[3].s64 = ctx.r[10].s64 + -15780;
	// 8326B4CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B4D0: 4AFC1A01  bl 0x8222ced0
	ctx.lr = 0x8326B4D4;
	sub_8222CED0(ctx, base);
	// 8326B4D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B4D8: 3869E5C8  addi r3, r9, -0x1a38
	ctx.r[3].s64 = ctx.r[9].s64 + -6712;
	// 8326B4DC: 4BA3EA45  bl 0x82ca9f20
	ctx.lr = 0x8326B4E0;
	sub_82CA9F20(ctx, base);
	// 8326B4E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B4F0 size=64
    let mut pc: u32 = 0x8326B4F0;
    'dispatch: loop {
        match pc {
            0x8326B4F0 => {
    //   block [0x8326B4F0..0x8326B530)
	// 8326B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B4FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326B500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B504: 388B13B0  addi r4, r11, 0x13b0
	ctx.r[4].s64 = ctx.r[11].s64 + 5040;
	// 8326B508: 386AC260  addi r3, r10, -0x3da0
	ctx.r[3].s64 = ctx.r[10].s64 + -15776;
	// 8326B50C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B510: 4AFC19C1  bl 0x8222ced0
	ctx.lr = 0x8326B514;
	sub_8222CED0(ctx, base);
	// 8326B514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B518: 3869E5D8  addi r3, r9, -0x1a28
	ctx.r[3].s64 = ctx.r[9].s64 + -6696;
	// 8326B51C: 4BA3EA05  bl 0x82ca9f20
	ctx.lr = 0x8326B520;
	sub_82CA9F20(ctx, base);
	// 8326B520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B530 size=64
    let mut pc: u32 = 0x8326B530;
    'dispatch: loop {
        match pc {
            0x8326B530 => {
    //   block [0x8326B530..0x8326B570)
	// 8326B530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B53C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326B540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B544: 388B13A0  addi r4, r11, 0x13a0
	ctx.r[4].s64 = ctx.r[11].s64 + 5024;
	// 8326B548: 386AC264  addi r3, r10, -0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15772;
	// 8326B54C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B550: 4AFC1981  bl 0x8222ced0
	ctx.lr = 0x8326B554;
	sub_8222CED0(ctx, base);
	// 8326B554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B558: 3869E5E8  addi r3, r9, -0x1a18
	ctx.r[3].s64 = ctx.r[9].s64 + -6680;
	// 8326B55C: 4BA3E9C5  bl 0x82ca9f20
	ctx.lr = 0x8326B560;
	sub_82CA9F20(ctx, base);
	// 8326B560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B570 size=64
    let mut pc: u32 = 0x8326B570;
    'dispatch: loop {
        match pc {
            0x8326B570 => {
    //   block [0x8326B570..0x8326B5B0)
	// 8326B570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B57C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B584: 388BEB60  addi r4, r11, -0x14a0
	ctx.r[4].s64 = ctx.r[11].s64 + -5280;
	// 8326B588: 386AC268  addi r3, r10, -0x3d98
	ctx.r[3].s64 = ctx.r[10].s64 + -15768;
	// 8326B58C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B590: 4AFC1941  bl 0x8222ced0
	ctx.lr = 0x8326B594;
	sub_8222CED0(ctx, base);
	// 8326B594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B598: 3869E5F8  addi r3, r9, -0x1a08
	ctx.r[3].s64 = ctx.r[9].s64 + -6664;
	// 8326B59C: 4BA3E985  bl 0x82ca9f20
	ctx.lr = 0x8326B5A0;
	sub_82CA9F20(ctx, base);
	// 8326B5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B5B0 size=64
    let mut pc: u32 = 0x8326B5B0;
    'dispatch: loop {
        match pc {
            0x8326B5B0 => {
    //   block [0x8326B5B0..0x8326B5F0)
	// 8326B5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B5B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B5BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B5C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B5C4: 388BEB74  addi r4, r11, -0x148c
	ctx.r[4].s64 = ctx.r[11].s64 + -5260;
	// 8326B5C8: 386AC26C  addi r3, r10, -0x3d94
	ctx.r[3].s64 = ctx.r[10].s64 + -15764;
	// 8326B5CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B5D0: 4AFC1901  bl 0x8222ced0
	ctx.lr = 0x8326B5D4;
	sub_8222CED0(ctx, base);
	// 8326B5D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B5D8: 3869E608  addi r3, r9, -0x19f8
	ctx.r[3].s64 = ctx.r[9].s64 + -6648;
	// 8326B5DC: 4BA3E945  bl 0x82ca9f20
	ctx.lr = 0x8326B5E0;
	sub_82CA9F20(ctx, base);
	// 8326B5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B5F0 size=64
    let mut pc: u32 = 0x8326B5F0;
    'dispatch: loop {
        match pc {
            0x8326B5F0 => {
    //   block [0x8326B5F0..0x8326B630)
	// 8326B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B5FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B604: 388BEB88  addi r4, r11, -0x1478
	ctx.r[4].s64 = ctx.r[11].s64 + -5240;
	// 8326B608: 386AC270  addi r3, r10, -0x3d90
	ctx.r[3].s64 = ctx.r[10].s64 + -15760;
	// 8326B60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B610: 4AFC18C1  bl 0x8222ced0
	ctx.lr = 0x8326B614;
	sub_8222CED0(ctx, base);
	// 8326B614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B618: 3869E618  addi r3, r9, -0x19e8
	ctx.r[3].s64 = ctx.r[9].s64 + -6632;
	// 8326B61C: 4BA3E905  bl 0x82ca9f20
	ctx.lr = 0x8326B620;
	sub_82CA9F20(ctx, base);
	// 8326B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B630 size=64
    let mut pc: u32 = 0x8326B630;
    'dispatch: loop {
        match pc {
            0x8326B630 => {
    //   block [0x8326B630..0x8326B670)
	// 8326B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B63C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B644: 388BEB9C  addi r4, r11, -0x1464
	ctx.r[4].s64 = ctx.r[11].s64 + -5220;
	// 8326B648: 386AC274  addi r3, r10, -0x3d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15756;
	// 8326B64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B650: 4AFC1881  bl 0x8222ced0
	ctx.lr = 0x8326B654;
	sub_8222CED0(ctx, base);
	// 8326B654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B658: 3869E628  addi r3, r9, -0x19d8
	ctx.r[3].s64 = ctx.r[9].s64 + -6616;
	// 8326B65C: 4BA3E8C5  bl 0x82ca9f20
	ctx.lr = 0x8326B660;
	sub_82CA9F20(ctx, base);
	// 8326B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B670 size=64
    let mut pc: u32 = 0x8326B670;
    'dispatch: loop {
        match pc {
            0x8326B670 => {
    //   block [0x8326B670..0x8326B6B0)
	// 8326B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B67C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B684: 388BEBB0  addi r4, r11, -0x1450
	ctx.r[4].s64 = ctx.r[11].s64 + -5200;
	// 8326B688: 386AC278  addi r3, r10, -0x3d88
	ctx.r[3].s64 = ctx.r[10].s64 + -15752;
	// 8326B68C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B690: 4AFC1841  bl 0x8222ced0
	ctx.lr = 0x8326B694;
	sub_8222CED0(ctx, base);
	// 8326B694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B698: 3869E638  addi r3, r9, -0x19c8
	ctx.r[3].s64 = ctx.r[9].s64 + -6600;
	// 8326B69C: 4BA3E885  bl 0x82ca9f20
	ctx.lr = 0x8326B6A0;
	sub_82CA9F20(ctx, base);
	// 8326B6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B6B0 size=64
    let mut pc: u32 = 0x8326B6B0;
    'dispatch: loop {
        match pc {
            0x8326B6B0 => {
    //   block [0x8326B6B0..0x8326B6F0)
	// 8326B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B6BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B6C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B6C4: 388BEBC4  addi r4, r11, -0x143c
	ctx.r[4].s64 = ctx.r[11].s64 + -5180;
	// 8326B6C8: 386AC27C  addi r3, r10, -0x3d84
	ctx.r[3].s64 = ctx.r[10].s64 + -15748;
	// 8326B6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B6D0: 4AFC1801  bl 0x8222ced0
	ctx.lr = 0x8326B6D4;
	sub_8222CED0(ctx, base);
	// 8326B6D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B6D8: 3869E648  addi r3, r9, -0x19b8
	ctx.r[3].s64 = ctx.r[9].s64 + -6584;
	// 8326B6DC: 4BA3E845  bl 0x82ca9f20
	ctx.lr = 0x8326B6E0;
	sub_82CA9F20(ctx, base);
	// 8326B6E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B6F0 size=64
    let mut pc: u32 = 0x8326B6F0;
    'dispatch: loop {
        match pc {
            0x8326B6F0 => {
    //   block [0x8326B6F0..0x8326B730)
	// 8326B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B6FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B704: 388BEBD4  addi r4, r11, -0x142c
	ctx.r[4].s64 = ctx.r[11].s64 + -5164;
	// 8326B708: 386AC280  addi r3, r10, -0x3d80
	ctx.r[3].s64 = ctx.r[10].s64 + -15744;
	// 8326B70C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B710: 4AFC17C1  bl 0x8222ced0
	ctx.lr = 0x8326B714;
	sub_8222CED0(ctx, base);
	// 8326B714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B718: 3869E658  addi r3, r9, -0x19a8
	ctx.r[3].s64 = ctx.r[9].s64 + -6568;
	// 8326B71C: 4BA3E805  bl 0x82ca9f20
	ctx.lr = 0x8326B720;
	sub_82CA9F20(ctx, base);
	// 8326B720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B730 size=64
    let mut pc: u32 = 0x8326B730;
    'dispatch: loop {
        match pc {
            0x8326B730 => {
    //   block [0x8326B730..0x8326B770)
	// 8326B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B73C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B744: 388BEBE4  addi r4, r11, -0x141c
	ctx.r[4].s64 = ctx.r[11].s64 + -5148;
	// 8326B748: 386AC284  addi r3, r10, -0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15740;
	// 8326B74C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B750: 4AFC1781  bl 0x8222ced0
	ctx.lr = 0x8326B754;
	sub_8222CED0(ctx, base);
	// 8326B754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B758: 3869E668  addi r3, r9, -0x1998
	ctx.r[3].s64 = ctx.r[9].s64 + -6552;
	// 8326B75C: 4BA3E7C5  bl 0x82ca9f20
	ctx.lr = 0x8326B760;
	sub_82CA9F20(ctx, base);
	// 8326B760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B770 size=64
    let mut pc: u32 = 0x8326B770;
    'dispatch: loop {
        match pc {
            0x8326B770 => {
    //   block [0x8326B770..0x8326B7B0)
	// 8326B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B77C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B784: 388BEBF4  addi r4, r11, -0x140c
	ctx.r[4].s64 = ctx.r[11].s64 + -5132;
	// 8326B788: 386AC288  addi r3, r10, -0x3d78
	ctx.r[3].s64 = ctx.r[10].s64 + -15736;
	// 8326B78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B790: 4AFC1741  bl 0x8222ced0
	ctx.lr = 0x8326B794;
	sub_8222CED0(ctx, base);
	// 8326B794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B798: 3869E678  addi r3, r9, -0x1988
	ctx.r[3].s64 = ctx.r[9].s64 + -6536;
	// 8326B79C: 4BA3E785  bl 0x82ca9f20
	ctx.lr = 0x8326B7A0;
	sub_82CA9F20(ctx, base);
	// 8326B7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B7B0 size=64
    let mut pc: u32 = 0x8326B7B0;
    'dispatch: loop {
        match pc {
            0x8326B7B0 => {
    //   block [0x8326B7B0..0x8326B7F0)
	// 8326B7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B7BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B7C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B7C4: 388BEC04  addi r4, r11, -0x13fc
	ctx.r[4].s64 = ctx.r[11].s64 + -5116;
	// 8326B7C8: 386AC28C  addi r3, r10, -0x3d74
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	// 8326B7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B7D0: 4AFC1701  bl 0x8222ced0
	ctx.lr = 0x8326B7D4;
	sub_8222CED0(ctx, base);
	// 8326B7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B7D8: 3869E688  addi r3, r9, -0x1978
	ctx.r[3].s64 = ctx.r[9].s64 + -6520;
	// 8326B7DC: 4BA3E745  bl 0x82ca9f20
	ctx.lr = 0x8326B7E0;
	sub_82CA9F20(ctx, base);
	// 8326B7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B7F0 size=64
    let mut pc: u32 = 0x8326B7F0;
    'dispatch: loop {
        match pc {
            0x8326B7F0 => {
    //   block [0x8326B7F0..0x8326B830)
	// 8326B7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B7FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B804: 388BEC18  addi r4, r11, -0x13e8
	ctx.r[4].s64 = ctx.r[11].s64 + -5096;
	// 8326B808: 386AC290  addi r3, r10, -0x3d70
	ctx.r[3].s64 = ctx.r[10].s64 + -15728;
	// 8326B80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B810: 4AFC16C1  bl 0x8222ced0
	ctx.lr = 0x8326B814;
	sub_8222CED0(ctx, base);
	// 8326B814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B818: 3869E698  addi r3, r9, -0x1968
	ctx.r[3].s64 = ctx.r[9].s64 + -6504;
	// 8326B81C: 4BA3E705  bl 0x82ca9f20
	ctx.lr = 0x8326B820;
	sub_82CA9F20(ctx, base);
	// 8326B820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B830 size=64
    let mut pc: u32 = 0x8326B830;
    'dispatch: loop {
        match pc {
            0x8326B830 => {
    //   block [0x8326B830..0x8326B870)
	// 8326B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B83C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B844: 388BEC2C  addi r4, r11, -0x13d4
	ctx.r[4].s64 = ctx.r[11].s64 + -5076;
	// 8326B848: 386AC294  addi r3, r10, -0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15724;
	// 8326B84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B850: 4AFC1681  bl 0x8222ced0
	ctx.lr = 0x8326B854;
	sub_8222CED0(ctx, base);
	// 8326B854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B858: 3869E6A8  addi r3, r9, -0x1958
	ctx.r[3].s64 = ctx.r[9].s64 + -6488;
	// 8326B85C: 4BA3E6C5  bl 0x82ca9f20
	ctx.lr = 0x8326B860;
	sub_82CA9F20(ctx, base);
	// 8326B860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B870 size=64
    let mut pc: u32 = 0x8326B870;
    'dispatch: loop {
        match pc {
            0x8326B870 => {
    //   block [0x8326B870..0x8326B8B0)
	// 8326B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B87C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B884: 388BEC40  addi r4, r11, -0x13c0
	ctx.r[4].s64 = ctx.r[11].s64 + -5056;
	// 8326B888: 386AC298  addi r3, r10, -0x3d68
	ctx.r[3].s64 = ctx.r[10].s64 + -15720;
	// 8326B88C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B890: 4AFC1641  bl 0x8222ced0
	ctx.lr = 0x8326B894;
	sub_8222CED0(ctx, base);
	// 8326B894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B898: 3869E6B8  addi r3, r9, -0x1948
	ctx.r[3].s64 = ctx.r[9].s64 + -6472;
	// 8326B89C: 4BA3E685  bl 0x82ca9f20
	ctx.lr = 0x8326B8A0;
	sub_82CA9F20(ctx, base);
	// 8326B8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B8B0 size=64
    let mut pc: u32 = 0x8326B8B0;
    'dispatch: loop {
        match pc {
            0x8326B8B0 => {
    //   block [0x8326B8B0..0x8326B8F0)
	// 8326B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B8B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B8BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B8C4: 388BEC54  addi r4, r11, -0x13ac
	ctx.r[4].s64 = ctx.r[11].s64 + -5036;
	// 8326B8C8: 386AC29C  addi r3, r10, -0x3d64
	ctx.r[3].s64 = ctx.r[10].s64 + -15716;
	// 8326B8CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B8D0: 4AFC1601  bl 0x8222ced0
	ctx.lr = 0x8326B8D4;
	sub_8222CED0(ctx, base);
	// 8326B8D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B8D8: 3869E6C8  addi r3, r9, -0x1938
	ctx.r[3].s64 = ctx.r[9].s64 + -6456;
	// 8326B8DC: 4BA3E645  bl 0x82ca9f20
	ctx.lr = 0x8326B8E0;
	sub_82CA9F20(ctx, base);
	// 8326B8E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B8F0 size=64
    let mut pc: u32 = 0x8326B8F0;
    'dispatch: loop {
        match pc {
            0x8326B8F0 => {
    //   block [0x8326B8F0..0x8326B930)
	// 8326B8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B8F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B8FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B904: 388BEC68  addi r4, r11, -0x1398
	ctx.r[4].s64 = ctx.r[11].s64 + -5016;
	// 8326B908: 386AC2A0  addi r3, r10, -0x3d60
	ctx.r[3].s64 = ctx.r[10].s64 + -15712;
	// 8326B90C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B910: 4AFC15C1  bl 0x8222ced0
	ctx.lr = 0x8326B914;
	sub_8222CED0(ctx, base);
	// 8326B914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B918: 3869E6D8  addi r3, r9, -0x1928
	ctx.r[3].s64 = ctx.r[9].s64 + -6440;
	// 8326B91C: 4BA3E605  bl 0x82ca9f20
	ctx.lr = 0x8326B920;
	sub_82CA9F20(ctx, base);
	// 8326B920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B930 size=64
    let mut pc: u32 = 0x8326B930;
    'dispatch: loop {
        match pc {
            0x8326B930 => {
    //   block [0x8326B930..0x8326B970)
	// 8326B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B93C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B944: 388BEC7C  addi r4, r11, -0x1384
	ctx.r[4].s64 = ctx.r[11].s64 + -4996;
	// 8326B948: 386AC2A4  addi r3, r10, -0x3d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15708;
	// 8326B94C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B950: 4AFC1581  bl 0x8222ced0
	ctx.lr = 0x8326B954;
	sub_8222CED0(ctx, base);
	// 8326B954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B958: 3869E6E8  addi r3, r9, -0x1918
	ctx.r[3].s64 = ctx.r[9].s64 + -6424;
	// 8326B95C: 4BA3E5C5  bl 0x82ca9f20
	ctx.lr = 0x8326B960;
	sub_82CA9F20(ctx, base);
	// 8326B960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B970 size=64
    let mut pc: u32 = 0x8326B970;
    'dispatch: loop {
        match pc {
            0x8326B970 => {
    //   block [0x8326B970..0x8326B9B0)
	// 8326B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B97C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B984: 388BEC8C  addi r4, r11, -0x1374
	ctx.r[4].s64 = ctx.r[11].s64 + -4980;
	// 8326B988: 386AC2A8  addi r3, r10, -0x3d58
	ctx.r[3].s64 = ctx.r[10].s64 + -15704;
	// 8326B98C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B990: 4AFC1541  bl 0x8222ced0
	ctx.lr = 0x8326B994;
	sub_8222CED0(ctx, base);
	// 8326B994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B998: 3869E6F8  addi r3, r9, -0x1908
	ctx.r[3].s64 = ctx.r[9].s64 + -6408;
	// 8326B99C: 4BA3E585  bl 0x82ca9f20
	ctx.lr = 0x8326B9A0;
	sub_82CA9F20(ctx, base);
	// 8326B9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B9B0 size=64
    let mut pc: u32 = 0x8326B9B0;
    'dispatch: loop {
        match pc {
            0x8326B9B0 => {
    //   block [0x8326B9B0..0x8326B9F0)
	// 8326B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B9BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326B9C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326B9C4: 388BEC9C  addi r4, r11, -0x1364
	ctx.r[4].s64 = ctx.r[11].s64 + -4964;
	// 8326B9C8: 386AC2AC  addi r3, r10, -0x3d54
	ctx.r[3].s64 = ctx.r[10].s64 + -15700;
	// 8326B9CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326B9D0: 4AFC1501  bl 0x8222ced0
	ctx.lr = 0x8326B9D4;
	sub_8222CED0(ctx, base);
	// 8326B9D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326B9D8: 3869E708  addi r3, r9, -0x18f8
	ctx.r[3].s64 = ctx.r[9].s64 + -6392;
	// 8326B9DC: 4BA3E545  bl 0x82ca9f20
	ctx.lr = 0x8326B9E0;
	sub_82CA9F20(ctx, base);
	// 8326B9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326B9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326B9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326B9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B9F0 size=64
    let mut pc: u32 = 0x8326B9F0;
    'dispatch: loop {
        match pc {
            0x8326B9F0 => {
    //   block [0x8326B9F0..0x8326BA30)
	// 8326B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326B9FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BA00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BA04: 388BECAC  addi r4, r11, -0x1354
	ctx.r[4].s64 = ctx.r[11].s64 + -4948;
	// 8326BA08: 386AC2B0  addi r3, r10, -0x3d50
	ctx.r[3].s64 = ctx.r[10].s64 + -15696;
	// 8326BA0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BA10: 4AFC14C1  bl 0x8222ced0
	ctx.lr = 0x8326BA14;
	sub_8222CED0(ctx, base);
	// 8326BA14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BA18: 3869E718  addi r3, r9, -0x18e8
	ctx.r[3].s64 = ctx.r[9].s64 + -6376;
	// 8326BA1C: 4BA3E505  bl 0x82ca9f20
	ctx.lr = 0x8326BA20;
	sub_82CA9F20(ctx, base);
	// 8326BA20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BA30 size=64
    let mut pc: u32 = 0x8326BA30;
    'dispatch: loop {
        match pc {
            0x8326BA30 => {
    //   block [0x8326BA30..0x8326BA70)
	// 8326BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BA3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BA40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BA44: 388BECBC  addi r4, r11, -0x1344
	ctx.r[4].s64 = ctx.r[11].s64 + -4932;
	// 8326BA48: 386AC2B4  addi r3, r10, -0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15692;
	// 8326BA4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BA50: 4AFC1481  bl 0x8222ced0
	ctx.lr = 0x8326BA54;
	sub_8222CED0(ctx, base);
	// 8326BA54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BA58: 3869E728  addi r3, r9, -0x18d8
	ctx.r[3].s64 = ctx.r[9].s64 + -6360;
	// 8326BA5C: 4BA3E4C5  bl 0x82ca9f20
	ctx.lr = 0x8326BA60;
	sub_82CA9F20(ctx, base);
	// 8326BA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326BA70 size=12
    let mut pc: u32 = 0x8326BA70;
    'dispatch: loop {
        match pc {
            0x8326BA70 => {
    //   block [0x8326BA70..0x8326BA7C)
	// 8326BA70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326BA74: 386BE738  addi r3, r11, -0x18c8
	ctx.r[3].s64 = ctx.r[11].s64 + -6344;
	// 8326BA78: 4BA3E4A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BA80 size=64
    let mut pc: u32 = 0x8326BA80;
    'dispatch: loop {
        match pc {
            0x8326BA80 => {
    //   block [0x8326BA80..0x8326BAC0)
	// 8326BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BA90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BA94: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326BA98: 386AC2C8  addi r3, r10, -0x3d38
	ctx.r[3].s64 = ctx.r[10].s64 + -15672;
	// 8326BA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BAA0: 4AFC1431  bl 0x8222ced0
	ctx.lr = 0x8326BAA4;
	sub_8222CED0(ctx, base);
	// 8326BAA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BAA8: 3869E7B8  addi r3, r9, -0x1848
	ctx.r[3].s64 = ctx.r[9].s64 + -6216;
	// 8326BAAC: 4BA3E475  bl 0x82ca9f20
	ctx.lr = 0x8326BAB0;
	sub_82CA9F20(ctx, base);
	// 8326BAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BAC0 size=64
    let mut pc: u32 = 0x8326BAC0;
    'dispatch: loop {
        match pc {
            0x8326BAC0 => {
    //   block [0x8326BAC0..0x8326BB00)
	// 8326BAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BAD4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326BAD8: 386AC2CC  addi r3, r10, -0x3d34
	ctx.r[3].s64 = ctx.r[10].s64 + -15668;
	// 8326BADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BAE0: 4AFC13F1  bl 0x8222ced0
	ctx.lr = 0x8326BAE4;
	sub_8222CED0(ctx, base);
	// 8326BAE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BAE8: 3869E7C8  addi r3, r9, -0x1838
	ctx.r[3].s64 = ctx.r[9].s64 + -6200;
	// 8326BAEC: 4BA3E435  bl 0x82ca9f20
	ctx.lr = 0x8326BAF0;
	sub_82CA9F20(ctx, base);
	// 8326BAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BB00 size=64
    let mut pc: u32 = 0x8326BB00;
    'dispatch: loop {
        match pc {
            0x8326BB00 => {
    //   block [0x8326BB00..0x8326BB40)
	// 8326BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BB0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BB14: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326BB18: 386AC2D0  addi r3, r10, -0x3d30
	ctx.r[3].s64 = ctx.r[10].s64 + -15664;
	// 8326BB1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BB20: 4AFC13B1  bl 0x8222ced0
	ctx.lr = 0x8326BB24;
	sub_8222CED0(ctx, base);
	// 8326BB24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BB28: 3869E7D8  addi r3, r9, -0x1828
	ctx.r[3].s64 = ctx.r[9].s64 + -6184;
	// 8326BB2C: 4BA3E3F5  bl 0x82ca9f20
	ctx.lr = 0x8326BB30;
	sub_82CA9F20(ctx, base);
	// 8326BB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BB40 size=56
    let mut pc: u32 = 0x8326BB40;
    'dispatch: loop {
        match pc {
            0x8326BB40 => {
    //   block [0x8326BB40..0x8326BB78)
	// 8326BB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BB4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326BB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326BB54: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 8326BB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326BB5C: 4AF881FD  bl 0x821f3d58
	ctx.lr = 0x8326BB60;
	sub_821F3D58(ctx, base);
	// 8326BB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BB64: 906AC2D4  stw r3, -0x3d2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15660 as u32), ctx.r[3].u32 ) };
	// 8326BB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326BB78 size=44
    let mut pc: u32 = 0x8326BB78;
    'dispatch: loop {
        match pc {
            0x8326BB78 => {
    //   block [0x8326BB78..0x8326BBA4)
	// 8326BB78: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326BB7C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8326BB80: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326BB84: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8326BB88: C9AA1A28  lfd f13, 0x1a28(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(6696 as u32) ) };
	// 8326BB8C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 8326BB90: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326BB94: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 8326BB98: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8326BB9C: 9169C2D8  stw r11, -0x3d28(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15656 as u32), ctx.r[11].u32 ) };
	// 8326BBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BBA8 size=64
    let mut pc: u32 = 0x8326BBA8;
    'dispatch: loop {
        match pc {
            0x8326BBA8 => {
    //   block [0x8326BBA8..0x8326BBE8)
	// 8326BBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BBB4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BBBC: 388BF18C  addi r4, r11, -0xe74
	ctx.r[4].s64 = ctx.r[11].s64 + -3700;
	// 8326BBC0: 386AC2DC  addi r3, r10, -0x3d24
	ctx.r[3].s64 = ctx.r[10].s64 + -15652;
	// 8326BBC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BBC8: 4AFC1309  bl 0x8222ced0
	ctx.lr = 0x8326BBCC;
	sub_8222CED0(ctx, base);
	// 8326BBCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BBD0: 3869E850  addi r3, r9, -0x17b0
	ctx.r[3].s64 = ctx.r[9].s64 + -6064;
	// 8326BBD4: 4BA3E34D  bl 0x82ca9f20
	ctx.lr = 0x8326BBD8;
	sub_82CA9F20(ctx, base);
	// 8326BBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BBE8 size=64
    let mut pc: u32 = 0x8326BBE8;
    'dispatch: loop {
        match pc {
            0x8326BBE8 => {
    //   block [0x8326BBE8..0x8326BC28)
	// 8326BBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BBF4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BBF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BBFC: 388BF1A8  addi r4, r11, -0xe58
	ctx.r[4].s64 = ctx.r[11].s64 + -3672;
	// 8326BC00: 386AC2E0  addi r3, r10, -0x3d20
	ctx.r[3].s64 = ctx.r[10].s64 + -15648;
	// 8326BC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BC08: 4AFC12C9  bl 0x8222ced0
	ctx.lr = 0x8326BC0C;
	sub_8222CED0(ctx, base);
	// 8326BC0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BC10: 3869E860  addi r3, r9, -0x17a0
	ctx.r[3].s64 = ctx.r[9].s64 + -6048;
	// 8326BC14: 4BA3E30D  bl 0x82ca9f20
	ctx.lr = 0x8326BC18;
	sub_82CA9F20(ctx, base);
	// 8326BC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BC28 size=64
    let mut pc: u32 = 0x8326BC28;
    'dispatch: loop {
        match pc {
            0x8326BC28 => {
    //   block [0x8326BC28..0x8326BC68)
	// 8326BC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BC34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BC38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BC3C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326BC40: 386AC310  addi r3, r10, -0x3cf0
	ctx.r[3].s64 = ctx.r[10].s64 + -15600;
	// 8326BC44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BC48: 4AFC1289  bl 0x8222ced0
	ctx.lr = 0x8326BC4C;
	sub_8222CED0(ctx, base);
	// 8326BC4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BC50: 3869E870  addi r3, r9, -0x1790
	ctx.r[3].s64 = ctx.r[9].s64 + -6032;
	// 8326BC54: 4BA3E2CD  bl 0x82ca9f20
	ctx.lr = 0x8326BC58;
	sub_82CA9F20(ctx, base);
	// 8326BC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BC68 size=64
    let mut pc: u32 = 0x8326BC68;
    'dispatch: loop {
        match pc {
            0x8326BC68 => {
    //   block [0x8326BC68..0x8326BCA8)
	// 8326BC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BC74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BC7C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326BC80: 386AC314  addi r3, r10, -0x3cec
	ctx.r[3].s64 = ctx.r[10].s64 + -15596;
	// 8326BC84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BC88: 4AFC1249  bl 0x8222ced0
	ctx.lr = 0x8326BC8C;
	sub_8222CED0(ctx, base);
	// 8326BC8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BC90: 3869E880  addi r3, r9, -0x1780
	ctx.r[3].s64 = ctx.r[9].s64 + -6016;
	// 8326BC94: 4BA3E28D  bl 0x82ca9f20
	ctx.lr = 0x8326BC98;
	sub_82CA9F20(ctx, base);
	// 8326BC98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BCA8 size=64
    let mut pc: u32 = 0x8326BCA8;
    'dispatch: loop {
        match pc {
            0x8326BCA8 => {
    //   block [0x8326BCA8..0x8326BCE8)
	// 8326BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BCB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BCB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BCB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BCBC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326BCC0: 386AC318  addi r3, r10, -0x3ce8
	ctx.r[3].s64 = ctx.r[10].s64 + -15592;
	// 8326BCC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BCC8: 4AFC1209  bl 0x8222ced0
	ctx.lr = 0x8326BCCC;
	sub_8222CED0(ctx, base);
	// 8326BCCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BCD0: 3869E890  addi r3, r9, -0x1770
	ctx.r[3].s64 = ctx.r[9].s64 + -6000;
	// 8326BCD4: 4BA3E24D  bl 0x82ca9f20
	ctx.lr = 0x8326BCD8;
	sub_82CA9F20(ctx, base);
	// 8326BCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BCE8 size=64
    let mut pc: u32 = 0x8326BCE8;
    'dispatch: loop {
        match pc {
            0x8326BCE8 => {
    //   block [0x8326BCE8..0x8326BD28)
	// 8326BCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BCF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326BCF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BCFC: 388B0E20  addi r4, r11, 0xe20
	ctx.r[4].s64 = ctx.r[11].s64 + 3616;
	// 8326BD00: 386AC330  addi r3, r10, -0x3cd0
	ctx.r[3].s64 = ctx.r[10].s64 + -15568;
	// 8326BD04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BD08: 4AFC11C9  bl 0x8222ced0
	ctx.lr = 0x8326BD0C;
	sub_8222CED0(ctx, base);
	// 8326BD0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BD10: 3869E8A0  addi r3, r9, -0x1760
	ctx.r[3].s64 = ctx.r[9].s64 + -5984;
	// 8326BD14: 4BA3E20D  bl 0x82ca9f20
	ctx.lr = 0x8326BD18;
	sub_82CA9F20(ctx, base);
	// 8326BD18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BD28 size=64
    let mut pc: u32 = 0x8326BD28;
    'dispatch: loop {
        match pc {
            0x8326BD28 => {
    //   block [0x8326BD28..0x8326BD68)
	// 8326BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BD34: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326BD38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BD3C: 388B67A8  addi r4, r11, 0x67a8
	ctx.r[4].s64 = ctx.r[11].s64 + 26536;
	// 8326BD40: 386AC334  addi r3, r10, -0x3ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -15564;
	// 8326BD44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BD48: 4AFC1189  bl 0x8222ced0
	ctx.lr = 0x8326BD4C;
	sub_8222CED0(ctx, base);
	// 8326BD4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BD50: 3869E8B0  addi r3, r9, -0x1750
	ctx.r[3].s64 = ctx.r[9].s64 + -5968;
	// 8326BD54: 4BA3E1CD  bl 0x82ca9f20
	ctx.lr = 0x8326BD58;
	sub_82CA9F20(ctx, base);
	// 8326BD58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326BD68 size=44
    let mut pc: u32 = 0x8326BD68;
    'dispatch: loop {
        match pc {
            0x8326BD68 => {
    //   block [0x8326BD68..0x8326BD94)
	// 8326BD68: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326BD6C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 8326BD70: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326BD74: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8326BD78: C9AA1A28  lfd f13, 0x1a28(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(6696 as u32) ) };
	// 8326BD7C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 8326BD80: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326BD84: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 8326BD88: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8326BD8C: 9169C338  stw r11, -0x3cc8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15560 as u32), ctx.r[11].u32 ) };
	// 8326BD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BD98 size=64
    let mut pc: u32 = 0x8326BD98;
    'dispatch: loop {
        match pc {
            0x8326BD98 => {
    //   block [0x8326BD98..0x8326BDD8)
	// 8326BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BDA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326BDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BDAC: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 8326BDB0: 386AC33C  addi r3, r10, -0x3cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15556;
	// 8326BDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BDB8: 4AFC1119  bl 0x8222ced0
	ctx.lr = 0x8326BDBC;
	sub_8222CED0(ctx, base);
	// 8326BDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BDC0: 3869E8C0  addi r3, r9, -0x1740
	ctx.r[3].s64 = ctx.r[9].s64 + -5952;
	// 8326BDC4: 4BA3E15D  bl 0x82ca9f20
	ctx.lr = 0x8326BDC8;
	sub_82CA9F20(ctx, base);
	// 8326BDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BDD8 size=64
    let mut pc: u32 = 0x8326BDD8;
    'dispatch: loop {
        match pc {
            0x8326BDD8 => {
    //   block [0x8326BDD8..0x8326BE18)
	// 8326BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BDE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BDEC: 388B01EC  addi r4, r11, 0x1ec
	ctx.r[4].s64 = ctx.r[11].s64 + 492;
	// 8326BDF0: 386AC340  addi r3, r10, -0x3cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -15552;
	// 8326BDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BDF8: 4AFC10D9  bl 0x8222ced0
	ctx.lr = 0x8326BDFC;
	sub_8222CED0(ctx, base);
	// 8326BDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BE00: 3869E8D0  addi r3, r9, -0x1730
	ctx.r[3].s64 = ctx.r[9].s64 + -5936;
	// 8326BE04: 4BA3E11D  bl 0x82ca9f20
	ctx.lr = 0x8326BE08;
	sub_82CA9F20(ctx, base);
	// 8326BE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BE18 size=64
    let mut pc: u32 = 0x8326BE18;
    'dispatch: loop {
        match pc {
            0x8326BE18 => {
    //   block [0x8326BE18..0x8326BE58)
	// 8326BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BE24: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BE2C: 388B01F8  addi r4, r11, 0x1f8
	ctx.r[4].s64 = ctx.r[11].s64 + 504;
	// 8326BE30: 386AC344  addi r3, r10, -0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15548;
	// 8326BE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BE38: 4AFC1099  bl 0x8222ced0
	ctx.lr = 0x8326BE3C;
	sub_8222CED0(ctx, base);
	// 8326BE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BE40: 3869E8E0  addi r3, r9, -0x1720
	ctx.r[3].s64 = ctx.r[9].s64 + -5920;
	// 8326BE44: 4BA3E0DD  bl 0x82ca9f20
	ctx.lr = 0x8326BE48;
	sub_82CA9F20(ctx, base);
	// 8326BE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BE58 size=64
    let mut pc: u32 = 0x8326BE58;
    'dispatch: loop {
        match pc {
            0x8326BE58 => {
    //   block [0x8326BE58..0x8326BE98)
	// 8326BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BE64: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326BE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326BE6C: 388B0200  addi r4, r11, 0x200
	ctx.r[4].s64 = ctx.r[11].s64 + 512;
	// 8326BE70: 386AC348  addi r3, r10, -0x3cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -15544;
	// 8326BE74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BE78: 4AFC1059  bl 0x8222ced0
	ctx.lr = 0x8326BE7C;
	sub_8222CED0(ctx, base);
	// 8326BE7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326BE80: 3869E8F0  addi r3, r9, -0x1710
	ctx.r[3].s64 = ctx.r[9].s64 + -5904;
	// 8326BE84: 4BA3E09D  bl 0x82ca9f20
	ctx.lr = 0x8326BE88;
	sub_82CA9F20(ctx, base);
	// 8326BE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326BE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326BE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326BE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8326BE98 size=96
    let mut pc: u32 = 0x8326BE98;
    'dispatch: loop {
        match pc {
            0x8326BE98 => {
    //   block [0x8326BE98..0x8326BEF8)
	// 8326BE98: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8326BE9C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8326BEA0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8326BEA4: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8326BEA8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326BEAC: C1AA9484  lfs f13, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8326BEB0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8326BEB4: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8326BEB8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BEF8 size=280
    let mut pc: u32 = 0x8326BEF8;
    'dispatch: loop {
        match pc {
            0x8326BEF8 => {
    //   block [0x8326BEF8..0x8326C010)
	// 8326BEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BF00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8326BF04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326BF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326BF0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326BF10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BF14: 388B0500  addi r4, r11, 0x500
	ctx.r[4].s64 = ctx.r[11].s64 + 1280;
	// 8326BF18: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8326BF1C: 4AFC0FB5  bl 0x8222ced0
	ctx.lr = 0x8326BF20;
	sub_8222CED0(ctx, base);
	// 8326BF20: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8326BF24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8326BF28: 388A02F4  addi r4, r10, 0x2f4
	ctx.r[4].s64 = ctx.r[10].s64 + 756;
	// 8326BF2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326BF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326BF34: 4AFC0F9D  bl 0x8222ced0
	ctx.lr = 0x8326BF38;
	sub_8222CED0(ctx, base);
	// 8326BF38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326BF3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326BF40: 4AF808B9  bl 0x821ec7f8
	ctx.lr = 0x8326BF44;
	sub_821EC7F8(ctx, base);
	// 8326BF44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8326BF48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF4C: 4AF82BED  bl 0x821eeb38
	ctx.lr = 0x8326BF50;
	sub_821EEB38(ctx, base);
	// 8326BF50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF54: 4B99789D  bl 0x82c037f0
	ctx.lr = 0x8326BF58;
	sub_82C037F0(ctx, base);
	// 8326BF58: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326BF5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326BF60: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF64: 9169C34C  stw r11, -0x3cb4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-15540 as u32), ctx.r[11].u32 ) };
	// 8326BF68: 4AF5A801  bl 0x821c6768
	ctx.lr = 0x8326BF6C;
	sub_821C6768(ctx, base);
	// 8326BF6C: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 8326BF70: 3BC87088  addi r30, r8, 0x7088
	ctx.r[30].s64 = ctx.r[8].s64 + 28808;
	// 8326BF74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8326BF78: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8326BF7C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BF80: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8326BF84: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8326BF88: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326BF8C: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BF90: 4082FFE8  bne 0x8326bf78
	if !ctx.cr[0].eq {
	pc = 0x8326BF78; continue 'dispatch;
	}
	// 8326BF94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8326BF98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326BF9C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8326BFA0: 4AF5A7C9  bl 0x821c6768
	ctx.lr = 0x8326BFA4;
	sub_821C6768(ctx, base);
	// 8326BFA4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8326BFA8: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8326BFAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFB0: 7C805828  lwarx r4, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8326BFB4: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326BFB8: 7C80592D  stwcx. r4, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326BFBC: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFC0: 4082FFE8  bne 0x8326bfa8
	if !ctx.cr[0].eq {
	pc = 0x8326BFA8; continue 'dispatch;
	}
	// 8326BFC4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8326BFC8: 4AF5A7A1  bl 0x821c6768
	ctx.lr = 0x8326BFCC;
	sub_821C6768(ctx, base);
	// 8326BFCC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8326BFD0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFD4: 7D40F028  lwarx r10, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8326BFD8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8326BFDC: 7D40F12D  stwcx. r10, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326BFE0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326BFE4: 4082FFE8  bne 0x8326bfcc
	if !ctx.cr[0].eq {
	pc = 0x8326BFCC; continue 'dispatch;
	}
	// 8326BFE8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8326BFEC: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326BFF0: 3868E900  addi r3, r8, -0x1700
	ctx.r[3].s64 = ctx.r[8].s64 + -5888;
	// 8326BFF4: 4BA3DF2D  bl 0x82ca9f20
	ctx.lr = 0x8326BFF8;
	sub_82CA9F20(ctx, base);
	// 8326BFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8326BFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8326C008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326C00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C010 size=60
    let mut pc: u32 = 0x8326C010;
    'dispatch: loop {
        match pc {
            0x8326C010 => {
    //   block [0x8326C010..0x8326C04C)
	// 8326C010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C01C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326C020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C024: 388B0AF8  addi r4, r11, 0xaf8
	ctx.r[4].s64 = ctx.r[11].s64 + 2808;
	// 8326C028: 386AC360  addi r3, r10, -0x3ca0
	ctx.r[3].s64 = ctx.r[10].s64 + -15520;
	// 8326C02C: 4B06A3DD  bl 0x822d6408
	ctx.lr = 0x8326C030;
	sub_822D6408(ctx, base);
	// 8326C030: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C034: 3869E908  addi r3, r9, -0x16f8
	ctx.r[3].s64 = ctx.r[9].s64 + -5880;
	// 8326C038: 4BA3DEE9  bl 0x82ca9f20
	ctx.lr = 0x8326C03C;
	sub_82CA9F20(ctx, base);
	// 8326C03C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C050 size=64
    let mut pc: u32 = 0x8326C050;
    'dispatch: loop {
        match pc {
            0x8326C050 => {
    //   block [0x8326C050..0x8326C090)
	// 8326C050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C05C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C064: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326C068: 386AC364  addi r3, r10, -0x3c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15516;
	// 8326C06C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C070: 4AFC0E61  bl 0x8222ced0
	ctx.lr = 0x8326C074;
	sub_8222CED0(ctx, base);
	// 8326C074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C078: 3869E918  addi r3, r9, -0x16e8
	ctx.r[3].s64 = ctx.r[9].s64 + -5864;
	// 8326C07C: 4BA3DEA5  bl 0x82ca9f20
	ctx.lr = 0x8326C080;
	sub_82CA9F20(ctx, base);
	// 8326C080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C090 size=64
    let mut pc: u32 = 0x8326C090;
    'dispatch: loop {
        match pc {
            0x8326C090 => {
    //   block [0x8326C090..0x8326C0D0)
	// 8326C090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C09C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C0A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C0A4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326C0A8: 386AC368  addi r3, r10, -0x3c98
	ctx.r[3].s64 = ctx.r[10].s64 + -15512;
	// 8326C0AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C0B0: 4AFC0E21  bl 0x8222ced0
	ctx.lr = 0x8326C0B4;
	sub_8222CED0(ctx, base);
	// 8326C0B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C0B8: 3869E928  addi r3, r9, -0x16d8
	ctx.r[3].s64 = ctx.r[9].s64 + -5848;
	// 8326C0BC: 4BA3DE65  bl 0x82ca9f20
	ctx.lr = 0x8326C0C0;
	sub_82CA9F20(ctx, base);
	// 8326C0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C0D0 size=64
    let mut pc: u32 = 0x8326C0D0;
    'dispatch: loop {
        match pc {
            0x8326C0D0 => {
    //   block [0x8326C0D0..0x8326C110)
	// 8326C0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C0DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C0E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C0E4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326C0E8: 386AC36C  addi r3, r10, -0x3c94
	ctx.r[3].s64 = ctx.r[10].s64 + -15508;
	// 8326C0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C0F0: 4AFC0DE1  bl 0x8222ced0
	ctx.lr = 0x8326C0F4;
	sub_8222CED0(ctx, base);
	// 8326C0F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C0F8: 3869E938  addi r3, r9, -0x16c8
	ctx.r[3].s64 = ctx.r[9].s64 + -5832;
	// 8326C0FC: 4BA3DE25  bl 0x82ca9f20
	ctx.lr = 0x8326C100;
	sub_82CA9F20(ctx, base);
	// 8326C100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C110 size=64
    let mut pc: u32 = 0x8326C110;
    'dispatch: loop {
        match pc {
            0x8326C110 => {
    //   block [0x8326C110..0x8326C150)
	// 8326C110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C11C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C124: 388B0B80  addi r4, r11, 0xb80
	ctx.r[4].s64 = ctx.r[11].s64 + 2944;
	// 8326C128: 386AC370  addi r3, r10, -0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + -15504;
	// 8326C12C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C130: 4AFC0DA1  bl 0x8222ced0
	ctx.lr = 0x8326C134;
	sub_8222CED0(ctx, base);
	// 8326C134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C138: 3869E948  addi r3, r9, -0x16b8
	ctx.r[3].s64 = ctx.r[9].s64 + -5816;
	// 8326C13C: 4BA3DDE5  bl 0x82ca9f20
	ctx.lr = 0x8326C140;
	sub_82CA9F20(ctx, base);
	// 8326C140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C150 size=64
    let mut pc: u32 = 0x8326C150;
    'dispatch: loop {
        match pc {
            0x8326C150 => {
    //   block [0x8326C150..0x8326C190)
	// 8326C150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C15C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C164: 388B0B8C  addi r4, r11, 0xb8c
	ctx.r[4].s64 = ctx.r[11].s64 + 2956;
	// 8326C168: 386AC374  addi r3, r10, -0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15500;
	// 8326C16C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C170: 4AFC0D61  bl 0x8222ced0
	ctx.lr = 0x8326C174;
	sub_8222CED0(ctx, base);
	// 8326C174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C178: 3869E958  addi r3, r9, -0x16a8
	ctx.r[3].s64 = ctx.r[9].s64 + -5800;
	// 8326C17C: 4BA3DDA5  bl 0x82ca9f20
	ctx.lr = 0x8326C180;
	sub_82CA9F20(ctx, base);
	// 8326C180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C190 size=64
    let mut pc: u32 = 0x8326C190;
    'dispatch: loop {
        match pc {
            0x8326C190 => {
    //   block [0x8326C190..0x8326C1D0)
	// 8326C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C19C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C1A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C1A4: 388B0B98  addi r4, r11, 0xb98
	ctx.r[4].s64 = ctx.r[11].s64 + 2968;
	// 8326C1A8: 386AC378  addi r3, r10, -0x3c88
	ctx.r[3].s64 = ctx.r[10].s64 + -15496;
	// 8326C1AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C1B0: 4AFC0D21  bl 0x8222ced0
	ctx.lr = 0x8326C1B4;
	sub_8222CED0(ctx, base);
	// 8326C1B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C1B8: 3869E968  addi r3, r9, -0x1698
	ctx.r[3].s64 = ctx.r[9].s64 + -5784;
	// 8326C1BC: 4BA3DD65  bl 0x82ca9f20
	ctx.lr = 0x8326C1C0;
	sub_82CA9F20(ctx, base);
	// 8326C1C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C1D0 size=64
    let mut pc: u32 = 0x8326C1D0;
    'dispatch: loop {
        match pc {
            0x8326C1D0 => {
    //   block [0x8326C1D0..0x8326C210)
	// 8326C1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C1D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C1DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C1E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C1E4: 388B0BA4  addi r4, r11, 0xba4
	ctx.r[4].s64 = ctx.r[11].s64 + 2980;
	// 8326C1E8: 386AC37C  addi r3, r10, -0x3c84
	ctx.r[3].s64 = ctx.r[10].s64 + -15492;
	// 8326C1EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C1F0: 4AFC0CE1  bl 0x8222ced0
	ctx.lr = 0x8326C1F4;
	sub_8222CED0(ctx, base);
	// 8326C1F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C1F8: 3869E978  addi r3, r9, -0x1688
	ctx.r[3].s64 = ctx.r[9].s64 + -5768;
	// 8326C1FC: 4BA3DD25  bl 0x82ca9f20
	ctx.lr = 0x8326C200;
	sub_82CA9F20(ctx, base);
	// 8326C200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C210 size=64
    let mut pc: u32 = 0x8326C210;
    'dispatch: loop {
        match pc {
            0x8326C210 => {
    //   block [0x8326C210..0x8326C250)
	// 8326C210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C21C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C224: 388B0BB4  addi r4, r11, 0xbb4
	ctx.r[4].s64 = ctx.r[11].s64 + 2996;
	// 8326C228: 386AC380  addi r3, r10, -0x3c80
	ctx.r[3].s64 = ctx.r[10].s64 + -15488;
	// 8326C22C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C230: 4AFC0CA1  bl 0x8222ced0
	ctx.lr = 0x8326C234;
	sub_8222CED0(ctx, base);
	// 8326C234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C238: 3869E988  addi r3, r9, -0x1678
	ctx.r[3].s64 = ctx.r[9].s64 + -5752;
	// 8326C23C: 4BA3DCE5  bl 0x82ca9f20
	ctx.lr = 0x8326C240;
	sub_82CA9F20(ctx, base);
	// 8326C240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C250 size=64
    let mut pc: u32 = 0x8326C250;
    'dispatch: loop {
        match pc {
            0x8326C250 => {
    //   block [0x8326C250..0x8326C290)
	// 8326C250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C25C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C264: 388B0B80  addi r4, r11, 0xb80
	ctx.r[4].s64 = ctx.r[11].s64 + 2944;
	// 8326C268: 386AC384  addi r3, r10, -0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15484;
	// 8326C26C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C270: 4AFC0C61  bl 0x8222ced0
	ctx.lr = 0x8326C274;
	sub_8222CED0(ctx, base);
	// 8326C274: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C278: 3869E998  addi r3, r9, -0x1668
	ctx.r[3].s64 = ctx.r[9].s64 + -5736;
	// 8326C27C: 4BA3DCA5  bl 0x82ca9f20
	ctx.lr = 0x8326C280;
	sub_82CA9F20(ctx, base);
	// 8326C280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C290 size=64
    let mut pc: u32 = 0x8326C290;
    'dispatch: loop {
        match pc {
            0x8326C290 => {
    //   block [0x8326C290..0x8326C2D0)
	// 8326C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C29C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C2A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C2A4: 388B0B98  addi r4, r11, 0xb98
	ctx.r[4].s64 = ctx.r[11].s64 + 2968;
	// 8326C2A8: 386AC388  addi r3, r10, -0x3c78
	ctx.r[3].s64 = ctx.r[10].s64 + -15480;
	// 8326C2AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C2B0: 4AFC0C21  bl 0x8222ced0
	ctx.lr = 0x8326C2B4;
	sub_8222CED0(ctx, base);
	// 8326C2B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C2B8: 3869E9A8  addi r3, r9, -0x1658
	ctx.r[3].s64 = ctx.r[9].s64 + -5720;
	// 8326C2BC: 4BA3DC65  bl 0x82ca9f20
	ctx.lr = 0x8326C2C0;
	sub_82CA9F20(ctx, base);
	// 8326C2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C2D0 size=64
    let mut pc: u32 = 0x8326C2D0;
    'dispatch: loop {
        match pc {
            0x8326C2D0 => {
    //   block [0x8326C2D0..0x8326C310)
	// 8326C2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C2DC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326C2E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C2E4: 388B0D04  addi r4, r11, 0xd04
	ctx.r[4].s64 = ctx.r[11].s64 + 3332;
	// 8326C2E8: 386AC38C  addi r3, r10, -0x3c74
	ctx.r[3].s64 = ctx.r[10].s64 + -15476;
	// 8326C2EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326C2F0: 4AFC0BE1  bl 0x8222ced0
	ctx.lr = 0x8326C2F4;
	sub_8222CED0(ctx, base);
	// 8326C2F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C2F8: 3869E9B8  addi r3, r9, -0x1648
	ctx.r[3].s64 = ctx.r[9].s64 + -5704;
	// 8326C2FC: 4BA3DC25  bl 0x82ca9f20
	ctx.lr = 0x8326C300;
	sub_82CA9F20(ctx, base);
	// 8326C300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326C310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C310 size=96
    let mut pc: u32 = 0x8326C310;
    'dispatch: loop {
        match pc {
            0x8326C310 => {
    //   block [0x8326C310..0x8326C370)
	// 8326C310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C31C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8326C320: 4AFB2F39  bl 0x8221f258
	ctx.lr = 0x8326C324;
	sub_8221F258(ctx, base);
	// 8326C324: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326C328: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8326C32C: 419A0008  beq cr6, 0x8326c334
	if ctx.cr[6].eq {
	pc = 0x8326C334; continue 'dispatch;
	}
	// 8326C330: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326C334: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8326C338: 41820008  beq 0x8326c340
	if ctx.cr[0].eq {
	pc = 0x8326C340; continue 'dispatch;
	}
	// 8326C33C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326C340: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326C344: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326C348: 3909C390  addi r8, r9, -0x3c70
	ctx.r[8].s64 = ctx.r[9].s64 + -15472;
	// 8326C34C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326C350: 3867E9C8  addi r3, r7, -0x1638
	ctx.r[3].s64 = ctx.r[7].s64 + -5688;
	// 8326C354: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326C358: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326C35C: 4BA3DBC5  bl 0x82ca9f20
	ctx.lr = 0x8326C360;
	sub_82CA9F20(ctx, base);
	// 8326C360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


