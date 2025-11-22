pub fn sub_83267D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D50 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D88 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267DC0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267DF8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267E30 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267E68 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267EA0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267ED8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F10 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F48 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F80 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267FB8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83267FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267FF0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268028 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268060 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268098 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832680D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832680D0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268108 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268140 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268178 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832681B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832681B0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832681E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832681E8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268220 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268258 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 83268270: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83268274: 386ADA08  addi r3, r10, -0x25f8
	ctx.r[3].s64 = ctx.r[10].s64 + -9720;
	// 83268278: 4BA41CA9  bl 0x82ca9f20
	ctx.lr = 0x8326827C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8326827C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268288: 4E800020  blr
	return;
}

pub fn sub_83268290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268290 size=12
	// 83268290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832682D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832682D0 size=12
	// 832682D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832682D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832682D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268310 size=12
	// 83268310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268350 size=12
	// 83268350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268390 size=12
	// 83268390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832683D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832683D0 size=12
	// 832683D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832683D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832683D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268410 size=12
	// 83268410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268450 size=12
	// 83268450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268490 size=12
	// 83268490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832684D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832684D0 size=12
	// 832684D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832684D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832684D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268510 size=12
	// 83268510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268550 size=12
	// 83268550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268590 size=12
	// 83268590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832685D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832685D0 size=12
	// 832685D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832685D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832685D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268610 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268648 size=12
	// 83268648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326864C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268688 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832686C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832686C0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832686F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832686F8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268730 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268768 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832687A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832687A0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832687D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832687D8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268810 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268848 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268880 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832688B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832688B8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832688F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832688F0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83268928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268928 size=608
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83268A9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268AA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AA4: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83268AA8: 4AFC4429  bl 0x8222ced0
	ctx.lr = 0x83268AAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83268AC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AC8: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83268ACC: 4AFC4405  bl 0x8222ced0
	ctx.lr = 0x83268AD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83268AE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268AE8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268AEC: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 83268AF0: 4AFC43E1  bl 0x8222ced0
	ctx.lr = 0x83268AF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83268B08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B10: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83268B14: 4AFC43BD  bl 0x8222ced0
	ctx.lr = 0x83268B18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83268B2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268B30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B34: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83268B38: 4AFC4399  bl 0x8222ced0
	ctx.lr = 0x83268B3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83268B50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83268B54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83268B58: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83268B5C: 4AFC4375  bl 0x8222ced0
	ctx.lr = 0x83268B60;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83268B60: 3C60832B  lis r3, -0x7cd5
	ctx.r[3].s64 = -2094333952;
	// 83268B64: 3863DB40  addi r3, r3, -0x24c0
	ctx.r[3].s64 = ctx.r[3].s64 + -9408;
	// 83268B68: 4BA413B9  bl 0x82ca9f20
	ctx.lr = 0x83268B6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
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

pub fn sub_83268B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268B88 size=12
	// 83268B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268B90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268BC8 size=12
	// 83268BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268C08 size=12
	// 83268C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268C48 size=208
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83268CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268CD4: 4E800020  blr
	return;
	// 83268CD8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83268CDC: 386BDBE8  addi r3, r11, -0x2418
	ctx.r[3].s64 = ctx.r[11].s64 + -9240;
	// 83268CE0: 4BA41240  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_83268D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268D18 size=12
	// 83268D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268D58 size=12
	// 83268D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268D98 size=12
	// 83268D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268DD8 size=12
	// 83268DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268E18 size=12
	// 83268E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83268E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268E58 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83268E78: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268E7C: 3869DC78  addi r3, r9, -0x2388
	ctx.r[3].s64 = ctx.r[9].s64 + -9096;
	// 83268E80: 4BA410A1  bl 0x82ca9f20
	ctx.lr = 0x83268E84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83268E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268E90: 4E800020  blr
	return;
}

pub fn sub_83268E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268E98 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83268EB8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268EBC: 3869DC88  addi r3, r9, -0x2378
	ctx.r[3].s64 = ctx.r[9].s64 + -9080;
	// 83268EC0: 4BA41061  bl 0x82ca9f20
	ctx.lr = 0x83268EC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83268EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268ED0: 4E800020  blr
	return;
}

pub fn sub_83268ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268ED8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83268EF8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268EFC: 3869DC98  addi r3, r9, -0x2368
	ctx.r[3].s64 = ctx.r[9].s64 + -9064;
	// 83268F00: 4BA41021  bl 0x82ca9f20
	ctx.lr = 0x83268F04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83268F04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268F10: 4E800020  blr
	return;
}

pub fn sub_83268F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268F18 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83268F38: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268F3C: 3869DCA8  addi r3, r9, -0x2358
	ctx.r[3].s64 = ctx.r[9].s64 + -9048;
	// 83268F40: 4BA40FE1  bl 0x82ca9f20
	ctx.lr = 0x83268F44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83268F44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268F50: 4E800020  blr
	return;
}

pub fn sub_83268F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268F58 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83268F78: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268F7C: 3869DCB8  addi r3, r9, -0x2348
	ctx.r[3].s64 = ctx.r[9].s64 + -9032;
	// 83268F80: 4BA40FA1  bl 0x82ca9f20
	ctx.lr = 0x83268F84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83268F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268F90: 4E800020  blr
	return;
}

pub fn sub_83268F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268F98 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83268FB8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268FBC: 3869DCC8  addi r3, r9, -0x2338
	ctx.r[3].s64 = ctx.r[9].s64 + -9016;
	// 83268FC0: 4BA40F61  bl 0x82ca9f20
	ctx.lr = 0x83268FC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83268FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268FD0: 4E800020  blr
	return;
}

pub fn sub_83268FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268FD8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83268FF8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83268FFC: 3869DCD8  addi r3, r9, -0x2328
	ctx.r[3].s64 = ctx.r[9].s64 + -9000;
	// 83269000: 4BA40F21  bl 0x82ca9f20
	ctx.lr = 0x83269004;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326900C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269010: 4E800020  blr
	return;
}

pub fn sub_83269018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269018 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269038: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326903C: 3869DCE8  addi r3, r9, -0x2318
	ctx.r[3].s64 = ctx.r[9].s64 + -8984;
	// 83269040: 4BA40EE1  bl 0x82ca9f20
	ctx.lr = 0x83269044;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326904C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269050: 4E800020  blr
	return;
}

pub fn sub_83269058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269058 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269078: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326907C: 3869DCF8  addi r3, r9, -0x2308
	ctx.r[3].s64 = ctx.r[9].s64 + -8968;
	// 83269080: 4BA40EA1  bl 0x82ca9f20
	ctx.lr = 0x83269084;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326908C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269090: 4E800020  blr
	return;
}

pub fn sub_83269098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269098 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832690B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832690BC: 3869DD08  addi r3, r9, -0x22f8
	ctx.r[3].s64 = ctx.r[9].s64 + -8952;
	// 832690C0: 4BA40E61  bl 0x82ca9f20
	ctx.lr = 0x832690C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832690C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832690C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832690CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832690D0: 4E800020  blr
	return;
}

pub fn sub_832690D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832690D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832690F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832690FC: 3869DD18  addi r3, r9, -0x22e8
	ctx.r[3].s64 = ctx.r[9].s64 + -8936;
	// 83269100: 4BA40E21  bl 0x82ca9f20
	ctx.lr = 0x83269104;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326910C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269110: 4E800020  blr
	return;
}

pub fn sub_83269118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269118 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269138: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326913C: 3869DD28  addi r3, r9, -0x22d8
	ctx.r[3].s64 = ctx.r[9].s64 + -8920;
	// 83269140: 4BA40DE1  bl 0x82ca9f20
	ctx.lr = 0x83269144;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326914C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269150: 4E800020  blr
	return;
}

pub fn sub_83269158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269158 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269178: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326917C: 3869DD38  addi r3, r9, -0x22c8
	ctx.r[3].s64 = ctx.r[9].s64 + -8904;
	// 83269180: 4BA40DA1  bl 0x82ca9f20
	ctx.lr = 0x83269184;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326918C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269190: 4E800020  blr
	return;
}

pub fn sub_83269198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269198 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832691B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832691BC: 3869DD48  addi r3, r9, -0x22b8
	ctx.r[3].s64 = ctx.r[9].s64 + -8888;
	// 832691C0: 4BA40D61  bl 0x82ca9f20
	ctx.lr = 0x832691C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832691C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832691C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832691CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832691D0: 4E800020  blr
	return;
}

pub fn sub_832691D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832691D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832691F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832691FC: 3869DD58  addi r3, r9, -0x22a8
	ctx.r[3].s64 = ctx.r[9].s64 + -8872;
	// 83269200: 4BA40D21  bl 0x82ca9f20
	ctx.lr = 0x83269204;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326920C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269210: 4E800020  blr
	return;
}

pub fn sub_83269218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269218 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269238: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326923C: 3869DD68  addi r3, r9, -0x2298
	ctx.r[3].s64 = ctx.r[9].s64 + -8856;
	// 83269240: 4BA40CE1  bl 0x82ca9f20
	ctx.lr = 0x83269244;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326924C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269250: 4E800020  blr
	return;
}

pub fn sub_83269258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269258 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269278: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326927C: 3869DD78  addi r3, r9, -0x2288
	ctx.r[3].s64 = ctx.r[9].s64 + -8840;
	// 83269280: 4BA40CA1  bl 0x82ca9f20
	ctx.lr = 0x83269284;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326928C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269290: 4E800020  blr
	return;
}

pub fn sub_83269298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269298 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832692B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832692BC: 3869DD88  addi r3, r9, -0x2278
	ctx.r[3].s64 = ctx.r[9].s64 + -8824;
	// 832692C0: 4BA40C61  bl 0x82ca9f20
	ctx.lr = 0x832692C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832692C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832692C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832692CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832692D0: 4E800020  blr
	return;
}

pub fn sub_832692D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832692D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832692F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832692FC: 3869DD98  addi r3, r9, -0x2268
	ctx.r[3].s64 = ctx.r[9].s64 + -8808;
	// 83269300: 4BA40C21  bl 0x82ca9f20
	ctx.lr = 0x83269304;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326930C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269310: 4E800020  blr
	return;
}

pub fn sub_83269318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269318 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269338: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326933C: 3869DDA8  addi r3, r9, -0x2258
	ctx.r[3].s64 = ctx.r[9].s64 + -8792;
	// 83269340: 4BA40BE1  bl 0x82ca9f20
	ctx.lr = 0x83269344;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326934C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269350: 4E800020  blr
	return;
}

pub fn sub_83269358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269358 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269378: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326937C: 3869DDB8  addi r3, r9, -0x2248
	ctx.r[3].s64 = ctx.r[9].s64 + -8776;
	// 83269380: 4BA40BA1  bl 0x82ca9f20
	ctx.lr = 0x83269384;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326938C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269390: 4E800020  blr
	return;
}

pub fn sub_83269398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269398 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832693B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832693BC: 3869DDC8  addi r3, r9, -0x2238
	ctx.r[3].s64 = ctx.r[9].s64 + -8760;
	// 832693C0: 4BA40B61  bl 0x82ca9f20
	ctx.lr = 0x832693C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832693C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832693C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832693CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832693D0: 4E800020  blr
	return;
}

pub fn sub_832693D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832693D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832693F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832693FC: 3869DDD8  addi r3, r9, -0x2228
	ctx.r[3].s64 = ctx.r[9].s64 + -8744;
	// 83269400: 4BA40B21  bl 0x82ca9f20
	ctx.lr = 0x83269404;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326940C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269410: 4E800020  blr
	return;
}

pub fn sub_83269418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269418 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269438: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326943C: 3869DDE8  addi r3, r9, -0x2218
	ctx.r[3].s64 = ctx.r[9].s64 + -8728;
	// 83269440: 4BA40AE1  bl 0x82ca9f20
	ctx.lr = 0x83269444;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326944C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269450: 4E800020  blr
	return;
}

pub fn sub_83269458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269458 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269478: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326947C: 3869DDF8  addi r3, r9, -0x2208
	ctx.r[3].s64 = ctx.r[9].s64 + -8712;
	// 83269480: 4BA40AA1  bl 0x82ca9f20
	ctx.lr = 0x83269484;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326948C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269490: 4E800020  blr
	return;
}

pub fn sub_83269498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269498 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832694B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832694BC: 3869DE08  addi r3, r9, -0x21f8
	ctx.r[3].s64 = ctx.r[9].s64 + -8696;
	// 832694C0: 4BA40A61  bl 0x82ca9f20
	ctx.lr = 0x832694C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832694C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832694C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832694CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832694D0: 4E800020  blr
	return;
}

pub fn sub_832694D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832694D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832694F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832694FC: 3869DE18  addi r3, r9, -0x21e8
	ctx.r[3].s64 = ctx.r[9].s64 + -8680;
	// 83269500: 4BA40A21  bl 0x82ca9f20
	ctx.lr = 0x83269504;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326950C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269510: 4E800020  blr
	return;
}

pub fn sub_83269518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269518 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269538: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326953C: 3869DE28  addi r3, r9, -0x21d8
	ctx.r[3].s64 = ctx.r[9].s64 + -8664;
	// 83269540: 4BA409E1  bl 0x82ca9f20
	ctx.lr = 0x83269544;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326954C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269550: 4E800020  blr
	return;
}

pub fn sub_83269558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269558 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269578: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326957C: 3869DE38  addi r3, r9, -0x21c8
	ctx.r[3].s64 = ctx.r[9].s64 + -8648;
	// 83269580: 4BA409A1  bl 0x82ca9f20
	ctx.lr = 0x83269584;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269590: 4E800020  blr
	return;
}

pub fn sub_83269598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269598 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832695B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832695BC: 3869DE48  addi r3, r9, -0x21b8
	ctx.r[3].s64 = ctx.r[9].s64 + -8632;
	// 832695C0: 4BA40961  bl 0x82ca9f20
	ctx.lr = 0x832695C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832695C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832695C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832695CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832695D0: 4E800020  blr
	return;
}

pub fn sub_832695D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832695D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832695F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832695FC: 3869DE58  addi r3, r9, -0x21a8
	ctx.r[3].s64 = ctx.r[9].s64 + -8616;
	// 83269600: 4BA40921  bl 0x82ca9f20
	ctx.lr = 0x83269604;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326960C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269610: 4E800020  blr
	return;
}

pub fn sub_83269618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269618 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269638: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326963C: 3869DE68  addi r3, r9, -0x2198
	ctx.r[3].s64 = ctx.r[9].s64 + -8600;
	// 83269640: 4BA408E1  bl 0x82ca9f20
	ctx.lr = 0x83269644;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326964C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269650: 4E800020  blr
	return;
}

pub fn sub_83269658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269658 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269678: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326967C: 3869DE78  addi r3, r9, -0x2188
	ctx.r[3].s64 = ctx.r[9].s64 + -8584;
	// 83269680: 4BA408A1  bl 0x82ca9f20
	ctx.lr = 0x83269684;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326968C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269690: 4E800020  blr
	return;
}

pub fn sub_83269698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269698 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832696B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832696BC: 3869DE88  addi r3, r9, -0x2178
	ctx.r[3].s64 = ctx.r[9].s64 + -8568;
	// 832696C0: 4BA40861  bl 0x82ca9f20
	ctx.lr = 0x832696C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832696C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832696C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832696CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832696D0: 4E800020  blr
	return;
}

pub fn sub_832696D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832696D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832696F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832696FC: 3869DE98  addi r3, r9, -0x2168
	ctx.r[3].s64 = ctx.r[9].s64 + -8552;
	// 83269700: 4BA40821  bl 0x82ca9f20
	ctx.lr = 0x83269704;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269710: 4E800020  blr
	return;
}

pub fn sub_83269718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269718 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269738: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326973C: 3869DEA8  addi r3, r9, -0x2158
	ctx.r[3].s64 = ctx.r[9].s64 + -8536;
	// 83269740: 4BA407E1  bl 0x82ca9f20
	ctx.lr = 0x83269744;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326974C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269750: 4E800020  blr
	return;
}

pub fn sub_83269758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269758 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269778: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326977C: 3869DEB8  addi r3, r9, -0x2148
	ctx.r[3].s64 = ctx.r[9].s64 + -8520;
	// 83269780: 4BA407A1  bl 0x82ca9f20
	ctx.lr = 0x83269784;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326978C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269790: 4E800020  blr
	return;
}

pub fn sub_83269798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269798 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832697B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832697BC: 3869DEC8  addi r3, r9, -0x2138
	ctx.r[3].s64 = ctx.r[9].s64 + -8504;
	// 832697C0: 4BA40761  bl 0x82ca9f20
	ctx.lr = 0x832697C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832697C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832697C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832697CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832697D0: 4E800020  blr
	return;
}

pub fn sub_832697D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832697D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832697F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832697FC: 3869DED8  addi r3, r9, -0x2128
	ctx.r[3].s64 = ctx.r[9].s64 + -8488;
	// 83269800: 4BA40721  bl 0x82ca9f20
	ctx.lr = 0x83269804;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326980C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269810: 4E800020  blr
	return;
}

pub fn sub_83269818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269818 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269838: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326983C: 3869DEE8  addi r3, r9, -0x2118
	ctx.r[3].s64 = ctx.r[9].s64 + -8472;
	// 83269840: 4BA406E1  bl 0x82ca9f20
	ctx.lr = 0x83269844;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326984C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269850: 4E800020  blr
	return;
}

pub fn sub_83269858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269858 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269878: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326987C: 3869DEF8  addi r3, r9, -0x2108
	ctx.r[3].s64 = ctx.r[9].s64 + -8456;
	// 83269880: 4BA406A1  bl 0x82ca9f20
	ctx.lr = 0x83269884;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326988C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269890: 4E800020  blr
	return;
}

pub fn sub_83269898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269898 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832698B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832698BC: 3869DF08  addi r3, r9, -0x20f8
	ctx.r[3].s64 = ctx.r[9].s64 + -8440;
	// 832698C0: 4BA40661  bl 0x82ca9f20
	ctx.lr = 0x832698C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832698C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832698C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832698CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832698D0: 4E800020  blr
	return;
}

pub fn sub_832698D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832698D8 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832698F8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832698FC: 3869DF18  addi r3, r9, -0x20e8
	ctx.r[3].s64 = ctx.r[9].s64 + -8424;
	// 83269900: 4BA40621  bl 0x82ca9f20
	ctx.lr = 0x83269904;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326990C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269910: 4E800020  blr
	return;
}

pub fn sub_83269918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269918 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269938: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326993C: 3869DF28  addi r3, r9, -0x20d8
	ctx.r[3].s64 = ctx.r[9].s64 + -8408;
	// 83269940: 4BA405E1  bl 0x82ca9f20
	ctx.lr = 0x83269944;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326994C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269950: 4E800020  blr
	return;
}

pub fn sub_83269958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269958 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83269978: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326997C: 3869DF38  addi r3, r9, -0x20c8
	ctx.r[3].s64 = ctx.r[9].s64 + -8392;
	// 83269980: 4BA405A1  bl 0x82ca9f20
	ctx.lr = 0x83269984;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326998C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269990: 4E800020  blr
	return;
}

pub fn sub_83269998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269998 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 832699B8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832699BC: 3869DF48  addi r3, r9, -0x20b8
	ctx.r[3].s64 = ctx.r[9].s64 + -8376;
	// 832699C0: 4BA40561  bl 0x82ca9f20
	ctx.lr = 0x832699C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832699C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832699C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832699CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832699D0: 4E800020  blr
	return;
}

pub fn sub_832699D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832699D8 size=144
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83269A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83269A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83269A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83269A64: 4E800020  blr
	return;
}

pub fn sub_83269A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269A68 size=12
	// 83269A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269AA8 size=12
	// 83269AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269AE8 size=12
	// 83269AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269B28 size=12
	// 83269B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269B68 size=12
	// 83269B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269BA8 size=12
	// 83269BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269BE8 size=12
	// 83269BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269C48 size=12
	// 83269C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269CA8 size=12
	// 83269CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269CE8 size=12
	// 83269CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269D28 size=12
	// 83269D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269D68 size=12
	// 83269D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269DA8 size=12
	// 83269DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269DE8 size=12
	// 83269DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269E28 size=12
	// 83269E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269E68 size=12
	// 83269E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269EA8 size=12
	// 83269EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269EE8 size=12
	// 83269EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269F28 size=12
	// 83269F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269F68 size=12
	// 83269F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269FA8 size=12
	// 83269FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83269FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83269FE8 size=12
	// 83269FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83269FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83269FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A028 size=12
	// 8326A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A068 size=12
	// 8326A068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A0A8 size=12
	// 8326A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A0B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A0E8 size=12
	// 8326A0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A0F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A128 size=12
	// 8326A128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A1A8 size=12
	// 8326A1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A1B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A1F8 size=12
	// 8326A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A238 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A270 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326A2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A2A8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A2E0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326A318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A318 size=12
	// 8326A318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A358 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326A390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A390 size=12
	// 8326A390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A3D0 size=12
	// 8326A3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A410 size=12
	// 8326A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A450 size=12
	// 8326A450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A490 size=12
	// 8326A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A4D0 size=12
	// 8326A4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A4D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A510 size=12
	// 8326A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A550 size=12
	// 8326A550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A590 size=12
	// 8326A590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A5D0 size=12
	// 8326A5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A5D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A610 size=12
	// 8326A610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A650 size=12
	// 8326A650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A690 size=12
	// 8326A690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A6D0 size=12
	// 8326A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A6D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A710 size=12
	// 8326A710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A750 size=12
	// 8326A750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A790 size=12
	// 8326A790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A7D0 size=12
	// 8326A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A7D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A810 size=12
	// 8326A810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A850 size=12
	// 8326A850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A890 size=12
	// 8326A890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A8D0 size=12
	// 8326A8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A8D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A910 size=12
	// 8326A910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A950 size=12
	// 8326A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A990 size=12
	// 8326A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326A9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326A9D0 size=12
	// 8326A9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326A9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326A9D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA10 size=12
	// 8326AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA50 size=12
	// 8326AA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AA90 size=12
	// 8326AA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AA98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AAD0 size=12
	// 8326AAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB10 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB48 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AB80 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ABB8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326ABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ABF0 size=12
	// 8326ABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ABF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ABF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AC30 size=12
	// 8326AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AC38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AC70 size=12
	// 8326AC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ACB0 size=12
	// 8326ACB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ACB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ACB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ACF0 size=12
	// 8326ACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ACF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AD30 size=12
	// 8326AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AD38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AD70 size=12
	// 8326AD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ADB0 size=12
	// 8326ADB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ADB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ADB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326ADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ADF0 size=12
	// 8326ADF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ADF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ADF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AE30 size=12
	// 8326AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AE70 size=12
	// 8326AE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AEB0 size=12
	// 8326AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326AEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AEF0 size=16
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
}

pub fn sub_8326AFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326AFB0 size=16
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
}

pub fn sub_8326B070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B070 size=16
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
}

pub fn sub_8326B130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B130 size=16
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
}

pub fn sub_8326B1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B1F0 size=16
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
}

pub fn sub_8326B2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B2B0 size=12
	// 8326B2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B2F0 size=12
	// 8326B2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B2F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B330 size=12
	// 8326B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B370 size=12
	// 8326B370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B3B0 size=12
	// 8326B3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B3F0 size=12
	// 8326B3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B3F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B430 size=12
	// 8326B430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B470 size=12
	// 8326B470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B4B0 size=12
	// 8326B4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B4F0 size=12
	// 8326B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B530 size=12
	// 8326B530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B570 size=12
	// 8326B570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B5B0 size=12
	// 8326B5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B5B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B5F0 size=12
	// 8326B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B630 size=12
	// 8326B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B670 size=12
	// 8326B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B6B0 size=12
	// 8326B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B6F0 size=12
	// 8326B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B730 size=12
	// 8326B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B770 size=12
	// 8326B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B7B0 size=12
	// 8326B7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B7F0 size=12
	// 8326B7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B830 size=12
	// 8326B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B870 size=12
	// 8326B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B8B0 size=12
	// 8326B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B8B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B8F0 size=12
	// 8326B8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B8F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B930 size=12
	// 8326B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B970 size=12
	// 8326B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B9B0 size=12
	// 8326B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326B9F0 size=12
	// 8326B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BA30 size=12
	// 8326BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BA80 size=12
	// 8326BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BAC0 size=12
	// 8326BAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BB00 size=12
	// 8326BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BB40 size=104
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326BBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BBA8 size=12
	// 8326BBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BBE8 size=12
	// 8326BBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BC28 size=12
	// 8326BC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BC68 size=12
	// 8326BC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BCA8 size=12
	// 8326BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BCB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BCE8 size=12
	// 8326BCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BD28 size=12
	// 8326BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BD98 size=12
	// 8326BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BDD8 size=12
	// 8326BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BE18 size=12
	// 8326BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BE58 size=12
	// 8326BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326BE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326BEF8 size=280
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326BF38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326BF3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326BF40: 4AF808B9  bl 0x821ec7f8
	ctx.lr = 0x8326BF44;
	crate::recompiler::externs::call(&mut ctx, base, 0x821EC7F8);
	// 8326BF44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8326BF48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF4C: 4AF82BED  bl 0x821eeb38
	ctx.lr = 0x8326BF50;
	crate::recompiler::externs::call(&mut ctx, base, 0x821EEB38);
	// 8326BF50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326BF54: 4B99789D  bl 0x82c037f0
	ctx.lr = 0x8326BF58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C037F0);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
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
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
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

pub fn sub_8326C010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C010 size=64
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
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 8326C030: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326C034: 3869E908  addi r3, r9, -0x16f8
	ctx.r[3].s64 = ctx.r[9].s64 + -5880;
	// 8326C038: 4BA3DEE9  bl 0x82ca9f20
	ctx.lr = 0x8326C03C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8326C03C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C048: 4E800020  blr
	return;
}

pub fn sub_8326C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C050 size=12
	// 8326C050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C090 size=12
	// 8326C090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C0D0 size=12
	// 8326C0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C110 size=12
	// 8326C110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C150 size=12
	// 8326C150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C190 size=12
	// 8326C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C1D0 size=12
	// 8326C1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C1D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

