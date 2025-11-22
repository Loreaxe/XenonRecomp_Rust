pub fn sub_83251D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251D28 size=12
	// 83251D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251D68 size=12
	// 83251D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251DA8 size=12
	// 83251DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251DE8 size=12
	// 83251DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251E28 size=12
	// 83251E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251E68 size=12
	// 83251E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251EA8 size=12
	// 83251EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251EE8 size=12
	// 83251EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251F28 size=12
	// 83251F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251F68 size=12
	// 83251F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251FA8 size=12
	// 83251FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83251FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251FE8 size=12
	// 83251FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252028 size=12
	// 83252028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252068 size=12
	// 83252068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832520A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832520A8 size=12
	// 832520A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832520AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832520B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832520E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832520E8 size=12
	// 832520E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832520EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832520F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252128 size=12
	// 83252128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325212C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252168 size=12
	// 83252168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325216C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832521A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832521A8 size=12
	// 832521A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832521AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832521B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832521E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832521E8 size=12
	// 832521E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832521EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832521F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252228 size=12
	// 83252228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325222C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252268 size=12
	// 83252268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832522A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832522A8 size=12
	// 832522A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832522AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832522B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832522E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832522E8 size=12
	// 832522E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832522EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832522F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252328 size=12
	// 83252328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325232C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252368 size=12
	// 83252368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832523A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832523A8 size=16
	// 832523A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832523AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832523B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832523B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252468 size=12
	// 83252468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325246C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832524A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832524A8 size=12
	// 832524A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832524AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832524B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832524E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832524E8 size=12
	// 832524E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832524EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832524F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252528 size=12
	// 83252528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252568 size=12
	// 83252568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325256C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832525A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832525A8 size=200
	// 832525A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832525AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832525B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832525B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832525B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832525BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832525C0: 3BEB8130  addi r31, r11, -0x7ed0
	ctx.r[31].s64 = ctx.r[11].s64 + -32464;
	// 832525C4: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 832525C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832525CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525D0: 4AFDA901  bl 0x8222ced0
	ctx.lr = 0x832525D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832525D4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832525D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525DC: 388934F8  addi r4, r9, 0x34f8
	ctx.r[4].s64 = ctx.r[9].s64 + 13560;
	// 832525E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832525E4: 4AFDA8ED  bl 0x8222ced0
	ctx.lr = 0x832525E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832525E8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832525EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525F0: 388834D0  addi r4, r8, 0x34d0
	ctx.r[4].s64 = ctx.r[8].s64 + 13520;
	// 832525F4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832525F8: 4AFDA8D9  bl 0x8222ced0
	ctx.lr = 0x832525FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832525FC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83252600: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252604: 388734A8  addi r4, r7, 0x34a8
	ctx.r[4].s64 = ctx.r[7].s64 + 13480;
	// 83252608: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8325260C: 4AFDA8C5  bl 0x8222ced0
	ctx.lr = 0x83252610;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83252610: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83252614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252618: 38863480  addi r4, r6, 0x3480
	ctx.r[4].s64 = ctx.r[6].s64 + 13440;
	// 8325261C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83252620: 4AFDA8B1  bl 0x8222ced0
	ctx.lr = 0x83252624;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83252624: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83252628: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325262C: 38843458  addi r4, r4, 0x3458
	ctx.r[4].s64 = ctx.r[4].s64 + 13400;
	// 83252630: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83252634: 4AFDA89D  bl 0x8222ced0
	ctx.lr = 0x83252638;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83252638: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8325263C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252640: 38833430  addi r4, r3, 0x3430
	ctx.r[4].s64 = ctx.r[3].s64 + 13360;
	// 83252644: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83252648: 4AFDA889  bl 0x8222ced0
	ctx.lr = 0x8325264C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325264C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83252650: 386B99E0  addi r3, r11, -0x6620
	ctx.r[3].s64 = ctx.r[11].s64 + -26144;
	// 83252654: 4BA578CD  bl 0x82ca9f20
	ctx.lr = 0x83252658;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83252658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252668: 4E800020  blr
	return;
}

pub fn sub_83252670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252670 size=848
	// 83252670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325267C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252680: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83252684: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83252688: 3BEB814C  addi r31, r11, -0x7eb4
	ctx.r[31].s64 = ctx.r[11].s64 + -32436;
	// 8325268C: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83252690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83252694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252698: 4AFDA839  bl 0x8222ced0
	ctx.lr = 0x8325269C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325269C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832526A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526A4: 38893598  addi r4, r9, 0x3598
	ctx.r[4].s64 = ctx.r[9].s64 + 13720;
	// 832526A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832526AC: 4AFDA825  bl 0x8222ced0
	ctx.lr = 0x832526B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832526B0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832526B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526B8: 38883580  addi r4, r8, 0x3580
	ctx.r[4].s64 = ctx.r[8].s64 + 13696;
	// 832526BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832526C0: 4AFDA811  bl 0x8222ced0
	ctx.lr = 0x832526C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832526C4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832526C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526CC: 38873568  addi r4, r7, 0x3568
	ctx.r[4].s64 = ctx.r[7].s64 + 13672;
	// 832526D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832526D4: 4AFDA7FD  bl 0x8222ced0
	ctx.lr = 0x832526D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832526D8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 832526DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526E0: 38863550  addi r4, r6, 0x3550
	ctx.r[4].s64 = ctx.r[6].s64 + 13648;
	// 832526E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832526E8: 4AFDA7E9  bl 0x8222ced0
	ctx.lr = 0x832526EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832526EC: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832526F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526F4: 38843538  addi r4, r4, 0x3538
	ctx.r[4].s64 = ctx.r[4].s64 + 13624;
	// 832526F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832526FC: 4AFDA7D5  bl 0x8222ced0
	ctx.lr = 0x83252700;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83252700: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83252704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252708: 38833520  addi r4, r3, 0x3520
	ctx.r[4].s64 = ctx.r[3].s64 + 13600;
	// 8325270C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83252710: 4AFDA7C1  bl 0x8222ced0
	ctx.lr = 0x83252714;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83252714: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83252718: 386B9A48  addi r3, r11, -0x65b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26040;
	// 8325271C: 4BA57805  bl 0x82ca9f20
	ctx.lr = 0x83252720;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83252720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325272C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252730: 4E800020  blr
	return;
}

pub fn sub_832529C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832529C0 size=12
	// 832529C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832529C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832529C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A00 size=12
	// 83252A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A40 size=12
	// 83252A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A80 size=12
	// 83252A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252AC0 size=12
	// 83252AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B00 size=12
	// 83252B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B40 size=12
	// 83252B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B80 size=12
	// 83252B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252BC0 size=12
	// 83252BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C00 size=12
	// 83252C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C40 size=12
	// 83252C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C80 size=12
	// 83252C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252CC0 size=12
	// 83252CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D00 size=56
	// 83252D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252D0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252D10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83252D14: 386B3FB4  addi r3, r11, 0x3fb4
	ctx.r[3].s64 = ctx.r[11].s64 + 16308;
	// 83252D18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83252D1C: 4AFA103D  bl 0x821f3d58
	ctx.lr = 0x83252D20;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83252D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252D24: 906A81FC  stw r3, -0x7e04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32260 as u32), ctx.r[3].u32 ) };
	// 83252D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252D34: 4E800020  blr
	return;
}

pub fn sub_83252D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D38 size=12
	// 83252D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D78 size=12
	// 83252D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252DB8 size=12
	// 83252DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252DF8 size=12
	// 83252DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252E38 size=12
	// 83252E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252E78 size=144
	// 83252E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252E84: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83252E88: 4AFCC3D1  bl 0x8221f258
	ctx.lr = 0x83252E8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83252E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83252E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83252E94: 419A0008  beq cr6, 0x83252e9c
	if ctx.cr[6].eq {
	pc = 0x83252E9C; continue 'dispatch;
	}
	// 83252E98: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252E9C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252EA0: 41820008  beq 0x83252ea8
	if ctx.cr[0].eq {
	pc = 0x83252EA8; continue 'dispatch;
	}
	// 83252EA4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252EA8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252EAC: 41820008  beq 0x83252eb4
	if ctx.cr[0].eq {
	pc = 0x83252EB4; continue 'dispatch;
	}
	// 83252EB0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252EB4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83252EB8: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 83252EBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83252EC0: 39098214  addi r8, r9, -0x7dec
	ctx.r[8].s64 = ctx.r[9].s64 + -32236;
	// 83252EC4: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83252EC8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83252ECC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83252ED0: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 83252ED4: 38679BD0  addi r3, r7, -0x6430
	ctx.r[3].s64 = ctx.r[7].s64 + -25648;
	// 83252ED8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EDC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83252EE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EE4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83252EE8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EEC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83252EF0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83252EF4: 4BA5702D  bl 0x82ca9f20
	ctx.lr = 0x83252EF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83252EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252F04: 4E800020  blr
	return;
}

pub fn sub_83252F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252F08 size=144
	// 83252F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252F14: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83252F18: 4AFCC341  bl 0x8221f258
	ctx.lr = 0x83252F1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83252F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83252F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83252F24: 419A0008  beq cr6, 0x83252f2c
	if ctx.cr[6].eq {
	pc = 0x83252F2C; continue 'dispatch;
	}
	// 83252F28: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252F2C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252F30: 41820008  beq 0x83252f38
	if ctx.cr[0].eq {
	pc = 0x83252F38; continue 'dispatch;
	}
	// 83252F34: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252F38: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252F3C: 41820008  beq 0x83252f44
	if ctx.cr[0].eq {
	pc = 0x83252F44; continue 'dispatch;
	}
	// 83252F40: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252F44: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83252F48: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83252F4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83252F50: 39098220  addi r8, r9, -0x7de0
	ctx.r[8].s64 = ctx.r[9].s64 + -32224;
	// 83252F54: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83252F58: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83252F5C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83252F60: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83252F64: 38679BE0  addi r3, r7, -0x6420
	ctx.r[3].s64 = ctx.r[7].s64 + -25632;
	// 83252F68: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F6C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83252F70: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F74: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83252F78: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F7C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83252F80: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83252F84: 4BA56F9D  bl 0x82ca9f20
	ctx.lr = 0x83252F88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83252F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252F94: 4E800020  blr
	return;
}

pub fn sub_83252F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252F98 size=12
	// 83252F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83252FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252FD8 size=12
	// 83252FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83253018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253018 size=12
	// 83253018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83253058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253058 size=56
	// 83253058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325305C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253064: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253068: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325306C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83253070: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253074: 4AFA0CE5  bl 0x821f3d58
	ctx.lr = 0x83253078;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325307C: 906A8238  stw r3, -0x7dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32200 as u32), ctx.r[3].u32 ) };
	// 83253080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325308C: 4E800020  blr
	return;
}

pub fn sub_83253090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253090 size=56
	// 83253090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325309C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832530A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832530A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832530A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832530AC: 4AFA0CAD  bl 0x821f3d58
	ctx.lr = 0x832530B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832530B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832530B4: 906A823C  stw r3, -0x7dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32196 as u32), ctx.r[3].u32 ) };
	// 832530B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832530BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832530C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832530C4: 4E800020  blr
	return;
}

pub fn sub_832530C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832530C8 size=56
	// 832530C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832530CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832530D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832530D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832530D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832530DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832530E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832530E4: 4AFA0C75  bl 0x821f3d58
	ctx.lr = 0x832530E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832530E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832530EC: 906A8240  stw r3, -0x7dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32192 as u32), ctx.r[3].u32 ) };
	// 832530F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832530F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832530F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832530FC: 4E800020  blr
	return;
}

pub fn sub_83253100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253100 size=56
	// 83253100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325310C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253110: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253114: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83253118: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325311C: 4AFA0C3D  bl 0x821f3d58
	ctx.lr = 0x83253120;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253124: 906A8244  stw r3, -0x7dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32188 as u32), ctx.r[3].u32 ) };
	// 83253128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325312C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253134: 4E800020  blr
	return;
}

pub fn sub_83253138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253138 size=56
	// 83253138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253148: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325314C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83253150: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253154: 4AFA0C05  bl 0x821f3d58
	ctx.lr = 0x83253158;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325315C: 906A8248  stw r3, -0x7db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32184 as u32), ctx.r[3].u32 ) };
	// 83253160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325316C: 4E800020  blr
	return;
}

pub fn sub_83253170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253170 size=56
	// 83253170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325317C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253180: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253184: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83253188: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325318C: 4AFA0BCD  bl 0x821f3d58
	ctx.lr = 0x83253190;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253194: 906A824C  stw r3, -0x7db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32180 as u32), ctx.r[3].u32 ) };
	// 83253198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325319C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832531A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832531A4: 4E800020  blr
	return;
}

pub fn sub_832531A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832531A8 size=56
	// 832531A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832531AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832531B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832531B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832531B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832531BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832531C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832531C4: 4AFA0B95  bl 0x821f3d58
	ctx.lr = 0x832531C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832531C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832531CC: 906A8250  stw r3, -0x7db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32176 as u32), ctx.r[3].u32 ) };
	// 832531D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832531D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832531D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832531DC: 4E800020  blr
	return;
}

pub fn sub_832531E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832531E0 size=56
	// 832531E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832531E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832531E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832531EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832531F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832531F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832531F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832531FC: 4AFA0B5D  bl 0x821f3d58
	ctx.lr = 0x83253200;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253204: 906A8254  stw r3, -0x7dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32172 as u32), ctx.r[3].u32 ) };
	// 83253208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325320C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253214: 4E800020  blr
	return;
}

pub fn sub_83253218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253218 size=56
	// 83253218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325321C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253224: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253228: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325322C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83253230: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253234: 4AFA0B25  bl 0x821f3d58
	ctx.lr = 0x83253238;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253238: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325323C: 906A8258  stw r3, -0x7da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32168 as u32), ctx.r[3].u32 ) };
	// 83253240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325324C: 4E800020  blr
	return;
}

pub fn sub_83253250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253250 size=56
	// 83253250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325325C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253260: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253264: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83253268: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325326C: 4AFA0AED  bl 0x821f3d58
	ctx.lr = 0x83253270;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253274: 906A825C  stw r3, -0x7da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32164 as u32), ctx.r[3].u32 ) };
	// 83253278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325327C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253284: 4E800020  blr
	return;
}

pub fn sub_83253288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253288 size=56
	// 83253288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325328C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253298: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325329C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832532A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832532A4: 4AFA0AB5  bl 0x821f3d58
	ctx.lr = 0x832532A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832532A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832532AC: 906A8260  stw r3, -0x7da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32160 as u32), ctx.r[3].u32 ) };
	// 832532B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832532B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832532B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832532BC: 4E800020  blr
	return;
}

pub fn sub_832532C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832532C0 size=56
	// 832532C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832532C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832532C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832532CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832532D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832532D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832532D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832532DC: 4AFA0A7D  bl 0x821f3d58
	ctx.lr = 0x832532E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832532E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832532E4: 906A8264  stw r3, -0x7d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32156 as u32), ctx.r[3].u32 ) };
	// 832532E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832532EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832532F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832532F4: 4E800020  blr
	return;
}

pub fn sub_832532F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832532F8 size=56
	// 832532F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832532FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253304: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253308: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325330C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83253310: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253314: 4AFA0A45  bl 0x821f3d58
	ctx.lr = 0x83253318;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325331C: 906A8268  stw r3, -0x7d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32152 as u32), ctx.r[3].u32 ) };
	// 83253320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325332C: 4E800020  blr
	return;
}

pub fn sub_83253330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253330 size=56
	// 83253330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325333C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253340: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253344: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83253348: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325334C: 4AFA0A0D  bl 0x821f3d58
	ctx.lr = 0x83253350;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253354: 906A826C  stw r3, -0x7d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32148 as u32), ctx.r[3].u32 ) };
	// 83253358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325335C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253364: 4E800020  blr
	return;
}

pub fn sub_83253368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253368 size=56
	// 83253368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325336C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253374: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253378: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325337C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83253380: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253384: 4AFA09D5  bl 0x821f3d58
	ctx.lr = 0x83253388;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325338C: 906A8270  stw r3, -0x7d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32144 as u32), ctx.r[3].u32 ) };
	// 83253390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325339C: 4E800020  blr
	return;
}

pub fn sub_832533A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832533A0 size=56
	// 832533A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832533A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832533A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832533AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832533B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832533B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832533B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832533BC: 4AFA099D  bl 0x821f3d58
	ctx.lr = 0x832533C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832533C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832533C4: 906A8274  stw r3, -0x7d8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32140 as u32), ctx.r[3].u32 ) };
	// 832533C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832533CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832533D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832533D4: 4E800020  blr
	return;
}

pub fn sub_832533D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832533D8 size=56
	// 832533D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832533DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832533E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832533E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832533E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832533EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832533F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832533F4: 4AFA0965  bl 0x821f3d58
	ctx.lr = 0x832533F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832533F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832533FC: 906A8278  stw r3, -0x7d88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32136 as u32), ctx.r[3].u32 ) };
	// 83253400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325340C: 4E800020  blr
	return;
}

pub fn sub_83253410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253410 size=56
	// 83253410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325341C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253420: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253424: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83253428: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325342C: 4AFA092D  bl 0x821f3d58
	ctx.lr = 0x83253430;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253434: 906A827C  stw r3, -0x7d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32132 as u32), ctx.r[3].u32 ) };
	// 83253438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325343C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253444: 4E800020  blr
	return;
}

pub fn sub_83253448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253448 size=56
	// 83253448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325344C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253458: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325345C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83253460: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253464: 4AFA08F5  bl 0x821f3d58
	ctx.lr = 0x83253468;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83253468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325346C: 906A8280  stw r3, -0x7d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32128 as u32), ctx.r[3].u32 ) };
	// 83253470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325347C: 4E800020  blr
	return;
}

pub fn sub_83253480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253480 size=56
	// 83253480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325348C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253490: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253494: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83253498: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325349C: 4AFA08BD  bl 0x821f3d58
	ctx.lr = 0x832534A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832534A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832534A4: 906A8284  stw r3, -0x7d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32124 as u32), ctx.r[3].u32 ) };
	// 832534A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832534AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832534B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832534B4: 4E800020  blr
	return;
}

pub fn sub_832534B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832534B8 size=56
	// 832534B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832534BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832534C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832534C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832534C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832534CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832534D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832534D4: 4AFA0885  bl 0x821f3d58
	ctx.lr = 0x832534D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832534D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832534DC: 906A8288  stw r3, -0x7d78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32120 as u32), ctx.r[3].u32 ) };
	// 832534E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832534E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832534E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832534EC: 4E800020  blr
	return;
}

pub fn sub_832534F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832534F0 size=12
	// 832534F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832534F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832534F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83253530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253530 size=12
	// 83253530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83253570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253570 size=12
	// 83253570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832535B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832535B0 size=12
	// 832535B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832535B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832535B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832535F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832535F0 size=12
	// 832535F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832535F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832535F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83253630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253630 size=12
	// 83253630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83253670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253670 size=296
	// 83253670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325367C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253680: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253684: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253688: 3BEB82A4  addi r31, r11, -0x7d5c
	ctx.r[31].s64 = ctx.r[11].s64 + -32092;
	// 8325368C: 388A8108  addi r4, r10, -0x7ef8
	ctx.r[4].s64 = ctx.r[10].s64 + -32504;
	// 83253690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253698: 4AFD9839  bl 0x8222ced0
	ctx.lr = 0x8325369C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325369C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832536A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536A4: 388980F8  addi r4, r9, -0x7f08
	ctx.r[4].s64 = ctx.r[9].s64 + -32520;
	// 832536A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832536AC: 4AFD9825  bl 0x8222ced0
	ctx.lr = 0x832536B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832536B0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832536B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536B8: 388880E8  addi r4, r8, -0x7f18
	ctx.r[4].s64 = ctx.r[8].s64 + -32536;
	// 832536BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832536C0: 4AFD9811  bl 0x8222ced0
	ctx.lr = 0x832536C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832536C4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832536C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536CC: 388780D4  addi r4, r7, -0x7f2c
	ctx.r[4].s64 = ctx.r[7].s64 + -32556;
	// 832536D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832536D4: 4AFD97FD  bl 0x8222ced0
	ctx.lr = 0x832536D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832536D8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832536DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536E0: 388680C0  addi r4, r6, -0x7f40
	ctx.r[4].s64 = ctx.r[6].s64 + -32576;
	// 832536E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832536E8: 4AFD97E9  bl 0x8222ced0
	ctx.lr = 0x832536EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832536EC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832536F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536F4: 388480AC  addi r4, r4, -0x7f54
	ctx.r[4].s64 = ctx.r[4].s64 + -32596;
	// 832536F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832536FC: 4AFD97D5  bl 0x8222ced0
	ctx.lr = 0x83253700;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253700: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253708: 3883809C  addi r4, r3, -0x7f64
	ctx.r[4].s64 = ctx.r[3].s64 + -32612;
	// 8325370C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253710: 4AFD97C1  bl 0x8222ced0
	ctx.lr = 0x83253714;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253714: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253718: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325371C: 388B8088  addi r4, r11, -0x7f78
	ctx.r[4].s64 = ctx.r[11].s64 + -32632;
	// 83253720: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253724: 4AFD97AD  bl 0x8222ced0
	ctx.lr = 0x83253728;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253728: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325372C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253730: 388A8078  addi r4, r10, -0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + -32648;
	// 83253734: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253738: 4AFD9799  bl 0x8222ced0
	ctx.lr = 0x8325373C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325373C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253740: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253744: 3889806C  addi r4, r9, -0x7f94
	ctx.r[4].s64 = ctx.r[9].s64 + -32660;
	// 83253748: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8325374C: 4AFD9785  bl 0x8222ced0
	ctx.lr = 0x83253750;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253750: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253758: 38888060  addi r4, r8, -0x7fa0
	ctx.r[4].s64 = ctx.r[8].s64 + -32672;
	// 8325375C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253760: 4AFD9771  bl 0x8222ced0
	ctx.lr = 0x83253764;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253764: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83253768: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325376C: 3887F10C  addi r4, r7, -0xef4
	ctx.r[4].s64 = ctx.r[7].s64 + -3828;
	// 83253770: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83253774: 4AFD975D  bl 0x8222ced0
	ctx.lr = 0x83253778;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253778: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 8325377C: 38669CF0  addi r3, r6, -0x6310
	ctx.r[3].s64 = ctx.r[6].s64 + -25360;
	// 83253780: 4BA567A1  bl 0x82ca9f20
	ctx.lr = 0x83253784;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83253784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325378C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253794: 4E800020  blr
	return;
}

pub fn sub_83253798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253798 size=296
	// 83253798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325379C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832537A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832537A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832537A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832537AC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832537B0: 3BEB82D4  addi r31, r11, -0x7d2c
	ctx.r[31].s64 = ctx.r[11].s64 + -32044;
	// 832537B4: 388A8268  addi r4, r10, -0x7d98
	ctx.r[4].s64 = ctx.r[10].s64 + -32152;
	// 832537B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832537BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537C0: 4AFD9711  bl 0x8222ced0
	ctx.lr = 0x832537C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832537C4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832537C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537CC: 3889824C  addi r4, r9, -0x7db4
	ctx.r[4].s64 = ctx.r[9].s64 + -32180;
	// 832537D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832537D4: 4AFD96FD  bl 0x8222ced0
	ctx.lr = 0x832537D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832537D8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832537DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537E0: 38888230  addi r4, r8, -0x7dd0
	ctx.r[4].s64 = ctx.r[8].s64 + -32208;
	// 832537E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832537E8: 4AFD96E9  bl 0x8222ced0
	ctx.lr = 0x832537EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832537EC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832537F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537F4: 38878214  addi r4, r7, -0x7dec
	ctx.r[4].s64 = ctx.r[7].s64 + -32236;
	// 832537F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832537FC: 4AFD96D5  bl 0x8222ced0
	ctx.lr = 0x83253800;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253800: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253808: 388681F0  addi r4, r6, -0x7e10
	ctx.r[4].s64 = ctx.r[6].s64 + -32272;
	// 8325380C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253810: 4AFD96C1  bl 0x8222ced0
	ctx.lr = 0x83253814;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253814: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253818: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325381C: 388481D0  addi r4, r4, -0x7e30
	ctx.r[4].s64 = ctx.r[4].s64 + -32304;
	// 83253820: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83253824: 4AFD96AD  bl 0x8222ced0
	ctx.lr = 0x83253828;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253828: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 8325382C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253830: 388381B0  addi r4, r3, -0x7e50
	ctx.r[4].s64 = ctx.r[3].s64 + -32336;
	// 83253834: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253838: 4AFD9699  bl 0x8222ced0
	ctx.lr = 0x8325383C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325383C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253840: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253844: 388B8190  addi r4, r11, -0x7e70
	ctx.r[4].s64 = ctx.r[11].s64 + -32368;
	// 83253848: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8325384C: 4AFD9685  bl 0x8222ced0
	ctx.lr = 0x83253850;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253850: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253858: 388A8174  addi r4, r10, -0x7e8c
	ctx.r[4].s64 = ctx.r[10].s64 + -32396;
	// 8325385C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253860: 4AFD9671  bl 0x8222ced0
	ctx.lr = 0x83253864;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253864: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253868: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325386C: 38898158  addi r4, r9, -0x7ea8
	ctx.r[4].s64 = ctx.r[9].s64 + -32424;
	// 83253870: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83253874: 4AFD965D  bl 0x8222ced0
	ctx.lr = 0x83253878;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253878: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 8325387C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253880: 3888813C  addi r4, r8, -0x7ec4
	ctx.r[4].s64 = ctx.r[8].s64 + -32452;
	// 83253884: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253888: 4AFD9649  bl 0x8222ced0
	ctx.lr = 0x8325388C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325388C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253890: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253894: 3887811C  addi r4, r7, -0x7ee4
	ctx.r[4].s64 = ctx.r[7].s64 + -32484;
	// 83253898: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8325389C: 4AFD9635  bl 0x8222ced0
	ctx.lr = 0x832538A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832538A0: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 832538A4: 38669D58  addi r3, r6, -0x62a8
	ctx.r[3].s64 = ctx.r[6].s64 + -25256;
	// 832538A8: 4BA56679  bl 0x82ca9f20
	ctx.lr = 0x832538AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832538AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832538B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832538B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832538B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832538BC: 4E800020  blr
	return;
}

pub fn sub_832538C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832538C0 size=456
	// 832538C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832538C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832538C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832538CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832538D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832538D4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832538D8: 3BEB8318  addi r31, r11, -0x7ce8
	ctx.r[31].s64 = ctx.r[11].s64 + -31976;
	// 832538DC: 388A8540  addi r4, r10, -0x7ac0
	ctx.r[4].s64 = ctx.r[10].s64 + -31424;
	// 832538E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832538E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832538E8: 4AFD95E9  bl 0x8222ced0
	ctx.lr = 0x832538EC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832538EC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832538F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832538F4: 38898520  addi r4, r9, -0x7ae0
	ctx.r[4].s64 = ctx.r[9].s64 + -31456;
	// 832538F8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832538FC: 4AFD95D5  bl 0x8222ced0
	ctx.lr = 0x83253900;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253900: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253908: 38888500  addi r4, r8, -0x7b00
	ctx.r[4].s64 = ctx.r[8].s64 + -31488;
	// 8325390C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253910: 4AFD95C1  bl 0x8222ced0
	ctx.lr = 0x83253914;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253914: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253918: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325391C: 388784E0  addi r4, r7, -0x7b20
	ctx.r[4].s64 = ctx.r[7].s64 + -31520;
	// 83253920: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253924: 4AFD95AD  bl 0x8222ced0
	ctx.lr = 0x83253928;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253928: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325392C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253930: 388684C0  addi r4, r6, -0x7b40
	ctx.r[4].s64 = ctx.r[6].s64 + -31552;
	// 83253934: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253938: 4AFD9599  bl 0x8222ced0
	ctx.lr = 0x8325393C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325393C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253940: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253944: 388484A0  addi r4, r4, -0x7b60
	ctx.r[4].s64 = ctx.r[4].s64 + -31584;
	// 83253948: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8325394C: 4AFD9585  bl 0x8222ced0
	ctx.lr = 0x83253950;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253950: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253958: 38838480  addi r4, r3, -0x7b80
	ctx.r[4].s64 = ctx.r[3].s64 + -31616;
	// 8325395C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253960: 4AFD9571  bl 0x8222ced0
	ctx.lr = 0x83253964;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253964: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325396C: 388B8460  addi r4, r11, -0x7ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -31648;
	// 83253970: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253974: 4AFD955D  bl 0x8222ced0
	ctx.lr = 0x83253978;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253978: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325397C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253980: 388A843C  addi r4, r10, -0x7bc4
	ctx.r[4].s64 = ctx.r[10].s64 + -31684;
	// 83253984: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253988: 4AFD9549  bl 0x8222ced0
	ctx.lr = 0x8325398C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325398C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253990: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253994: 38898418  addi r4, r9, -0x7be8
	ctx.r[4].s64 = ctx.r[9].s64 + -31720;
	// 83253998: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8325399C: 4AFD9535  bl 0x8222ced0
	ctx.lr = 0x832539A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832539A0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832539A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539A8: 388883F8  addi r4, r8, -0x7c08
	ctx.r[4].s64 = ctx.r[8].s64 + -31752;
	// 832539AC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 832539B0: 4AFD9521  bl 0x8222ced0
	ctx.lr = 0x832539B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832539B4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832539B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539BC: 388783D8  addi r4, r7, -0x7c28
	ctx.r[4].s64 = ctx.r[7].s64 + -31784;
	// 832539C0: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 832539C4: 4AFD950D  bl 0x8222ced0
	ctx.lr = 0x832539C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832539C8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832539CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539D0: 388683AC  addi r4, r6, -0x7c54
	ctx.r[4].s64 = ctx.r[6].s64 + -31828;
	// 832539D4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832539D8: 4AFD94F9  bl 0x8222ced0
	ctx.lr = 0x832539DC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832539DC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832539E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539E4: 38848384  addi r4, r4, -0x7c7c
	ctx.r[4].s64 = ctx.r[4].s64 + -31868;
	// 832539E8: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832539EC: 4AFD94E5  bl 0x8222ced0
	ctx.lr = 0x832539F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832539F0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832539F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539F8: 38838358  addi r4, r3, -0x7ca8
	ctx.r[4].s64 = ctx.r[3].s64 + -31912;
	// 832539FC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83253A00: 4AFD94D1  bl 0x8222ced0
	ctx.lr = 0x83253A04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253A04: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253A08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A0C: 388B8330  addi r4, r11, -0x7cd0
	ctx.r[4].s64 = ctx.r[11].s64 + -31952;
	// 83253A10: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83253A14: 4AFD94BD  bl 0x8222ced0
	ctx.lr = 0x83253A18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253A18: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A20: 388A8308  addi r4, r10, -0x7cf8
	ctx.r[4].s64 = ctx.r[10].s64 + -31992;
	// 83253A24: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83253A28: 4AFD94A9  bl 0x8222ced0
	ctx.lr = 0x83253A2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253A2C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253A30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A34: 388982E0  addi r4, r9, -0x7d20
	ctx.r[4].s64 = ctx.r[9].s64 + -32032;
	// 83253A38: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83253A3C: 4AFD9495  bl 0x8222ced0
	ctx.lr = 0x83253A40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253A40: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A48: 388882B4  addi r4, r8, -0x7d4c
	ctx.r[4].s64 = ctx.r[8].s64 + -32076;
	// 83253A4C: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83253A50: 4AFD9481  bl 0x8222ced0
	ctx.lr = 0x83253A54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253A54: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253A58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A5C: 38878288  addi r4, r7, -0x7d78
	ctx.r[4].s64 = ctx.r[7].s64 + -32120;
	// 83253A60: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83253A64: 4AFD946D  bl 0x8222ced0
	ctx.lr = 0x83253A68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253A68: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 83253A6C: 38669DC0  addi r3, r6, -0x6240
	ctx.r[3].s64 = ctx.r[6].s64 + -25152;
	// 83253A70: 4BA564B1  bl 0x82ca9f20
	ctx.lr = 0x83253A74;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83253A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253A84: 4E800020  blr
	return;
}

pub fn sub_83253A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253A88 size=480
	// 83253A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253A90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253A94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253A98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253A9C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83253AA0: 3BEB8368  addi r31, r11, -0x7c98
	ctx.r[31].s64 = ctx.r[11].s64 + -31896;
	// 83253AA4: 388A25D4  addi r4, r10, 0x25d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9684;
	// 83253AA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253AAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AB0: 4AFD9421  bl 0x8222ced0
	ctx.lr = 0x83253AB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253AB4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253AB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253ABC: 38898644  addi r4, r9, -0x79bc
	ctx.r[4].s64 = ctx.r[9].s64 + -31164;
	// 83253AC0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253AC4: 4AFD940D  bl 0x8222ced0
	ctx.lr = 0x83253AC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253AC8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AD0: 38888634  addi r4, r8, -0x79cc
	ctx.r[4].s64 = ctx.r[8].s64 + -31180;
	// 83253AD4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253AD8: 4AFD93F9  bl 0x8222ced0
	ctx.lr = 0x83253ADC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253ADC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253AE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AE4: 38878628  addi r4, r7, -0x79d8
	ctx.r[4].s64 = ctx.r[7].s64 + -31192;
	// 83253AE8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253AEC: 4AFD93E5  bl 0x8222ced0
	ctx.lr = 0x83253AF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253AF0: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253AF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AF8: 38868618  addi r4, r6, -0x79e8
	ctx.r[4].s64 = ctx.r[6].s64 + -31208;
	// 83253AFC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253B00: 4AFD93D1  bl 0x8222ced0
	ctx.lr = 0x83253B04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B04: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253B08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B0C: 38848608  addi r4, r4, -0x79f8
	ctx.r[4].s64 = ctx.r[4].s64 + -31224;
	// 83253B10: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83253B14: 4AFD93BD  bl 0x8222ced0
	ctx.lr = 0x83253B18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B18: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B20: 38838600  addi r4, r3, -0x7a00
	ctx.r[4].s64 = ctx.r[3].s64 + -31232;
	// 83253B24: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253B28: 4AFD93A9  bl 0x8222ced0
	ctx.lr = 0x83253B2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253B30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B34: 388B85F0  addi r4, r11, -0x7a10
	ctx.r[4].s64 = ctx.r[11].s64 + -31248;
	// 83253B38: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253B3C: 4AFD9395  bl 0x8222ced0
	ctx.lr = 0x83253B40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B40: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83253B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B48: 388A25E4  addi r4, r10, 0x25e4
	ctx.r[4].s64 = ctx.r[10].s64 + 9700;
	// 83253B4C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253B50: 4AFD9381  bl 0x8222ced0
	ctx.lr = 0x83253B54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B54: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253B58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B5C: 388985E4  addi r4, r9, -0x7a1c
	ctx.r[4].s64 = ctx.r[9].s64 + -31260;
	// 83253B60: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83253B64: 4AFD936D  bl 0x8222ced0
	ctx.lr = 0x83253B68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B68: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253B6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B70: 388885D8  addi r4, r8, -0x7a28
	ctx.r[4].s64 = ctx.r[8].s64 + -31272;
	// 83253B74: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253B78: 4AFD9359  bl 0x8222ced0
	ctx.lr = 0x83253B7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B7C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253B80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B84: 388785CC  addi r4, r7, -0x7a34
	ctx.r[4].s64 = ctx.r[7].s64 + -31284;
	// 83253B88: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83253B8C: 4AFD9345  bl 0x8222ced0
	ctx.lr = 0x83253B90;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253B90: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83253B94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B98: 38863FD8  addi r4, r6, 0x3fd8
	ctx.r[4].s64 = ctx.r[6].s64 + 16344;
	// 83253B9C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83253BA0: 4AFD9331  bl 0x8222ced0
	ctx.lr = 0x83253BA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253BA4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83253BA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BAC: 388426C0  addi r4, r4, 0x26c0
	ctx.r[4].s64 = ctx.r[4].s64 + 9920;
	// 83253BB0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83253BB4: 4AFD931D  bl 0x8222ced0
	ctx.lr = 0x83253BB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253BB8: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253BBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BC0: 388385BC  addi r4, r3, -0x7a44
	ctx.r[4].s64 = ctx.r[3].s64 + -31300;
	// 83253BC4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83253BC8: 4AFD9309  bl 0x8222ced0
	ctx.lr = 0x83253BCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253BCC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253BD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BD4: 388B85AC  addi r4, r11, -0x7a54
	ctx.r[4].s64 = ctx.r[11].s64 + -31316;
	// 83253BD8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83253BDC: 4AFD92F5  bl 0x8222ced0
	ctx.lr = 0x83253BE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253BE0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253BE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BE8: 388A859C  addi r4, r10, -0x7a64
	ctx.r[4].s64 = ctx.r[10].s64 + -31332;
	// 83253BEC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83253BF0: 4AFD92E1  bl 0x8222ced0
	ctx.lr = 0x83253BF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253BF4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253BF8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BFC: 3889858C  addi r4, r9, -0x7a74
	ctx.r[4].s64 = ctx.r[9].s64 + -31348;
	// 83253C00: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83253C04: 4AFD92CD  bl 0x8222ced0
	ctx.lr = 0x83253C08;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253C08: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C10: 38888580  addi r4, r8, -0x7a80
	ctx.r[4].s64 = ctx.r[8].s64 + -31360;
	// 83253C14: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83253C18: 4AFD92B9  bl 0x8222ced0
	ctx.lr = 0x83253C1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253C1C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253C20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C24: 38878574  addi r4, r7, -0x7a8c
	ctx.r[4].s64 = ctx.r[7].s64 + -31372;
	// 83253C28: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83253C2C: 4AFD92A5  bl 0x8222ced0
	ctx.lr = 0x83253C30;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253C30: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253C34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C38: 38868564  addi r4, r6, -0x7a9c
	ctx.r[4].s64 = ctx.r[6].s64 + -31388;
	// 83253C3C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83253C40: 4AFD9291  bl 0x8222ced0
	ctx.lr = 0x83253C44;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253C44: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253C48: 38659E28  addi r3, r5, -0x61d8
	ctx.r[3].s64 = ctx.r[5].s64 + -25048;
	// 83253C4C: 4BA562D5  bl 0x82ca9f20
	ctx.lr = 0x83253C50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83253C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253C60: 4E800020  blr
	return;
}

pub fn sub_83253C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253C68 size=480
	// 83253C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253C78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253C7C: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253C80: 3BEB83D0  addi r31, r11, -0x7c30
	ctx.r[31].s64 = ctx.r[11].s64 + -31792;
	// 83253C84: 388A8900  addi r4, r10, -0x7700
	ctx.r[4].s64 = ctx.r[10].s64 + -30464;
	// 83253C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C90: 4AFD9241  bl 0x8222ced0
	ctx.lr = 0x83253C94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253C94: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253C98: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C9C: 388988E0  addi r4, r9, -0x7720
	ctx.r[4].s64 = ctx.r[9].s64 + -30496;
	// 83253CA0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253CA4: 4AFD922D  bl 0x8222ced0
	ctx.lr = 0x83253CA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253CA8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253CAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CB0: 388888BC  addi r4, r8, -0x7744
	ctx.r[4].s64 = ctx.r[8].s64 + -30532;
	// 83253CB4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253CB8: 4AFD9219  bl 0x8222ced0
	ctx.lr = 0x83253CBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253CBC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253CC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CC4: 38878898  addi r4, r7, -0x7768
	ctx.r[4].s64 = ctx.r[7].s64 + -30568;
	// 83253CC8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253CCC: 4AFD9205  bl 0x8222ced0
	ctx.lr = 0x83253CD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253CD0: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253CD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CD8: 38868870  addi r4, r6, -0x7790
	ctx.r[4].s64 = ctx.r[6].s64 + -30608;
	// 83253CDC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253CE0: 4AFD91F1  bl 0x8222ced0
	ctx.lr = 0x83253CE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253CE4: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253CE8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CEC: 3884884C  addi r4, r4, -0x77b4
	ctx.r[4].s64 = ctx.r[4].s64 + -30644;
	// 83253CF0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83253CF4: 4AFD91DD  bl 0x8222ced0
	ctx.lr = 0x83253CF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253CF8: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253CFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D00: 3883882C  addi r4, r3, -0x77d4
	ctx.r[4].s64 = ctx.r[3].s64 + -30676;
	// 83253D04: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253D08: 4AFD91C9  bl 0x8222ced0
	ctx.lr = 0x83253D0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D0C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253D10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D14: 388B8808  addi r4, r11, -0x77f8
	ctx.r[4].s64 = ctx.r[11].s64 + -30712;
	// 83253D18: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253D1C: 4AFD91B5  bl 0x8222ced0
	ctx.lr = 0x83253D20;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D20: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253D24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D28: 388A87E8  addi r4, r10, -0x7818
	ctx.r[4].s64 = ctx.r[10].s64 + -30744;
	// 83253D2C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253D30: 4AFD91A1  bl 0x8222ced0
	ctx.lr = 0x83253D34;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D34: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253D38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D3C: 388987C8  addi r4, r9, -0x7838
	ctx.r[4].s64 = ctx.r[9].s64 + -30776;
	// 83253D40: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83253D44: 4AFD918D  bl 0x8222ced0
	ctx.lr = 0x83253D48;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D48: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D50: 388887A8  addi r4, r8, -0x7858
	ctx.r[4].s64 = ctx.r[8].s64 + -30808;
	// 83253D54: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253D58: 4AFD9179  bl 0x8222ced0
	ctx.lr = 0x83253D5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D5C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253D60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D64: 38878784  addi r4, r7, -0x787c
	ctx.r[4].s64 = ctx.r[7].s64 + -30844;
	// 83253D68: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83253D6C: 4AFD9165  bl 0x8222ced0
	ctx.lr = 0x83253D70;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D70: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253D74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D78: 38868764  addi r4, r6, -0x789c
	ctx.r[4].s64 = ctx.r[6].s64 + -30876;
	// 83253D7C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83253D80: 4AFD9151  bl 0x8222ced0
	ctx.lr = 0x83253D84;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D84: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253D88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D8C: 38848748  addi r4, r4, -0x78b8
	ctx.r[4].s64 = ctx.r[4].s64 + -30904;
	// 83253D90: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83253D94: 4AFD913D  bl 0x8222ced0
	ctx.lr = 0x83253D98;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253D98: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DA0: 38838720  addi r4, r3, -0x78e0
	ctx.r[4].s64 = ctx.r[3].s64 + -30944;
	// 83253DA4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83253DA8: 4AFD9129  bl 0x8222ced0
	ctx.lr = 0x83253DAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253DAC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253DB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DB4: 388B86FC  addi r4, r11, -0x7904
	ctx.r[4].s64 = ctx.r[11].s64 + -30980;
	// 83253DB8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83253DBC: 4AFD9115  bl 0x8222ced0
	ctx.lr = 0x83253DC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253DC0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DC8: 388A86D8  addi r4, r10, -0x7928
	ctx.r[4].s64 = ctx.r[10].s64 + -31016;
	// 83253DCC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83253DD0: 4AFD9101  bl 0x8222ced0
	ctx.lr = 0x83253DD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253DD4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253DD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DDC: 388986B4  addi r4, r9, -0x794c
	ctx.r[4].s64 = ctx.r[9].s64 + -31052;
	// 83253DE0: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83253DE4: 4AFD90ED  bl 0x8222ced0
	ctx.lr = 0x83253DE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253DE8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253DEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DF0: 38888694  addi r4, r8, -0x796c
	ctx.r[4].s64 = ctx.r[8].s64 + -31084;
	// 83253DF4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83253DF8: 4AFD90D9  bl 0x8222ced0
	ctx.lr = 0x83253DFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253DFC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253E00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E04: 38878674  addi r4, r7, -0x798c
	ctx.r[4].s64 = ctx.r[7].s64 + -31116;
	// 83253E08: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83253E0C: 4AFD90C5  bl 0x8222ced0
	ctx.lr = 0x83253E10;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253E10: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E18: 38868650  addi r4, r6, -0x79b0
	ctx.r[4].s64 = ctx.r[6].s64 + -31152;
	// 83253E1C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83253E20: 4AFD90B1  bl 0x8222ced0
	ctx.lr = 0x83253E24;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253E24: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253E28: 38659E90  addi r3, r5, -0x6170
	ctx.r[3].s64 = ctx.r[5].s64 + -24944;
	// 83253E2C: 4BA560F5  bl 0x82ca9f20
	ctx.lr = 0x83253E30;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83253E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253E40: 4E800020  blr
	return;
}

pub fn sub_83253E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253E48 size=136
	// 83253E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253E58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253E5C: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253E60: 3BEB8424  addi r31, r11, -0x7bdc
	ctx.r[31].s64 = ctx.r[11].s64 + -31708;
	// 83253E64: 388A89B8  addi r4, r10, -0x7648
	ctx.r[4].s64 = ctx.r[10].s64 + -30280;
	// 83253E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253E6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E70: 4AFD9061  bl 0x8222ced0
	ctx.lr = 0x83253E74;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253E74: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253E78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E7C: 38898988  addi r4, r9, -0x7678
	ctx.r[4].s64 = ctx.r[9].s64 + -30328;
	// 83253E80: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253E84: 4AFD904D  bl 0x8222ced0
	ctx.lr = 0x83253E88;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253E88: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E90: 38888958  addi r4, r8, -0x76a8
	ctx.r[4].s64 = ctx.r[8].s64 + -30376;
	// 83253E94: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253E98: 4AFD9039  bl 0x8222ced0
	ctx.lr = 0x83253E9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253E9C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253EA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253EA4: 38878928  addi r4, r7, -0x76d8
	ctx.r[4].s64 = ctx.r[7].s64 + -30424;
	// 83253EA8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253EAC: 4AFD9025  bl 0x8222ced0
	ctx.lr = 0x83253EB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253EB0: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 83253EB4: 38669EF8  addi r3, r6, -0x6108
	ctx.r[3].s64 = ctx.r[6].s64 + -24840;
	// 83253EB8: 4BA56069  bl 0x82ca9f20
	ctx.lr = 0x83253EBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83253EBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253ECC: 4E800020  blr
	return;
}

pub fn sub_83253ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253ED0 size=160
	// 83253ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253ED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253EDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253EE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253EE4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253EE8: 3BEB8304  addi r31, r11, -0x7cfc
	ctx.r[31].s64 = ctx.r[11].s64 + -31996;
	// 83253EEC: 388A8AC8  addi r4, r10, -0x7538
	ctx.r[4].s64 = ctx.r[10].s64 + -30008;
	// 83253EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253EF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253EF8: 4AFD8FD9  bl 0x8222ced0
	ctx.lr = 0x83253EFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253EFC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253F00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F04: 38898A90  addi r4, r9, -0x7570
	ctx.r[4].s64 = ctx.r[9].s64 + -30064;
	// 83253F08: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253F0C: 4AFD8FC5  bl 0x8222ced0
	ctx.lr = 0x83253F10;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253F10: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253F14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F18: 38888A58  addi r4, r8, -0x75a8
	ctx.r[4].s64 = ctx.r[8].s64 + -30120;
	// 83253F1C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253F20: 4AFD8FB1  bl 0x8222ced0
	ctx.lr = 0x83253F24;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253F24: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253F28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F2C: 38878A24  addi r4, r7, -0x75dc
	ctx.r[4].s64 = ctx.r[7].s64 + -30172;
	// 83253F30: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253F34: 4AFD8F9D  bl 0x8222ced0
	ctx.lr = 0x83253F38;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253F38: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F40: 388689EC  addi r4, r6, -0x7614
	ctx.r[4].s64 = ctx.r[6].s64 + -30228;
	// 83253F44: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253F48: 4AFD8F89  bl 0x8222ced0
	ctx.lr = 0x83253F4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253F4C: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253F50: 38659F60  addi r3, r5, -0x60a0
	ctx.r[3].s64 = ctx.r[5].s64 + -24736;
	// 83253F54: 4BA55FCD  bl 0x82ca9f20
	ctx.lr = 0x83253F58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83253F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253F68: 4E800020  blr
	return;
}

pub fn sub_83253F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253F70 size=160
	// 83253F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253F78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253F7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253F80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253F84: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253F88: 3BEB83BC  addi r31, r11, -0x7c44
	ctx.r[31].s64 = ctx.r[11].s64 + -31812;
	// 83253F8C: 388A8BE8  addi r4, r10, -0x7418
	ctx.r[4].s64 = ctx.r[10].s64 + -29720;
	// 83253F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253F94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F98: 4AFD8F39  bl 0x8222ced0
	ctx.lr = 0x83253F9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253F9C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253FA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FA4: 38898BB0  addi r4, r9, -0x7450
	ctx.r[4].s64 = ctx.r[9].s64 + -29776;
	// 83253FA8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253FAC: 4AFD8F25  bl 0x8222ced0
	ctx.lr = 0x83253FB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253FB0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253FB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FB8: 38888B78  addi r4, r8, -0x7488
	ctx.r[4].s64 = ctx.r[8].s64 + -29832;
	// 83253FBC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253FC0: 4AFD8F11  bl 0x8222ced0
	ctx.lr = 0x83253FC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253FC4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253FC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FCC: 38878B40  addi r4, r7, -0x74c0
	ctx.r[4].s64 = ctx.r[7].s64 + -29888;
	// 83253FD0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253FD4: 4AFD8EFD  bl 0x8222ced0
	ctx.lr = 0x83253FD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253FD8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253FDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FE0: 38868B04  addi r4, r6, -0x74fc
	ctx.r[4].s64 = ctx.r[6].s64 + -29948;
	// 83253FE4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253FE8: 4AFD8EE9  bl 0x8222ced0
	ctx.lr = 0x83253FEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83253FEC: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253FF0: 38659FC8  addi r3, r5, -0x6038
	ctx.r[3].s64 = ctx.r[5].s64 + -24632;
	// 83253FF4: 4BA55F2D  bl 0x82ca9f20
	ctx.lr = 0x83253FF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83253FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83254008: 4E800020  blr
	return;
}

pub fn sub_83254010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254010 size=16
	// 83254010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325401C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832540D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832540D0 size=12
	// 832540D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832540D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832540D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254110 size=12
	// 83254110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254150 size=12
	// 83254150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254190 size=12
	// 83254190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832541D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832541D0 size=12
	// 832541D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832541D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832541D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254210 size=12
	// 83254210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254250 size=12
	// 83254250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254290 size=72
	// 83254290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325429C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832542A0: 386B8454  addi r3, r11, -0x7bac
	ctx.r[3].s64 = ctx.r[11].s64 + -31660;
	// 832542A4: 4B2308CD  bl 0x82484b70
	ctx.lr = 0x832542A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82484B70);
	// 832542A8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832542AC: 386AA0A8  addi r3, r10, -0x5f58
	ctx.r[3].s64 = ctx.r[10].s64 + -24408;
	// 832542B0: 4BA55C71  bl 0x82ca9f20
	ctx.lr = 0x832542B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832542B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832542B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832542BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832542C0: 4E800020  blr
	return;
}

pub fn sub_832542D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832542D8 size=12
	// 832542D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832542DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832542E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254318 size=12
	// 83254318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254358 size=12
	// 83254358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325435C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254398 size=12
	// 83254398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832543A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832543D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832543D8 size=12
	// 832543D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832543DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832543E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254418 size=12
	// 83254418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325441C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254458 size=12
	// 83254458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254498 size=12
	// 83254498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325449C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832544A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832544D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832544D8 size=12
	// 832544D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832544DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832544E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254518 size=88
	// 83254518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325451C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254524: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83254528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325452C: 386B25D4  addi r3, r11, 0x25d4
	ctx.r[3].s64 = ctx.r[11].s64 + 9684;
	// 83254530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83254534: 4AF9F825  bl 0x821f3d58
	ctx.lr = 0x83254538;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83254538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325453C: 906A8484  stw r3, -0x7b7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31612 as u32), ctx.r[3].u32 ) };
	// 83254540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325454C: 4E800020  blr
	return;
	// 83254550: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83254554: 386BA1A0  addi r3, r11, -0x5e60
	ctx.r[3].s64 = ctx.r[11].s64 + -24160;
	// 83254558: 4BA559C8  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_83254570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254570 size=12
	// 83254570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832545B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832545B0 size=12
	// 832545B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832545B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832545B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832545F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832545F0 size=12
	// 832545F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832545F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832545F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254630 size=12
	// 83254630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254670 size=12
	// 83254670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832546B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832546B0 size=12
	// 832546B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832546B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832546B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832546F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832546F0 size=12
	// 832546F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832546F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832546F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254730 size=12
	// 83254730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254770 size=12
	// 83254770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832547B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832547B0 size=12
	// 832547B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832547B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832547B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832547F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832547F0 size=12
	// 832547F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832547F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832547F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254830 size=12
	// 83254830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254870 size=12
	// 83254870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832548B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832548B0 size=12
	// 832548B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832548B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832548B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832548F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832548F0 size=12
	// 832548F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832548F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832548F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254930 size=12
	// 83254930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254970 size=12
	// 83254970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832549B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832549B0 size=12
	// 832549B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832549B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832549B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832549F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832549F0 size=12
	// 832549F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832549F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832549F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254A30 size=12
	// 83254A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254A70 size=12
	// 83254A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254AB0 size=12
	// 83254AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254AF0 size=12
	// 83254AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254B30 size=12
	// 83254B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254B70 size=12
	// 83254B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254BB0 size=12
	// 83254BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254BF0 size=12
	// 83254BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254C30 size=12
	// 83254C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254C70 size=12
	// 83254C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254CB0 size=12
	// 83254CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254CF0 size=12
	// 83254CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254D30 size=12
	// 83254D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254D70 size=12
	// 83254D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254DB0 size=12
	// 83254DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254DF0 size=12
	// 83254DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254E30 size=56
	// 83254E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254E3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254E40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83254E44: 386BFD58  addi r3, r11, -0x2a8
	ctx.r[3].s64 = ctx.r[11].s64 + -680;
	// 83254E48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83254E4C: 4AF9EF0D  bl 0x821f3d58
	ctx.lr = 0x83254E50;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83254E50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254E54: 906A8534  stw r3, -0x7acc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31436 as u32), ctx.r[3].u32 ) };
	// 83254E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254E64: 4E800020  blr
	return;
}

pub fn sub_83254E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254E68 size=144
	// 83254E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254E74: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254E78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83254E7C: 386BFD68  addi r3, r11, -0x298
	ctx.r[3].s64 = ctx.r[11].s64 + -664;
	// 83254E80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83254E84: 4AF9EED5  bl 0x821f3d58
	ctx.lr = 0x83254E88;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83254E88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254E8C: 906A8538  stw r3, -0x7ac8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31432 as u32), ctx.r[3].u32 ) };
	// 83254E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254E9C: 4E800020  blr
	return;
	// 83254EA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83254EA4: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 83254EA8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83254EAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83254EB0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83254EB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83254EB8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83254EBC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83254EC0: 4082FFE8  bne 0x83254ea8
	if !ctx.cr[0].eq {
	pc = 0x83254EA8; continue 'dispatch;
	}
	// 83254EC4: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 83254EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83254ECC: 38C7853C  addi r6, r7, -0x7ac4
	ctx.r[6].s64 = ctx.r[7].s64 + -31428;
	// 83254ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83254ED4: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83254ED8: 3865A508  addi r3, r5, -0x5af8
	ctx.r[3].s64 = ctx.r[5].s64 + -23288;
	// 83254EDC: 9166000C  stw r11, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83254EE0: 99460010  stb r10, 0x10(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 83254EE4: 4BA5503C  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
	// 83254EE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83254EEC: 386BA580  addi r3, r11, -0x5a80
	ctx.r[3].s64 = ctx.r[11].s64 + -23168;
	// 83254EF0: 4BA55030  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_83254EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254EF8 size=12
	// 83254EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254F38 size=12
	// 83254F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254F78 size=12
	// 83254F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83254FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254FB8 size=848
	// 83254FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83254FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83254FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254FCC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83254FD0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83254FD4: 3BEB8568  addi r31, r11, -0x7a98
	ctx.r[31].s64 = ctx.r[11].s64 + -31384;
	// 83254FD8: 388A0840  addi r4, r10, 0x840
	ctx.r[4].s64 = ctx.r[10].s64 + 2112;
	// 83254FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83254FE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254FE4: 4AFD7EED  bl 0x8222ced0
	ctx.lr = 0x83254FE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83254FE8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 83254FEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254FF0: 3BC90CA0  addi r30, r9, 0xca0
	ctx.r[30].s64 = ctx.r[9].s64 + 3232;
	// 83254FF4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83254FF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83254FFC: 4AFD7ED5  bl 0x8222ced0
	ctx.lr = 0x83255000;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255000: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255008: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8325500C: 4AFD7EC5  bl 0x8222ced0
	ctx.lr = 0x83255010;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255010: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83255014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255018: 38880830  addi r4, r8, 0x830
	ctx.r[4].s64 = ctx.r[8].s64 + 2096;
	// 8325501C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83255020: 4AFD7EB1  bl 0x8222ced0
	ctx.lr = 0x83255024;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255024: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83255028: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325502C: 3887081C  addi r4, r7, 0x81c
	ctx.r[4].s64 = ctx.r[7].s64 + 2076;
	// 83255030: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83255034: 4AFD7E9D  bl 0x8222ced0
	ctx.lr = 0x83255038;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255038: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325503C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255040: 3886080C  addi r4, r6, 0x80c
	ctx.r[4].s64 = ctx.r[6].s64 + 2060;
	// 83255044: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83255048: 4AFD7E89  bl 0x8222ced0
	ctx.lr = 0x8325504C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325504C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83255050: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255054: 388407FC  addi r4, r4, 0x7fc
	ctx.r[4].s64 = ctx.r[4].s64 + 2044;
	// 83255058: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8325505C: 4AFD7E75  bl 0x8222ced0
	ctx.lr = 0x83255060;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255060: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83255064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255068: 388307E8  addi r4, r3, 0x7e8
	ctx.r[4].s64 = ctx.r[3].s64 + 2024;
	// 8325506C: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83255070: 4AFD7E61  bl 0x8222ced0
	ctx.lr = 0x83255074;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255074: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255078: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325507C: 388B07D8  addi r4, r11, 0x7d8
	ctx.r[4].s64 = ctx.r[11].s64 + 2008;
	// 83255080: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83255084: 4AFD7E4D  bl 0x8222ced0
	ctx.lr = 0x83255088;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255088: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325508C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255090: 388A07C4  addi r4, r10, 0x7c4
	ctx.r[4].s64 = ctx.r[10].s64 + 1988;
	// 83255094: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83255098: 4AFD7E39  bl 0x8222ced0
	ctx.lr = 0x8325509C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325509C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832550A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550A4: 388907AC  addi r4, r9, 0x7ac
	ctx.r[4].s64 = ctx.r[9].s64 + 1964;
	// 832550A8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 832550AC: 4AFD7E25  bl 0x8222ced0
	ctx.lr = 0x832550B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832550B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832550B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550B8: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 832550BC: 4AFD7E15  bl 0x8222ced0
	ctx.lr = 0x832550C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832550C0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832550C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550C8: 38880798  addi r4, r8, 0x798
	ctx.r[4].s64 = ctx.r[8].s64 + 1944;
	// 832550CC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832550D0: 4AFD7E01  bl 0x8222ced0
	ctx.lr = 0x832550D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832550D4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832550D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550DC: 38870780  addi r4, r7, 0x780
	ctx.r[4].s64 = ctx.r[7].s64 + 1920;
	// 832550E0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832550E4: 4AFD7DED  bl 0x8222ced0
	ctx.lr = 0x832550E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832550E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832550EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550F0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832550F4: 4AFD7DDD  bl 0x8222ced0
	ctx.lr = 0x832550F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832550F8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832550FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255100: 38860768  addi r4, r6, 0x768
	ctx.r[4].s64 = ctx.r[6].s64 + 1896;
	// 83255104: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83255108: 4AFD7DC9  bl 0x8222ced0
	ctx.lr = 0x8325510C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325510C: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 83255110: 38850754  addi r4, r5, 0x754
	ctx.r[4].s64 = ctx.r[5].s64 + 1876;
	// 83255114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255118: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8325511C: 4AFD7DB5  bl 0x8222ced0
	ctx.lr = 0x83255120;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255120: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83255124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255128: 38840748  addi r4, r4, 0x748
	ctx.r[4].s64 = ctx.r[4].s64 + 1864;
	// 8325512C: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83255130: 4AFD7DA1  bl 0x8222ced0
	ctx.lr = 0x83255134;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255134: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83255138: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325513C: 38830738  addi r4, r3, 0x738
	ctx.r[4].s64 = ctx.r[3].s64 + 1848;
	// 83255140: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83255144: 4AFD7D8D  bl 0x8222ced0
	ctx.lr = 0x83255148;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255148: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8325514C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255150: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83255154: 4AFD7D7D  bl 0x8222ced0
	ctx.lr = 0x83255158;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255158: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8325515C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255160: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83255164: 4AFD7D6D  bl 0x8222ced0
	ctx.lr = 0x83255168;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255168: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8325516C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255170: 388B0728  addi r4, r11, 0x728
	ctx.r[4].s64 = ctx.r[11].s64 + 1832;
	// 83255174: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 83255178: 4AFD7D59  bl 0x8222ced0
	ctx.lr = 0x8325517C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325517C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255180: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255184: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83255188: 4AFD7D49  bl 0x8222ced0
	ctx.lr = 0x8325518C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325518C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255190: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255194: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83255198: 4AFD7D39  bl 0x8222ced0
	ctx.lr = 0x8325519C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325519C: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832551A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551A4: 388A0718  addi r4, r10, 0x718
	ctx.r[4].s64 = ctx.r[10].s64 + 1816;
	// 832551A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 832551AC: 4AFD7D25  bl 0x8222ced0
	ctx.lr = 0x832551B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832551B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551B8: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 832551BC: 4AFD7D15  bl 0x8222ced0
	ctx.lr = 0x832551C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832551C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551C8: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 832551CC: 4AFD7D05  bl 0x8222ced0
	ctx.lr = 0x832551D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832551D0: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832551D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551D8: 38890708  addi r4, r9, 0x708
	ctx.r[4].s64 = ctx.r[9].s64 + 1800;
	// 832551DC: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 832551E0: 4AFD7CF1  bl 0x8222ced0
	ctx.lr = 0x832551E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832551E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551EC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 832551F0: 4AFD7CE1  bl 0x8222ced0
	ctx.lr = 0x832551F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832551F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551FC: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 83255200: 4AFD7CD1  bl 0x8222ced0
	ctx.lr = 0x83255204;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255204: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 83255208: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325520C: 388813B0  addi r4, r8, 0x13b0
	ctx.r[4].s64 = ctx.r[8].s64 + 5040;
	// 83255210: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83255214: 4AFD7CBD  bl 0x8222ced0
	ctx.lr = 0x83255218;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255218: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 8325521C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255220: 388706F4  addi r4, r7, 0x6f4
	ctx.r[4].s64 = ctx.r[7].s64 + 1780;
	// 83255224: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83255228: 4AFD7CA9  bl 0x8222ced0
	ctx.lr = 0x8325522C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325522C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255230: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255234: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83255238: 4AFD7C99  bl 0x8222ced0
	ctx.lr = 0x8325523C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325523C: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 83255240: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255244: 388613A0  addi r4, r6, 0x13a0
	ctx.r[4].s64 = ctx.r[6].s64 + 5024;
	// 83255248: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 8325524C: 4AFD7C85  bl 0x8222ced0
	ctx.lr = 0x83255250;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255250: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 83255254: 388506E0  addi r4, r5, 0x6e0
	ctx.r[4].s64 = ctx.r[5].s64 + 1760;
	// 83255258: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325525C: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 83255260: 4AFD7C71  bl 0x8222ced0
	ctx.lr = 0x83255264;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255264: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255268: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325526C: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 83255270: 4AFD7C61  bl 0x8222ced0
	ctx.lr = 0x83255274;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255274: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83255278: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325527C: 388406CC  addi r4, r4, 0x6cc
	ctx.r[4].s64 = ctx.r[4].s64 + 1740;
	// 83255280: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83255284: 4AFD7C4D  bl 0x8222ced0
	ctx.lr = 0x83255288;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83255288: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 8325528C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255290: 388306B4  addi r4, r3, 0x6b4
	ctx.r[4].s64 = ctx.r[3].s64 + 1716;
	// 83255294: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 83255298: 4AFD7C39  bl 0x8222ced0
	ctx.lr = 0x8325529C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8325529C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832552A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552A4: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 832552A8: 4AFD7C29  bl 0x8222ced0
	ctx.lr = 0x832552AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832552AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832552B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552B4: 388B06A0  addi r4, r11, 0x6a0
	ctx.r[4].s64 = ctx.r[11].s64 + 1696;
	// 832552B8: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 832552BC: 4AFD7C15  bl 0x8222ced0
	ctx.lr = 0x832552C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832552C0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832552C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552C8: 388A0688  addi r4, r10, 0x688
	ctx.r[4].s64 = ctx.r[10].s64 + 1672;
	// 832552CC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 832552D0: 4AFD7C01  bl 0x8222ced0
	ctx.lr = 0x832552D4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832552D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832552D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552DC: 387F00A4  addi r3, r31, 0xa4
	ctx.r[3].s64 = ctx.r[31].s64 + 164;
	// 832552E0: 4AFD7BF1  bl 0x8222ced0
	ctx.lr = 0x832552E4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832552E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832552E8: 3869A5C0  addi r3, r9, -0x5a40
	ctx.r[3].s64 = ctx.r[9].s64 + -23104;
	// 832552EC: 4BA54C35  bl 0x82ca9f20
	ctx.lr = 0x832552F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832552F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832552F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832552F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832552FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83255300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83255304: 4E800020  blr
	return;
}

pub fn sub_83255308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255308 size=12
	// 83255308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325530C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255348 size=12
	// 83255348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255388 size=12
	// 83255388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832553C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832553C8 size=56
	// 832553C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832553CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832553D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832553D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832553D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832553DC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832553E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832553E4: 4AF9E975  bl 0x821f3d58
	ctx.lr = 0x832553E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832553E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832553EC: 906A861C  stw r3, -0x79e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31204 as u32), ctx.r[3].u32 ) };
	// 832553F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832553F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832553F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832553FC: 4E800020  blr
	return;
}

pub fn sub_83255400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255400 size=56
	// 83255400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325540C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255410: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255414: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83255418: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325541C: 4AF9E93D  bl 0x821f3d58
	ctx.lr = 0x83255420;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255424: 906A8620  stw r3, -0x79e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31200 as u32), ctx.r[3].u32 ) };
	// 83255428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325542C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255434: 4E800020  blr
	return;
}

pub fn sub_83255438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255438 size=56
	// 83255438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255444: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255448: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325544C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83255450: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255454: 4AF9E905  bl 0x821f3d58
	ctx.lr = 0x83255458;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325545C: 906A8624  stw r3, -0x79dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31196 as u32), ctx.r[3].u32 ) };
	// 83255460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325546C: 4E800020  blr
	return;
}

pub fn sub_83255470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255470 size=56
	// 83255470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325547C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255480: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255484: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83255488: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325548C: 4AF9E8CD  bl 0x821f3d58
	ctx.lr = 0x83255490;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255494: 906A8628  stw r3, -0x79d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31192 as u32), ctx.r[3].u32 ) };
	// 83255498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325549C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832554A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832554A4: 4E800020  blr
	return;
}

pub fn sub_832554A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832554A8 size=56
	// 832554A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832554AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832554B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832554B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832554B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832554BC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832554C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832554C4: 4AF9E895  bl 0x821f3d58
	ctx.lr = 0x832554C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832554C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832554CC: 906A862C  stw r3, -0x79d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31188 as u32), ctx.r[3].u32 ) };
	// 832554D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832554D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832554D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832554DC: 4E800020  blr
	return;
}

pub fn sub_832554E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832554E0 size=56
	// 832554E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832554E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832554E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832554EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832554F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832554F4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832554F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832554FC: 4AF9E85D  bl 0x821f3d58
	ctx.lr = 0x83255500;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255504: 906A8630  stw r3, -0x79d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31184 as u32), ctx.r[3].u32 ) };
	// 83255508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325550C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255514: 4E800020  blr
	return;
}

pub fn sub_83255518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255518 size=56
	// 83255518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325551C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255524: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325552C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83255530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255534: 4AF9E825  bl 0x821f3d58
	ctx.lr = 0x83255538;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325553C: 906A8634  stw r3, -0x79cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31180 as u32), ctx.r[3].u32 ) };
	// 83255540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325554C: 4E800020  blr
	return;
}

pub fn sub_83255550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255550 size=56
	// 83255550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325555C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255560: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255564: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83255568: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325556C: 4AF9E7ED  bl 0x821f3d58
	ctx.lr = 0x83255570;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255574: 906A8638  stw r3, -0x79c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31176 as u32), ctx.r[3].u32 ) };
	// 83255578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325557C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255584: 4E800020  blr
	return;
}

pub fn sub_83255588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255588 size=56
	// 83255588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255594: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255598: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325559C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832555A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832555A4: 4AF9E7B5  bl 0x821f3d58
	ctx.lr = 0x832555A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832555A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832555AC: 906A863C  stw r3, -0x79c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31172 as u32), ctx.r[3].u32 ) };
	// 832555B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832555B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832555B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832555BC: 4E800020  blr
	return;
}

pub fn sub_832555C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832555C0 size=56
	// 832555C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832555C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832555C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832555CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832555D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832555D4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832555D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832555DC: 4AF9E77D  bl 0x821f3d58
	ctx.lr = 0x832555E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832555E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832555E4: 906A8640  stw r3, -0x79c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31168 as u32), ctx.r[3].u32 ) };
	// 832555E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832555EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832555F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832555F4: 4E800020  blr
	return;
}

pub fn sub_832555F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832555F8 size=56
	// 832555F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832555FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255608: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325560C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83255610: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255614: 4AF9E745  bl 0x821f3d58
	ctx.lr = 0x83255618;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325561C: 906A8644  stw r3, -0x79bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31164 as u32), ctx.r[3].u32 ) };
	// 83255620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325562C: 4E800020  blr
	return;
}

pub fn sub_83255630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255630 size=56
	// 83255630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325563C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255640: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255644: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83255648: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325564C: 4AF9E70D  bl 0x821f3d58
	ctx.lr = 0x83255650;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255654: 906A8648  stw r3, -0x79b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31160 as u32), ctx.r[3].u32 ) };
	// 83255658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325565C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255664: 4E800020  blr
	return;
}

pub fn sub_83255668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255668 size=56
	// 83255668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255674: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255678: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325567C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83255680: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255684: 4AF9E6D5  bl 0x821f3d58
	ctx.lr = 0x83255688;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325568C: 906A864C  stw r3, -0x79b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31156 as u32), ctx.r[3].u32 ) };
	// 83255690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325569C: 4E800020  blr
	return;
}

pub fn sub_832556A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832556A0 size=56
	// 832556A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832556A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832556A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832556AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832556B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832556B4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832556B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832556BC: 4AF9E69D  bl 0x821f3d58
	ctx.lr = 0x832556C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832556C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832556C4: 906A8650  stw r3, -0x79b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31152 as u32), ctx.r[3].u32 ) };
	// 832556C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832556CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832556D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832556D4: 4E800020  blr
	return;
}

pub fn sub_832556D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832556D8 size=56
	// 832556D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832556DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832556E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832556E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832556E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832556EC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832556F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832556F4: 4AF9E665  bl 0x821f3d58
	ctx.lr = 0x832556F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832556F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832556FC: 906A8654  stw r3, -0x79ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31148 as u32), ctx.r[3].u32 ) };
	// 83255700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325570C: 4E800020  blr
	return;
}

pub fn sub_83255710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255710 size=56
	// 83255710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325571C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255724: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83255728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325572C: 4AF9E62D  bl 0x821f3d58
	ctx.lr = 0x83255730;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255734: 906A8658  stw r3, -0x79a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31144 as u32), ctx.r[3].u32 ) };
	// 83255738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325573C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255744: 4E800020  blr
	return;
}

pub fn sub_83255748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255748 size=56
	// 83255748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325574C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325575C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83255760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255764: 4AF9E5F5  bl 0x821f3d58
	ctx.lr = 0x83255768;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325576C: 906A865C  stw r3, -0x79a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31140 as u32), ctx.r[3].u32 ) };
	// 83255770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325577C: 4E800020  blr
	return;
}

pub fn sub_83255780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255780 size=56
	// 83255780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325578C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255794: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83255798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325579C: 4AF9E5BD  bl 0x821f3d58
	ctx.lr = 0x832557A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832557A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832557A4: 906A8660  stw r3, -0x79a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31136 as u32), ctx.r[3].u32 ) };
	// 832557A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832557AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832557B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832557B4: 4E800020  blr
	return;
}

pub fn sub_832557B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832557B8 size=56
	// 832557B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832557BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832557C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832557C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832557C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832557CC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832557D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832557D4: 4AF9E585  bl 0x821f3d58
	ctx.lr = 0x832557D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832557D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832557DC: 906A8664  stw r3, -0x799c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31132 as u32), ctx.r[3].u32 ) };
	// 832557E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832557E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832557E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832557EC: 4E800020  blr
	return;
}

pub fn sub_832557F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832557F0 size=56
	// 832557F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832557F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832557F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832557FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255804: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83255808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325580C: 4AF9E54D  bl 0x821f3d58
	ctx.lr = 0x83255810;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255814: 906A8668  stw r3, -0x7998(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31128 as u32), ctx.r[3].u32 ) };
	// 83255818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325581C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255824: 4E800020  blr
	return;
}

pub fn sub_83255828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255828 size=56
	// 83255828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325582C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325583C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83255840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255844: 4AF9E515  bl 0x821f3d58
	ctx.lr = 0x83255848;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325584C: 906A866C  stw r3, -0x7994(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31124 as u32), ctx.r[3].u32 ) };
	// 83255850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325585C: 4E800020  blr
	return;
}

pub fn sub_83255860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255860 size=12
	// 83255860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832558A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832558A0 size=12
	// 832558A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832558A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832558A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832558E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832558E0 size=12
	// 832558E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832558E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832558E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255920 size=12
	// 83255920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255960 size=12
	// 83255960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832559A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832559A0 size=12
	// 832559A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832559A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832559A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832559E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832559E0 size=12
	// 832559E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832559E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832559E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255A20 size=12
	// 83255A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255A60 size=12
	// 83255A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255AA0 size=12
	// 83255AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255AE0 size=12
	// 83255AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255AE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255B20 size=12
	// 83255B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255B60 size=12
	// 83255B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255BA0 size=12
	// 83255BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255BE0 size=12
	// 83255BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255C20 size=12
	// 83255C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255C60 size=12
	// 83255C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255CA0 size=12
	// 83255CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255CE0 size=12
	// 83255CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255D20 size=12
	// 83255D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255D60 size=56
	// 83255D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255D6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83255D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255D74: 386B25D4  addi r3, r11, 0x25d4
	ctx.r[3].s64 = ctx.r[11].s64 + 9684;
	// 83255D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255D7C: 4AF9DFDD  bl 0x821f3d58
	ctx.lr = 0x83255D80;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255D84: 906A86C0  stw r3, -0x7940(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31040 as u32), ctx.r[3].u32 ) };
	// 83255D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255D94: 4E800020  blr
	return;
}

pub fn sub_83255D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255D98 size=12
	// 83255D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255DD8 size=12
	// 83255DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255E18 size=12
	// 83255E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83255E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255E58 size=56
	// 83255E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255E64: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255E68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255E6C: 386B0E58  addi r3, r11, 0xe58
	ctx.r[3].s64 = ctx.r[11].s64 + 3672;
	// 83255E70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255E74: 4AF9DEE5  bl 0x821f3d58
	ctx.lr = 0x83255E78;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255E7C: 906A86D0  stw r3, -0x7930(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31024 as u32), ctx.r[3].u32 ) };
	// 83255E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255E8C: 4E800020  blr
	return;
}

pub fn sub_83255E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255E90 size=56
	// 83255E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255E9C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255EA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255EA4: 386B0E68  addi r3, r11, 0xe68
	ctx.r[3].s64 = ctx.r[11].s64 + 3688;
	// 83255EA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255EAC: 4AF9DEAD  bl 0x821f3d58
	ctx.lr = 0x83255EB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255EB4: 906A86D4  stw r3, -0x792c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31020 as u32), ctx.r[3].u32 ) };
	// 83255EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255EC4: 4E800020  blr
	return;
}

pub fn sub_83255EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255EC8 size=56
	// 83255EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255ED4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255ED8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255EDC: 386B0E74  addi r3, r11, 0xe74
	ctx.r[3].s64 = ctx.r[11].s64 + 3700;
	// 83255EE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255EE4: 4AF9DE75  bl 0x821f3d58
	ctx.lr = 0x83255EE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255EE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255EEC: 906A86D8  stw r3, -0x7928(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31016 as u32), ctx.r[3].u32 ) };
	// 83255EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255EFC: 4E800020  blr
	return;
}

pub fn sub_83255F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255F00 size=56
	// 83255F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255F0C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255F10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255F14: 386B0E84  addi r3, r11, 0xe84
	ctx.r[3].s64 = ctx.r[11].s64 + 3716;
	// 83255F18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255F1C: 4AF9DE3D  bl 0x821f3d58
	ctx.lr = 0x83255F20;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255F24: 906A86DC  stw r3, -0x7924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31012 as u32), ctx.r[3].u32 ) };
	// 83255F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255F34: 4E800020  blr
	return;
}

pub fn sub_83255F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255F38 size=56
	// 83255F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255F44: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255F48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255F4C: 386B0E94  addi r3, r11, 0xe94
	ctx.r[3].s64 = ctx.r[11].s64 + 3732;
	// 83255F50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255F54: 4AF9DE05  bl 0x821f3d58
	ctx.lr = 0x83255F58;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255F58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255F5C: 906A86E0  stw r3, -0x7920(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31008 as u32), ctx.r[3].u32 ) };
	// 83255F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255F6C: 4E800020  blr
	return;
}

pub fn sub_83255F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255F70 size=56
	// 83255F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255F7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255F80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255F84: 386B0EA4  addi r3, r11, 0xea4
	ctx.r[3].s64 = ctx.r[11].s64 + 3748;
	// 83255F88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255F8C: 4AF9DDCD  bl 0x821f3d58
	ctx.lr = 0x83255F90;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255F90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255F94: 906A86E4  stw r3, -0x791c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31004 as u32), ctx.r[3].u32 ) };
	// 83255F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255FA4: 4E800020  blr
	return;
}

pub fn sub_83255FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255FA8 size=56
	// 83255FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255FB4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255FB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255FBC: 386B0EB0  addi r3, r11, 0xeb0
	ctx.r[3].s64 = ctx.r[11].s64 + 3760;
	// 83255FC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255FC4: 4AF9DD95  bl 0x821f3d58
	ctx.lr = 0x83255FC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83255FC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255FCC: 906A86E8  stw r3, -0x7918(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31000 as u32), ctx.r[3].u32 ) };
	// 83255FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255FDC: 4E800020  blr
	return;
}

pub fn sub_83255FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255FE0 size=12
	// 83255FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256020 size=12
	// 83256020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256060 size=12
	// 83256060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832560A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832560A0 size=12
	// 832560A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832560A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832560A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832560E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832560E0 size=12
	// 832560E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832560E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832560E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256120 size=12
	// 83256120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256160 size=12
	// 83256160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256268 size=12
	// 83256268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832562A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832562A8 size=12
	// 832562A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832562AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832562B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832562E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832562E8 size=88
	// 832562E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832562EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832562F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832562F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832562F8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832562FC: 386BA858  addi r3, r11, -0x57a8
	ctx.r[3].s64 = ctx.r[11].s64 + -22440;
	// 83256300: 4BA53C21  bl 0x82ca9f20
	ctx.lr = 0x83256304;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83256304: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 83256308: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 8325630C: 3BEAC4C8  addi r31, r10, -0x3b38
	ctx.r[31].s64 = ctx.r[10].s64 + -15160;
	// 83256310: 38891030  addi r4, r9, 0x1030
	ctx.r[4].s64 = ctx.r[9].s64 + 4144;
	// 83256314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83256318: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325631C: 4AFD6BB5  bl 0x8222ced0
	ctx.lr = 0x83256320;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83256320: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83256324: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83256328: 91688730  stw r11, -0x78d0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-30928 as u32), ctx.r[11].u32 ) };
	// 8325632C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325633C: 4E800020  blr
	return;
}

pub fn sub_83256340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256340 size=88
	// 83256340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325634C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256350: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256354: 386BA868  addi r3, r11, -0x5798
	ctx.r[3].s64 = ctx.r[11].s64 + -22424;
	// 83256358: 4BA53BC9  bl 0x82ca9f20
	ctx.lr = 0x8325635C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325635C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 83256360: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83256364: 3BEAC4CC  addi r31, r10, -0x3b34
	ctx.r[31].s64 = ctx.r[10].s64 + -15156;
	// 83256368: 38891030  addi r4, r9, 0x1030
	ctx.r[4].s64 = ctx.r[9].s64 + 4144;
	// 8325636C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83256370: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256374: 4AFD6B5D  bl 0x8222ced0
	ctx.lr = 0x83256378;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83256378: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8325637C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83256380: 91688734  stw r11, -0x78cc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-30924 as u32), ctx.r[11].u32 ) };
	// 83256384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83256394: 4E800020  blr
	return;
}

pub fn sub_83256398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256398 size=88
	// 83256398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832563A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832563A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832563A8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832563AC: 386BA878  addi r3, r11, -0x5788
	ctx.r[3].s64 = ctx.r[11].s64 + -22408;
	// 832563B0: 4BA53B71  bl 0x82ca9f20
	ctx.lr = 0x832563B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832563B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832563B8: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832563BC: 3BEAC4D0  addi r31, r10, -0x3b30
	ctx.r[31].s64 = ctx.r[10].s64 + -15152;
	// 832563C0: 3889104C  addi r4, r9, 0x104c
	ctx.r[4].s64 = ctx.r[9].s64 + 4172;
	// 832563C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832563C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832563CC: 4AFD6B05  bl 0x8222ced0
	ctx.lr = 0x832563D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832563D0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832563D4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 832563D8: 91688738  stw r11, -0x78c8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-30920 as u32), ctx.r[11].u32 ) };
	// 832563DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832563E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832563E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832563E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832563EC: 4E800020  blr
	return;
}

pub fn sub_832563F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832563F0 size=56
	// 832563F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832563F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832563F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832563FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256404: 386B1068  addi r3, r11, 0x1068
	ctx.r[3].s64 = ctx.r[11].s64 + 4200;
	// 83256408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325640C: 4AF9D94D  bl 0x821f3d58
	ctx.lr = 0x83256410;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256414: 906A873C  stw r3, -0x78c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30916 as u32), ctx.r[3].u32 ) };
	// 83256418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256424: 4E800020  blr
	return;
}

pub fn sub_83256428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256428 size=56
	// 83256428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256434: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83256438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325643C: 386BABCC  addi r3, r11, -0x5434
	ctx.r[3].s64 = ctx.r[11].s64 + -21556;
	// 83256440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256444: 4AF9D915  bl 0x821f3d58
	ctx.lr = 0x83256448;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325644C: 906A8740  stw r3, -0x78c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30912 as u32), ctx.r[3].u32 ) };
	// 83256450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325645C: 4E800020  blr
	return;
}

pub fn sub_83256460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256460 size=56
	// 83256460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325646C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83256470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256474: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 83256478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325647C: 4AF9D8DD  bl 0x821f3d58
	ctx.lr = 0x83256480;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256484: 906A8744  stw r3, -0x78bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30908 as u32), ctx.r[3].u32 ) };
	// 83256488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325648C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256494: 4E800020  blr
	return;
}

pub fn sub_83256498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256498 size=56
	// 83256498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832564A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832564A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832564A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832564AC: 386BABD8  addi r3, r11, -0x5428
	ctx.r[3].s64 = ctx.r[11].s64 + -21544;
	// 832564B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832564B4: 4AF9D8A5  bl 0x821f3d58
	ctx.lr = 0x832564B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832564B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832564BC: 906A8748  stw r3, -0x78b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30904 as u32), ctx.r[3].u32 ) };
	// 832564C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832564C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832564C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832564CC: 4E800020  blr
	return;
}

pub fn sub_832564D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832564D0 size=56
	// 832564D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832564D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832564D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832564DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832564E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832564E4: 386B1074  addi r3, r11, 0x1074
	ctx.r[3].s64 = ctx.r[11].s64 + 4212;
	// 832564E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832564EC: 4AF9D86D  bl 0x821f3d58
	ctx.lr = 0x832564F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832564F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832564F4: 906A874C  stw r3, -0x78b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30900 as u32), ctx.r[3].u32 ) };
	// 832564F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832564FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256504: 4E800020  blr
	return;
}

pub fn sub_83256508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256508 size=56
	// 83256508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256514: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325651C: 386B1080  addi r3, r11, 0x1080
	ctx.r[3].s64 = ctx.r[11].s64 + 4224;
	// 83256520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256524: 4AF9D835  bl 0x821f3d58
	ctx.lr = 0x83256528;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325652C: 906A8750  stw r3, -0x78b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30896 as u32), ctx.r[3].u32 ) };
	// 83256530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325653C: 4E800020  blr
	return;
}

pub fn sub_83256540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256540 size=56
	// 83256540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325654C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256550: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256554: 386B108C  addi r3, r11, 0x108c
	ctx.r[3].s64 = ctx.r[11].s64 + 4236;
	// 83256558: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325655C: 4AF9D7FD  bl 0x821f3d58
	ctx.lr = 0x83256560;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256564: 906A8754  stw r3, -0x78ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30892 as u32), ctx.r[3].u32 ) };
	// 83256568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325656C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256574: 4E800020  blr
	return;
}

pub fn sub_83256578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256578 size=56
	// 83256578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325657C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256584: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256588: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325658C: 386B109C  addi r3, r11, 0x109c
	ctx.r[3].s64 = ctx.r[11].s64 + 4252;
	// 83256590: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256594: 4AF9D7C5  bl 0x821f3d58
	ctx.lr = 0x83256598;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256598: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325659C: 906A8758  stw r3, -0x78a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30888 as u32), ctx.r[3].u32 ) };
	// 832565A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832565A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832565A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832565AC: 4E800020  blr
	return;
}

pub fn sub_832565B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832565B0 size=56
	// 832565B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832565B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832565B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832565BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832565C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832565C4: 386B10AC  addi r3, r11, 0x10ac
	ctx.r[3].s64 = ctx.r[11].s64 + 4268;
	// 832565C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832565CC: 4AF9D78D  bl 0x821f3d58
	ctx.lr = 0x832565D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 832565D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832565D4: 906A875C  stw r3, -0x78a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30884 as u32), ctx.r[3].u32 ) };
	// 832565D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832565DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832565E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832565E4: 4E800020  blr
	return;
}

pub fn sub_832565E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832565E8 size=56
	// 832565E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832565EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832565F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832565F4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832565F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832565FC: 386B10B8  addi r3, r11, 0x10b8
	ctx.r[3].s64 = ctx.r[11].s64 + 4280;
	// 83256600: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256604: 4AF9D755  bl 0x821f3d58
	ctx.lr = 0x83256608;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325660C: 906A8760  stw r3, -0x78a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30880 as u32), ctx.r[3].u32 ) };
	// 83256610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325661C: 4E800020  blr
	return;
}

pub fn sub_83256620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256620 size=56
	// 83256620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325662C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256630: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256634: 386B10D0  addi r3, r11, 0x10d0
	ctx.r[3].s64 = ctx.r[11].s64 + 4304;
	// 83256638: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325663C: 4AF9D71D  bl 0x821f3d58
	ctx.lr = 0x83256640;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256644: 906A8764  stw r3, -0x789c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30876 as u32), ctx.r[3].u32 ) };
	// 83256648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325664C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256654: 4E800020  blr
	return;
}

pub fn sub_83256658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256658 size=56
	// 83256658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256664: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256668: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325666C: 386B10E4  addi r3, r11, 0x10e4
	ctx.r[3].s64 = ctx.r[11].s64 + 4324;
	// 83256670: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256674: 4AF9D6E5  bl 0x821f3d58
	ctx.lr = 0x83256678;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 83256678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325667C: 906A8768  stw r3, -0x7898(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30872 as u32), ctx.r[3].u32 ) };
	// 83256680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325668C: 4E800020  blr
	return;
}

pub fn sub_83256690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256690 size=12
	// 83256690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832566D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832566D0 size=12
	// 832566D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832566D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832566D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256710 size=12
	// 83256710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256750 size=12
	// 83256750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256790 size=12
	// 83256790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832567D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832567D0 size=12
	// 832567D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832567D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832567D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256810 size=56
	// 83256810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325681C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256820: 386B8784  addi r3, r11, -0x787c
	ctx.r[3].s64 = ctx.r[11].s64 + -30844;
	// 83256824: 4B7B3D8D  bl 0x82a0a5b0
	ctx.lr = 0x83256828;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A0A5B0);
	// 83256828: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325682C: 386AA8E8  addi r3, r10, -0x5718
	ctx.r[3].s64 = ctx.r[10].s64 + -22296;
	// 83256830: 4BA536F1  bl 0x82ca9f20
	ctx.lr = 0x83256834;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83256834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256840: 4E800020  blr
	return;
}

pub fn sub_83256848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256848 size=56
	// 83256848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256854: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256858: 386B8790  addi r3, r11, -0x7870
	ctx.r[3].s64 = ctx.r[11].s64 + -30832;
	// 8325685C: 4B7B3D55  bl 0x82a0a5b0
	ctx.lr = 0x83256860;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A0A5B0);
	// 83256860: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256864: 386AA8F8  addi r3, r10, -0x5708
	ctx.r[3].s64 = ctx.r[10].s64 + -22280;
	// 83256868: 4BA536B9  bl 0x82ca9f20
	ctx.lr = 0x8325686C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8325686C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256878: 4E800020  blr
	return;
}

pub fn sub_83256880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256880 size=200
	// 83256880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325688C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256890: 386B879C  addi r3, r11, -0x7864
	ctx.r[3].s64 = ctx.r[11].s64 + -30820;
	// 83256894: 4B7B3D1D  bl 0x82a0a5b0
	ctx.lr = 0x83256898;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A0A5B0);
	// 83256898: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325689C: 386AA908  addi r3, r10, -0x56f8
	ctx.r[3].s64 = ctx.r[10].s64 + -22264;
	// 832568A0: 4BA53681  bl 0x82ca9f20
	ctx.lr = 0x832568A4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832568A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832568A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832568AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832568B0: 4E800020  blr
	return;
}

pub fn sub_83256948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256948 size=12
	// 83256948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325694C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832569B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832569B8 size=12
	// 832569B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832569BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832569C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832569F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832569F8 size=12
	// 832569F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832569FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256A38 size=12
	// 83256A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256A78 size=12
	// 83256A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256AB8 size=12
	// 83256AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256AF8 size=12
	// 83256AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83256BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256BB8 size=104
	// 83256BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256BC4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256BC8: 386B9F34  addi r3, r11, -0x60cc
	ctx.r[3].s64 = ctx.r[11].s64 + -24780;
	// 83256BCC: 4AF3D26D  bl 0x82193e38
	ctx.lr = 0x83256BD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82193E38);
	// 83256BD0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256BD4: 386AAA68  addi r3, r10, -0x5598
	ctx.r[3].s64 = ctx.r[10].s64 + -21912;
	// 83256BD8: 4BA53349  bl 0x82ca9f20
	ctx.lr = 0x83256BDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83256BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256BE8: 4E800020  blr
	return;
}

pub fn sub_83256C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256C20 size=12
	// 83256C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

