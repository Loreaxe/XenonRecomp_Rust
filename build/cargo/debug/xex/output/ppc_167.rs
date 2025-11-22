pub fn sub_83256C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256C60 size=12
	// 83256C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256CA0 size=12
	// 83256CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256CE0 size=12
	// 83256CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D20 size=56
	// 83256D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256D2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256D30: 386B884C  addi r3, r11, -0x77b4
	ctx.r[3].s64 = ctx.r[11].s64 + -30644;
	// 83256D34: 4B229505  bl 0x82480238
	ctx.lr = 0x83256D38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 83256D38: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256D3C: 386AABC0  addi r3, r10, -0x5440
	ctx.r[3].s64 = ctx.r[10].s64 + -21568;
	// 83256D40: 4BA531E1  bl 0x82ca9f20
	ctx.lr = 0x83256D44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83256D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256D50: 4E800020  blr
	return;
}

pub fn sub_83256D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D58 size=12
	// 83256D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D98 size=12
	// 83256D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256DD8 size=12
	// 83256DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256E48 size=12
	// 83256E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256E88 size=12
	// 83256E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256EC8 size=12
	// 83256EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F08 size=12
	// 83256F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F48 size=12
	// 83256F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F88 size=12
	// 83256F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256FC8 size=12
	// 83256FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257008 size=56
	// 83257008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257014: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83257018: 386B8884  addi r3, r11, -0x777c
	ctx.r[3].s64 = ctx.r[11].s64 + -30588;
	// 8325701C: 4B22921D  bl 0x82480238
	ctx.lr = 0x83257020;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 83257020: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83257024: 386AAC88  addi r3, r10, -0x5378
	ctx.r[3].s64 = ctx.r[10].s64 + -21368;
	// 83257028: 4BA52EF9  bl 0x82ca9f20
	ctx.lr = 0x8325702C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325702C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257038: 4E800020  blr
	return;
}

pub fn sub_83257040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257040 size=12
	// 83257040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257080 size=12
	// 83257080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832570C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832570C0 size=12
	// 832570C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832570C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832570C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257100 size=12
	// 83257100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257140 size=12
	// 83257140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257180 size=12
	// 83257180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832571C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832571C0 size=12
	// 832571C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832571C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832571C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257200 size=12
	// 83257200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257240 size=12
	// 83257240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832572A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832572A0 size=12
	// 832572A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832572A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832572A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832572E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832572E0 size=12
	// 832572E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832572E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832572E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257320 size=12
	// 83257320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257360 size=12
	// 83257360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832573A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832573A0 size=12
	// 832573A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832573A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832573A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832573E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832573E0 size=12
	// 832573E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832573E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832573E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257420 size=12
	// 83257420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257460 size=12
	// 83257460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832574A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832574A0 size=12
	// 832574A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832574A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832574A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832574E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832574E0 size=12
	// 832574E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832574E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832574E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257520 size=12
	// 83257520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257560 size=12
	// 83257560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832575A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832575A0 size=12
	// 832575A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832575A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832575A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832575E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832575E0 size=12
	// 832575E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832575E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832575E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257620 size=12
	// 83257620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257660 size=12
	// 83257660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832576A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832576A0 size=12
	// 832576A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832576A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832576A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832576E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832576E0 size=12
	// 832576E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832576E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832576E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257720 size=12
	// 83257720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257760 size=12
	// 83257760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832577A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832577A0 size=12
	// 832577A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832577A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832577A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832577E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832577E0 size=12
	// 832577E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832577E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832577E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257820 size=12
	// 83257820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257860 size=12
	// 83257860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832578A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832578A0 size=12
	// 832578A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832578A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832578A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832578E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832578E0 size=12
	// 832578E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832578E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832578E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257920 size=12
	// 83257920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257960 size=12
	// 83257960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832579A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832579A0 size=12
	// 832579A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832579A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832579A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832579E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832579E0 size=12
	// 832579E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832579E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832579E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257A20 size=12
	// 83257A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257A60 size=12
	// 83257A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257AA0 size=12
	// 83257AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257AE0 size=12
	// 83257AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257AE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257B20 size=12
	// 83257B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257B60 size=12
	// 83257B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257BA0 size=12
	// 83257BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257BE0 size=12
	// 83257BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257C20 size=12
	// 83257C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257C60 size=12
	// 83257C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257CA0 size=12
	// 83257CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257CE0 size=56
	// 83257CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257CEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257CF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257CF4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83257CF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257CFC: 4AF9C05D  bl 0x821f3d58
	ctx.lr = 0x83257D00;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83257D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D04: 906A8960  stw r3, -0x76a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30368 as u32), ctx.r[3].u32 ) };
	// 83257D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D14: 4E800020  blr
	return;
}

pub fn sub_83257D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D18 size=56
	// 83257D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D24: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257D28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D2C: 386B4DBC  addi r3, r11, 0x4dbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19900;
	// 83257D30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257D34: 4AF9C025  bl 0x821f3d58
	ctx.lr = 0x83257D38;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83257D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D3C: 906A8964  stw r3, -0x769c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30364 as u32), ctx.r[3].u32 ) };
	// 83257D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D4C: 4E800020  blr
	return;
}

pub fn sub_83257D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D50 size=56
	// 83257D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257D60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D64: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83257D68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257D6C: 4AF9BFED  bl 0x821f3d58
	ctx.lr = 0x83257D70;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83257D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D74: 906A8968  stw r3, -0x7698(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30360 as u32), ctx.r[3].u32 ) };
	// 83257D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D84: 4E800020  blr
	return;
}

pub fn sub_83257D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D88 size=56
	// 83257D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257D98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D9C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83257DA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257DA4: 4AF9BFB5  bl 0x821f3d58
	ctx.lr = 0x83257DA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83257DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257DAC: 906A896C  stw r3, -0x7694(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30356 as u32), ctx.r[3].u32 ) };
	// 83257DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257DBC: 4E800020  blr
	return;
}

pub fn sub_83257DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257DC0 size=56
	// 83257DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257DCC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257DD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257DD4: 386B4DD4  addi r3, r11, 0x4dd4
	ctx.r[3].s64 = ctx.r[11].s64 + 19924;
	// 83257DD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257DDC: 4AF9BF7D  bl 0x821f3d58
	ctx.lr = 0x83257DE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83257DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257DE4: 906A8970  stw r3, -0x7690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30352 as u32), ctx.r[3].u32 ) };
	// 83257DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257DF4: 4E800020  blr
	return;
}

pub fn sub_83257DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257DF8 size=56
	// 83257DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257E04: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257E08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257E0C: 386B4DEC  addi r3, r11, 0x4dec
	ctx.r[3].s64 = ctx.r[11].s64 + 19948;
	// 83257E10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257E14: 4AF9BF45  bl 0x821f3d58
	ctx.lr = 0x83257E18;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83257E18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257E1C: 906A8974  stw r3, -0x768c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30348 as u32), ctx.r[3].u32 ) };
	// 83257E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257E2C: 4E800020  blr
	return;
}

pub fn sub_83257E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257E30 size=12
	// 83257E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257E70 size=12
	// 83257E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257EB0 size=12
	// 83257EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257EF0 size=12
	// 83257EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257F30 size=12
	// 83257F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257F70 size=12
	// 83257F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257FB0 size=12
	// 83257FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83257FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257FF0 size=12
	// 83257FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258030 size=12
	// 83258030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258070 size=12
	// 83258070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832580C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832580C0 size=12
	// 832580C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832580C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832580C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258100 size=12
	// 83258100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258140 size=12
	// 83258140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832581B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832581B0 size=12
	// 832581B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832581B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832581B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832581F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832581F0 size=12
	// 832581F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832581F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832581F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258230 size=12
	// 83258230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258270 size=12
	// 83258270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832582B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832582B0 size=12
	// 832582B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832582B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832582B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832582F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832582F0 size=12
	// 832582F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832582F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832582F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258330 size=12
	// 83258330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258370 size=712
	// 83258370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325837C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83258384: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83258388: 3BEB8A38  addi r31, r11, -0x75c8
	ctx.r[31].s64 = ctx.r[11].s64 + -30152;
	// 8325838C: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83258390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83258394: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258398: 4AFD4B39  bl 0x8222ced0
	ctx.lr = 0x8325839C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325839C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832583A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583A4: 3889674C  addi r4, r9, 0x674c
	ctx.r[4].s64 = ctx.r[9].s64 + 26444;
	// 832583A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832583AC: 4AFD4B25  bl 0x8222ced0
	ctx.lr = 0x832583B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832583B0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832583B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583B8: 38886748  addi r4, r8, 0x6748
	ctx.r[4].s64 = ctx.r[8].s64 + 26440;
	// 832583BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832583C0: 4AFD4B11  bl 0x8222ced0
	ctx.lr = 0x832583C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832583C4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832583C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583CC: 38876744  addi r4, r7, 0x6744
	ctx.r[4].s64 = ctx.r[7].s64 + 26436;
	// 832583D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832583D4: 4AFD4AFD  bl 0x8222ced0
	ctx.lr = 0x832583D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832583D8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832583DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583E0: 38866740  addi r4, r6, 0x6740
	ctx.r[4].s64 = ctx.r[6].s64 + 26432;
	// 832583E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832583E8: 4AFD4AE9  bl 0x8222ced0
	ctx.lr = 0x832583EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832583EC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832583F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583F4: 3884673C  addi r4, r4, 0x673c
	ctx.r[4].s64 = ctx.r[4].s64 + 26428;
	// 832583F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832583FC: 4AFD4AD5  bl 0x8222ced0
	ctx.lr = 0x83258400;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258400: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258408: 38836738  addi r4, r3, 0x6738
	ctx.r[4].s64 = ctx.r[3].s64 + 26424;
	// 8325840C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83258410: 4AFD4AC1  bl 0x8222ced0
	ctx.lr = 0x83258414;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258414: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258418: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325841C: 388B6734  addi r4, r11, 0x6734
	ctx.r[4].s64 = ctx.r[11].s64 + 26420;
	// 83258420: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83258424: 4AFD4AAD  bl 0x8222ced0
	ctx.lr = 0x83258428;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258428: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325842C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258430: 388A6730  addi r4, r10, 0x6730
	ctx.r[4].s64 = ctx.r[10].s64 + 26416;
	// 83258434: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83258438: 4AFD4A99  bl 0x8222ced0
	ctx.lr = 0x8325843C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325843C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258440: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258444: 3889672C  addi r4, r9, 0x672c
	ctx.r[4].s64 = ctx.r[9].s64 + 26412;
	// 83258448: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8325844C: 4AFD4A85  bl 0x8222ced0
	ctx.lr = 0x83258450;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258450: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258458: 38886728  addi r4, r8, 0x6728
	ctx.r[4].s64 = ctx.r[8].s64 + 26408;
	// 8325845C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83258460: 4AFD4A71  bl 0x8222ced0
	ctx.lr = 0x83258464;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258464: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258468: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325846C: 38876724  addi r4, r7, 0x6724
	ctx.r[4].s64 = ctx.r[7].s64 + 26404;
	// 83258470: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83258474: 4AFD4A5D  bl 0x8222ced0
	ctx.lr = 0x83258478;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258478: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325847C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258480: 38866720  addi r4, r6, 0x6720
	ctx.r[4].s64 = ctx.r[6].s64 + 26400;
	// 83258484: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83258488: 4AFD4A49  bl 0x8222ced0
	ctx.lr = 0x8325848C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325848C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258490: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258494: 3884671C  addi r4, r4, 0x671c
	ctx.r[4].s64 = ctx.r[4].s64 + 26396;
	// 83258498: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8325849C: 4AFD4A35  bl 0x8222ced0
	ctx.lr = 0x832584A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832584A0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832584A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584A8: 38836718  addi r4, r3, 0x6718
	ctx.r[4].s64 = ctx.r[3].s64 + 26392;
	// 832584AC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832584B0: 4AFD4A21  bl 0x8222ced0
	ctx.lr = 0x832584B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832584B4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832584B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584BC: 388B6714  addi r4, r11, 0x6714
	ctx.r[4].s64 = ctx.r[11].s64 + 26388;
	// 832584C0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832584C4: 4AFD4A0D  bl 0x8222ced0
	ctx.lr = 0x832584C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832584C8: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832584CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584D0: 388A6710  addi r4, r10, 0x6710
	ctx.r[4].s64 = ctx.r[10].s64 + 26384;
	// 832584D4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832584D8: 4AFD49F9  bl 0x8222ced0
	ctx.lr = 0x832584DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832584DC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832584E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584E4: 38896708  addi r4, r9, 0x6708
	ctx.r[4].s64 = ctx.r[9].s64 + 26376;
	// 832584E8: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 832584EC: 4AFD49E5  bl 0x8222ced0
	ctx.lr = 0x832584F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832584F0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832584F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584F8: 38886700  addi r4, r8, 0x6700
	ctx.r[4].s64 = ctx.r[8].s64 + 26368;
	// 832584FC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83258500: 4AFD49D1  bl 0x8222ced0
	ctx.lr = 0x83258504;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258504: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258508: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325850C: 388766F8  addi r4, r7, 0x66f8
	ctx.r[4].s64 = ctx.r[7].s64 + 26360;
	// 83258510: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83258514: 4AFD49BD  bl 0x8222ced0
	ctx.lr = 0x83258518;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258518: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325851C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258520: 388666F0  addi r4, r6, 0x66f0
	ctx.r[4].s64 = ctx.r[6].s64 + 26352;
	// 83258524: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83258528: 4AFD49A9  bl 0x8222ced0
	ctx.lr = 0x8325852C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325852C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258530: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258534: 388466E8  addi r4, r4, 0x66e8
	ctx.r[4].s64 = ctx.r[4].s64 + 26344;
	// 83258538: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8325853C: 4AFD4995  bl 0x8222ced0
	ctx.lr = 0x83258540;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258540: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258544: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258548: 388366E0  addi r4, r3, 0x66e0
	ctx.r[4].s64 = ctx.r[3].s64 + 26336;
	// 8325854C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83258550: 4AFD4981  bl 0x8222ced0
	ctx.lr = 0x83258554;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258554: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258558: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325855C: 388B66D8  addi r4, r11, 0x66d8
	ctx.r[4].s64 = ctx.r[11].s64 + 26328;
	// 83258560: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83258564: 4AFD496D  bl 0x8222ced0
	ctx.lr = 0x83258568;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258568: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325856C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258570: 388A66D0  addi r4, r10, 0x66d0
	ctx.r[4].s64 = ctx.r[10].s64 + 26320;
	// 83258574: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83258578: 4AFD4959  bl 0x8222ced0
	ctx.lr = 0x8325857C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325857C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258580: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258584: 388966C8  addi r4, r9, 0x66c8
	ctx.r[4].s64 = ctx.r[9].s64 + 26312;
	// 83258588: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 8325858C: 4AFD4945  bl 0x8222ced0
	ctx.lr = 0x83258590;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258590: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258598: 388866C0  addi r4, r8, 0x66c0
	ctx.r[4].s64 = ctx.r[8].s64 + 26304;
	// 8325859C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 832585A0: 4AFD4931  bl 0x8222ced0
	ctx.lr = 0x832585A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832585A4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832585A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585AC: 388766B8  addi r4, r7, 0x66b8
	ctx.r[4].s64 = ctx.r[7].s64 + 26296;
	// 832585B0: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 832585B4: 4AFD491D  bl 0x8222ced0
	ctx.lr = 0x832585B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832585B8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832585BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585C0: 388666B0  addi r4, r6, 0x66b0
	ctx.r[4].s64 = ctx.r[6].s64 + 26288;
	// 832585C4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 832585C8: 4AFD4909  bl 0x8222ced0
	ctx.lr = 0x832585CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832585CC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832585D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585D4: 388466A8  addi r4, r4, 0x66a8
	ctx.r[4].s64 = ctx.r[4].s64 + 26280;
	// 832585D8: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 832585DC: 4AFD48F5  bl 0x8222ced0
	ctx.lr = 0x832585E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832585E0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832585E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585E8: 388366A0  addi r4, r3, 0x66a0
	ctx.r[4].s64 = ctx.r[3].s64 + 26272;
	// 832585EC: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 832585F0: 4AFD48E1  bl 0x8222ced0
	ctx.lr = 0x832585F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832585F4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832585F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585FC: 388B6698  addi r4, r11, 0x6698
	ctx.r[4].s64 = ctx.r[11].s64 + 26264;
	// 83258600: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83258604: 4AFD48CD  bl 0x8222ced0
	ctx.lr = 0x83258608;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258608: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325860C: 386AB2B8  addi r3, r10, -0x4d48
	ctx.r[3].s64 = ctx.r[10].s64 + -19784;
	// 83258610: 4BA51911  bl 0x82ca9f20
	ctx.lr = 0x83258614;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83258614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325861C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83258624: 4E800020  blr
	return;
	// 83258628: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325862C: 386BB320  addi r3, r11, -0x4ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -19680;
	// 83258630: 4BA518F0  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_83258638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258638 size=56
	// 83258638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325863C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258644: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325864C: 386B6E20  addi r3, r11, 0x6e20
	ctx.r[3].s64 = ctx.r[11].s64 + 28192;
	// 83258650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258654: 4AF9B705  bl 0x821f3d58
	ctx.lr = 0x83258658;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325865C: 906A8ACC  stw r3, -0x7534(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30004 as u32), ctx.r[3].u32 ) };
	// 83258660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325866C: 4E800020  blr
	return;
}

pub fn sub_83258670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258670 size=12
	// 83258670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832586B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832586B0 size=12
	// 832586B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832586B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832586B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832586F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832586F0 size=12
	// 832586F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832586F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832586F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258730 size=12
	// 83258730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258770 size=12
	// 83258770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832587B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832587B0 size=12
	// 832587B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832587B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832587B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832587F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832587F0 size=56
	// 832587F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832587F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832587F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832587FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258804: 386B7198  addi r3, r11, 0x7198
	ctx.r[3].s64 = ctx.r[11].s64 + 29080;
	// 83258808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325880C: 4AF9B54D  bl 0x821f3d58
	ctx.lr = 0x83258810;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258814: 906A8AE8  stw r3, -0x7518(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29976 as u32), ctx.r[3].u32 ) };
	// 83258818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325881C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258824: 4E800020  blr
	return;
}

pub fn sub_83258828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258828 size=56
	// 83258828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325882C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258834: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325883C: 386B71A8  addi r3, r11, 0x71a8
	ctx.r[3].s64 = ctx.r[11].s64 + 29096;
	// 83258840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258844: 4AF9B515  bl 0x821f3d58
	ctx.lr = 0x83258848;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325884C: 906A8AEC  stw r3, -0x7514(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29972 as u32), ctx.r[3].u32 ) };
	// 83258850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325885C: 4E800020  blr
	return;
}

pub fn sub_83258860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258860 size=56
	// 83258860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325886C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258874: 386B71B8  addi r3, r11, 0x71b8
	ctx.r[3].s64 = ctx.r[11].s64 + 29112;
	// 83258878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325887C: 4AF9B4DD  bl 0x821f3d58
	ctx.lr = 0x83258880;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258884: 906A8AF0  stw r3, -0x7510(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29968 as u32), ctx.r[3].u32 ) };
	// 83258888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325888C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258894: 4E800020  blr
	return;
}

pub fn sub_83258898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258898 size=56
	// 83258898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325889C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832588A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832588A4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832588A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832588AC: 386B71D0  addi r3, r11, 0x71d0
	ctx.r[3].s64 = ctx.r[11].s64 + 29136;
	// 832588B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832588B4: 4AF9B4A5  bl 0x821f3d58
	ctx.lr = 0x832588B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832588B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832588BC: 906A8AF4  stw r3, -0x750c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29964 as u32), ctx.r[3].u32 ) };
	// 832588C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832588C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832588C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832588CC: 4E800020  blr
	return;
}

pub fn sub_832588D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832588D0 size=56
	// 832588D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832588D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832588D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832588DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832588E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832588E4: 386B71DC  addi r3, r11, 0x71dc
	ctx.r[3].s64 = ctx.r[11].s64 + 29148;
	// 832588E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832588EC: 4AF9B46D  bl 0x821f3d58
	ctx.lr = 0x832588F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832588F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832588F4: 906A8AF8  stw r3, -0x7508(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29960 as u32), ctx.r[3].u32 ) };
	// 832588F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832588FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258904: 4E800020  blr
	return;
}

pub fn sub_83258908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258908 size=56
	// 83258908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325890C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258914: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325891C: 386B71EC  addi r3, r11, 0x71ec
	ctx.r[3].s64 = ctx.r[11].s64 + 29164;
	// 83258920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258924: 4AF9B435  bl 0x821f3d58
	ctx.lr = 0x83258928;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325892C: 906A8AFC  stw r3, -0x7504(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29956 as u32), ctx.r[3].u32 ) };
	// 83258930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325893C: 4E800020  blr
	return;
}

pub fn sub_83258940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258940 size=56
	// 83258940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325894C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258954: 386B7200  addi r3, r11, 0x7200
	ctx.r[3].s64 = ctx.r[11].s64 + 29184;
	// 83258958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325895C: 4AF9B3FD  bl 0x821f3d58
	ctx.lr = 0x83258960;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258964: 906A8B00  stw r3, -0x7500(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29952 as u32), ctx.r[3].u32 ) };
	// 83258968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325896C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258974: 4E800020  blr
	return;
}

pub fn sub_83258978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258978 size=56
	// 83258978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325897C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258984: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325898C: 386B7214  addi r3, r11, 0x7214
	ctx.r[3].s64 = ctx.r[11].s64 + 29204;
	// 83258990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258994: 4AF9B3C5  bl 0x821f3d58
	ctx.lr = 0x83258998;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325899C: 906A8B04  stw r3, -0x74fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29948 as u32), ctx.r[3].u32 ) };
	// 832589A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832589A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832589A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832589AC: 4E800020  blr
	return;
}

pub fn sub_832589B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832589B0 size=12
	// 832589B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832589B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832589B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832589F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832589F0 size=12
	// 832589F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832589F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832589F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258A30 size=12
	// 83258A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258A70 size=12
	// 83258A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258AB0 size=12
	// 83258AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258AF0 size=12
	// 83258AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258B30 size=12
	// 83258B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258B70 size=12
	// 83258B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258BB0 size=232
	// 83258BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83258BBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258BC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83258BC4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83258BC8: 3BEB8B28  addi r31, r11, -0x74d8
	ctx.r[31].s64 = ctx.r[11].s64 + -29912;
	// 83258BCC: 388A7358  addi r4, r10, 0x7358
	ctx.r[4].s64 = ctx.r[10].s64 + 29528;
	// 83258BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83258BD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258BD8: 4AFD42F9  bl 0x8222ced0
	ctx.lr = 0x83258BDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258BDC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258BE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258BE4: 3889734C  addi r4, r9, 0x734c
	ctx.r[4].s64 = ctx.r[9].s64 + 29516;
	// 83258BE8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83258BEC: 4AFD42E5  bl 0x8222ced0
	ctx.lr = 0x83258BF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258BF0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258BF8: 38887334  addi r4, r8, 0x7334
	ctx.r[4].s64 = ctx.r[8].s64 + 29492;
	// 83258BFC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83258C00: 4AFD42D1  bl 0x8222ced0
	ctx.lr = 0x83258C04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258C04: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258C08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C0C: 3887731C  addi r4, r7, 0x731c
	ctx.r[4].s64 = ctx.r[7].s64 + 29468;
	// 83258C10: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83258C14: 4AFD42BD  bl 0x8222ced0
	ctx.lr = 0x83258C18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258C18: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83258C1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C20: 38867310  addi r4, r6, 0x7310
	ctx.r[4].s64 = ctx.r[6].s64 + 29456;
	// 83258C24: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83258C28: 4AFD42A9  bl 0x8222ced0
	ctx.lr = 0x83258C2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258C2C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258C30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C34: 38847304  addi r4, r4, 0x7304
	ctx.r[4].s64 = ctx.r[4].s64 + 29444;
	// 83258C38: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83258C3C: 4AFD4295  bl 0x8222ced0
	ctx.lr = 0x83258C40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258C40: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C48: 388372F4  addi r4, r3, 0x72f4
	ctx.r[4].s64 = ctx.r[3].s64 + 29428;
	// 83258C4C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83258C50: 4AFD4281  bl 0x8222ced0
	ctx.lr = 0x83258C54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258C54: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258C58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C5C: 388B72E0  addi r4, r11, 0x72e0
	ctx.r[4].s64 = ctx.r[11].s64 + 29408;
	// 83258C60: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83258C64: 4AFD426D  bl 0x8222ced0
	ctx.lr = 0x83258C68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83258C68: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83258C6C: 386AB458  addi r3, r10, -0x4ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -19368;
	// 83258C70: 4BA512B1  bl 0x82ca9f20
	ctx.lr = 0x83258C74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83258C74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258C80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83258C84: 4E800020  blr
	return;
	// 83258C88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83258C8C: 386B8B50  addi r3, r11, -0x74b0
	ctx.r[3].s64 = ctx.r[11].s64 + -29872;
	// 83258C90: 4AFE3120  b 0x8223bdb0
	crate::recompiler::externs::call(&mut ctx, base, 0x8223BDB0);
	return;
}

pub fn sub_83258C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258C98 size=12
	// 83258C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258CF8 size=12
	// 83258CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258D38 size=12
	// 83258D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258D78 size=12
	// 83258D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258DB8 size=12
	// 83258DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258DF8 size=12
	// 83258DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258E38 size=12
	// 83258E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258E78 size=56
	// 83258E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258E84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83258E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258E8C: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 83258E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258E94: 4AF9AEC5  bl 0x821f3d58
	ctx.lr = 0x83258E98;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258E9C: 906A8BB4  stw r3, -0x744c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29772 as u32), ctx.r[3].u32 ) };
	// 83258EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258EAC: 4E800020  blr
	return;
}

pub fn sub_83258EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258EB0 size=168
	// 83258EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83258EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83258EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258EC4: 3D60811C  lis r11, -0x7ee4
	ctx.r[11].s64 = -2128871424;
	// 83258EC8: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83258ECC: 617F9DC5  ori r31, r11, 0x9dc5
	ctx.r[31].u64 = ctx.r[11].u64 | 40389;
	// 83258ED0: 386A75F8  addi r3, r10, 0x75f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30200;
	// 83258ED4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258ED8: 4AF9AE81  bl 0x821f3d58
	ctx.lr = 0x83258EDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258EDC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83258EE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83258EE4: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258EE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258EEC: 386875F0  addi r3, r8, 0x75f0
	ctx.r[3].s64 = ctx.r[8].s64 + 30192;
	// 83258EF0: 91698BB8  stw r11, -0x7448(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-29768 as u32), ctx.r[11].u32 ) };
	// 83258EF4: 3BC98BB8  addi r30, r9, -0x7448
	ctx.r[30].s64 = ctx.r[9].s64 + -29768;
	// 83258EF8: 4AF9AE61  bl 0x821f3d58
	ctx.lr = 0x83258EFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258EFC: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83258F00: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258F04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258F08: 386775E8  addi r3, r7, 0x75e8
	ctx.r[3].s64 = ctx.r[7].s64 + 30184;
	// 83258F0C: 4AF9AE4D  bl 0x821f3d58
	ctx.lr = 0x83258F10;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258F10: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83258F14: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83258F18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258F1C: 386675E0  addi r3, r6, 0x75e0
	ctx.r[3].s64 = ctx.r[6].s64 + 30176;
	// 83258F20: 4AF9AE39  bl 0x821f3d58
	ctx.lr = 0x83258F24;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258F24: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83258F28: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 83258F2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258F30: 386575D8  addi r3, r5, 0x75d8
	ctx.r[3].s64 = ctx.r[5].s64 + 30168;
	// 83258F34: 4AF9AE25  bl 0x821f3d58
	ctx.lr = 0x83258F38;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83258F38: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83258F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83258F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83258F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83258F50: 4E800020  blr
	return;
}

pub fn sub_83258F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258F58 size=12
	// 83258F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258FA8 size=12
	// 83258FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83258FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258FE8 size=12
	// 83258FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259028 size=12
	// 83259028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325902C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259068 size=12
	// 83259068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832590A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832590A8 size=144
	// 832590A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832590AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832590B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832590B4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 832590B8: 4AFC61A1  bl 0x8221f258
	ctx.lr = 0x832590BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 832590BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832590C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832590C4: 419A0008  beq cr6, 0x832590cc
	if ctx.cr[6].eq {
	pc = 0x832590CC; continue 'dispatch;
	}
	// 832590C8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832590CC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832590D0: 41820008  beq 0x832590d8
	if ctx.cr[0].eq {
	pc = 0x832590D8; continue 'dispatch;
	}
	// 832590D4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832590D8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832590DC: 41820008  beq 0x832590e4
	if ctx.cr[0].eq {
	pc = 0x832590E4; continue 'dispatch;
	}
	// 832590E0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832590E4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832590E8: 99430059  stb r10, 0x59(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 832590EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832590F0: 39098BF4  addi r8, r9, -0x740c
	ctx.r[8].s64 = ctx.r[9].s64 + -29708;
	// 832590F4: 99630058  stb r11, 0x58(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 832590F8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832590FC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83259100: 99630059  stb r11, 0x59(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 83259104: 3867B678  addi r3, r7, -0x4988
	ctx.r[3].s64 = ctx.r[7].s64 + -18824;
	// 83259108: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325910C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83259110: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83259114: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83259118: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325911C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83259120: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83259124: 4BA50DFD  bl 0x82ca9f20
	ctx.lr = 0x83259128;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325912C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259134: 4E800020  blr
	return;
}

pub fn sub_83259138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259138 size=176
	// 83259138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325913C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83259144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259148: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325914C: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 83259150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83259154: 394A833C  addi r10, r10, -0x7cc4
	ctx.r[10].s64 = ctx.r[10].s64 + -31940;
	// 83259158: 3BE9A0F4  addi r31, r9, -0x5f0c
	ctx.r[31].s64 = ctx.r[9].s64 + -24332;
	// 8325915C: 7D2B50AE  lbzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83259160: 391F000C  addi r8, r31, 0xc
	ctx.r[8].s64 = ctx.r[31].s64 + 12;
	// 83259164: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83259168: 7D2B41AE  stbx r9, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u8) };
	// 8325916C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83259170: 409AFFEC  bne cr6, 0x8325915c
	if !ctx.cr[6].eq {
	pc = 0x8325915C; continue 'dispatch;
	}
	// 83259174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83259178: 4B3BD6A9  bl 0x82616820
	ctx.lr = 0x8325917C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82616820);
	// 8325917C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83259180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83259184: 409A000C  bne cr6, 0x83259190
	if !ctx.cr[6].eq {
	pc = 0x83259190; continue 'dispatch;
	}
	// 83259188: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8325918C: 48000010  b 0x8325919c
	pc = 0x8325919C; continue 'dispatch;
	// 83259190: 4B3BD5F9  bl 0x82616788
	ctx.lr = 0x83259194;
	crate::recompiler::externs::call(&mut ctx, base, 0x82616788);
	// 83259194: 81630400  lwz r11, 0x400(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1024 as u32) ) } as u64;
	// 83259198: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8325919C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832591A0: 3D608261  lis r11, -0x7d9f
	ctx.r[11].s64 = -2107572224;
	// 832591A4: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832591A8: 396B4F18  addi r11, r11, 0x4f18
	ctx.r[11].s64 = ctx.r[11].s64 + 20248;
	// 832591AC: 392A2B90  addi r9, r10, 0x2b90
	ctx.r[9].s64 = ctx.r[10].s64 + 11152;
	// 832591B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832591B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832591B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832591BC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832591C0: 915F010C  stw r10, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[10].u32 ) };
	// 832591C4: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832591C8: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 832591CC: 386AB688  addi r3, r10, -0x4978
	ctx.r[3].s64 = ctx.r[10].s64 + -18808;
	// 832591D0: 4BA50D51  bl 0x82ca9f20
	ctx.lr = 0x832591D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832591D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832591D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832591DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832591E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832591E4: 4E800020  blr
	return;
}

pub fn sub_832591E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832591E8 size=192
	// 832591E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832591EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832591F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832591F4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832591F8: 3D408261  lis r10, -0x7d9f
	ctx.r[10].s64 = -2107572224;
	// 832591FC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259200: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259204: 38CBB6AC  addi r6, r11, -0x4954
	ctx.r[6].s64 = ctx.r[11].s64 + -18772;
	// 83259208: 388983EC  addi r4, r9, -0x7c14
	ctx.r[4].s64 = ctx.r[9].s64 + -31764;
	// 8325920C: 38688C00  addi r3, r8, -0x7400
	ctx.r[3].s64 = ctx.r[8].s64 + -29696;
	// 83259210: 38AA69A8  addi r5, r10, 0x69a8
	ctx.r[5].s64 = ctx.r[10].s64 + 27048;
	// 83259214: 4BC3355D  bl 0x82e8c770
	ctx.lr = 0x83259218;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E8C770);
	// 83259218: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325921C: 3867B6B8  addi r3, r7, -0x4948
	ctx.r[3].s64 = ctx.r[7].s64 + -18760;
	// 83259220: 4BA50D01  bl 0x82ca9f20
	ctx.lr = 0x83259224;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325922C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259230: 4E800020  blr
	return;
}

pub fn sub_832592A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832592A8 size=56
	// 832592A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832592AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832592B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832592B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832592B8: 386B8D14  addi r3, r11, -0x72ec
	ctx.r[3].s64 = ctx.r[11].s64 + -29420;
	// 832592BC: 480609C9  bl 0x832b9c84
	ctx.lr = 0x832592C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832592C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832592C4: 386AB6D0  addi r3, r10, -0x4930
	ctx.r[3].s64 = ctx.r[10].s64 + -18736;
	// 832592C8: 4BA50C59  bl 0x82ca9f20
	ctx.lr = 0x832592CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832592CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832592D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832592D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832592D8: 4E800020  blr
	return;
}

pub fn sub_832592E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832592E0 size=56
	// 832592E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832592E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832592E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832592EC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832592F0: 386B8D30  addi r3, r11, -0x72d0
	ctx.r[3].s64 = ctx.r[11].s64 + -29392;
	// 832592F4: 48060991  bl 0x832b9c84
	ctx.lr = 0x832592F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832592F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832592FC: 386AB6D8  addi r3, r10, -0x4928
	ctx.r[3].s64 = ctx.r[10].s64 + -18728;
	// 83259300: 4BA50C21  bl 0x82ca9f20
	ctx.lr = 0x83259304;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325930C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259310: 4E800020  blr
	return;
}

pub fn sub_83259318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259318 size=88
	// 83259318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325931C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259324: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259328: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8325932C: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 83259330: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259334: 38688D4C  addi r3, r8, -0x72b4
	ctx.r[3].s64 = ctx.r[8].s64 + -29364;
	// 83259338: 38CB8C64  addi r6, r11, -0x739c
	ctx.r[6].s64 = ctx.r[11].s64 + -29596;
	// 8325933C: 388984D4  addi r4, r9, -0x7b2c
	ctx.r[4].s64 = ctx.r[9].s64 + -31532;
	// 83259340: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83259344: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83259348: 38AA8EA0  addi r5, r10, -0x7160
	ctx.r[5].s64 = ctx.r[10].s64 + -29024;
	// 8325934C: 4BC2C435  bl 0x82e85780
	ctx.lr = 0x83259350;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85780);
	// 83259350: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259354: 3867B6E0  addi r3, r7, -0x4920
	ctx.r[3].s64 = ctx.r[7].s64 + -18720;
	// 83259358: 4BA50BC9  bl 0x82ca9f20
	ctx.lr = 0x8325935C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325935C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259368: 4E800020  blr
	return;
}

pub fn sub_83259370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259370 size=80
	// 83259370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325937C: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259380: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259384: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259388: 388A84E8  addi r4, r10, -0x7b18
	ctx.r[4].s64 = ctx.r[10].s64 + -31512;
	// 8325938C: 38698E60  addi r3, r9, -0x71a0
	ctx.r[3].s64 = ctx.r[9].s64 + -29088;
	// 83259390: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83259394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83259398: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8325939C: 38AB9218  addi r5, r11, -0x6de8
	ctx.r[5].s64 = ctx.r[11].s64 + -28136;
	// 832593A0: 4BC2C421  bl 0x82e857c0
	ctx.lr = 0x832593A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E857C0);
	// 832593A4: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832593A8: 3868B6F8  addi r3, r8, -0x4908
	ctx.r[3].s64 = ctx.r[8].s64 + -18696;
	// 832593AC: 4BA50B75  bl 0x82ca9f20
	ctx.lr = 0x832593B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832593B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832593B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832593B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832593BC: 4E800020  blr
	return;
}

pub fn sub_832593C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832593C0 size=72
	// 832593C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832593C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832593C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832593CC: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832593D0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832593D4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832593D8: 388A8528  addi r4, r10, -0x7ad8
	ctx.r[4].s64 = ctx.r[10].s64 + -31448;
	// 832593DC: 38698F74  addi r3, r9, -0x708c
	ctx.r[3].s64 = ctx.r[9].s64 + -28812;
	// 832593E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832593E4: 38AB96B8  addi r5, r11, -0x6948
	ctx.r[5].s64 = ctx.r[11].s64 + -26952;
	// 832593E8: 4BC26179  bl 0x82e7f560
	ctx.lr = 0x832593EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 832593EC: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832593F0: 3868B710  addi r3, r8, -0x48f0
	ctx.r[3].s64 = ctx.r[8].s64 + -18672;
	// 832593F4: 4BA50B2D  bl 0x82ca9f20
	ctx.lr = 0x832593F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832593F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832593FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259404: 4E800020  blr
	return;
}

pub fn sub_83259408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259408 size=72
	// 83259408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325940C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259414: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259418: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325941C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259420: 388A8538  addi r4, r10, -0x7ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -31432;
	// 83259424: 38699088  addi r3, r9, -0x6f78
	ctx.r[3].s64 = ctx.r[9].s64 + -28536;
	// 83259428: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8325942C: 38AB9710  addi r5, r11, -0x68f0
	ctx.r[5].s64 = ctx.r[11].s64 + -26864;
	// 83259430: 4BC26131  bl 0x82e7f560
	ctx.lr = 0x83259434;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 83259434: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259438: 3868B728  addi r3, r8, -0x48d8
	ctx.r[3].s64 = ctx.r[8].s64 + -18648;
	// 8325943C: 4BA50AE5  bl 0x82ca9f20
	ctx.lr = 0x83259440;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325944C: 4E800020  blr
	return;
}

pub fn sub_83259450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259450 size=72
	// 83259450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325945C: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259460: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259464: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259468: 388A8590  addi r4, r10, -0x7a70
	ctx.r[4].s64 = ctx.r[10].s64 + -31344;
	// 8325946C: 3869919C  addi r3, r9, -0x6e64
	ctx.r[3].s64 = ctx.r[9].s64 + -28260;
	// 83259470: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83259474: 38AB9CC0  addi r5, r11, -0x6340
	ctx.r[5].s64 = ctx.r[11].s64 + -25408;
	// 83259478: 4BC260E9  bl 0x82e7f560
	ctx.lr = 0x8325947C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 8325947C: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259480: 3868B740  addi r3, r8, -0x48c0
	ctx.r[3].s64 = ctx.r[8].s64 + -18624;
	// 83259484: 4BA50A9D  bl 0x82ca9f20
	ctx.lr = 0x83259488;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325948C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259494: 4E800020  blr
	return;
}

pub fn sub_83259498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259498 size=12
	// 83259498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325949C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832594A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832594F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832594F8 size=72
	// 832594F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832594FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259504: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259508: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325950C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259510: 388A8CC8  addi r4, r10, -0x7338
	ctx.r[4].s64 = ctx.r[10].s64 + -29496;
	// 83259514: 386992BC  addi r3, r9, -0x6d44
	ctx.r[3].s64 = ctx.r[9].s64 + -27972;
	// 83259518: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8325951C: 38ABE058  addi r5, r11, -0x1fa8
	ctx.r[5].s64 = ctx.r[11].s64 + -8104;
	// 83259520: 4BC26041  bl 0x82e7f560
	ctx.lr = 0x83259524;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 83259524: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259528: 3868B7A0  addi r3, r8, -0x4860
	ctx.r[3].s64 = ctx.r[8].s64 + -18528;
	// 8325952C: 4BA509F5  bl 0x82ca9f20
	ctx.lr = 0x83259530;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325953C: 4E800020  blr
	return;
}

pub fn sub_83259540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259540 size=72
	// 83259540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325954C: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259550: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259554: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259558: 388A8CDC  addi r4, r10, -0x7324
	ctx.r[4].s64 = ctx.r[10].s64 + -29476;
	// 8325955C: 386993D0  addi r3, r9, -0x6c30
	ctx.r[3].s64 = ctx.r[9].s64 + -27696;
	// 83259560: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83259564: 38ABE1A8  addi r5, r11, -0x1e58
	ctx.r[5].s64 = ctx.r[11].s64 + -7768;
	// 83259568: 4BC24759  bl 0x82e7dcc0
	ctx.lr = 0x8325956C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7DCC0);
	// 8325956C: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259570: 3868B7B8  addi r3, r8, -0x4848
	ctx.r[3].s64 = ctx.r[8].s64 + -18504;
	// 83259574: 4BA509AD  bl 0x82ca9f20
	ctx.lr = 0x83259578;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325957C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259584: 4E800020  blr
	return;
}

pub fn sub_83259588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259588 size=80
	// 83259588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325958C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259594: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259598: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325959C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832595A0: 388A8CE8  addi r4, r10, -0x7318
	ctx.r[4].s64 = ctx.r[10].s64 + -29464;
	// 832595A4: 386994E4  addi r3, r9, -0x6b1c
	ctx.r[3].s64 = ctx.r[9].s64 + -27420;
	// 832595A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832595AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832595B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832595B4: 38ABE3A0  addi r5, r11, -0x1c60
	ctx.r[5].s64 = ctx.r[11].s64 + -7264;
	// 832595B8: 4BC25369  bl 0x82e7e920
	ctx.lr = 0x832595BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7E920);
	// 832595BC: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832595C0: 3868B7D0  addi r3, r8, -0x4830
	ctx.r[3].s64 = ctx.r[8].s64 + -18480;
	// 832595C4: 4BA5095D  bl 0x82ca9f20
	ctx.lr = 0x832595C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832595C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832595CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832595D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832595D4: 4E800020  blr
	return;
}

pub fn sub_832595D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832595D8 size=80
	// 832595D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832595DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832595E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832595E4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832595E8: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 832595EC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832595F0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832595F4: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 832595F8: 38898D28  addi r4, r9, -0x72d8
	ctx.r[4].s64 = ctx.r[9].s64 + -29400;
	// 832595FC: 386895F8  addi r3, r8, -0x6a08
	ctx.r[3].s64 = ctx.r[8].s64 + -27144;
	// 83259600: 38AAE8F0  addi r5, r10, -0x1710
	ctx.r[5].s64 = ctx.r[10].s64 + -5904;
	// 83259604: 4BC25F5D  bl 0x82e7f560
	ctx.lr = 0x83259608;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 83259608: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325960C: 3867B7E8  addi r3, r7, -0x4818
	ctx.r[3].s64 = ctx.r[7].s64 + -18456;
	// 83259610: 4BA50911  bl 0x82ca9f20
	ctx.lr = 0x83259614;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325961C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259620: 4E800020  blr
	return;
}

pub fn sub_83259628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259628 size=80
	// 83259628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325962C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259634: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259638: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 8325963C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259640: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259644: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 83259648: 38898D44  addi r4, r9, -0x72bc
	ctx.r[4].s64 = ctx.r[9].s64 + -29372;
	// 8325964C: 3868970C  addi r3, r8, -0x68f4
	ctx.r[3].s64 = ctx.r[8].s64 + -26868;
	// 83259650: 38AAFF08  addi r5, r10, -0xf8
	ctx.r[5].s64 = ctx.r[10].s64 + -248;
	// 83259654: 4BC25F0D  bl 0x82e7f560
	ctx.lr = 0x83259658;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 83259658: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325965C: 3867B800  addi r3, r7, -0x4800
	ctx.r[3].s64 = ctx.r[7].s64 + -18432;
	// 83259660: 4BA508C1  bl 0x82ca9f20
	ctx.lr = 0x83259664;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325966C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259670: 4E800020  blr
	return;
}

pub fn sub_83259678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259678 size=56
	// 83259678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325967C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259684: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83259688: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325968C: 386B4034  addi r3, r11, 0x4034
	ctx.r[3].s64 = ctx.r[11].s64 + 16436;
	// 83259690: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83259694: 4AF9A6C5  bl 0x821f3d58
	ctx.lr = 0x83259698;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83259698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325969C: 906A9820  stw r3, -0x67e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26592 as u32), ctx.r[3].u32 ) };
	// 832596A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832596A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832596A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832596AC: 4E800020  blr
	return;
}

pub fn sub_832596B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832596B0 size=80
	// 832596B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832596B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832596B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832596BC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832596C0: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 832596C4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832596C8: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832596CC: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 832596D0: 38898DF4  addi r4, r9, -0x720c
	ctx.r[4].s64 = ctx.r[9].s64 + -29196;
	// 832596D4: 38689824  addi r3, r8, -0x67dc
	ctx.r[3].s64 = ctx.r[8].s64 + -26588;
	// 832596D8: 38AA2E88  addi r5, r10, 0x2e88
	ctx.r[5].s64 = ctx.r[10].s64 + 11912;
	// 832596DC: 4BC25E85  bl 0x82e7f560
	ctx.lr = 0x832596E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 832596E0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832596E4: 3867B818  addi r3, r7, -0x47e8
	ctx.r[3].s64 = ctx.r[7].s64 + -18408;
	// 832596E8: 4BA50839  bl 0x82ca9f20
	ctx.lr = 0x832596EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832596EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832596F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832596F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832596F8: 4E800020  blr
	return;
}

pub fn sub_83259700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259700 size=80
	// 83259700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325970C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259710: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 83259714: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259718: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8325971C: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 83259720: 38898E6C  addi r4, r9, -0x7194
	ctx.r[4].s64 = ctx.r[9].s64 + -29076;
	// 83259724: 38689938  addi r3, r8, -0x66c8
	ctx.r[3].s64 = ctx.r[8].s64 + -26312;
	// 83259728: 38AA33C0  addi r5, r10, 0x33c0
	ctx.r[5].s64 = ctx.r[10].s64 + 13248;
	// 8325972C: 4BC25E35  bl 0x82e7f560
	ctx.lr = 0x83259730;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 83259730: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259734: 3867B830  addi r3, r7, -0x47d0
	ctx.r[3].s64 = ctx.r[7].s64 + -18384;
	// 83259738: 4BA507E9  bl 0x82ca9f20
	ctx.lr = 0x8325973C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325973C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259748: 4E800020  blr
	return;
}

pub fn sub_83259750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259750 size=80
	// 83259750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325975C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259760: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 83259764: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259768: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8325976C: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 83259770: 38898EEC  addi r4, r9, -0x7114
	ctx.r[4].s64 = ctx.r[9].s64 + -28948;
	// 83259774: 38689A4C  addi r3, r8, -0x65b4
	ctx.r[3].s64 = ctx.r[8].s64 + -26036;
	// 83259778: 38AA3DC0  addi r5, r10, 0x3dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 15808;
	// 8325977C: 4BC25DE5  bl 0x82e7f560
	ctx.lr = 0x83259780;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 83259780: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259784: 3867B848  addi r3, r7, -0x47b8
	ctx.r[3].s64 = ctx.r[7].s64 + -18360;
	// 83259788: 4BA50799  bl 0x82ca9f20
	ctx.lr = 0x8325978C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325978C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259798: 4E800020  blr
	return;
}

pub fn sub_832597A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832597A0 size=104
	// 832597A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832597A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832597A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832597AC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832597B0: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 832597B4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832597B8: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832597BC: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 832597C0: 38898F64  addi r4, r9, -0x709c
	ctx.r[4].s64 = ctx.r[9].s64 + -28828;
	// 832597C4: 38689B60  addi r3, r8, -0x64a0
	ctx.r[3].s64 = ctx.r[8].s64 + -25760;
	// 832597C8: 38AA4948  addi r5, r10, 0x4948
	ctx.r[5].s64 = ctx.r[10].s64 + 18760;
	// 832597CC: 4BC25D95  bl 0x82e7f560
	ctx.lr = 0x832597D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E7F560);
	// 832597D0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832597D4: 3867B860  addi r3, r7, -0x47a0
	ctx.r[3].s64 = ctx.r[7].s64 + -18336;
	// 832597D8: 4BA50749  bl 0x82ca9f20
	ctx.lr = 0x832597DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832597DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832597E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832597E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832597E8: 4E800020  blr
	return;
}

pub fn sub_83259808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259808 size=144
	// 83259808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325980C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259814: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 83259818: 4AFC5A41  bl 0x8221f258
	ctx.lr = 0x8325981C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8325981C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83259820: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83259824: 419A0008  beq cr6, 0x8325982c
	if ctx.cr[6].eq {
	pc = 0x8325982C; continue 'dispatch;
	}
	// 83259828: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325982C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83259830: 41820008  beq 0x83259838
	if ctx.cr[0].eq {
	pc = 0x83259838; continue 'dispatch;
	}
	// 83259834: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83259838: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325983C: 41820008  beq 0x83259844
	if ctx.cr[0].eq {
	pc = 0x83259844; continue 'dispatch;
	}
	// 83259840: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83259844: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259848: 99430061  stb r10, 0x61(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(97 as u32), ctx.r[10].u8 ) };
	// 8325984C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83259850: 39099C78  addi r8, r9, -0x6388
	ctx.r[8].s64 = ctx.r[9].s64 + -25480;
	// 83259854: 99630060  stb r11, 0x60(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 83259858: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325985C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83259860: 99630061  stb r11, 0x61(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 83259864: 3867B878  addi r3, r7, -0x4788
	ctx.r[3].s64 = ctx.r[7].s64 + -18312;
	// 83259868: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325986C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83259870: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83259874: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83259878: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325987C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83259880: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83259884: 4BA5069D  bl 0x82ca9f20
	ctx.lr = 0x83259888;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325988C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259894: 4E800020  blr
	return;
}

pub fn sub_83259898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259898 size=72
	// 83259898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325989C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832598A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832598A4: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832598A8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832598AC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832598B0: 388A9000  addi r4, r10, -0x7000
	ctx.r[4].s64 = ctx.r[10].s64 + -28672;
	// 832598B4: 38699C84  addi r3, r9, -0x637c
	ctx.r[3].s64 = ctx.r[9].s64 + -25468;
	// 832598B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832598BC: 38AB67E0  addi r5, r11, 0x67e0
	ctx.r[5].s64 = ctx.r[11].s64 + 26592;
	// 832598C0: 4BC2CA11  bl 0x82e862d0
	ctx.lr = 0x832598C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E862D0);
	// 832598C4: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832598C8: 3868B898  addi r3, r8, -0x4768
	ctx.r[3].s64 = ctx.r[8].s64 + -18280;
	// 832598CC: 4BA50655  bl 0x82ca9f20
	ctx.lr = 0x832598D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832598D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832598D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832598D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832598DC: 4E800020  blr
	return;
}

pub fn sub_832598E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832598E0 size=72
	// 832598E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832598E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832598E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832598EC: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832598F0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832598F4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832598F8: 388A9020  addi r4, r10, -0x6fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -28640;
	// 832598FC: 38699D98  addi r3, r9, -0x6268
	ctx.r[3].s64 = ctx.r[9].s64 + -25192;
	// 83259900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83259904: 38AB6A90  addi r5, r11, 0x6a90
	ctx.r[5].s64 = ctx.r[11].s64 + 27280;
	// 83259908: 4BC2C9C9  bl 0x82e862d0
	ctx.lr = 0x8325990C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E862D0);
	// 8325990C: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259910: 3868B8B0  addi r3, r8, -0x4750
	ctx.r[3].s64 = ctx.r[8].s64 + -18256;
	// 83259914: 4BA5060D  bl 0x82ca9f20
	ctx.lr = 0x83259918;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325991C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259924: 4E800020  blr
	return;
}

pub fn sub_83259928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259928 size=12
	// 83259928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325992C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259988 size=72
	// 83259988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259994: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259998: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325999C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832599A0: 388A9098  addi r4, r10, -0x6f68
	ctx.r[4].s64 = ctx.r[10].s64 + -28520;
	// 832599A4: 38699EB8  addi r3, r9, -0x6148
	ctx.r[3].s64 = ctx.r[9].s64 + -24904;
	// 832599A8: 38AB70F8  addi r5, r11, 0x70f8
	ctx.r[5].s64 = ctx.r[11].s64 + 28920;
	// 832599AC: 4B3D45C5  bl 0x8262df70
	ctx.lr = 0x832599B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262DF70);
	// 832599B0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832599B4: 3868B910  addi r3, r8, -0x46f0
	ctx.r[3].s64 = ctx.r[8].s64 + -18160;
	// 832599B8: 4BA50569  bl 0x82ca9f20
	ctx.lr = 0x832599BC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832599BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832599C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832599C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832599C8: 4E800020  blr
	return;
}

pub fn sub_832599D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832599D0 size=72
	// 832599D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832599D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832599D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832599DC: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832599E0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832599E4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832599E8: 388A90A0  addi r4, r10, -0x6f60
	ctx.r[4].s64 = ctx.r[10].s64 + -28512;
	// 832599EC: 38699FC4  addi r3, r9, -0x603c
	ctx.r[3].s64 = ctx.r[9].s64 + -24636;
	// 832599F0: 38AB72D8  addi r5, r11, 0x72d8
	ctx.r[5].s64 = ctx.r[11].s64 + 29400;
	// 832599F4: 4B3D479D  bl 0x8262e190
	ctx.lr = 0x832599F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E190);
	// 832599F8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832599FC: 3868B928  addi r3, r8, -0x46d8
	ctx.r[3].s64 = ctx.r[8].s64 + -18136;
	// 83259A00: 4BA50521  bl 0x82ca9f20
	ctx.lr = 0x83259A04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259A10: 4E800020  blr
	return;
}

pub fn sub_83259A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259A18 size=80
	// 83259A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259A24: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259A28: 3D408263  lis r10, -0x7d9d
	ctx.r[10].s64 = -2107441152;
	// 83259A2C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259A30: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259A34: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 83259A38: 388990B0  addi r4, r9, -0x6f50
	ctx.r[4].s64 = ctx.r[9].s64 + -28496;
	// 83259A3C: 3868A0D0  addi r3, r8, -0x5f30
	ctx.r[3].s64 = ctx.r[8].s64 + -24368;
	// 83259A40: 38AAB0A0  addi r5, r10, -0x4f60
	ctx.r[5].s64 = ctx.r[10].s64 + -20320;
	// 83259A44: 4BC2C1B5  bl 0x82e85bf8
	ctx.lr = 0x83259A48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 83259A48: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259A4C: 3867B940  addi r3, r7, -0x46c0
	ctx.r[3].s64 = ctx.r[7].s64 + -18112;
	// 83259A50: 4BA504D1  bl 0x82ca9f20
	ctx.lr = 0x83259A54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259A60: 4E800020  blr
	return;
}

pub fn sub_83259A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259A68 size=72
	// 83259A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259A74: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 83259A78: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259A7C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259A80: 388A90D0  addi r4, r10, -0x6f30
	ctx.r[4].s64 = ctx.r[10].s64 + -28464;
	// 83259A84: 3869A1E4  addi r3, r9, -0x5e1c
	ctx.r[3].s64 = ctx.r[9].s64 + -24092;
	// 83259A88: 38AB88D0  addi r5, r11, -0x7730
	ctx.r[5].s64 = ctx.r[11].s64 + -30512;
	// 83259A8C: 4B3D45F5  bl 0x8262e080
	ctx.lr = 0x83259A90;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E080);
	// 83259A90: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259A94: 3868B958  addi r3, r8, -0x46a8
	ctx.r[3].s64 = ctx.r[8].s64 + -18088;
	// 83259A98: 4BA50489  bl 0x82ca9f20
	ctx.lr = 0x83259A9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259A9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259AA8: 4E800020  blr
	return;
}

pub fn sub_83259AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259AB0 size=72
	// 83259AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259ABC: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 83259AC0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259AC4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259AC8: 388A90EC  addi r4, r10, -0x6f14
	ctx.r[4].s64 = ctx.r[10].s64 + -28436;
	// 83259ACC: 3869A2F0  addi r3, r9, -0x5d10
	ctx.r[3].s64 = ctx.r[9].s64 + -23824;
	// 83259AD0: 38AB9A10  addi r5, r11, -0x65f0
	ctx.r[5].s64 = ctx.r[11].s64 + -26096;
	// 83259AD4: 4B3D4635  bl 0x8262e108
	ctx.lr = 0x83259AD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262E108);
	// 83259AD8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259ADC: 3868B970  addi r3, r8, -0x4690
	ctx.r[3].s64 = ctx.r[8].s64 + -18064;
	// 83259AE0: 4BA50441  bl 0x82ca9f20
	ctx.lr = 0x83259AE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259AE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259AF0: 4E800020  blr
	return;
}

pub fn sub_83259AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259AF8 size=72
	// 83259AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259B04: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 83259B08: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259B0C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259B10: 388A9108  addi r4, r10, -0x6ef8
	ctx.r[4].s64 = ctx.r[10].s64 + -28408;
	// 83259B14: 3869A3FC  addi r3, r9, -0x5c04
	ctx.r[3].s64 = ctx.r[9].s64 + -23556;
	// 83259B18: 38ABA4F8  addi r5, r11, -0x5b08
	ctx.r[5].s64 = ctx.r[11].s64 + -23304;
	// 83259B1C: 4B3D44DD  bl 0x8262dff8
	ctx.lr = 0x83259B20;
	crate::recompiler::externs::call(&mut ctx, base, 0x8262DFF8);
	// 83259B20: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259B24: 3868B988  addi r3, r8, -0x4678
	ctx.r[3].s64 = ctx.r[8].s64 + -18040;
	// 83259B28: 4BA503F9  bl 0x82ca9f20
	ctx.lr = 0x83259B2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259B38: 4E800020  blr
	return;
}

pub fn sub_83259B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259B40 size=88
	// 83259B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259B4C: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259B50: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83259B54: 3D408263  lis r10, -0x7d9d
	ctx.r[10].s64 = -2107441152;
	// 83259B58: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259B5C: 3868A508  addi r3, r8, -0x5af8
	ctx.r[3].s64 = ctx.r[8].s64 + -23288;
	// 83259B60: 38CB8C64  addi r6, r11, -0x739c
	ctx.r[6].s64 = ctx.r[11].s64 + -29596;
	// 83259B64: 38899164  addi r4, r9, -0x6e9c
	ctx.r[4].s64 = ctx.r[9].s64 + -28316;
	// 83259B68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83259B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83259B70: 38AAF5F8  addi r5, r10, -0xa08
	ctx.r[5].s64 = ctx.r[10].s64 + -2568;
	// 83259B74: 4BC2BC0D  bl 0x82e85780
	ctx.lr = 0x83259B78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85780);
	// 83259B78: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259B7C: 3867BA40  addi r3, r7, -0x45c0
	ctx.r[3].s64 = ctx.r[7].s64 + -17856;
	// 83259B80: 4BA503A1  bl 0x82ca9f20
	ctx.lr = 0x83259B84;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259B84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259B90: 4E800020  blr
	return;
}

pub fn sub_83259B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259B98 size=12
	// 83259B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259BD8 size=12
	// 83259BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259C18 size=12
	// 83259C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259C58 size=12
	// 83259C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259C98 size=12
	// 83259C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259CD8 size=12
	// 83259CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259D18 size=104
	// 83259D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259D20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83259D24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83259D28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259D2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83259D30: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83259D34: 3BCBA638  addi r30, r11, -0x59c8
	ctx.r[30].s64 = ctx.r[11].s64 + -22984;
	// 83259D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83259D3C: 4B400DE5  bl 0x8265ab20
	ctx.lr = 0x83259D40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8265AB20);
	// 83259D40: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83259D44: 3BDE00B0  addi r30, r30, 0xb0
	ctx.r[30].s64 = ctx.r[30].s64 + 176;
	// 83259D48: 4080FFF0  bge 0x83259d38
	if !ctx.cr[0].lt {
	pc = 0x83259D38; continue 'dispatch;
	}
	// 83259D4C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83259D50: 386BBAB8  addi r3, r11, -0x4548
	ctx.r[3].s64 = ctx.r[11].s64 + -17736;
	// 83259D54: 4BA501CD  bl 0x82ca9f20
	ctx.lr = 0x83259D58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83259D58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83259D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259D64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83259D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83259D6C: 4E800020  blr
	return;
	// 83259D70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83259D74: 386BBB08  addi r3, r11, -0x44f8
	ctx.r[3].s64 = ctx.r[11].s64 + -17656;
	// 83259D78: 4BA501A8  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_83259D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259D80 size=12
	// 83259D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259DE8 size=12
	// 83259DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259E48 size=12
	// 83259E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259F00 size=12
	// 83259F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259F40 size=12
	// 83259F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259F80 size=12
	// 83259F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83259FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259FC0 size=12
	// 83259FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A000 size=12
	// 8325A000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A040 size=12
	// 8325A040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A080 size=12
	// 8325A080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A0C0 size=12
	// 8325A0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A0C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A100 size=144
	// 8325A100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A10C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8325A110: 4AFC5149  bl 0x8221f258
	ctx.lr = 0x8325A114;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8325A114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A118: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8325A11C: 419A0008  beq cr6, 0x8325a124
	if ctx.cr[6].eq {
	pc = 0x8325A124; continue 'dispatch;
	}
	// 8325A120: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A124: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A128: 41820008  beq 0x8325a130
	if ctx.cr[0].eq {
	pc = 0x8325A130; continue 'dispatch;
	}
	// 8325A12C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A130: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A134: 41820008  beq 0x8325a13c
	if ctx.cr[0].eq {
	pc = 0x8325A13C; continue 'dispatch;
	}
	// 8325A138: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A13C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A140: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8325A144: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8325A148: 3909A980  addi r8, r9, -0x5680
	ctx.r[8].s64 = ctx.r[9].s64 + -22144;
	// 8325A14C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8325A150: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A154: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325A158: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8325A15C: 3867BC68  addi r3, r7, -0x4398
	ctx.r[3].s64 = ctx.r[7].s64 + -17304;
	// 8325A160: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A164: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A168: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A16C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325A170: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A174: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325A178: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A17C: 4BA4FDA5  bl 0x82ca9f20
	ctx.lr = 0x8325A180;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325A180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A18C: 4E800020  blr
	return;
}

pub fn sub_8325A190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A190 size=144
	// 8325A190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A19C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8325A1A0: 4AFC50B9  bl 0x8221f258
	ctx.lr = 0x8325A1A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8325A1A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A1A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8325A1AC: 419A0008  beq cr6, 0x8325a1b4
	if ctx.cr[6].eq {
	pc = 0x8325A1B4; continue 'dispatch;
	}
	// 8325A1B0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A1B4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A1B8: 41820008  beq 0x8325a1c0
	if ctx.cr[0].eq {
	pc = 0x8325A1C0; continue 'dispatch;
	}
	// 8325A1BC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A1C0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A1C4: 41820008  beq 0x8325a1cc
	if ctx.cr[0].eq {
	pc = 0x8325A1CC; continue 'dispatch;
	}
	// 8325A1C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A1CC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A1D0: 99430021  stb r10, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 8325A1D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8325A1D8: 3909A98C  addi r8, r9, -0x5674
	ctx.r[8].s64 = ctx.r[9].s64 + -22132;
	// 8325A1DC: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8325A1E0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A1E4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325A1E8: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8325A1EC: 3867BC78  addi r3, r7, -0x4388
	ctx.r[3].s64 = ctx.r[7].s64 + -17288;
	// 8325A1F0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A1F4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A1F8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A1FC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325A200: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A204: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325A208: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A20C: 4BA4FD15  bl 0x82ca9f20
	ctx.lr = 0x8325A210;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325A210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A21C: 4E800020  blr
	return;
}

pub fn sub_8325A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A220 size=144
	// 8325A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A22C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8325A230: 4AFC5029  bl 0x8221f258
	ctx.lr = 0x8325A234;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8325A234: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8325A23C: 419A0008  beq cr6, 0x8325a244
	if ctx.cr[6].eq {
	pc = 0x8325A244; continue 'dispatch;
	}
	// 8325A240: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A244: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A248: 41820008  beq 0x8325a250
	if ctx.cr[0].eq {
	pc = 0x8325A250; continue 'dispatch;
	}
	// 8325A24C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A250: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A254: 41820008  beq 0x8325a25c
	if ctx.cr[0].eq {
	pc = 0x8325A25C; continue 'dispatch;
	}
	// 8325A258: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8325A25C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A260: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8325A264: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8325A268: 3909A998  addi r8, r9, -0x5668
	ctx.r[8].s64 = ctx.r[9].s64 + -22120;
	// 8325A26C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8325A270: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A274: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325A278: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8325A27C: 3867BC88  addi r3, r7, -0x4378
	ctx.r[3].s64 = ctx.r[7].s64 + -17272;
	// 8325A280: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A284: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A288: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A28C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325A290: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A294: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325A298: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A29C: 4BA4FC85  bl 0x82ca9f20
	ctx.lr = 0x8325A2A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325A2A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A2AC: 4E800020  blr
	return;
}

pub fn sub_8325A2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A2B0 size=12
	// 8325A2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A338 size=12
	// 8325A338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A398 size=12
	// 8325A398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A3F8 size=12
	// 8325A3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A448 size=336
	// 8325A448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8325A454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325A458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A45C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A460: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A464: 388B04E8  addi r4, r11, 0x4e8
	ctx.r[4].s64 = ctx.r[11].s64 + 1256;
	// 8325A468: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8325A46C: 4AFD2A65  bl 0x8222ced0
	ctx.lr = 0x8325A470;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A470: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8325A478: 388AB1F0  addi r4, r10, -0x4e10
	ctx.r[4].s64 = ctx.r[10].s64 + -19984;
	// 8325A47C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A480: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8325A484: 4AFD2A4D  bl 0x8222ced0
	ctx.lr = 0x8325A488;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A488: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8325A48C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325A490: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8325A494: 4AF88C75  bl 0x821e3108
	ctx.lr = 0x8325A498;
	crate::recompiler::externs::call(&mut ctx, base, 0x821E3108);
	// 8325A498: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8325A49C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325A4A0: 4AF94699  bl 0x821eeb38
	ctx.lr = 0x8325A4A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821EEB38);
	// 8325A4A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325A4A8: 4B9A9349  bl 0x82c037f0
	ctx.lr = 0x8325A4AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82C037F0);
	// 8325A4AC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A4B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325A4B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325A4B8: 9169AA04  stw r11, -0x55fc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-22012 as u32), ctx.r[11].u32 ) };
	// 8325A4BC: 4AF6C2AD  bl 0x821c6768
	ctx.lr = 0x8325A4C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 8325A4C0: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 8325A4C4: 3BC87088  addi r30, r8, 0x7088
	ctx.r[30].s64 = ctx.r[8].s64 + 28808;
	// 8325A4C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8325A4CC: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8325A4D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A4D4: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8325A4D8: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8325A4DC: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A4E0: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A4E4: 4082FFE8  bne 0x8325a4cc
	if !ctx.cr[0].eq {
	pc = 0x8325A4CC; continue 'dispatch;
	}
	// 8325A4E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8325A4EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325A4F0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8325A4F4: 4AF6C275  bl 0x821c6768
	ctx.lr = 0x8325A4F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 8325A4F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8325A4FC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325A500: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A504: 7C805828  lwarx r4, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325A508: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325A50C: 7C80592D  stwcx. r4, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A510: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A514: 4082FFE8  bne 0x8325a4fc
	if !ctx.cr[0].eq {
	pc = 0x8325A4FC; continue 'dispatch;
	}
	// 8325A518: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8325A51C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8325A520: 4AF6C249  bl 0x821c6768
	ctx.lr = 0x8325A524;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 8325A524: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8325A528: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8325A52C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A530: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8325A534: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8325A538: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A53C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A540: 4082FFE8  bne 0x8325a528
	if !ctx.cr[0].eq {
	pc = 0x8325A528; continue 'dispatch;
	}
	// 8325A544: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8325A548: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8325A54C: 4AF6C21D  bl 0x821c6768
	ctx.lr = 0x8325A550;
	crate::recompiler::externs::call(&mut ctx, base, 0x821C6768);
	// 8325A550: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8325A554: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A558: 7CE0F028  lwarx r7, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8325A55C: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8325A560: 7CE0F12D  stwcx. r7, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A564: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A568: 4082FFE8  bne 0x8325a550
	if !ctx.cr[0].eq {
	pc = 0x8325A550; continue 'dispatch;
	}
	// 8325A56C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8325A570: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8325A574: 3865BDE8  addi r3, r5, -0x4218
	ctx.r[3].s64 = ctx.r[5].s64 + -16920;
	// 8325A578: 4BA4F9A9  bl 0x82ca9f20
	ctx.lr = 0x8325A57C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325A57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8325A580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8325A58C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325A590: 4E800020  blr
	return;
}

pub fn sub_8325A598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A598 size=536
	// 8325A598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A5A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325A5A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A5A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325A5AC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A5B0: 3BEBAA08  addi r31, r11, -0x55f8
	ctx.r[31].s64 = ctx.r[11].s64 + -22008;
	// 8325A5B4: 388AB560  addi r4, r10, -0x4aa0
	ctx.r[4].s64 = ctx.r[10].s64 + -19104;
	// 8325A5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325A5BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A5C0: 4AFD2911  bl 0x8222ced0
	ctx.lr = 0x8325A5C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A5C4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8325A5C8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325A5CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A5D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A5D4: 3889B54C  addi r4, r9, -0x4ab4
	ctx.r[4].s64 = ctx.r[9].s64 + -19124;
	// 8325A5D8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8325A5DC: 4AFD28F5  bl 0x8222ced0
	ctx.lr = 0x8325A5E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A5E0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8325A5E4: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325A5E8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8325A5EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A5F0: 3888B534  addi r4, r8, -0x4acc
	ctx.r[4].s64 = ctx.r[8].s64 + -19148;
	// 8325A5F4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8325A5F8: 4AFD28D9  bl 0x8222ced0
	ctx.lr = 0x8325A5FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A5FC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8325A600: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8325A604: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8325A608: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A60C: 3887B51C  addi r4, r7, -0x4ae4
	ctx.r[4].s64 = ctx.r[7].s64 + -19172;
	// 8325A610: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8325A614: 4AFD28BD  bl 0x8222ced0
	ctx.lr = 0x8325A618;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8325A61C: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8325A620: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8325A624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A628: 3886B50C  addi r4, r6, -0x4af4
	ctx.r[4].s64 = ctx.r[6].s64 + -19188;
	// 8325A62C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8325A630: 4AFD28A1  bl 0x8222ced0
	ctx.lr = 0x8325A634;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A634: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8325A638: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8325A63C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8325A640: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A644: 3884B4F8  addi r4, r4, -0x4b08
	ctx.r[4].s64 = ctx.r[4].s64 + -19208;
	// 8325A648: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8325A64C: 4AFD2885  bl 0x8222ced0
	ctx.lr = 0x8325A650;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A650: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8325A654: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8325A658: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8325A65C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A660: 38833E18  addi r4, r3, 0x3e18
	ctx.r[4].s64 = ctx.r[3].s64 + 15896;
	// 8325A664: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8325A668: 4AFD2869  bl 0x8222ced0
	ctx.lr = 0x8325A66C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A66C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8325A670: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A674: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8325A678: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A67C: 388AB4EC  addi r4, r10, -0x4b14
	ctx.r[4].s64 = ctx.r[10].s64 + -19220;
	// 8325A680: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8325A684: 4AFD284D  bl 0x8222ced0
	ctx.lr = 0x8325A688;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8325A68C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325A690: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8325A694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A698: 3889B4CC  addi r4, r9, -0x4b34
	ctx.r[4].s64 = ctx.r[9].s64 + -19252;
	// 8325A69C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8325A6A0: 4AFD2831  bl 0x8222ced0
	ctx.lr = 0x8325A6A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A6A4: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 8325A6A8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325A6AC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8325A6B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A6B4: 3888B4AC  addi r4, r8, -0x4b54
	ctx.r[4].s64 = ctx.r[8].s64 + -19284;
	// 8325A6B8: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 8325A6BC: 4AFD2815  bl 0x8222ced0
	ctx.lr = 0x8325A6C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A6C0: 39600029  li r11, 0x29
	ctx.r[11].s64 = 41;
	// 8325A6C4: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8325A6C8: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8325A6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A6D0: 38878D58  addi r4, r7, -0x72a8
	ctx.r[4].s64 = ctx.r[7].s64 + -29352;
	// 8325A6D4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8325A6D8: 4AFD27F9  bl 0x8222ced0
	ctx.lr = 0x8325A6DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A6DC: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 8325A6E0: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8325A6E4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8325A6E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A6EC: 3886B498  addi r4, r6, -0x4b68
	ctx.r[4].s64 = ctx.r[6].s64 + -19304;
	// 8325A6F0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8325A6F4: 4AFD27DD  bl 0x8222ced0
	ctx.lr = 0x8325A6F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A6F8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8325A6FC: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8325A700: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8325A704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A708: 3884B484  addi r4, r4, -0x4b7c
	ctx.r[4].s64 = ctx.r[4].s64 + -19324;
	// 8325A70C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8325A710: 4AFD27C1  bl 0x8222ced0
	ctx.lr = 0x8325A714;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A714: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 8325A718: 3C60820D  lis r3, -0x7df3
	ctx.r[3].s64 = -2113077248;
	// 8325A71C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8325A720: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A724: 3883B470  addi r4, r3, -0x4b90
	ctx.r[4].s64 = ctx.r[3].s64 + -19344;
	// 8325A728: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8325A72C: 4AFD27A5  bl 0x8222ced0
	ctx.lr = 0x8325A730;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A730: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8325A734: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A738: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8325A73C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A740: 388AB458  addi r4, r10, -0x4ba8
	ctx.r[4].s64 = ctx.r[10].s64 + -19368;
	// 8325A744: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8325A748: 4AFD2789  bl 0x8222ced0
	ctx.lr = 0x8325A74C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A74C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8325A750: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325A754: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8325A758: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A75C: 3889B440  addi r4, r9, -0x4bc0
	ctx.r[4].s64 = ctx.r[9].s64 + -19392;
	// 8325A760: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8325A764: 4AFD276D  bl 0x8222ced0
	ctx.lr = 0x8325A768;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A768: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8325A76C: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325A770: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8325A774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A778: 3888B428  addi r4, r8, -0x4bd8
	ctx.r[4].s64 = ctx.r[8].s64 + -19416;
	// 8325A77C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8325A780: 4AFD2751  bl 0x8222ced0
	ctx.lr = 0x8325A784;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325A784: 3960002A  li r11, 0x2a
	ctx.r[11].s64 = 42;
	// 8325A788: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A78C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8325A790: 3867BDF0  addi r3, r7, -0x4210
	ctx.r[3].s64 = ctx.r[7].s64 + -16912;
	// 8325A794: 4BA4F78D  bl 0x82ca9f20
	ctx.lr = 0x8325A798;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325A798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A7A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325A7A8: 4E800020  blr
	return;
}

pub fn sub_8325A7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A7B0 size=12
	// 8325A7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A7F0 size=12
	// 8325A7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A830 size=12
	// 8325A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A880 size=12
	// 8325A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A8C0 size=12
	// 8325A8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A900 size=12
	// 8325A900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A940 size=12
	// 8325A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A980 size=12
	// 8325A980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A9C0 size=12
	// 8325A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA00 size=12
	// 8325AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA40 size=12
	// 8325AA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA80 size=12
	// 8325AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AAC0 size=12
	// 8325AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB00 size=12
	// 8325AB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB40 size=12
	// 8325AB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB80 size=12
	// 8325AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ABC0 size=12
	// 8325ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ABC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC00 size=12
	// 8325AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC40 size=12
	// 8325AC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC80 size=12
	// 8325AC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ACC0 size=12
	// 8325ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ACC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD00 size=12
	// 8325AD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD40 size=12
	// 8325AD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD80 size=12
	// 8325AD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325ADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ADC0 size=12
	// 8325ADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ADC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AE00 size=12
	// 8325AE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AE40 size=112
	// 8325AE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AE48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325AE4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AE50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AE54: 3BEBAB0C  addi r31, r11, -0x54f4
	ctx.r[31].s64 = ctx.r[11].s64 + -21748;
	// 8325AE58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325AE5C: 4B0C7AA5  bl 0x82322900
	ctx.lr = 0x8325AE60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82322900);
	// 8325AE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8325AE64: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325AE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325AE6C: 99230015  stb r9, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[9].u8 ) };
	// 8325AE70: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8325AE74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE78: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325AE7C: 3868C060  addi r3, r8, -0x3fa0
	ctx.r[3].s64 = ctx.r[8].s64 + -16288;
	// 8325AE80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE84: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325AE88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE8C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325AE90: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325AE94: 4BA4F08D  bl 0x82ca9f20
	ctx.lr = 0x8325AE98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325AE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AEA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325AEA8: 4E800020  blr
	return;
}

pub fn sub_8325AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AEB0 size=12
	// 8325AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AEF0 size=12
	// 8325AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AEF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AF30 size=12
	// 8325AF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325AF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AF70 size=56
	// 8325AF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AF78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AF7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AF80: 386BAB24  addi r3, r11, -0x54dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21724;
	// 8325AF84: 4B2252B5  bl 0x82480238
	ctx.lr = 0x8325AF88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 8325AF88: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325AF8C: 386AC0A0  addi r3, r10, -0x3f60
	ctx.r[3].s64 = ctx.r[10].s64 + -16224;
	// 8325AF90: 4BA4EF91  bl 0x82ca9f20
	ctx.lr = 0x8325AF94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325AF94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AFA0: 4E800020  blr
	return;
}

pub fn sub_8325AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AFA8 size=240
	// 8325AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AFB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325AFB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AFB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AFBC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325AFC0: 3BEBAB30  addi r31, r11, -0x54d0
	ctx.r[31].s64 = ctx.r[11].s64 + -21712;
	// 8325AFC4: 388AC798  addi r4, r10, -0x3868
	ctx.r[4].s64 = ctx.r[10].s64 + -14440;
	// 8325AFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325AFCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFD0: 4AFD1F01  bl 0x8222ced0
	ctx.lr = 0x8325AFD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325AFD4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325AFD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFDC: 3889C77C  addi r4, r9, -0x3884
	ctx.r[4].s64 = ctx.r[9].s64 + -14468;
	// 8325AFE0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8325AFE4: 4AFD1EED  bl 0x8222ced0
	ctx.lr = 0x8325AFE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325AFE8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325AFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFF0: 3888C760  addi r4, r8, -0x38a0
	ctx.r[4].s64 = ctx.r[8].s64 + -14496;
	// 8325AFF4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8325AFF8: 4AFD1ED9  bl 0x8222ced0
	ctx.lr = 0x8325AFFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325AFFC: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8325B000: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B004: 3887C744  addi r4, r7, -0x38bc
	ctx.r[4].s64 = ctx.r[7].s64 + -14524;
	// 8325B008: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8325B00C: 4AFD1EC5  bl 0x8222ced0
	ctx.lr = 0x8325B010;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325B010: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8325B014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B018: 3886C728  addi r4, r6, -0x38d8
	ctx.r[4].s64 = ctx.r[6].s64 + -14552;
	// 8325B01C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8325B020: 4AFD1EB1  bl 0x8222ced0
	ctx.lr = 0x8325B024;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325B024: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8325B028: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B02C: 3884C70C  addi r4, r4, -0x38f4
	ctx.r[4].s64 = ctx.r[4].s64 + -14580;
	// 8325B030: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8325B034: 4AFD1E9D  bl 0x8222ced0
	ctx.lr = 0x8325B038;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325B038: 3C60820D  lis r3, -0x7df3
	ctx.r[3].s64 = -2113077248;
	// 8325B03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B040: 3883C6F0  addi r4, r3, -0x3910
	ctx.r[4].s64 = ctx.r[3].s64 + -14608;
	// 8325B044: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8325B048: 4AFD1E89  bl 0x8222ced0
	ctx.lr = 0x8325B04C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325B04C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B050: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B054: 388BC6D4  addi r4, r11, -0x392c
	ctx.r[4].s64 = ctx.r[11].s64 + -14636;
	// 8325B058: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8325B05C: 4AFD1E75  bl 0x8222ced0
	ctx.lr = 0x8325B060;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325B060: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325B064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B068: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 8325B06C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8325B070: 4AFD1E61  bl 0x8222ced0
	ctx.lr = 0x8325B074;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325B074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B078: 3869C0B0  addi r3, r9, -0x3f50
	ctx.r[3].s64 = ctx.r[9].s64 + -16208;
	// 8325B07C: 4BA4EEA5  bl 0x82ca9f20
	ctx.lr = 0x8325B080;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325B080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B08C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325B090: 4E800020  blr
	return;
}

pub fn sub_8325B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B098 size=12
	// 8325B098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B0D8 size=12
	// 8325B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B118 size=12
	// 8325B118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325B158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B158 size=56
	// 8325B158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B16C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325B170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B174: 4AF98BE5  bl 0x821f3d58
	ctx.lr = 0x8325B178;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B17C: 906AAB60  stw r3, -0x54a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21664 as u32), ctx.r[3].u32 ) };
	// 8325B180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B18C: 4E800020  blr
	return;
}

pub fn sub_8325B190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B190 size=56
	// 8325B190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B1A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325B1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B1AC: 4AF98BAD  bl 0x821f3d58
	ctx.lr = 0x8325B1B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B1B4: 906AAB64  stw r3, -0x549c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21660 as u32), ctx.r[3].u32 ) };
	// 8325B1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B1C4: 4E800020  blr
	return;
}

pub fn sub_8325B1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B1C8 size=56
	// 8325B1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B1DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325B1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B1E4: 4AF98B75  bl 0x821f3d58
	ctx.lr = 0x8325B1E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B1EC: 906AAB68  stw r3, -0x5498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21656 as u32), ctx.r[3].u32 ) };
	// 8325B1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B1FC: 4E800020  blr
	return;
}

pub fn sub_8325B200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B200 size=56
	// 8325B200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B214: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325B218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B21C: 4AF98B3D  bl 0x821f3d58
	ctx.lr = 0x8325B220;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B224: 906AAB6C  stw r3, -0x5494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21652 as u32), ctx.r[3].u32 ) };
	// 8325B228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B234: 4E800020  blr
	return;
}

pub fn sub_8325B238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B238 size=56
	// 8325B238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B24C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325B250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B254: 4AF98B05  bl 0x821f3d58
	ctx.lr = 0x8325B258;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B25C: 906AAB70  stw r3, -0x5490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21648 as u32), ctx.r[3].u32 ) };
	// 8325B260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B26C: 4E800020  blr
	return;
}

pub fn sub_8325B270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B270 size=56
	// 8325B270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B284: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325B288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B28C: 4AF98ACD  bl 0x821f3d58
	ctx.lr = 0x8325B290;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B294: 906AAB74  stw r3, -0x548c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21644 as u32), ctx.r[3].u32 ) };
	// 8325B298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B2A4: 4E800020  blr
	return;
}

pub fn sub_8325B2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B2A8 size=56
	// 8325B2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B2BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325B2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B2C4: 4AF98A95  bl 0x821f3d58
	ctx.lr = 0x8325B2C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B2CC: 906AAB78  stw r3, -0x5488(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21640 as u32), ctx.r[3].u32 ) };
	// 8325B2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B2DC: 4E800020  blr
	return;
}

pub fn sub_8325B2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B2E0 size=56
	// 8325B2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B2F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325B2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B2FC: 4AF98A5D  bl 0x821f3d58
	ctx.lr = 0x8325B300;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B304: 906AAB7C  stw r3, -0x5484(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21636 as u32), ctx.r[3].u32 ) };
	// 8325B308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B314: 4E800020  blr
	return;
}

pub fn sub_8325B318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B318 size=56
	// 8325B318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B32C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325B330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B334: 4AF98A25  bl 0x821f3d58
	ctx.lr = 0x8325B338;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B33C: 906AAB80  stw r3, -0x5480(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21632 as u32), ctx.r[3].u32 ) };
	// 8325B340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B34C: 4E800020  blr
	return;
}

pub fn sub_8325B350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B350 size=56
	// 8325B350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B364: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325B368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B36C: 4AF989ED  bl 0x821f3d58
	ctx.lr = 0x8325B370;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B374: 906AAB84  stw r3, -0x547c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21628 as u32), ctx.r[3].u32 ) };
	// 8325B378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B384: 4E800020  blr
	return;
}

pub fn sub_8325B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B388 size=56
	// 8325B388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B39C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325B3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B3A4: 4AF989B5  bl 0x821f3d58
	ctx.lr = 0x8325B3A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B3AC: 906AAB88  stw r3, -0x5478(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21624 as u32), ctx.r[3].u32 ) };
	// 8325B3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B3BC: 4E800020  blr
	return;
}

pub fn sub_8325B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B3C0 size=56
	// 8325B3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B3D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325B3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B3DC: 4AF9897D  bl 0x821f3d58
	ctx.lr = 0x8325B3E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B3E4: 906AAB8C  stw r3, -0x5474(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21620 as u32), ctx.r[3].u32 ) };
	// 8325B3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B3F4: 4E800020  blr
	return;
}

pub fn sub_8325B3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B3F8 size=56
	// 8325B3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B40C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325B410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B414: 4AF98945  bl 0x821f3d58
	ctx.lr = 0x8325B418;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B41C: 906AAB90  stw r3, -0x5470(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21616 as u32), ctx.r[3].u32 ) };
	// 8325B420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B42C: 4E800020  blr
	return;
}

pub fn sub_8325B430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B430 size=56
	// 8325B430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B444: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325B448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B44C: 4AF9890D  bl 0x821f3d58
	ctx.lr = 0x8325B450;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B454: 906AAB94  stw r3, -0x546c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21612 as u32), ctx.r[3].u32 ) };
	// 8325B458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B464: 4E800020  blr
	return;
}

pub fn sub_8325B468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B468 size=56
	// 8325B468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B47C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325B480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B484: 4AF988D5  bl 0x821f3d58
	ctx.lr = 0x8325B488;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B48C: 906AAB98  stw r3, -0x5468(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21608 as u32), ctx.r[3].u32 ) };
	// 8325B490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B49C: 4E800020  blr
	return;
}

pub fn sub_8325B4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B4A0 size=56
	// 8325B4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B4B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325B4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B4BC: 4AF9889D  bl 0x821f3d58
	ctx.lr = 0x8325B4C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B4C4: 906AAB9C  stw r3, -0x5464(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21604 as u32), ctx.r[3].u32 ) };
	// 8325B4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B4D4: 4E800020  blr
	return;
}

pub fn sub_8325B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B4D8 size=56
	// 8325B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B4EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325B4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B4F4: 4AF98865  bl 0x821f3d58
	ctx.lr = 0x8325B4F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B4F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B4FC: 906AABA0  stw r3, -0x5460(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21600 as u32), ctx.r[3].u32 ) };
	// 8325B500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B50C: 4E800020  blr
	return;
}

pub fn sub_8325B510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B510 size=56
	// 8325B510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B524: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325B528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B52C: 4AF9882D  bl 0x821f3d58
	ctx.lr = 0x8325B530;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B534: 906AABA4  stw r3, -0x545c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21596 as u32), ctx.r[3].u32 ) };
	// 8325B538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B544: 4E800020  blr
	return;
}

pub fn sub_8325B548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B548 size=56
	// 8325B548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B554: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B55C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325B560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B564: 4AF987F5  bl 0x821f3d58
	ctx.lr = 0x8325B568;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B56C: 906AABA8  stw r3, -0x5458(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21592 as u32), ctx.r[3].u32 ) };
	// 8325B570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B57C: 4E800020  blr
	return;
}

pub fn sub_8325B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B580 size=56
	// 8325B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B58C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B594: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325B598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B59C: 4AF987BD  bl 0x821f3d58
	ctx.lr = 0x8325B5A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B5A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B5A4: 906AABAC  stw r3, -0x5454(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21588 as u32), ctx.r[3].u32 ) };
	// 8325B5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B5B4: 4E800020  blr
	return;
}

pub fn sub_8325B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B5B8 size=56
	// 8325B5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B5C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B5C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B5CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325B5D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B5D4: 4AF98785  bl 0x821f3d58
	ctx.lr = 0x8325B5D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8325B5D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B5DC: 906AABB0  stw r3, -0x5450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21584 as u32), ctx.r[3].u32 ) };
	// 8325B5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B5EC: 4E800020  blr
	return;
}

pub fn sub_8325B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B5F0 size=12
	// 8325B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B630 size=12
	// 8325B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B670 size=12
	// 8325B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B6B0 size=12
	// 8325B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8325B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B6F0 size=12
	// 8325B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

