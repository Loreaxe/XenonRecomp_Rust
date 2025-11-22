pub fn sub_83286FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286FF0 size=12
	// 83286FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287030 size=12
	// 83287030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287070 size=12
	// 83287070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832870B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832870B0 size=12
	// 832870B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832870B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832870B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832870F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832870F0 size=12
	// 832870F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832870F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832870F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287130 size=12
	// 83287130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287170 size=12
	// 83287170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832871B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832871B0 size=12
	// 832871B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832871B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832871B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832871F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832871F0 size=12
	// 832871F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832871F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832871F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287230 size=12
	// 83287230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287270 size=12
	// 83287270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832872B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832872B0 size=12
	// 832872B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832872B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832872B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832872F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832872F0 size=12
	// 832872F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832872F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832872F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287330 size=12
	// 83287330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287370 size=12
	// 83287370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832873B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832873B0 size=12
	// 832873B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832873B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832873B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832873F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832873F0 size=12
	// 832873F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832873F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832873F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287430 size=12
	// 83287430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287470 size=12
	// 83287470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832874B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832874B0 size=12
	// 832874B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832874B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832874B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832874F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832874F0 size=12
	// 832874F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832874F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832874F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287530 size=12
	// 83287530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287570 size=12
	// 83287570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832875B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832875B0 size=12
	// 832875B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832875B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832875B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832875F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832875F0 size=12
	// 832875F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832875F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832875F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287630 size=12
	// 83287630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287670 size=12
	// 83287670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832876B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832876B0 size=12
	// 832876B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832876B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832876B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832876F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832876F0 size=12
	// 832876F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832876F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832876F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287730 size=12
	// 83287730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287770 size=12
	// 83287770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832877B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832877B0 size=12
	// 832877B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832877B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832877B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832877F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832877F0 size=320
	// 832877F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832877F4: 4BA21C11  bl 0x82ca9404
	ctx.lr = 0x832877F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 832877F8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832877FC: 3901FFA4  addi r8, r1, -0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + -92;
	// 83287800: 392BD5C8  addi r9, r11, -0x2a38
	ctx.r[9].s64 = ctx.r[11].s64 + -10808;
	// 83287804: 38E1FFA8  addi r7, r1, -0x58
	ctx.r[7].s64 = ctx.r[1].s64 + -88;
	// 83287808: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 8328780C: 3BA1FFC0  addi r29, r1, -0x40
	ctx.r[29].s64 = ctx.r[1].s64 + -64;
	// 83287810: C18BD5C8  lfs f12, -0x2a38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10808 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83287814: 3B81FFC4  addi r28, r1, -0x3c
	ctx.r[28].s64 = ctx.r[1].s64 + -60;
	// 83287818: D181FFA0  stfs f12, -0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), tmp.u32 ) };
	// 8328781C: C009BEBC  lfs f0, -0x4144(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83287820: 38A1FFB0  addi r5, r1, -0x50
	ctx.r[5].s64 = ctx.r[1].s64 + -80;
	// 83287824: D001FFA4  stfs f0, -0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-92 as u32), tmp.u32 ) };
	// 83287828: 38C1FFAC  addi r6, r1, -0x54
	ctx.r[6].s64 = ctx.r[1].s64 + -84;
	// 8328782C: D001FFA8  stfs f0, -0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 83287830: 3BE1FFB8  addi r31, r1, -0x48
	ctx.r[31].s64 = ctx.r[1].s64 + -72;
}

pub fn sub_83287930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287930 size=12
	// 83287930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287970 size=12
	// 83287970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832879B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832879B0 size=12
	// 832879B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832879B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832879B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832879F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832879F0 size=12
	// 832879F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832879F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832879F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287A30 size=12
	// 83287A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287A70 size=12
	// 83287A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287AB0 size=12
	// 83287AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287AF0 size=12
	// 83287AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287B30 size=12
	// 83287B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287B70 size=12
	// 83287B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287BB0 size=12
	// 83287BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287BF0 size=12
	// 83287BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287C30 size=12
	// 83287C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287C70 size=12
	// 83287C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287CB0 size=12
	// 83287CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287CF0 size=12
	// 83287CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287D30 size=12
	// 83287D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287D70 size=12
	// 83287D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287DB0 size=12
	// 83287DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287DF0 size=12
	// 83287DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287E30 size=12
	// 83287E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287E70 size=12
	// 83287E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287EB0 size=12
	// 83287EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287EF0 size=12
	// 83287EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287F30 size=12
	// 83287F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287F70 size=12
	// 83287F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287FB0 size=12
	// 83287FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83287FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287FF0 size=12
	// 83287FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288030 size=12
	// 83288030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288070 size=12
	// 83288070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832880B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832880B0 size=12
	// 832880B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832880B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832880B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832880F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832880F0 size=12
	// 832880F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832880F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832880F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288130 size=12
	// 83288130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288170 size=12
	// 83288170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832881B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832881B0 size=12
	// 832881B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832881B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832881B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832881F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832881F0 size=12
	// 832881F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832881F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832881F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288230 size=12
	// 83288230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288270 size=12
	// 83288270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832882B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832882B0 size=12
	// 832882B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832882B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832882B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832882F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832882F0 size=12
	// 832882F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832882F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832882F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288330 size=12
	// 83288330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288370 size=12
	// 83288370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832883B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832883B0 size=12
	// 832883B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832883B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832883B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832883F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832883F0 size=12
	// 832883F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832883F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832883F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288430 size=12
	// 83288430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288470 size=12
	// 83288470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832884B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832884B0 size=12
	// 832884B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832884B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832884B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832884F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832884F0 size=12
	// 832884F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832884F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832884F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288530 size=12
	// 83288530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288570 size=12
	// 83288570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832885C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832885C0 size=12
	// 832885C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832885C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832885C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288630 size=12
	// 83288630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288670 size=12
	// 83288670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832886C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832886C0 size=56
	// 832886C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832886C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832886C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832886CC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832886D0: 386BEA1C  addi r3, r11, -0x15e4
	ctx.r[3].s64 = ctx.r[11].s64 + -5604;
	// 832886D4: 4B0B843D  bl 0x82340b10
	ctx.lr = 0x832886D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82340B10);
	// 832886D8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832886DC: 386A2A50  addi r3, r10, 0x2a50
	ctx.r[3].s64 = ctx.r[10].s64 + 10832;
	// 832886E0: 4BA21841  bl 0x82ca9f20
	ctx.lr = 0x832886E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832886E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832886E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832886EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832886F0: 4E800020  blr
	return;
}

pub fn sub_832886F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832886F8 size=56
	// 832886F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832886FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83288704: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83288708: 386BEA28  addi r3, r11, -0x15d8
	ctx.r[3].s64 = ctx.r[11].s64 + -5592;
	// 8328870C: 4B0B8405  bl 0x82340b10
	ctx.lr = 0x83288710;
	crate::recompiler::externs::call(&mut ctx, base, 0x82340B10);
	// 83288710: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83288714: 386A2A60  addi r3, r10, 0x2a60
	ctx.r[3].s64 = ctx.r[10].s64 + 10848;
	// 83288718: 4BA21809  bl 0x82ca9f20
	ctx.lr = 0x8328871C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328871C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83288728: 4E800020  blr
	return;
}

pub fn sub_83288730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288730 size=12
	// 83288730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288770 size=12
	// 83288770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288810 size=12
	// 83288810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288850 size=12
	// 83288850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83288890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288890 size=1040
	// 83288890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288894: 4BA20B79  bl 0x82ca940c
	ctx.lr = 0x83288898;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA940C);
	// 83288898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328889C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832888A0: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832888A4: 3BEBEA58  addi r31, r11, -0x15a8
	ctx.r[31].s64 = ctx.r[11].s64 + -5544;
	// 832888A8: 388A49A8  addi r4, r10, 0x49a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18856;
	// 832888AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832888B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888B4: 4AFA461D  bl 0x8222ced0
	ctx.lr = 0x832888B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832888B8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 832888BC: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832888C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832888C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888C8: 38894990  addi r4, r9, 0x4990
	ctx.r[4].s64 = ctx.r[9].s64 + 18832;
	// 832888CC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832888D0: 4AFA4601  bl 0x8222ced0
	ctx.lr = 0x832888D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832888D4: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832888D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888DC: 38884980  addi r4, r8, 0x4980
	ctx.r[4].s64 = ctx.r[8].s64 + 18816;
	// 832888E0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832888E4: 4AFA45ED  bl 0x8222ced0
	ctx.lr = 0x832888E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832888E8: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832888EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888F0: 38874974  addi r4, r7, 0x4974
	ctx.r[4].s64 = ctx.r[7].s64 + 18804;
	// 832888F4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832888F8: 4AFA45D9  bl 0x8222ced0
	ctx.lr = 0x832888FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832888FC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288900: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288904: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83288908: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328890C: 3BC64970  addi r30, r6, 0x4970
	ctx.r[30].s64 = ctx.r[6].s64 + 18800;
	// 83288910: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83288914: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288918: 4AFA45B9  bl 0x8222ced0
	ctx.lr = 0x8328891C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8328891C: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288920: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288924: 3884495C  addi r4, r4, 0x495c
	ctx.r[4].s64 = ctx.r[4].s64 + 18780;
	// 83288928: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8328892C: 4AFA45A5  bl 0x8222ced0
	ctx.lr = 0x83288930;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288930: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288938: 3883494C  addi r4, r3, 0x494c
	ctx.r[4].s64 = ctx.r[3].s64 + 18764;
	// 8328893C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83288940: 4AFA4591  bl 0x8222ced0
	ctx.lr = 0x83288944;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288944: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288948: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328894C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83288950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288954: 3BAA4948  addi r29, r10, 0x4948
	ctx.r[29].s64 = ctx.r[10].s64 + 18760;
	// 83288958: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8328895C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288960: 4AFA4571  bl 0x8222ced0
	ctx.lr = 0x83288964;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288964: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328896C: 38894938  addi r4, r9, 0x4938
	ctx.r[4].s64 = ctx.r[9].s64 + 18744;
	// 83288970: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83288974: 4AFA455D  bl 0x8222ced0
	ctx.lr = 0x83288978;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288978: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328897C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288980: 3888492C  addi r4, r8, 0x492c
	ctx.r[4].s64 = ctx.r[8].s64 + 18732;
	// 83288984: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83288988: 4AFA4549  bl 0x8222ced0
	ctx.lr = 0x8328898C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8328898C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288990: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288994: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83288998: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328899C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832889A0: 4AFA4531  bl 0x8222ced0
	ctx.lr = 0x832889A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832889A4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832889A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889AC: 38874920  addi r4, r7, 0x4920
	ctx.r[4].s64 = ctx.r[7].s64 + 18720;
	// 832889B0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832889B4: 4AFA451D  bl 0x8222ced0
	ctx.lr = 0x832889B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832889B8: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832889BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889C0: 3886490C  addi r4, r6, 0x490c
	ctx.r[4].s64 = ctx.r[6].s64 + 18700;
	// 832889C4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832889C8: 4AFA4509  bl 0x8222ced0
	ctx.lr = 0x832889CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832889CC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 832889D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832889D4: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 832889D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889DC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 832889E0: 4AFA44F1  bl 0x8222ced0
	ctx.lr = 0x832889E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832889E4: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 832889E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889EC: 388448FC  addi r4, r4, 0x48fc
	ctx.r[4].s64 = ctx.r[4].s64 + 18684;
	// 832889F0: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 832889F4: 4AFA44DD  bl 0x8222ced0
	ctx.lr = 0x832889F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832889F8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 832889FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A00: 388348E8  addi r4, r3, 0x48e8
	ctx.r[4].s64 = ctx.r[3].s64 + 18664;
	// 83288A04: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83288A08: 4AFA44C9  bl 0x8222ced0
	ctx.lr = 0x83288A0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288A0C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288A10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288A14: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83288A18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A1C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83288A20: 4AFA44B1  bl 0x8222ced0
	ctx.lr = 0x83288A24;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288A24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288A28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A2C: 388B48D8  addi r4, r11, 0x48d8
	ctx.r[4].s64 = ctx.r[11].s64 + 18648;
	// 83288A30: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83288A34: 4AFA449D  bl 0x8222ced0
	ctx.lr = 0x83288A38;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288A38: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288A3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A40: 388A48C8  addi r4, r10, 0x48c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18632;
	// 83288A44: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83288A48: 4AFA4489  bl 0x8222ced0
	ctx.lr = 0x83288A4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288A4C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288A50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288A54: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83288A58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A5C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83288A60: 4AFA4471  bl 0x8222ced0
	ctx.lr = 0x83288A64;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288A64: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288A68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A6C: 388948BC  addi r4, r9, 0x48bc
	ctx.r[4].s64 = ctx.r[9].s64 + 18620;
	// 83288A70: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83288A74: 4AFA445D  bl 0x8222ced0
	ctx.lr = 0x83288A78;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288A78: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288A7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A80: 388848A4  addi r4, r8, 0x48a4
	ctx.r[4].s64 = ctx.r[8].s64 + 18596;
	// 83288A84: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83288A88: 4AFA4449  bl 0x8222ced0
	ctx.lr = 0x83288A8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288A8C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288A90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A94: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83288A98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288A9C: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83288AA0: 4AFA4431  bl 0x8222ced0
	ctx.lr = 0x83288AA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288AA4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288AA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288AAC: 38874890  addi r4, r7, 0x4890
	ctx.r[4].s64 = ctx.r[7].s64 + 18576;
	// 83288AB0: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83288AB4: 4AFA441D  bl 0x8222ced0
	ctx.lr = 0x83288AB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288AB8: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288ABC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288AC0: 38864874  addi r4, r6, 0x4874
	ctx.r[4].s64 = ctx.r[6].s64 + 18548;
	// 83288AC4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83288AC8: 4AFA4409  bl 0x8222ced0
	ctx.lr = 0x83288ACC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288ACC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288AD0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288AD4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83288AD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288ADC: 38844864  addi r4, r4, 0x4864
	ctx.r[4].s64 = ctx.r[4].s64 + 18532;
	// 83288AE0: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 83288AE4: 4AFA43ED  bl 0x8222ced0
	ctx.lr = 0x83288AE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288AE8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288AEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288AF0: 3BC34858  addi r30, r3, 0x4858
	ctx.r[30].s64 = ctx.r[3].s64 + 18520;
	// 83288AF4: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 83288AF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288AFC: 4AFA43D5  bl 0x8222ced0
	ctx.lr = 0x83288B00;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B00: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B08: 388B483C  addi r4, r11, 0x483c
	ctx.r[4].s64 = ctx.r[11].s64 + 18492;
	// 83288B0C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83288B10: 4AFA43C1  bl 0x8222ced0
	ctx.lr = 0x83288B14;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B14: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288B18: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288B1C: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83288B20: 388A482C  addi r4, r10, 0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + 18476;
	// 83288B24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B28: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 83288B2C: 4AFA43A5  bl 0x8222ced0
	ctx.lr = 0x83288B30;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288B34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B38: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 83288B3C: 4AFA4395  bl 0x8222ced0
	ctx.lr = 0x83288B40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B40: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B48: 38894810  addi r4, r9, 0x4810
	ctx.r[4].s64 = ctx.r[9].s64 + 18448;
	// 83288B4C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83288B50: 4AFA4381  bl 0x8222ced0
	ctx.lr = 0x83288B54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B54: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288B58: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288B5C: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 83288B60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B64: 38884800  addi r4, r8, 0x4800
	ctx.r[4].s64 = ctx.r[8].s64 + 18432;
	// 83288B68: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 83288B6C: 4AFA4365  bl 0x8222ced0
	ctx.lr = 0x83288B70;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288B74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B78: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 83288B7C: 4AFA4355  bl 0x8222ced0
	ctx.lr = 0x83288B80;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B80: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288B84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B88: 388747E4  addi r4, r7, 0x47e4
	ctx.r[4].s64 = ctx.r[7].s64 + 18404;
	// 83288B8C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83288B90: 4AFA4341  bl 0x8222ced0
	ctx.lr = 0x83288B94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288B94: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288B98: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288B9C: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 83288BA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BA4: 388647D4  addi r4, r6, 0x47d4
	ctx.r[4].s64 = ctx.r[6].s64 + 18388;
	// 83288BA8: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 83288BAC: 4AFA4325  bl 0x8222ced0
	ctx.lr = 0x83288BB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288BB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288BB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BB8: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 83288BBC: 4AFA4315  bl 0x8222ced0
	ctx.lr = 0x83288BC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288BC0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BC8: 388447C0  addi r4, r4, 0x47c0
	ctx.r[4].s64 = ctx.r[4].s64 + 18368;
	// 83288BCC: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83288BD0: 4AFA4301  bl 0x8222ced0
	ctx.lr = 0x83288BD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288BD4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288BD8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288BDC: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 83288BE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BE4: 3BC347B8  addi r30, r3, 0x47b8
	ctx.r[30].s64 = ctx.r[3].s64 + 18360;
	// 83288BE8: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 83288BEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288BF0: 4AFA42E1  bl 0x8222ced0
	ctx.lr = 0x83288BF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288BF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288BF8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BFC: 388B47A8  addi r4, r11, 0x47a8
	ctx.r[4].s64 = ctx.r[11].s64 + 18344;
	// 83288C00: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 83288C04: 4AFA42CD  bl 0x8222ced0
	ctx.lr = 0x83288C08;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288C08: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C10: 388A4794  addi r4, r10, 0x4794
	ctx.r[4].s64 = ctx.r[10].s64 + 18324;
	// 83288C14: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83288C18: 4AFA42B9  bl 0x8222ced0
	ctx.lr = 0x83288C1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288C1C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288C20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288C24: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 83288C28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C2C: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 83288C30: 4AFA42A1  bl 0x8222ced0
	ctx.lr = 0x83288C34;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288C34: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288C38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C3C: 38894784  addi r4, r9, 0x4784
	ctx.r[4].s64 = ctx.r[9].s64 + 18308;
	// 83288C40: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 83288C44: 4AFA428D  bl 0x8222ced0
	ctx.lr = 0x83288C48;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288C48: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C50: 38884770  addi r4, r8, 0x4770
	ctx.r[4].s64 = ctx.r[8].s64 + 18288;
	// 83288C54: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 83288C58: 4AFA4279  bl 0x8222ced0
	ctx.lr = 0x83288C5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288C5C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288C60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288C64: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83288C68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C6C: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 83288C70: 4AFA4261  bl 0x8222ced0
	ctx.lr = 0x83288C74;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288C74: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288C78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C7C: 38874760  addi r4, r7, 0x4760
	ctx.r[4].s64 = ctx.r[7].s64 + 18272;
	// 83288C80: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 83288C84: 4AFA424D  bl 0x8222ced0
	ctx.lr = 0x83288C88;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83288C88: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 83288C8C: 38662AB0  addi r3, r6, 0x2ab0
	ctx.r[3].s64 = ctx.r[6].s64 + 10928;
	// 83288C90: 4BA21291  bl 0x82ca9f20
	ctx.lr = 0x83288C94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83288C94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83288C98: 4BA207C4  b 0x82ca945c
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA945C);
	return;
}

pub fn sub_83288CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288CA0 size=12
	// 83288CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288CA4: 4BA2072D  bl 0x82ca93d0
	ctx.lr = 0x83288CA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93D0);
	// 83288CA8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328BD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BD60 size=12
	// 8328BD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BD68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328BDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BDA0 size=12
	// 8328BDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BDA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328BDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BDE0 size=16
	// 8328BDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328BDEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328BEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BEA0 size=16
	// 8328BEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BEA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328BEAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328BF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BF60 size=16
	// 8328BF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BF68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328BF6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C020 size=16
	// 8328C020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328C02C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C0E0 size=12
	// 8328C0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C120 size=12
	// 8328C120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C160 size=12
	// 8328C160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C1A0 size=12
	// 8328C1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C1E0 size=12
	// 8328C1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C1E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C220 size=12
	// 8328C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C3D0 size=344
	// 8328C3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C3DC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328C3E0: 386BF7FC  addi r3, r11, -0x804
	ctx.r[3].s64 = ctx.r[11].s64 + -2052;
	// 8328C3E4: 4802D8A1  bl 0x832b9c84
	ctx.lr = 0x8328C3E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 8328C3E8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328C3EC: 386A30A0  addi r3, r10, 0x30a0
	ctx.r[3].s64 = ctx.r[10].s64 + 12448;
	// 8328C3F0: 4BA1DB31  bl 0x82ca9f20
	ctx.lr = 0x8328C3F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328C3F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C400: 4E800020  blr
	return;
}

pub fn sub_8328C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C528 size=56
	// 8328C528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C534: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328C538: 386BF850  addi r3, r11, -0x7b0
	ctx.r[3].s64 = ctx.r[11].s64 + -1968;
	// 8328C53C: 4B80302D  bl 0x82a8f568
	ctx.lr = 0x8328C540;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A8F568);
	// 8328C540: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328C544: 386A3390  addi r3, r10, 0x3390
	ctx.r[3].s64 = ctx.r[10].s64 + 13200;
	// 8328C548: 4BA1D9D9  bl 0x82ca9f20
	ctx.lr = 0x8328C54C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328C54C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C558: 4E800020  blr
	return;
}

pub fn sub_8328C560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C560 size=328
	// 8328C560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C56C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328C570: 386BF8E0  addi r3, r11, -0x720
	ctx.r[3].s64 = ctx.r[11].s64 + -1824;
	// 8328C574: 4B802FF5  bl 0x82a8f568
	ctx.lr = 0x8328C578;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A8F568);
	// 8328C578: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328C57C: 386A33A0  addi r3, r10, 0x33a0
	ctx.r[3].s64 = ctx.r[10].s64 + 13216;
	// 8328C580: 4BA1D9A1  bl 0x82ca9f20
	ctx.lr = 0x8328C584;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328C584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C590: 4E800020  blr
	return;
}

pub fn sub_8328C6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C6A8 size=12
	// 8328C6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C6E8 size=12
	// 8328C6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C728 size=12
	// 8328C728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C788 size=144
	// 8328C788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C794: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8328C798: 4AF92AC1  bl 0x8221f258
	ctx.lr = 0x8328C79C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328C79C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328C7A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328C7A4: 419A0008  beq cr6, 0x8328c7ac
	if ctx.cr[6].eq {
	pc = 0x8328C7AC; continue 'dispatch;
	}
	// 8328C7A8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328C7AC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328C7B0: 41820008  beq 0x8328c7b8
	if ctx.cr[0].eq {
	pc = 0x8328C7B8; continue 'dispatch;
	}
	// 8328C7B4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328C7B8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328C7BC: 41820008  beq 0x8328c7c4
	if ctx.cr[0].eq {
	pc = 0x8328C7C4; continue 'dispatch;
	}
	// 8328C7C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328C7C4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328C7C8: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8328C7CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328C7D0: 3909F9E4  addi r8, r9, -0x61c
	ctx.r[8].s64 = ctx.r[9].s64 + -1564;
	// 8328C7D4: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8328C7D8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328C7DC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328C7E0: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8328C7E4: 386736D8  addi r3, r7, 0x36d8
	ctx.r[3].s64 = ctx.r[7].s64 + 14040;
	// 8328C7E8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328C7EC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328C7F0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328C7F4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328C7F8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328C7FC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328C800: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328C804: 4BA1D71D  bl 0x82ca9f20
	ctx.lr = 0x8328C808;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328C808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C814: 4E800020  blr
	return;
}

pub fn sub_8328C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C818 size=12
	// 8328C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C8F8 size=12
	// 8328C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C938 size=12
	// 8328C938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C978 size=12
	// 8328C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C9B8 size=12
	// 8328C9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C9F8 size=12
	// 8328C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CA58 size=12
	// 8328CA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CA60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CA98 size=12
	// 8328CA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CAD8 size=12
	// 8328CAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CAE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CB18 size=12
	// 8328CB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CB20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CD28 size=12
	// 8328CD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CD68 size=12
	// 8328CD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CD70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CDB8 size=12
	// 8328CDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CDC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CDF8 size=12
	// 8328CDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CE00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CE38 size=144
	// 8328CE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328CE44: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8328CE48: 4AF92411  bl 0x8221f258
	ctx.lr = 0x8328CE4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328CE4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328CE50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328CE54: 419A0008  beq cr6, 0x8328ce5c
	if ctx.cr[6].eq {
	pc = 0x8328CE5C; continue 'dispatch;
	}
	// 8328CE58: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328CE5C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328CE60: 41820008  beq 0x8328ce68
	if ctx.cr[0].eq {
	pc = 0x8328CE68; continue 'dispatch;
	}
	// 8328CE64: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328CE68: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328CE6C: 41820008  beq 0x8328ce74
	if ctx.cr[0].eq {
	pc = 0x8328CE74; continue 'dispatch;
	}
	// 8328CE70: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328CE74: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328CE78: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8328CE7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328CE80: 3909FB48  addi r8, r9, -0x4b8
	ctx.r[8].s64 = ctx.r[9].s64 + -1208;
	// 8328CE84: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8328CE88: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328CE8C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328CE90: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8328CE94: 38673CF0  addi r3, r7, 0x3cf0
	ctx.r[3].s64 = ctx.r[7].s64 + 15600;
	// 8328CE98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328CE9C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328CEA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328CEA4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328CEA8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328CEAC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328CEB0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328CEB4: 4BA1D06D  bl 0x82ca9f20
	ctx.lr = 0x8328CEB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328CEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328CEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328CEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328CEC4: 4E800020  blr
	return;
}

pub fn sub_8328CEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CEC8 size=12
	// 8328CEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CF38 size=12
	// 8328CF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CF40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CF78 size=12
	// 8328CF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CFB8 size=12
	// 8328CFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328CFC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328CFF8 size=12
	// 8328CFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328CFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D060 size=12
	// 8328D060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D0A0 size=12
	// 8328D0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D0A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D0E0 size=12
	// 8328D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D218 size=12
	// 8328D218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D258 size=12
	// 8328D258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D328 size=12
	// 8328D328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D368 size=12
	// 8328D368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D3A8 size=12
	// 8328D3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D3B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D3E8 size=12
	// 8328D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D3F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D428 size=12
	// 8328D428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D468 size=12
	// 8328D468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D4A8 size=12
	// 8328D4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D4B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D4E8 size=12
	// 8328D4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D528 size=12
	// 8328D528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D568 size=12
	// 8328D568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D5A8 size=12
	// 8328D5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D5B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D5E8 size=12
	// 8328D5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D5F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D648 size=12
	// 8328D648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D6A8 size=80
	// 8328D6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328D6B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328D6B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328D6BC: 388B6EEC  addi r4, r11, 0x6eec
	ctx.r[4].s64 = ctx.r[11].s64 + 28396;
	// 8328D6C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328D6C4: 4AF9F80D  bl 0x8222ced0
	ctx.lr = 0x8328D6C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8328D6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328D6CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8328D6D0: 386A0FFC  addi r3, r10, 0xffc
	ctx.r[3].s64 = ctx.r[10].s64 + 4092;
	// 8328D6D4: 4B01DE35  bl 0x822ab508
	ctx.lr = 0x8328D6D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x822AB508);
	// 8328D6D8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328D6DC: 38694188  addi r3, r9, 0x4188
	ctx.r[3].s64 = ctx.r[9].s64 + 16776;
	// 8328D6E0: 4BA1C841  bl 0x82ca9f20
	ctx.lr = 0x8328D6E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328D6E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328D6E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328D6EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328D6F0: 4E800020  blr
	return;
}

pub fn sub_8328D6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D6F8 size=80
	// 8328D6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328D704: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328D708: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328D70C: 388B6F08  addi r4, r11, 0x6f08
	ctx.r[4].s64 = ctx.r[11].s64 + 28424;
	// 8328D710: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328D714: 4AF9F7BD  bl 0x8222ced0
	ctx.lr = 0x8328D718;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8328D718: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328D71C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8328D720: 386A1048  addi r3, r10, 0x1048
	ctx.r[3].s64 = ctx.r[10].s64 + 4168;
	// 8328D724: 4B01DDE5  bl 0x822ab508
	ctx.lr = 0x8328D728;
	crate::recompiler::externs::call(&mut ctx, base, 0x822AB508);
	// 8328D728: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328D72C: 38694198  addi r3, r9, 0x4198
	ctx.r[3].s64 = ctx.r[9].s64 + 16792;
	// 8328D730: 4BA1C7F1  bl 0x82ca9f20
	ctx.lr = 0x8328D734;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328D734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328D738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328D73C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328D740: 4E800020  blr
	return;
}

pub fn sub_8328D748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D748 size=12
	// 8328D748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D788 size=12
	// 8328D788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D7C8 size=12
	// 8328D7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D808 size=12
	// 8328D808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D848 size=12
	// 8328D848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D888 size=12
	// 8328D888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D940 size=12
	// 8328D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328D980 size=12
	// 8328D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328D984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328D988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DA98 size=12
	// 8328DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DAD8 size=12
	// 8328DAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DAE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DB80 size=12
	// 8328DB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DBC0 size=12
	// 8328DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328DC28 size=8
	// 8328DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DC2C: 4BA1B7C9  bl 0x82ca93f4
	ctx.lr = 0x8328DC30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93F4);
}

pub fn sub_8328DDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DDD0 size=12
	// 8328DDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DDD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DE10 size=12
	// 8328DE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DE18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DE78 size=12
	// 8328DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DEB8 size=12
	// 8328DEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DEC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DEF8 size=12
	// 8328DEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328DF38 size=12
	// 8328DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328DF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328DF40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E000 size=12
	// 8328E000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E040 size=12
	// 8328E040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E080 size=12
	// 8328E080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E0C0 size=12
	// 8328E0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E0C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E100 size=12
	// 8328E100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E140 size=12
	// 8328E140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E290 size=12
	// 8328E290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E2D0 size=12
	// 8328E2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E340 size=12
	// 8328E340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E380 size=12
	// 8328E380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E3C0 size=12
	// 8328E3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E400 size=12
	// 8328E400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E440 size=12
	// 8328E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E480 size=12
	// 8328E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E4C0 size=12
	// 8328E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E500 size=12
	// 8328E500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E540 size=12
	// 8328E540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E580 size=12
	// 8328E580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E5C0 size=12
	// 8328E5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E5C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E600 size=12
	// 8328E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E640 size=12
	// 8328E640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E680 size=12
	// 8328E680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E6C0 size=12
	// 8328E6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E6C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E700 size=12
	// 8328E700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E740 size=12
	// 8328E740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E780 size=12
	// 8328E780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E7E8 size=12
	// 8328E7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E7F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E828 size=12
	// 8328E828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E8E0 size=112
	// 8328E8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328E8EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328E8F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328E8F4: 388B73E8  addi r4, r11, 0x73e8
	ctx.r[4].s64 = ctx.r[11].s64 + 29672;
	// 8328E8F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328E8FC: 4AF9E5D5  bl 0x8222ced0
	ctx.lr = 0x8328E900;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8328E900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328E904: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8328E908: 386A43F4  addi r3, r10, 0x43f4
	ctx.r[3].s64 = ctx.r[10].s64 + 17396;
	// 8328E90C: 4B01CBFD  bl 0x822ab508
	ctx.lr = 0x8328E910;
	crate::recompiler::externs::call(&mut ctx, base, 0x822AB508);
	// 8328E910: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328E914: 38694D20  addi r3, r9, 0x4d20
	ctx.r[3].s64 = ctx.r[9].s64 + 19744;
	// 8328E918: 4BA1B609  bl 0x82ca9f20
	ctx.lr = 0x8328E91C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328E91C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328E920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328E924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328E928: 4E800020  blr
	return;
}

pub fn sub_8328E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E950 size=12
	// 8328E950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328E990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328E990 size=12
	// 8328E990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328E994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328E998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EAB8 size=12
	// 8328EAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EAF8 size=12
	// 8328EAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EB00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EB38 size=12
	// 8328EB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EB40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EB78 size=12
	// 8328EB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EC18 size=12
	// 8328EC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EC58 size=12
	// 8328EC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EC98 size=12
	// 8328EC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ECA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328ECD8 size=12
	// 8328ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328ECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ECE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328ED18 size=12
	// 8328ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ED20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328ED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328ED58 size=12
	// 8328ED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328ED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328ED60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EEF8 size=144
	// 8328EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328EF04: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8328EF08: 4AF90351  bl 0x8221f258
	ctx.lr = 0x8328EF0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328EF0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328EF10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328EF14: 419A0008  beq cr6, 0x8328ef1c
	if ctx.cr[6].eq {
	pc = 0x8328EF1C; continue 'dispatch;
	}
	// 8328EF18: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328EF1C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328EF20: 41820008  beq 0x8328ef28
	if ctx.cr[0].eq {
	pc = 0x8328EF28; continue 'dispatch;
	}
	// 8328EF24: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328EF28: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328EF2C: 41820008  beq 0x8328ef34
	if ctx.cr[0].eq {
	pc = 0x8328EF34; continue 'dispatch;
	}
	// 8328EF30: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328EF34: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328EF38: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8328EF3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328EF40: 39094E6C  addi r8, r9, 0x4e6c
	ctx.r[8].s64 = ctx.r[9].s64 + 20076;
	// 8328EF44: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8328EF48: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328EF4C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328EF50: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8328EF54: 38675210  addi r3, r7, 0x5210
	ctx.r[3].s64 = ctx.r[7].s64 + 21008;
	// 8328EF58: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328EF5C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328EF60: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328EF64: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328EF68: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328EF6C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328EF70: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328EF74: 4BA1AFAD  bl 0x82ca9f20
	ctx.lr = 0x8328EF78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328EF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328EF84: 4E800020  blr
	return;
}

pub fn sub_8328EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EF88 size=12
	// 8328EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328EFC8 size=12
	// 8328EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F008 size=144
	// 8328F008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328F014: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8328F018: 4AF90241  bl 0x8221f258
	ctx.lr = 0x8328F01C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328F01C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328F020: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328F024: 419A0008  beq cr6, 0x8328f02c
	if ctx.cr[6].eq {
	pc = 0x8328F02C; continue 'dispatch;
	}
	// 8328F028: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328F02C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328F030: 41820008  beq 0x8328f038
	if ctx.cr[0].eq {
	pc = 0x8328F038; continue 'dispatch;
	}
	// 8328F034: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328F038: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328F03C: 41820008  beq 0x8328f044
	if ctx.cr[0].eq {
	pc = 0x8328F044; continue 'dispatch;
	}
	// 8328F040: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328F044: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328F048: 99430019  stb r10, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 8328F04C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328F050: 39094E80  addi r8, r9, 0x4e80
	ctx.r[8].s64 = ctx.r[9].s64 + 20096;
	// 8328F054: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 8328F058: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328F05C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328F060: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 8328F064: 38675250  addi r3, r7, 0x5250
	ctx.r[3].s64 = ctx.r[7].s64 + 21072;
	// 8328F068: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328F06C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328F070: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328F074: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328F078: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328F07C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328F080: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328F084: 4BA1AE9D  bl 0x82ca9f20
	ctx.lr = 0x8328F088;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328F088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328F094: 4E800020  blr
	return;
}

pub fn sub_8328F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F098 size=12
	// 8328F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F0D8 size=12
	// 8328F0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F128 size=12
	// 8328F128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F168 size=12
	// 8328F168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F238 size=12
	// 8328F238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F278 size=12
	// 8328F278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F308 size=12
	// 8328F308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F348 size=12
	// 8328F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F450 size=12
	// 8328F450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F490 size=12
	// 8328F490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F500 size=12
	// 8328F500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F540 size=12
	// 8328F540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F590 size=12
	// 8328F590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F5F0 size=12
	// 8328F5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F650 size=12
	// 8328F650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F6B0 size=12
	// 8328F6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F7E0 size=12
	// 8328F7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F7E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328F820 size=12
	// 8328F820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328F824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328F828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328FB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FB30 size=144
	// 8328FB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FB3C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FB40: 4AF8F719  bl 0x8221f258
	ctx.lr = 0x8328FB44;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328FB44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FB48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FB4C: 419A0008  beq cr6, 0x8328fb54
	if ctx.cr[6].eq {
	pc = 0x8328FB54; continue 'dispatch;
	}
	// 8328FB50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FB54: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FB58: 41820008  beq 0x8328fb60
	if ctx.cr[0].eq {
	pc = 0x8328FB60; continue 'dispatch;
	}
	// 8328FB5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FB60: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FB64: 41820008  beq 0x8328fb6c
	if ctx.cr[0].eq {
	pc = 0x8328FB6C; continue 'dispatch;
	}
	// 8328FB68: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FB6C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FB70: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FB74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FB78: 39095158  addi r8, r9, 0x5158
	ctx.r[8].s64 = ctx.r[9].s64 + 20824;
	// 8328FB7C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FB80: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FB84: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FB88: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FB8C: 38675A08  addi r3, r7, 0x5a08
	ctx.r[3].s64 = ctx.r[7].s64 + 23048;
	// 8328FB90: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FB94: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FB98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FB9C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FBA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FBA4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FBA8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FBAC: 4BA1A375  bl 0x82ca9f20
	ctx.lr = 0x8328FBB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328FBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FBBC: 4E800020  blr
	return;
}

pub fn sub_8328FBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FBC0 size=144
	// 8328FBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FBCC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FBD0: 4AF8F689  bl 0x8221f258
	ctx.lr = 0x8328FBD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328FBD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FBD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FBDC: 419A0008  beq cr6, 0x8328fbe4
	if ctx.cr[6].eq {
	pc = 0x8328FBE4; continue 'dispatch;
	}
	// 8328FBE0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FBE4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FBE8: 41820008  beq 0x8328fbf0
	if ctx.cr[0].eq {
	pc = 0x8328FBF0; continue 'dispatch;
	}
	// 8328FBEC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FBF0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FBF4: 41820008  beq 0x8328fbfc
	if ctx.cr[0].eq {
	pc = 0x8328FBFC; continue 'dispatch;
	}
	// 8328FBF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FBFC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FC00: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FC04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FC08: 39095164  addi r8, r9, 0x5164
	ctx.r[8].s64 = ctx.r[9].s64 + 20836;
	// 8328FC0C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FC10: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FC14: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FC18: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FC1C: 38675A18  addi r3, r7, 0x5a18
	ctx.r[3].s64 = ctx.r[7].s64 + 23064;
	// 8328FC20: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC24: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FC28: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC2C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FC30: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FC34: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FC38: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FC3C: 4BA1A2E5  bl 0x82ca9f20
	ctx.lr = 0x8328FC40;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328FC40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FC4C: 4E800020  blr
	return;
}

pub fn sub_8328FC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FC50 size=144
	// 8328FC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FC58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FC5C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FC60: 4AF8F5F9  bl 0x8221f258
	ctx.lr = 0x8328FC64;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328FC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FC68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FC6C: 419A0008  beq cr6, 0x8328fc74
	if ctx.cr[6].eq {
	pc = 0x8328FC74; continue 'dispatch;
	}
	// 8328FC70: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FC74: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FC78: 41820008  beq 0x8328fc80
	if ctx.cr[0].eq {
	pc = 0x8328FC80; continue 'dispatch;
	}
	// 8328FC7C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FC80: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FC84: 41820008  beq 0x8328fc8c
	if ctx.cr[0].eq {
	pc = 0x8328FC8C; continue 'dispatch;
	}
	// 8328FC88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FC8C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FC90: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FC94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FC98: 39095170  addi r8, r9, 0x5170
	ctx.r[8].s64 = ctx.r[9].s64 + 20848;
	// 8328FC9C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FCA0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FCA4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FCA8: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FCAC: 38675A28  addi r3, r7, 0x5a28
	ctx.r[3].s64 = ctx.r[7].s64 + 23080;
	// 8328FCB0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCB4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FCB8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCBC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FCC0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FCC4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FCC8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FCCC: 4BA1A255  bl 0x82ca9f20
	ctx.lr = 0x8328FCD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328FCD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FCDC: 4E800020  blr
	return;
}

pub fn sub_8328FCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FCE0 size=144
	// 8328FCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FCE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FCEC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FCF0: 4AF8F569  bl 0x8221f258
	ctx.lr = 0x8328FCF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328FCF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FCF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FCFC: 419A0008  beq cr6, 0x8328fd04
	if ctx.cr[6].eq {
	pc = 0x8328FD04; continue 'dispatch;
	}
	// 8328FD00: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FD04: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD08: 41820008  beq 0x8328fd10
	if ctx.cr[0].eq {
	pc = 0x8328FD10; continue 'dispatch;
	}
	// 8328FD0C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FD10: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD14: 41820008  beq 0x8328fd1c
	if ctx.cr[0].eq {
	pc = 0x8328FD1C; continue 'dispatch;
	}
	// 8328FD18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FD1C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FD20: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FD24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FD28: 3909517C  addi r8, r9, 0x517c
	ctx.r[8].s64 = ctx.r[9].s64 + 20860;
	// 8328FD2C: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FD30: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FD34: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FD38: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FD3C: 38675A38  addi r3, r7, 0x5a38
	ctx.r[3].s64 = ctx.r[7].s64 + 23096;
	// 8328FD40: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD44: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FD48: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD4C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FD50: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FD54: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FD58: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FD5C: 4BA1A1C5  bl 0x82ca9f20
	ctx.lr = 0x8328FD60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328FD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FD6C: 4E800020  blr
	return;
}

pub fn sub_8328FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FD70 size=144
	// 8328FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328FD7C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8328FD80: 4AF8F4D9  bl 0x8221f258
	ctx.lr = 0x8328FD84;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8328FD84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8328FD88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328FD8C: 419A0008  beq cr6, 0x8328fd94
	if ctx.cr[6].eq {
	pc = 0x8328FD94; continue 'dispatch;
	}
	// 8328FD90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FD94: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FD98: 41820008  beq 0x8328fda0
	if ctx.cr[0].eq {
	pc = 0x8328FDA0; continue 'dispatch;
	}
	// 8328FD9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FDA0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328FDA4: 41820008  beq 0x8328fdac
	if ctx.cr[0].eq {
	pc = 0x8328FDAC; continue 'dispatch;
	}
	// 8328FDA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8328FDAC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328FDB0: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8328FDB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328FDB8: 39095188  addi r8, r9, 0x5188
	ctx.r[8].s64 = ctx.r[9].s64 + 20872;
	// 8328FDBC: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8328FDC0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328FDC4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8328FDC8: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8328FDCC: 38675A48  addi r3, r7, 0x5a48
	ctx.r[3].s64 = ctx.r[7].s64 + 23112;
	// 8328FDD0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDD4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8328FDD8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDDC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8328FDE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328FDE4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8328FDE8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328FDEC: 4BA1A135  bl 0x82ca9f20
	ctx.lr = 0x8328FDF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8328FDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328FDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328FDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328FDFC: 4E800020  blr
	return;
}

pub fn sub_8328FE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FE00 size=12
	// 8328FE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8328FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328FE40 size=12
	// 8328FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328FE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290080 size=12
	// 83290080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832900C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832900C0 size=12
	// 832900C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832900C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832900C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290100 size=56
	// 83290100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329010C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290110: 386B5298  addi r3, r11, 0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + 21144;
	// 83290114: 4AFFBC45  bl 0x8228bd58
	ctx.lr = 0x83290118;
	crate::recompiler::externs::call(&mut ctx, base, 0x8228BD58);
	// 83290118: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329011C: 386A5AA0  addi r3, r10, 0x5aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 23200;
	// 83290120: 4BA19E01  bl 0x82ca9f20
	ctx.lr = 0x83290124;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83290124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329012C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290130: 4E800020  blr
	return;
}

pub fn sub_83290138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290138 size=80
	// 83290138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290144: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83290148: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8329014C: 388B7F00  addi r4, r11, 0x7f00
	ctx.r[4].s64 = ctx.r[11].s64 + 32512;
	// 83290150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83290154: 4AF9CD7D  bl 0x8222ced0
	ctx.lr = 0x83290158;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83290158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8329015C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83290160: 386A5654  addi r3, r10, 0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + 22100;
	// 83290164: 4B01B3A5  bl 0x822ab508
	ctx.lr = 0x83290168;
	crate::recompiler::externs::call(&mut ctx, base, 0x822AB508);
	// 83290168: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8329016C: 38695AB0  addi r3, r9, 0x5ab0
	ctx.r[3].s64 = ctx.r[9].s64 + 23216;
	// 83290170: 4BA19DB1  bl 0x82ca9f20
	ctx.lr = 0x83290174;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83290174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329017C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290180: 4E800020  blr
	return;
}

pub fn sub_83290188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290188 size=12
	// 83290188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832901C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832901C8 size=12
	// 832901C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832901CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832901D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290208 size=12
	// 83290208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290248 size=12
	// 83290248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329024C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

