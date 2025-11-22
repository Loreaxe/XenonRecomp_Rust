pub fn sub_83290288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290288 size=12
	// 83290288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329028C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832902C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832902C8 size=12
	// 832902C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832902CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832902D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290308 size=12
	// 83290308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290348 size=12
	// 83290348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290388 size=12
	// 83290388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832903E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832903E8 size=12
	// 832903E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832903EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832903F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290448 size=12
	// 83290448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329044C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832904A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832904A8 size=12
	// 832904A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832904AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832904B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290508 size=12
	// 83290508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290568 size=12
	// 83290568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832905F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832905F8 size=12
	// 832905F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832905FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290638 size=12
	// 83290638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329063C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832906A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832906A8 size=56
	// 832906A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832906AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832906B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832906B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832906B8: 386B573C  addi r3, r11, 0x573c
	ctx.r[3].s64 = ctx.r[11].s64 + 22332;
	// 832906BC: 480295C9  bl 0x832b9c84
	ctx.lr = 0x832906C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832906C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832906C4: 386A5DB8  addi r3, r10, 0x5db8
	ctx.r[3].s64 = ctx.r[10].s64 + 23992;
	// 832906C8: 4BA19859  bl 0x82ca9f20
	ctx.lr = 0x832906CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832906CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832906D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832906D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832906D8: 4E800020  blr
	return;
}

pub fn sub_832906E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832906E0 size=12
	// 832906E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832906E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832906E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290720 size=12
	// 83290720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290770 size=12
	// 83290770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832907B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832907B0 size=12
	// 832907B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832907B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832907B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832907F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832907F0 size=12
	// 832907F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832907F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832907F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290830 size=12
	// 83290830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290890 size=12
	// 83290890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832908D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832908D0 size=12
	// 832908D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832908D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832908D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290910 size=12
	// 83290910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290950 size=12
	// 83290950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290990 size=12
	// 83290990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832909D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832909D0 size=12
	// 832909D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832909D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832909D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290A20 size=184
	// 83290A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83290A2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83290A30: 386B57C0  addi r3, r11, 0x57c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22464;
	// 83290A34: 4B82ADF5  bl 0x82abb828
	ctx.lr = 0x83290A38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82ABB828);
	// 83290A38: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83290A3C: 386A6098  addi r3, r10, 0x6098
	ctx.r[3].s64 = ctx.r[10].s64 + 24728;
	// 83290A40: 4BA194E1  bl 0x82ca9f20
	ctx.lr = 0x83290A44;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83290A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83290A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83290A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83290A50: 4E800020  blr
	return;
}

pub fn sub_83290AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290AD8 size=12
	// 83290AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B18 size=12
	// 83290B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B58 size=12
	// 83290B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290B98 size=12
	// 83290B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290BD8 size=12
	// 83290BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290C18 size=12
	// 83290C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290CC0 size=12
	// 83290CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D00 size=12
	// 83290D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D40 size=12
	// 83290D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83290D80 size=12
	// 83290D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83290D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83290DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83290DC0 size=8
	// 83290DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83290DC4: 4BA18621  bl 0x82ca93e4
	ctx.lr = 0x83290DC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
}

pub fn sub_832911A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832911A0 size=12
	// 832911A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832911A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832911A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832911E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832911E0 size=12
	// 832911E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832911E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832911E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291220 size=12
	// 83291220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291260 size=12
	// 83291260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832912A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832912A0 size=12
	// 832912A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832912A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832912A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832912E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832912E0 size=12
	// 832912E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832912E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832912E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291320 size=12
	// 83291320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291360 size=12
	// 83291360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832913A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832913A0 size=12
	// 832913A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832913A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832913A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832913E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832913E0 size=12
	// 832913E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832913E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832913E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291420 size=12
	// 83291420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291460 size=12
	// 83291460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832914A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832914A0 size=12
	// 832914A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832914A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832914A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832914E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832914E0 size=12
	// 832914E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832914E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832914E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291520 size=12
	// 83291520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291560 size=12
	// 83291560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832915A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832915A0 size=12
	// 832915A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832915A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832915A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832915E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832915E0 size=12
	// 832915E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832915E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832915E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291700 size=12
	// 83291700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291740 size=12
	// 83291740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291780 size=12
	// 83291780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832917C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832917C0 size=12
	// 832917C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832917C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832917C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291800 size=12
	// 83291800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291840 size=12
	// 83291840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291880 size=12
	// 83291880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832918C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832918C0 size=12
	// 832918C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832918C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832918C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291900 size=12
	// 83291900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291940 size=12
	// 83291940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291980 size=12
	// 83291980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832919C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832919C0 size=12
	// 832919C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832919C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832919C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A00 size=12
	// 83291A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A40 size=12
	// 83291A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291A80 size=12
	// 83291A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291AC0 size=12
	// 83291AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B00 size=12
	// 83291B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B40 size=12
	// 83291B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291B90 size=12
	// 83291B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291BD0 size=12
	// 83291BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C10 size=12
	// 83291C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C50 size=12
	// 83291C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291C90 size=12
	// 83291C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291CD0 size=12
	// 83291CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291D10 size=12
	// 83291D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291D50 size=12
	// 83291D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291DA0 size=12
	// 83291DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83291DE0 size=12
	// 83291DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83291DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83291E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83291E20 size=8
	// 83291E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83291E24: 4BA175C1  bl 0x82ca93e4
	ctx.lr = 0x83291E28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E4);
}

pub fn sub_83292140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83292140 size=456
	// 83292140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292144: 4BA172A5  bl 0x82ca93e8
	ctx.lr = 0x83292148;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA93E8);
	// 83292148: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8329214C: 3901FF44  addi r8, r1, -0xbc
	ctx.r[8].s64 = ctx.r[1].s64 + -188;
	// 83292150: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83292154: 3941FF40  addi r10, r1, -0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + -192;
	// 83292158: 38E1FF48  addi r7, r1, -0xb8
	ctx.r[7].s64 = ctx.r[1].s64 + -184;
	// 8329215C: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83292160: 38C1FF4C  addi r6, r1, -0xb4
	ctx.r[6].s64 = ctx.r[1].s64 + -180;
	// 83292164: D1A1FF40  stfs f13, -0xc0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-192 as u32), tmp.u32 ) };
	// 83292168: 38A1FF50  addi r5, r1, -0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + -176;
	// 8329216C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83292170: 3881FF54  addi r4, r1, -0xac
	ctx.r[4].s64 = ctx.r[1].s64 + -172;
	// 83292174: D001FF44  stfs f0, -0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-188 as u32), tmp.u32 ) };
	// 83292178: 3861FF58  addi r3, r1, -0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + -168;
	// 8329217C: D001FF48  stfs f0, -0xb8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-184 as u32), tmp.u32 ) };
	// 83292180: 3961FF5C  addi r11, r1, -0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + -164;
	// 83292184: D001FF4C  stfs f0, -0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-180 as u32), tmp.u32 ) };
	// 83292188: 3921FF60  addi r9, r1, -0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + -160;
	// 8329218C: D1A1FF50  stfs f13, -0xb0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 83292190: 3BE1FF64  addi r31, r1, -0x9c
	ctx.r[31].s64 = ctx.r[1].s64 + -156;
	// 83292194: D1A1FF54  stfs f13, -0xac(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 83292198: 3BC1FF68  addi r30, r1, -0x98
	ctx.r[30].s64 = ctx.r[1].s64 + -152;
	// 8329219C: D001FF58  stfs f0, -0xa8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 832921A0: 3BA1FF6C  addi r29, r1, -0x94
	ctx.r[29].s64 = ctx.r[1].s64 + -148;
	// 832921A4: D001FF5C  stfs f0, -0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 832921A8: 3B81FF70  addi r28, r1, -0x90
	ctx.r[28].s64 = ctx.r[1].s64 + -144;
	// 832921AC: D001FF60  stfs f0, -0xa0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 832921B0: 3B61FF74  addi r27, r1, -0x8c
	ctx.r[27].s64 = ctx.r[1].s64 + -140;
	// 832921B4: D001FF64  stfs f0, -0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-156 as u32), tmp.u32 ) };
	// 832921B8: 3B41FF78  addi r26, r1, -0x88
	ctx.r[26].s64 = ctx.r[1].s64 + -136;
	// 832921BC: D001FF68  stfs f0, -0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), tmp.u32 ) };
	// 832921C0: 3B21FF7C  addi r25, r1, -0x84
	ctx.r[25].s64 = ctx.r[1].s64 + -132;
	// 832921C4: D1A1FF6C  stfs f13, -0x94(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-148 as u32), tmp.u32 ) };
	// 832921C8: 3B01FF80  addi r24, r1, -0x80
	ctx.r[24].s64 = ctx.r[1].s64 + -128;
	// 832921CC: D001FF70  stfs f0, -0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), tmp.u32 ) };
	// 832921D0: 3AE1FF84  addi r23, r1, -0x7c
	ctx.r[23].s64 = ctx.r[1].s64 + -124;
	// 832921D4: D001FF74  stfs f0, -0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-140 as u32), tmp.u32 ) };
	// 832921D8: 3AC1FF88  addi r22, r1, -0x78
	ctx.r[22].s64 = ctx.r[1].s64 + -120;
	// 832921DC: D001FF78  stfs f0, -0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), tmp.u32 ) };
	// 832921E0: 3AA1FF8C  addi r21, r1, -0x74
	ctx.r[21].s64 = ctx.r[1].s64 + -116;
	// 832921E4: D001FF7C  stfs f0, -0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-132 as u32), tmp.u32 ) };
	// 832921E8: 3E808332  lis r20, -0x7cce
	ctx.r[20].s64 = -2093875200;
	// 832921EC: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 832921F0: D1A1FF84  stfs f13, -0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-124 as u32), tmp.u32 ) };
	// 832921F4: 3A94AFB0  addi r20, r20, -0x5050
	ctx.r[20].s64 = ctx.r[20].s64 + -20560;
	// 832921F8: D001FF88  stfs f0, -0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), tmp.u32 ) };
	// 832921FC: D1A1FF8C  stfs f13, -0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-116 as u32), tmp.u32 ) };
}

pub fn sub_83292308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292308 size=12
	// 83292308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329230C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292348 size=12
	// 83292348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292388 size=12
	// 83292388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329238C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832923C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832923C8 size=12
	// 832923C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832923CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832923D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292408 size=12
	// 83292408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292448 size=12
	// 83292448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329244C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292488 size=12
	// 83292488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329248C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832924C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832924C8 size=12
	// 832924C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832924CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832924D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292508 size=12
	// 83292508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292548 size=12
	// 83292548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329254C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832925F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832925F0 size=12
	// 832925F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832925F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832925F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292630 size=12
	// 83292630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292698 size=12
	// 83292698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329269C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832926A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_832926D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832926D8 size=12
	// 832926D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832926DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832926E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83292718 size=464
	// 83292718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329271C: 4BA16CE9  bl 0x82ca9404
	ctx.lr = 0x83292720;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9404);
	// 83292720: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83292724: 38E1FFA8  addi r7, r1, -0x58
	ctx.r[7].s64 = ctx.r[1].s64 + -88;
	// 83292728: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 8329272C: 38C1FFAC  addi r6, r1, -0x54
	ctx.r[6].s64 = ctx.r[1].s64 + -84;
	// 83292730: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 83292734: C1AB9484  lfs f13, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83292738: 3901FFA4  addi r8, r1, -0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + -92;
	// 8329273C: D1A1FFAC  stfs f13, -0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-84 as u32), tmp.u32 ) };
	// 83292740: 3BC1FFC0  addi r30, r1, -0x40
	ctx.r[30].s64 = ctx.r[1].s64 + -64;
	// 83292744: C009000C  lfs f0, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83292748: 3BA1FFC4  addi r29, r1, -0x3c
	ctx.r[29].s64 = ctx.r[1].s64 + -60;
	// 8329274C: D001FFA8  stfs f0, -0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 83292750: 38A1FFB0  addi r5, r1, -0x50
	ctx.r[5].s64 = ctx.r[1].s64 + -80;
}

pub fn sub_832928E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832928E8 size=104
	// 832928E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832928EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832928F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832928F4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832928F8: 386B5E3C  addi r3, r11, 0x5e3c
	ctx.r[3].s64 = ctx.r[11].s64 + 24124;
	// 832928FC: 48027389  bl 0x832b9c84
	ctx.lr = 0x83292900;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 83292900: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292904: 386A6620  addi r3, r10, 0x6620
	ctx.r[3].s64 = ctx.r[10].s64 + 26144;
	// 83292908: 4BA17619  bl 0x82ca9f20
	ctx.lr = 0x8329290C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8329290C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292918: 4E800020  blr
	return;
}

pub fn sub_83292950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292950 size=56
	// 83292950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329295C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292960: 386B5E60  addi r3, r11, 0x5e60
	ctx.r[3].s64 = ctx.r[11].s64 + 24160;
	// 83292964: 4B1ED8D5  bl 0x82480238
	ctx.lr = 0x83292968;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 83292968: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329296C: 386A6628  addi r3, r10, 0x6628
	ctx.r[3].s64 = ctx.r[10].s64 + 26152;
	// 83292970: 4BA175B1  bl 0x82ca9f20
	ctx.lr = 0x83292974;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329297C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292980: 4E800020  blr
	return;
}

pub fn sub_83292988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292988 size=56
	// 83292988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329298C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292994: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292998: 386B5E6C  addi r3, r11, 0x5e6c
	ctx.r[3].s64 = ctx.r[11].s64 + 24172;
	// 8329299C: 4B1ED89D  bl 0x82480238
	ctx.lr = 0x832929A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 832929A0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832929A4: 386A6638  addi r3, r10, 0x6638
	ctx.r[3].s64 = ctx.r[10].s64 + 26168;
	// 832929A8: 4BA17579  bl 0x82ca9f20
	ctx.lr = 0x832929AC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832929AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832929B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832929B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832929B8: 4E800020  blr
	return;
}

pub fn sub_832929C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832929C0 size=12
	// 832929C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832929C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832929C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83292A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292A00 size=112
	// 83292A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292A0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83292A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83292A14: 388B0AF8  addi r4, r11, 0xaf8
	ctx.r[4].s64 = ctx.r[11].s64 + 2808;
	// 83292A18: 386A5E7C  addi r3, r10, 0x5e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 24188;
	// 83292A1C: 4B0439ED  bl 0x822d6408
	ctx.lr = 0x83292A20;
	crate::recompiler::externs::call(&mut ctx, base, 0x822D6408);
	// 83292A20: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83292A24: 38696658  addi r3, r9, 0x6658
	ctx.r[3].s64 = ctx.r[9].s64 + 26200;
	// 83292A28: 4BA174F9  bl 0x82ca9f20
	ctx.lr = 0x83292A2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292A38: 4E800020  blr
	return;
}

pub fn sub_83292A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292A70 size=312
	// 83292A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292A7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83292A80: 4AF8C7D9  bl 0x8221f258
	ctx.lr = 0x83292A84;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83292A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292A88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292A8C: 419A0008  beq cr6, 0x83292a94
	if ctx.cr[6].eq {
	pc = 0x83292A94; continue 'dispatch;
	}
	// 83292A90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292A94: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292A98: 41820008  beq 0x83292aa0
	if ctx.cr[0].eq {
	pc = 0x83292AA0; continue 'dispatch;
	}
	// 83292A9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292AA0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292AA4: 41820008  beq 0x83292aac
	if ctx.cr[0].eq {
	pc = 0x83292AAC; continue 'dispatch;
	}
	// 83292AA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292AAC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292AB0: 9943000E  stb r10, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[10].u8 ) };
	// 83292AB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292AB8: 39095E84  addi r8, r9, 0x5e84
	ctx.r[8].s64 = ctx.r[9].s64 + 24196;
	// 83292ABC: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 83292AC0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292AC4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292AC8: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 83292ACC: 38676678  addi r3, r7, 0x6678
	ctx.r[3].s64 = ctx.r[7].s64 + 26232;
	// 83292AD0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292AD4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292AD8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292ADC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292AE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292AE4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292AE8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292AEC: 4BA17435  bl 0x82ca9f20
	ctx.lr = 0x83292AF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292AFC: 4E800020  blr
	return;
	// 83292B00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292B08: 914B5E98  stw r10, 0x5e98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24216 as u32), ctx.r[10].u32 ) };
	// 83292B0C: 4E800020  blr
	return;
	// 83292B10: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83292B14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83292B18: 38C85EA8  addi r6, r8, 0x5ea8
	ctx.r[6].s64 = ctx.r[8].s64 + 24232;
	// 83292B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292B20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83292B24: 91685EA8  stw r11, 0x5ea8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24232 as u32), ctx.r[11].u32 ) };
	// 83292B28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83292B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83292B30: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83292B34: 91260008  stw r9, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83292B38: 9106000C  stw r8, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 83292B3C: 91660010  stw r11, 0x10(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83292B40: 90E60014  stw r7, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83292B44: 91460018  stw r10, 0x18(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83292B48: 9126001C  stw r9, 0x1c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 83292B4C: 91060020  stw r8, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 83292B50: 91660024  stw r11, 0x24(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83292B54: 90E60028  stw r7, 0x28(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(40 as u32), ctx.r[7].u32 ) };
	// 83292B58: 9146002C  stw r10, 0x2c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 83292B5C: 91260030  stw r9, 0x30(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 83292B60: 91060034  stw r8, 0x34(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 83292B64: 91660038  stw r11, 0x38(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83292B68: 90E6003C  stw r7, 0x3c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(60 as u32), ctx.r[7].u32 ) };
	// 83292B6C: 91460040  stw r10, 0x40(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83292B70: 91260044  stw r9, 0x44(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 83292B74: 91060048  stw r8, 0x48(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 83292B78: 9166004C  stw r11, 0x4c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83292B7C: 90E60050  stw r7, 0x50(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 83292B80: 91460054  stw r10, 0x54(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83292B84: 91260058  stw r9, 0x58(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 83292B88: 9106005C  stw r8, 0x5c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83292B8C: 91660060  stw r11, 0x60(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83292B90: 90E60064  stw r7, 0x64(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 83292B94: 91460068  stw r10, 0x68(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 83292B98: 9126006C  stw r9, 0x6c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 83292B9C: 91060070  stw r8, 0x70(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 83292BA0: 91660074  stw r11, 0x74(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83292BA4: 4E800020  blr
	return;
}

pub fn sub_83292BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292BA8 size=144
	// 83292BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292BB4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83292BB8: 4AF8C6A1  bl 0x8221f258
	ctx.lr = 0x83292BBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83292BBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292BC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292BC4: 419A0008  beq cr6, 0x83292bcc
	if ctx.cr[6].eq {
	pc = 0x83292BCC; continue 'dispatch;
	}
	// 83292BC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292BCC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292BD0: 41820008  beq 0x83292bd8
	if ctx.cr[0].eq {
	pc = 0x83292BD8; continue 'dispatch;
	}
	// 83292BD4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292BD8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292BDC: 41820008  beq 0x83292be4
	if ctx.cr[0].eq {
	pc = 0x83292BE4; continue 'dispatch;
	}
	// 83292BE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292BE4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292BE8: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83292BEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292BF0: 39095E9C  addi r8, r9, 0x5e9c
	ctx.r[8].s64 = ctx.r[9].s64 + 24220;
	// 83292BF4: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83292BF8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292BFC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292C00: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83292C04: 386766F8  addi r3, r7, 0x66f8
	ctx.r[3].s64 = ctx.r[7].s64 + 26360;
	// 83292C08: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292C0C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292C10: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292C14: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292C18: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292C1C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292C20: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292C24: 4BA172FD  bl 0x82ca9f20
	ctx.lr = 0x83292C28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292C34: 4E800020  blr
	return;
}

pub fn sub_83292C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292C38 size=56
	// 83292C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292C44: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292C48: 386B5F20  addi r3, r11, 0x5f20
	ctx.r[3].s64 = ctx.r[11].s64 + 24352;
	// 83292C4C: 4B0ADEC5  bl 0x82340b10
	ctx.lr = 0x83292C50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82340B10);
	// 83292C50: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292C54: 386A6708  addi r3, r10, 0x6708
	ctx.r[3].s64 = ctx.r[10].s64 + 26376;
	// 83292C58: 4BA172C9  bl 0x82ca9f20
	ctx.lr = 0x83292C5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292C68: 4E800020  blr
	return;
}

pub fn sub_83292C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292C70 size=56
	// 83292C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292C7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292C80: 386B5F2C  addi r3, r11, 0x5f2c
	ctx.r[3].s64 = ctx.r[11].s64 + 24364;
	// 83292C84: 48027001  bl 0x832b9c84
	ctx.lr = 0x83292C88;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 83292C88: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292C8C: 386A6718  addi r3, r10, 0x6718
	ctx.r[3].s64 = ctx.r[10].s64 + 26392;
	// 83292C90: 4BA17291  bl 0x82ca9f20
	ctx.lr = 0x83292C94;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292CA0: 4E800020  blr
	return;
}

pub fn sub_83292CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292CA8 size=144
	// 83292CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292CB4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83292CB8: 4AF8C5A1  bl 0x8221f258
	ctx.lr = 0x83292CBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83292CBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292CC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292CC4: 419A0008  beq cr6, 0x83292ccc
	if ctx.cr[6].eq {
	pc = 0x83292CCC; continue 'dispatch;
	}
	// 83292CC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292CCC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292CD0: 41820008  beq 0x83292cd8
	if ctx.cr[0].eq {
	pc = 0x83292CD8; continue 'dispatch;
	}
	// 83292CD4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292CD8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292CDC: 41820008  beq 0x83292ce4
	if ctx.cr[0].eq {
	pc = 0x83292CE4; continue 'dispatch;
	}
	// 83292CE0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292CE4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292CE8: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83292CEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292CF0: 39095F48  addi r8, r9, 0x5f48
	ctx.r[8].s64 = ctx.r[9].s64 + 24392;
	// 83292CF4: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83292CF8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292CFC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292D00: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83292D04: 38676720  addi r3, r7, 0x6720
	ctx.r[3].s64 = ctx.r[7].s64 + 26400;
	// 83292D08: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D0C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292D10: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D14: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292D18: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D1C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292D20: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292D24: 4BA171FD  bl 0x82ca9f20
	ctx.lr = 0x83292D28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292D34: 4E800020  blr
	return;
}

pub fn sub_83292D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292D38 size=176
	// 83292D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292D44: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83292D48: 4AF8C511  bl 0x8221f258
	ctx.lr = 0x83292D4C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83292D4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292D50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292D54: 419A0008  beq cr6, 0x83292d5c
	if ctx.cr[6].eq {
	pc = 0x83292D5C; continue 'dispatch;
	}
	// 83292D58: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292D5C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292D60: 41820008  beq 0x83292d68
	if ctx.cr[0].eq {
	pc = 0x83292D68; continue 'dispatch;
	}
	// 83292D64: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292D68: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292D6C: 41820008  beq 0x83292d74
	if ctx.cr[0].eq {
	pc = 0x83292D74; continue 'dispatch;
	}
	// 83292D70: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292D74: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292D78: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83292D7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292D80: 39095F54  addi r8, r9, 0x5f54
	ctx.r[8].s64 = ctx.r[9].s64 + 24404;
	// 83292D84: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83292D88: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292D8C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292D90: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83292D94: 38676730  addi r3, r7, 0x6730
	ctx.r[3].s64 = ctx.r[7].s64 + 26416;
	// 83292D98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292D9C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292DA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292DA4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292DA8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292DAC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292DB0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292DB4: 4BA1716D  bl 0x82ca9f20
	ctx.lr = 0x83292DB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292DC4: 4E800020  blr
	return;
	// 83292DC8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83292DCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83292DD0: 392BB088  addi r9, r11, -0x4f78
	ctx.r[9].s64 = ctx.r[11].s64 + -20344;
	// 83292DD4: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83292DD8: 38686740  addi r3, r8, 0x6740
	ctx.r[3].s64 = ctx.r[8].s64 + 26432;
	// 83292DDC: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292DE0: 4BA17140  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_83292DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292DE8 size=256
	// 83292DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292DF4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83292DF8: 4AF8C461  bl 0x8221f258
	ctx.lr = 0x83292DFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83292DFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83292E00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83292E04: 419A0008  beq cr6, 0x83292e0c
	if ctx.cr[6].eq {
	pc = 0x83292E0C; continue 'dispatch;
	}
	// 83292E08: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292E0C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292E10: 41820008  beq 0x83292e18
	if ctx.cr[0].eq {
	pc = 0x83292E18; continue 'dispatch;
	}
	// 83292E14: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292E18: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83292E1C: 41820008  beq 0x83292e24
	if ctx.cr[0].eq {
	pc = 0x83292E24; continue 'dispatch;
	}
	// 83292E20: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83292E24: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83292E28: 99430019  stb r10, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 83292E2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83292E30: 39095F60  addi r8, r9, 0x5f60
	ctx.r[8].s64 = ctx.r[9].s64 + 24416;
	// 83292E34: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 83292E38: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292E3C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83292E40: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 83292E44: 38676750  addi r3, r7, 0x6750
	ctx.r[3].s64 = ctx.r[7].s64 + 26448;
	// 83292E48: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292E4C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83292E50: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292E54: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83292E58: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83292E5C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83292E60: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83292E64: 4BA170BD  bl 0x82ca9f20
	ctx.lr = 0x83292E68;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292E74: 4E800020  blr
	return;
	// 83292E78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83292E7C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 83292E80: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83292E84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83292E88: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83292E8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83292E90: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83292E94: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83292E98: 4082FFE8  bne 0x83292e80
	if !ctx.cr[0].eq {
	pc = 0x83292E80; continue 'dispatch;
	}
	// 83292E9C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83292EA0: 38676760  addi r3, r7, 0x6760
	ctx.r[3].s64 = ctx.r[7].s64 + 26464;
	// 83292EA4: 4BA1707C  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
	// 83292EA8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83292EAC: 386B6770  addi r3, r11, 0x6770
	ctx.r[3].s64 = ctx.r[11].s64 + 26480;
	// 83292EB0: 4BA17070  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_83292EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292EE8 size=72
	// 83292EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292EF4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292EF8: 386B5F7C  addi r3, r11, 0x5f7c
	ctx.r[3].s64 = ctx.r[11].s64 + 24444;
	// 83292EFC: 48026D89  bl 0x832b9c84
	ctx.lr = 0x83292F00;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 83292F00: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292F04: 386A68A0  addi r3, r10, 0x68a0
	ctx.r[3].s64 = ctx.r[10].s64 + 26784;
	// 83292F08: 4BA17019  bl 0x82ca9f20
	ctx.lr = 0x83292F0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292F18: 4E800020  blr
	return;
}

pub fn sub_83292F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292F30 size=56
	// 83292F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292F3C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292F40: 386B5F98  addi r3, r11, 0x5f98
	ctx.r[3].s64 = ctx.r[11].s64 + 24472;
	// 83292F44: 4B56C6F5  bl 0x827ff638
	ctx.lr = 0x83292F48;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 83292F48: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292F4C: 386A68B8  addi r3, r10, 0x68b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26808;
	// 83292F50: 4BA16FD1  bl 0x82ca9f20
	ctx.lr = 0x83292F54;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292F60: 4E800020  blr
	return;
}

pub fn sub_83292F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292F68 size=56
	// 83292F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292F74: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292F78: 386B5FA4  addi r3, r11, 0x5fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 24484;
	// 83292F7C: 4B56C6BD  bl 0x827ff638
	ctx.lr = 0x83292F80;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 83292F80: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292F84: 386A68C8  addi r3, r10, 0x68c8
	ctx.r[3].s64 = ctx.r[10].s64 + 26824;
	// 83292F88: 4BA16F99  bl 0x82ca9f20
	ctx.lr = 0x83292F8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292F8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292F98: 4E800020  blr
	return;
}

pub fn sub_83292FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292FA0 size=56
	// 83292FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292FAC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292FB0: 386B5FB0  addi r3, r11, 0x5fb0
	ctx.r[3].s64 = ctx.r[11].s64 + 24496;
	// 83292FB4: 4B56C685  bl 0x827ff638
	ctx.lr = 0x83292FB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 83292FB8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292FBC: 386A68D8  addi r3, r10, 0x68d8
	ctx.r[3].s64 = ctx.r[10].s64 + 26840;
	// 83292FC0: 4BA16F61  bl 0x82ca9f20
	ctx.lr = 0x83292FC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83292FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83292FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83292FD0: 4E800020  blr
	return;
}

pub fn sub_83292FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83292FD8 size=56
	// 83292FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83292FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83292FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83292FE4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83292FE8: 386B5FBC  addi r3, r11, 0x5fbc
	ctx.r[3].s64 = ctx.r[11].s64 + 24508;
	// 83292FEC: 4B56C64D  bl 0x827ff638
	ctx.lr = 0x83292FF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x827FF638);
	// 83292FF0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83292FF4: 386A68E8  addi r3, r10, 0x68e8
	ctx.r[3].s64 = ctx.r[10].s64 + 26856;
	// 83292FF8: 4BA16F29  bl 0x82ca9f20
	ctx.lr = 0x83292FFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83292FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293008: 4E800020  blr
	return;
}

pub fn sub_83293010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293010 size=192
	// 83293010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8329301C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293020: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83293024: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83293028: 3BEBB0E8  addi r31, r11, -0x4f18
	ctx.r[31].s64 = ctx.r[11].s64 + -20248;
	// 8329302C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83293030: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83293034: 4AF8C225  bl 0x8221f258
	ctx.lr = 0x83293038;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83293038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329303C: 419A0008  beq cr6, 0x83293044
	if ctx.cr[6].eq {
	pc = 0x83293044; continue 'dispatch;
	}
	// 83293040: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83293044: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293048: 41820008  beq 0x83293050
	if ctx.cr[0].eq {
	pc = 0x83293050; continue 'dispatch;
	}
	// 8329304C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83293050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83293054: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83293058: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329305C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83293060: 386A68F8  addi r3, r10, 0x68f8
	ctx.r[3].s64 = ctx.r[10].s64 + 26872;
	// 83293064: 4BA16EBD  bl 0x82ca9f20
	ctx.lr = 0x83293068;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83293068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329306C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293074: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83293078: 4E800020  blr
	return;
}

pub fn sub_832930D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832930D0 size=56
	// 832930D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832930D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832930D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832930DC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832930E0: 386B5FF0  addi r3, r11, 0x5ff0
	ctx.r[3].s64 = ctx.r[11].s64 + 24560;
	// 832930E4: 48026BA1  bl 0x832b9c84
	ctx.lr = 0x832930E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832930E8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832930EC: 386A6950  addi r3, r10, 0x6950
	ctx.r[3].s64 = ctx.r[10].s64 + 26960;
	// 832930F0: 4BA16E31  bl 0x82ca9f20
	ctx.lr = 0x832930F4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832930F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832930F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832930FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293100: 4E800020  blr
	return;
}

pub fn sub_83293108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293108 size=152
	// 83293108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83293114: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83293118: 386B6010  addi r3, r11, 0x6010
	ctx.r[3].s64 = ctx.r[11].s64 + 24592;
	// 8329311C: 48026B69  bl 0x832b9c84
	ctx.lr = 0x83293120;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 83293120: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83293124: 386A6958  addi r3, r10, 0x6958
	ctx.r[3].s64 = ctx.r[10].s64 + 26968;
	// 83293128: 4BA16DF9  bl 0x82ca9f20
	ctx.lr = 0x8329312C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8329312C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293138: 4E800020  blr
	return;
}

pub fn sub_832931A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832931A0 size=56
	// 832931A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832931A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832931A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832931AC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832931B0: 386B6040  addi r3, r11, 0x6040
	ctx.r[3].s64 = ctx.r[11].s64 + 24640;
	// 832931B4: 4B78578D  bl 0x82a18940
	ctx.lr = 0x832931B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A18940);
	// 832931B8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832931BC: 386A6968  addi r3, r10, 0x6968
	ctx.r[3].s64 = ctx.r[10].s64 + 26984;
	// 832931C0: 4BA16D61  bl 0x82ca9f20
	ctx.lr = 0x832931C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832931C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832931C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832931CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832931D0: 4E800020  blr
	return;
}

pub fn sub_832931D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832931D8 size=56
	// 832931D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832931DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832931E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832931E4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832931E8: 386B604C  addi r3, r11, 0x604c
	ctx.r[3].s64 = ctx.r[11].s64 + 24652;
	// 832931EC: 4B785755  bl 0x82a18940
	ctx.lr = 0x832931F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A18940);
	// 832931F0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832931F4: 386A6978  addi r3, r10, 0x6978
	ctx.r[3].s64 = ctx.r[10].s64 + 27000;
	// 832931F8: 4BA16D29  bl 0x82ca9f20
	ctx.lr = 0x832931FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832931FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293208: 4E800020  blr
	return;
}

pub fn sub_83293210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293210 size=144
	// 83293210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329321C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 83293220: 4AF8C039  bl 0x8221f258
	ctx.lr = 0x83293224;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83293224: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83293228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329322C: 419A0008  beq cr6, 0x83293234
	if ctx.cr[6].eq {
	pc = 0x83293234; continue 'dispatch;
	}
	// 83293230: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83293234: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293238: 41820008  beq 0x83293240
	if ctx.cr[0].eq {
	pc = 0x83293240; continue 'dispatch;
	}
	// 8329323C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83293240: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293244: 41820008  beq 0x8329324c
	if ctx.cr[0].eq {
	pc = 0x8329324C; continue 'dispatch;
	}
	// 83293248: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329324C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83293250: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 83293254: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293258: 39096058  addi r8, r9, 0x6058
	ctx.r[8].s64 = ctx.r[9].s64 + 24664;
	// 8329325C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 83293260: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83293264: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83293268: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8329326C: 38676988  addi r3, r7, 0x6988
	ctx.r[3].s64 = ctx.r[7].s64 + 27016;
	// 83293270: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293274: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83293278: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8329327C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83293280: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293284: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83293288: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329328C: 4BA16C95  bl 0x82ca9f20
	ctx.lr = 0x83293290;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83293290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329329C: 4E800020  blr
	return;
}

pub fn sub_832932A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832932A0 size=176
	// 832932A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832932A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832932A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832932AC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832932B0: 386B606C  addi r3, r11, 0x606c
	ctx.r[3].s64 = ctx.r[11].s64 + 24684;
	// 832932B4: 480269D1  bl 0x832b9c84
	ctx.lr = 0x832932B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832932B8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832932BC: 386A6998  addi r3, r10, 0x6998
	ctx.r[3].s64 = ctx.r[10].s64 + 27032;
	// 832932C0: 4BA16C61  bl 0x82ca9f20
	ctx.lr = 0x832932C4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832932C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832932C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832932CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832932D0: 4E800020  blr
	return;
}

pub fn sub_83293350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293350 size=176
	// 83293350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293354: 4BA160B5  bl 0x82ca9408
	ctx.lr = 0x83293358;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9408);
	// 83293358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329335C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83293360: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 83293364: 396B60C8  addi r11, r11, 0x60c8
	ctx.r[11].s64 = ctx.r[11].s64 + 24776;
	// 83293368: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8329336C: 3BEB0038  addi r31, r11, 0x38
	ctx.r[31].s64 = ctx.r[11].s64 + 56;
	// 83293370: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83293374: 3B8B2A2C  addi r28, r11, 0x2a2c
	ctx.r[28].s64 = ctx.r[11].s64 + 10796;
	// 83293378: 3D6082A5  lis r11, -0x7d5b
	ctx.r[11].s64 = -2103115776;
	// 8329337C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83293380: 38CB0668  addi r6, r11, 0x668
	ctx.r[6].s64 = ctx.r[11].s64 + 1640;
	// 83293384: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 83293388: 387FFFC8  addi r3, r31, -0x38
	ctx.r[3].s64 = ctx.r[31].s64 + -56;
	// 8329338C: 4AF8C77D  bl 0x8221fb08
	ctx.lr = 0x83293390;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221FB08);
	// 83293390: 939FFFF8  stw r28, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[28].u32 ) };
	// 83293394: 93BFFFFC  stw r29, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[29].u32 ) };
	// 83293398: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8329339C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832933A0: 3BFF004C  addi r31, r31, 0x4c
	ctx.r[31].s64 = ctx.r[31].s64 + 76;
	// 832933A4: 4080FFD4  bge 0x83293378
	if !ctx.cr[0].lt {
	pc = 0x83293378; continue 'dispatch;
	}
	// 832933A8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832933AC: 386B6A50  addi r3, r11, 0x6a50
	ctx.r[3].s64 = ctx.r[11].s64 + 27216;
	// 832933B0: 4BA16B71  bl 0x82ca9f20
	ctx.lr = 0x832933B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832933B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832933B8: 4BA160A0  b 0x82ca9458
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9458);
	return;
}

pub fn sub_83293400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293400 size=144
	// 83293400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329340C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83293410: 4AF8BE49  bl 0x8221f258
	ctx.lr = 0x83293414;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 83293414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83293418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329341C: 419A0008  beq cr6, 0x83293424
	if ctx.cr[6].eq {
	pc = 0x83293424; continue 'dispatch;
	}
	// 83293420: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83293424: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293428: 41820008  beq 0x83293430
	if ctx.cr[0].eq {
	pc = 0x83293430; continue 'dispatch;
	}
	// 8329342C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83293430: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83293434: 41820008  beq 0x8329343c
	if ctx.cr[0].eq {
	pc = 0x8329343C; continue 'dispatch;
	}
	// 83293438: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329343C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83293440: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83293444: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293448: 3909632C  addi r8, r9, 0x632c
	ctx.r[8].s64 = ctx.r[9].s64 + 25388;
	// 8329344C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83293450: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83293454: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83293458: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8329345C: 38676BC0  addi r3, r7, 0x6bc0
	ctx.r[3].s64 = ctx.r[7].s64 + 27584;
	// 83293460: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293464: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83293468: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8329346C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83293470: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83293474: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83293478: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329347C: 4BA16AA5  bl 0x82ca9f20
	ctx.lr = 0x83293480;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83293480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329348C: 4E800020  blr
	return;
}

pub fn sub_83293490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293490 size=56
	// 83293490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329349C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832934A0: 386B6338  addi r3, r11, 0x6338
	ctx.r[3].s64 = ctx.r[11].s64 + 25400;
	// 832934A4: 480267E1  bl 0x832b9c84
	ctx.lr = 0x832934A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832934A8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832934AC: 386A6BD0  addi r3, r10, 0x6bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 27600;
	// 832934B0: 4BA16A71  bl 0x82ca9f20
	ctx.lr = 0x832934B4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832934B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832934B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832934BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832934C0: 4E800020  blr
	return;
}

pub fn sub_832934C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832934C8 size=152
	// 832934C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832934CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832934D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832934D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832934D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832934DC: 3BEB6354  addi r31, r11, 0x6354
	ctx.r[31].s64 = ctx.r[11].s64 + 25428;
	// 832934E0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832934E4: 480267A1  bl 0x832b9c84
	ctx.lr = 0x832934E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x832B9C84);
	// 832934E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832934EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832934F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832934F4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 832934F8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 832934FC: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83293500: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 83293504: 386A6BD8  addi r3, r10, 0x6bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 27608;
	// 83293508: 4BA16A19  bl 0x82ca9f20
	ctx.lr = 0x8329350C;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8329350C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83293514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293518: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8329351C: 4E800020  blr
	return;
	// 83293520: 3D40834B  lis r10, -0x7cb5
	ctx.r[10].s64 = -2092236800;
	// 83293524: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293528: 38EA2390  addi r7, r10, 0x2390
	ctx.r[7].s64 = ctx.r[10].s64 + 9104;
	// 8329352C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83293530: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83293534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83293538: 91672000  stw r11, 0x2000(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8192 as u32), ctx.r[11].u32 ) };
	// 8329353C: 91472004  stw r10, 0x2004(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8196 as u32), ctx.r[10].u32 ) };
	// 83293540: 91274008  stw r9, 0x4008(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16392 as u32), ctx.r[9].u32 ) };
	// 83293544: 9107400C  stw r8, 0x400c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16396 as u32), ctx.r[8].u32 ) };
	// 83293548: 4E800020  blr
	return;
}

pub fn sub_83293560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293560 size=72
	// 83293560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329356C: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 83293570: 386BDE90  addi r3, r11, -0x2170
	ctx.r[3].s64 = ctx.r[11].s64 + -8560;
	// 83293574: 4B1ECCC5  bl 0x82480238
	ctx.lr = 0x83293578;
	crate::recompiler::externs::call(&mut ctx, base, 0x82480238);
	// 83293578: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329357C: 386A6BE8  addi r3, r10, 0x6be8
	ctx.r[3].s64 = ctx.r[10].s64 + 27624;
	// 83293580: 4BA169A1  bl 0x82ca9f20
	ctx.lr = 0x83293584;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83293584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329358C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293590: 4E800020  blr
	return;
}

pub fn sub_832935A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832935A8 size=56
	// 832935A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832935AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832935B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832935B4: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832935B8: 386BDEAC  addi r3, r11, -0x2154
	ctx.r[3].s64 = ctx.r[11].s64 + -8532;
	// 832935BC: 4B776FF5  bl 0x82a0a5b0
	ctx.lr = 0x832935C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A0A5B0);
	// 832935C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832935C4: 386A6C08  addi r3, r10, 0x6c08
	ctx.r[3].s64 = ctx.r[10].s64 + 27656;
	// 832935C8: 4BA16959  bl 0x82ca9f20
	ctx.lr = 0x832935CC;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 832935CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832935D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832935D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832935D8: 4E800020  blr
	return;
}

pub fn sub_832935E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832935E0 size=232
	// 832935E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832935E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832935E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832935EC: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832935F0: 386BDEB8  addi r3, r11, -0x2148
	ctx.r[3].s64 = ctx.r[11].s64 + -8520;
	// 832935F4: 4B776FBD  bl 0x82a0a5b0
	ctx.lr = 0x832935F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82A0A5B0);
	// 832935F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832935FC: 386A6C18  addi r3, r10, 0x6c18
	ctx.r[3].s64 = ctx.r[10].s64 + 27672;
	// 83293600: 4BA16921  bl 0x82ca9f20
	ctx.lr = 0x83293604;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 83293604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83293608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329360C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83293610: 4E800020  blr
	return;
}

pub fn sub_832936C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832936C8 size=12
	// 832936C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832936CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832936D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293748 size=12
	// 83293748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329374C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293788 size=1044
	// 83293788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329378C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83293794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83293798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329379C: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832937A0: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 832937A4: 3BEBE010  addi r31, r11, -0x1ff0
	ctx.r[31].s64 = ctx.r[11].s64 + -8176;
	// 832937A8: 388AB378  addi r4, r10, -0x4c88
	ctx.r[4].s64 = ctx.r[10].s64 + -19592;
	// 832937AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832937B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832937B4: 4AF9971D  bl 0x8222ced0
	ctx.lr = 0x832937B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832937B8: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 832937BC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 832937C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832937C4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832937C8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832937CC: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 832937D0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832937D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832937D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832937DC: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 832937E0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832937E4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 832937E8: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 832937EC: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 832937F0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832937F4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 832937F8: 3D008210  lis r8, -0x7df0
	ctx.r[8].s64 = -2112880640;
	// 832937FC: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 83293800: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293804: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83293808: 3888B33C  addi r4, r8, -0x4cc4
	ctx.r[4].s64 = ctx.r[8].s64 + -19652;
	// 8329380C: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 83293810: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83293814: 4AF996BD  bl 0x8222ced0
	ctx.lr = 0x83293818;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83293818: 39600400  li r11, 0x400
	ctx.r[11].s64 = 1024;
	// 8329381C: 39400240  li r10, 0x240
	ctx.r[10].s64 = 576;
	// 83293820: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293824: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 83293828: 915F0074  stw r10, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8329382C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293830: 913F0078  stw r9, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 83293834: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293838: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329383C: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83293840: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 83293844: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293848: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8329384C: 39400400  li r10, 0x400
	ctx.r[10].s64 = 1024;
	// 83293850: 39200240  li r9, 0x240
	ctx.r[9].s64 = 576;
	// 83293854: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83293858: 997F00D4  stb r11, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u8 ) };
	// 8329385C: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83293860: 915F008C  stw r10, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 83293864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293868: 913F0090  stw r9, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 8329386C: 3887B2F8  addi r4, r7, -0x4d08
	ctx.r[4].s64 = ctx.r[7].s64 + -19720;
	// 83293870: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 83293874: 4AF9965D  bl 0x8222ced0
	ctx.lr = 0x83293878;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83293878: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 8329387C: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293880: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 83293884: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293888: 915F00E0  stw r10, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 8329388C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83293890: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 83293894: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293898: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8329389C: 915F00EC  stw r10, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 832938A0: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 832938A4: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 832938A8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832938AC: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 832938B0: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 832938B4: 915F00F8  stw r10, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 832938B8: 913F00F0  stw r9, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[9].u32 ) };
	// 832938BC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 832938C0: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 832938C4: 917F00FC  stw r11, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[11].u32 ) };
	// 832938C8: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 832938CC: 915F0100  stw r10, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[10].u32 ) };
	// 832938D0: 913F0104  stw r9, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[9].u32 ) };
	// 832938D4: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 832938D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832938DC: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 832938E0: 915F010C  stw r10, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[10].u32 ) };
	// 832938E4: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 832938E8: 993F0140  stb r9, 0x140(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[9].u8 ) };
	// 832938EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832938F0: 3886B2BC  addi r4, r6, -0x4d44
	ctx.r[4].s64 = ctx.r[6].s64 + -19780;
	// 832938F4: 387F0144  addi r3, r31, 0x144
	ctx.r[3].s64 = ctx.r[31].s64 + 324;
	// 832938F8: 4AF995D9  bl 0x8222ced0
	ctx.lr = 0x832938FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832938FC: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293900: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293904: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 83293908: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329390C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293910: 915F014C  stw r10, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[10].u32 ) };
	// 83293914: 913F0150  stw r9, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[9].u32 ) };
	// 83293918: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329391C: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 83293920: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293924: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293928: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 8329392C: 913F015C  stw r9, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[9].u32 ) };
	// 83293930: 39400280  li r10, 0x280
	ctx.r[10].s64 = 640;
	// 83293934: 917F0160  stw r11, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 83293938: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 8329393C: 39600280  li r11, 0x280
	ctx.r[11].s64 = 640;
	// 83293940: 915F0164  stw r10, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 83293944: 913F0168  stw r9, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[9].u32 ) };
	// 83293948: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8329394C: 917F016C  stw r11, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 83293950: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 83293954: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 83293958: 915F0170  stw r10, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 8329395C: 995F01AC  stb r10, 0x1ac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[10].u8 ) };
	// 83293960: 3C808210  lis r4, -0x7df0
	ctx.r[4].s64 = -2112880640;
	// 83293964: 913F0174  stw r9, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[9].u32 ) };
	// 83293968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8329396C: 917F0178  stw r11, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[11].u32 ) };
	// 83293970: 3884B278  addi r4, r4, -0x4d88
	ctx.r[4].s64 = ctx.r[4].s64 + -19848;
	// 83293974: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 83293978: 4AF99559  bl 0x8222ced0
	ctx.lr = 0x8329397C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8329397C: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293980: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293984: 917F01B4  stw r11, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[11].u32 ) };
	// 83293988: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8329398C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83293990: 915F01B8  stw r10, 0x1b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 83293994: 913F01BC  stw r9, 0x1bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(444 as u32), ctx.r[9].u32 ) };
	// 83293998: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329399C: 917F01C0  stw r11, 0x1c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 832939A0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 832939A4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832939A8: 915F01C4  stw r10, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[10].u32 ) };
	// 832939AC: 913F01C8  stw r9, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[9].u32 ) };
	// 832939B0: 39400280  li r10, 0x280
	ctx.r[10].s64 = 640;
	// 832939B4: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 832939B8: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 832939BC: 39600280  li r11, 0x280
	ctx.r[11].s64 = 640;
	// 832939C0: 915F01D0  stw r10, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 832939C4: 913F01D4  stw r9, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[9].u32 ) };
	// 832939C8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 832939CC: 917F01D8  stw r11, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[11].u32 ) };
	// 832939D0: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 832939D4: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 832939D8: 915F01DC  stw r10, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[10].u32 ) };
	// 832939DC: 913F01E0  stw r9, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[9].u32 ) };
	// 832939E0: 3C608210  lis r3, -0x7df0
	ctx.r[3].s64 = -2112880640;
	// 832939E4: 917F01E4  stw r11, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[11].u32 ) };
	// 832939E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832939EC: 995F0218  stb r10, 0x218(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[10].u8 ) };
	// 832939F0: 3883B230  addi r4, r3, -0x4dd0
	ctx.r[4].s64 = ctx.r[3].s64 + -19920;
	// 832939F4: 387F021C  addi r3, r31, 0x21c
	ctx.r[3].s64 = ctx.r[31].s64 + 540;
	// 832939F8: 4AF994D9  bl 0x8222ced0
	ctx.lr = 0x832939FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 832939FC: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293A00: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293A04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293A08: 917F0220  stw r11, 0x220(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(544 as u32), ctx.r[11].u32 ) };
	// 83293A0C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83293A10: 915F0224  stw r10, 0x224(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(548 as u32), ctx.r[10].u32 ) };
	// 83293A14: 913F0228  stw r9, 0x228(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[9].u32 ) };
	// 83293A18: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83293A1C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293A20: 917F022C  stw r11, 0x22c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(556 as u32), ctx.r[11].u32 ) };
	// 83293A24: 915F0230  stw r10, 0x230(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 83293A28: 39400500  li r10, 0x500
	ctx.r[10].s64 = 1280;
	// 83293A2C: 913F0234  stw r9, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[9].u32 ) };
	// 83293A30: 39200100  li r9, 0x100
	ctx.r[9].s64 = 256;
	// 83293A34: 915F023C  stw r10, 0x23c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(572 as u32), ctx.r[10].u32 ) };
	// 83293A38: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293A3C: 913F0240  stw r9, 0x240(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(576 as u32), ctx.r[9].u32 ) };
	// 83293A40: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 83293A44: 39200500  li r9, 0x500
	ctx.r[9].s64 = 1280;
	// 83293A48: 917F0238  stw r11, 0x238(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(568 as u32), ctx.r[11].u32 ) };
	// 83293A4C: 917F0244  stw r11, 0x244(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(580 as u32), ctx.r[11].u32 ) };
	// 83293A50: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 83293A54: 915F0248  stw r10, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[10].u32 ) };
	// 83293A58: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83293A5C: 913F024C  stw r9, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[9].u32 ) };
	// 83293A60: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 83293A64: 917F0250  stw r11, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[11].u32 ) };
	// 83293A68: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293A6C: 915F0254  stw r10, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[10].u32 ) };
	// 83293A70: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293A74: 913F0258  stw r9, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[9].u32 ) };
	// 83293A78: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293A7C: 917F025C  stw r11, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[11].u32 ) };
	// 83293A80: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83293A84: 915F0260  stw r10, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[10].u32 ) };
	// 83293A88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293A8C: 993F0284  stb r9, 0x284(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[9].u8 ) };
	// 83293A90: 388BB1F4  addi r4, r11, -0x4e0c
	ctx.r[4].s64 = ctx.r[11].s64 + -19980;
	// 83293A94: 387F0288  addi r3, r31, 0x288
	ctx.r[3].s64 = ctx.r[31].s64 + 648;
	// 83293A98: 4AF99439  bl 0x8222ced0
	ctx.lr = 0x83293A9C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83293A9C: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293AA0: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293AA4: 917F028C  stw r11, 0x28c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), ctx.r[11].u32 ) };
	// 83293AA8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83293AAC: 915F0290  stw r10, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[10].u32 ) };
	// 83293AB0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83293AB4: 917F0298  stw r11, 0x298(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[11].u32 ) };
	// 83293AB8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293ABC: 915F029C  stw r10, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[10].u32 ) };
	// 83293AC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 83293AC4: 917F02A4  stw r11, 0x2a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), ctx.r[11].u32 ) };
	// 83293AC8: 39400140  li r10, 0x140
	ctx.r[10].s64 = 320;
	// 83293ACC: 39600140  li r11, 0x140
	ctx.r[11].s64 = 320;
	// 83293AD0: 913F0294  stw r9, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[9].u32 ) };
	// 83293AD4: 915F02A8  stw r10, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[10].u32 ) };
	// 83293AD8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293ADC: 917F02B0  stw r11, 0x2b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(688 as u32), ctx.r[11].u32 ) };
	// 83293AE0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83293AE4: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 83293AE8: 913F02A0  stw r9, 0x2a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(672 as u32), ctx.r[9].u32 ) };
	// 83293AEC: 915F02B4  stw r10, 0x2b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[10].u32 ) };
	// 83293AF0: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 83293AF4: 917F02BC  stw r11, 0x2bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(700 as u32), ctx.r[11].u32 ) };
	// 83293AF8: 39400280  li r10, 0x280
	ctx.r[10].s64 = 640;
	// 83293AFC: 396003C0  li r11, 0x3c0
	ctx.r[11].s64 = 960;
	// 83293B00: 913F02AC  stw r9, 0x2ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[9].u32 ) };
	// 83293B04: 915F02C0  stw r10, 0x2c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(704 as u32), ctx.r[10].u32 ) };
	// 83293B08: 39200280  li r9, 0x280
	ctx.r[9].s64 = 640;
	// 83293B0C: 917F02C8  stw r11, 0x2c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(712 as u32), ctx.r[11].u32 ) };
	// 83293B10: 394003C0  li r10, 0x3c0
	ctx.r[10].s64 = 960;
	// 83293B14: 396002D0  li r11, 0x2d0
	ctx.r[11].s64 = 720;
	// 83293B18: 913F02B8  stw r9, 0x2b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(696 as u32), ctx.r[9].u32 ) };
	// 83293B1C: 915F02D0  stw r10, 0x2d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(720 as u32), ctx.r[10].u32 ) };
	// 83293B20: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293B24: 917F02CC  stw r11, 0x2cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(716 as u32), ctx.r[11].u32 ) };
	// 83293B28: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293B2C: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293B30: 913F02C4  stw r9, 0x2c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(708 as u32), ctx.r[9].u32 ) };
	// 83293B34: 913F02D4  stw r9, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[9].u32 ) };
	// 83293B38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83293B3C: 915F02DC  stw r10, 0x2dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(732 as u32), ctx.r[10].u32 ) };
	// 83293B40: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83293B44: 917F02D8  stw r11, 0x2d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(728 as u32), ctx.r[11].u32 ) };
	// 83293B48: 387F02F4  addi r3, r31, 0x2f4
	ctx.r[3].s64 = ctx.r[31].s64 + 756;
	// 83293B4C: 993F02F0  stb r9, 0x2f0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[9].u8 ) };
	// 83293B50: 388AB1B0  addi r4, r10, -0x4e50
	ctx.r[4].s64 = ctx.r[10].s64 + -20048;
	// 83293B54: 4AF9937D  bl 0x8222ced0
	ctx.lr = 0x83293B58;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 83293B58: 39600500  li r11, 0x500
	ctx.r[11].s64 = 1280;
	// 83293B5C: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 83293B60: 917F02F8  stw r11, 0x2f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), ctx.r[11].u32 ) };
	// 83293B64: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 83293B68: 915F02FC  stw r10, 0x2fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(764 as u32), ctx.r[10].u32 ) };
	// 83293B6C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83293B70: 913F0300  stw r9, 0x300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(768 as u32), ctx.r[9].u32 ) };
	// 83293B74: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83293B78: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83293B7C: 917F0304  stw r11, 0x304(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(772 as u32), ctx.r[11].u32 ) };
	// 83293B80: 913F030C  stw r9, 0x30c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(780 as u32), ctx.r[9].u32 ) };
	// 83293B84: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83293B88: 915F0308  stw r10, 0x308(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(776 as u32), ctx.r[10].u32 ) };
	// 83293B8C: 392002D0  li r9, 0x2d0
	ctx.r[9].s64 = 720;
	// 83293B90: 39400140  li r10, 0x140
	ctx.r[10].s64 = 320;
	// 83293B94: 917F0310  stw r11, 0x310(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(784 as u32), ctx.r[11].u32 ) };
	// 83293B98: 913F0318  stw r9, 0x318(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(792 as u32), ctx.r[9].u32 ) };
}

pub fn sub_83293D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293D80 size=12
	// 83293D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293DC0 size=12
	// 83293DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293E00 size=12
	// 83293E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293E40 size=12
	// 83293E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293E80 size=12
	// 83293E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293EC0 size=12
	// 83293EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293F00 size=12
	// 83293F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293F40 size=12
	// 83293F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293F80 size=12
	// 83293F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83293FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83293FC0 size=12
	// 83293FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83293FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83293FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83294000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294000 size=12
	// 83294000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83294040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294040 size=12
	// 83294040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83294080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294080 size=12
	// 83294080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83294100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294100 size=56
	// 83294100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329410C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83294110: 386B86B4  addi r3, r11, -0x794c
	ctx.r[3].s64 = ctx.r[11].s64 + -31052;
	// 83294114: 4AEF502D  bl 0x82189140
	ctx.lr = 0x83294118;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294118: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329411C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294120: 916AE50C  stw r11, -0x1af4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6900 as u32), ctx.r[11].u32 ) };
	// 83294124: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329412C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294130: 4E800020  blr
	return;
}

pub fn sub_83294138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294138 size=56
	// 83294138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294144: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294148: 386BCA10  addi r3, r11, -0x35f0
	ctx.r[3].s64 = ctx.r[11].s64 + -13808;
	// 8329414C: 4AEF4FF5  bl 0x82189140
	ctx.lr = 0x83294150;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294150: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294154: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294158: 916AE510  stw r11, -0x1af0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6896 as u32), ctx.r[11].u32 ) };
	// 8329415C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294168: 4E800020  blr
	return;
}

pub fn sub_83294170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294170 size=56
	// 83294170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329417C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294180: 386BCA1C  addi r3, r11, -0x35e4
	ctx.r[3].s64 = ctx.r[11].s64 + -13796;
	// 83294184: 4AEF4FBD  bl 0x82189140
	ctx.lr = 0x83294188;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294188: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329418C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294190: 916AE514  stw r11, -0x1aec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6892 as u32), ctx.r[11].u32 ) };
	// 83294194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329419C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832941A0: 4E800020  blr
	return;
}

pub fn sub_832941A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832941A8 size=56
	// 832941A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832941AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832941B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832941B4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832941B8: 386BCA24  addi r3, r11, -0x35dc
	ctx.r[3].s64 = ctx.r[11].s64 + -13788;
	// 832941BC: 4AEF4F85  bl 0x82189140
	ctx.lr = 0x832941C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832941C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832941C4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832941C8: 916AE518  stw r11, -0x1ae8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6888 as u32), ctx.r[11].u32 ) };
	// 832941CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832941D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832941D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832941D8: 4E800020  blr
	return;
}

pub fn sub_832941E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832941E0 size=56
	// 832941E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832941E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832941E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832941EC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832941F0: 386BCA2C  addi r3, r11, -0x35d4
	ctx.r[3].s64 = ctx.r[11].s64 + -13780;
	// 832941F4: 4AEF4F4D  bl 0x82189140
	ctx.lr = 0x832941F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832941F8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832941FC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294200: 916AE51C  stw r11, -0x1ae4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6884 as u32), ctx.r[11].u32 ) };
	// 83294204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329420C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294210: 4E800020  blr
	return;
}

pub fn sub_83294218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294218 size=56
	// 83294218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294224: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294228: 386BCA38  addi r3, r11, -0x35c8
	ctx.r[3].s64 = ctx.r[11].s64 + -13768;
	// 8329422C: 4AEF4F15  bl 0x82189140
	ctx.lr = 0x83294230;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294230: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294234: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294238: 916AE520  stw r11, -0x1ae0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6880 as u32), ctx.r[11].u32 ) };
	// 8329423C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294248: 4E800020  blr
	return;
}

pub fn sub_83294250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294250 size=56
	// 83294250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329425C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294260: 386BCA44  addi r3, r11, -0x35bc
	ctx.r[3].s64 = ctx.r[11].s64 + -13756;
	// 83294264: 4AEF4EDD  bl 0x82189140
	ctx.lr = 0x83294268;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294268: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329426C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294270: 916AE524  stw r11, -0x1adc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6876 as u32), ctx.r[11].u32 ) };
	// 83294274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329427C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294280: 4E800020  blr
	return;
}

pub fn sub_83294288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294288 size=56
	// 83294288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329428C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294294: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294298: 386BCA50  addi r3, r11, -0x35b0
	ctx.r[3].s64 = ctx.r[11].s64 + -13744;
	// 8329429C: 4AEF4EA5  bl 0x82189140
	ctx.lr = 0x832942A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832942A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832942A4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832942A8: 916AE528  stw r11, -0x1ad8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6872 as u32), ctx.r[11].u32 ) };
	// 832942AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832942B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832942B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832942B8: 4E800020  blr
	return;
}

pub fn sub_832942C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832942C0 size=56
	// 832942C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832942C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832942C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832942CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832942D0: 386BCA5C  addi r3, r11, -0x35a4
	ctx.r[3].s64 = ctx.r[11].s64 + -13732;
	// 832942D4: 4AEF4E6D  bl 0x82189140
	ctx.lr = 0x832942D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832942D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832942DC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832942E0: 916AE52C  stw r11, -0x1ad4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6868 as u32), ctx.r[11].u32 ) };
	// 832942E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832942E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832942EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832942F0: 4E800020  blr
	return;
}

pub fn sub_832942F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832942F8 size=56
	// 832942F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832942FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294304: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294308: 386BCA68  addi r3, r11, -0x3598
	ctx.r[3].s64 = ctx.r[11].s64 + -13720;
	// 8329430C: 4AEF4E35  bl 0x82189140
	ctx.lr = 0x83294310;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294310: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294314: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294318: 916AE530  stw r11, -0x1ad0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6864 as u32), ctx.r[11].u32 ) };
	// 8329431C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294328: 4E800020  blr
	return;
}

pub fn sub_83294330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294330 size=56
	// 83294330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329433C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294340: 386BCA74  addi r3, r11, -0x358c
	ctx.r[3].s64 = ctx.r[11].s64 + -13708;
	// 83294344: 4AEF4DFD  bl 0x82189140
	ctx.lr = 0x83294348;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294348: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329434C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294350: 916AE534  stw r11, -0x1acc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6860 as u32), ctx.r[11].u32 ) };
	// 83294354: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329435C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294360: 4E800020  blr
	return;
}

pub fn sub_83294368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294368 size=56
	// 83294368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329436C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294374: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294378: 386BCA7C  addi r3, r11, -0x3584
	ctx.r[3].s64 = ctx.r[11].s64 + -13700;
	// 8329437C: 4AEF4DC5  bl 0x82189140
	ctx.lr = 0x83294380;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294380: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294384: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294388: 916AE538  stw r11, -0x1ac8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6856 as u32), ctx.r[11].u32 ) };
	// 8329438C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294398: 4E800020  blr
	return;
}

pub fn sub_832943A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832943A0 size=56
	// 832943A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832943A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832943A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832943AC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832943B0: 386BCA84  addi r3, r11, -0x357c
	ctx.r[3].s64 = ctx.r[11].s64 + -13692;
	// 832943B4: 4AEF4D8D  bl 0x82189140
	ctx.lr = 0x832943B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832943B8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832943BC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832943C0: 916AE53C  stw r11, -0x1ac4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6852 as u32), ctx.r[11].u32 ) };
	// 832943C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832943C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832943CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832943D0: 4E800020  blr
	return;
}

pub fn sub_832943D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832943D8 size=56
	// 832943D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832943DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832943E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832943E4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832943E8: 386BCA8C  addi r3, r11, -0x3574
	ctx.r[3].s64 = ctx.r[11].s64 + -13684;
	// 832943EC: 4AEF4D55  bl 0x82189140
	ctx.lr = 0x832943F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832943F0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832943F4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832943F8: 916AE540  stw r11, -0x1ac0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6848 as u32), ctx.r[11].u32 ) };
	// 832943FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294408: 4E800020  blr
	return;
}

pub fn sub_83294410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294410 size=56
	// 83294410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329441C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294420: 386BCA94  addi r3, r11, -0x356c
	ctx.r[3].s64 = ctx.r[11].s64 + -13676;
	// 83294424: 4AEF4D1D  bl 0x82189140
	ctx.lr = 0x83294428;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294428: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329442C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294430: 916AE544  stw r11, -0x1abc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6844 as u32), ctx.r[11].u32 ) };
	// 83294434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329443C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294440: 4E800020  blr
	return;
}

pub fn sub_83294448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294448 size=56
	// 83294448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294454: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294458: 386BCA9C  addi r3, r11, -0x3564
	ctx.r[3].s64 = ctx.r[11].s64 + -13668;
	// 8329445C: 4AEF4CE5  bl 0x82189140
	ctx.lr = 0x83294460;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294460: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294464: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294468: 916AE548  stw r11, -0x1ab8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6840 as u32), ctx.r[11].u32 ) };
	// 8329446C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294478: 4E800020  blr
	return;
}

pub fn sub_83294480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294480 size=56
	// 83294480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329448C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294490: 386BCAA4  addi r3, r11, -0x355c
	ctx.r[3].s64 = ctx.r[11].s64 + -13660;
	// 83294494: 4AEF4CAD  bl 0x82189140
	ctx.lr = 0x83294498;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294498: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329449C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832944A0: 916AE54C  stw r11, -0x1ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6836 as u32), ctx.r[11].u32 ) };
	// 832944A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832944A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832944AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832944B0: 4E800020  blr
	return;
}

pub fn sub_832944B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832944B8 size=56
	// 832944B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832944BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832944C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832944C4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832944C8: 386BCAB4  addi r3, r11, -0x354c
	ctx.r[3].s64 = ctx.r[11].s64 + -13644;
	// 832944CC: 4AEF4C75  bl 0x82189140
	ctx.lr = 0x832944D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832944D0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832944D4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832944D8: 916AE550  stw r11, -0x1ab0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6832 as u32), ctx.r[11].u32 ) };
	// 832944DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832944E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832944E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832944E8: 4E800020  blr
	return;
}

pub fn sub_832944F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832944F0 size=56
	// 832944F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832944F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832944F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832944FC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294500: 386BCAC0  addi r3, r11, -0x3540
	ctx.r[3].s64 = ctx.r[11].s64 + -13632;
	// 83294504: 4AEF4C3D  bl 0x82189140
	ctx.lr = 0x83294508;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294508: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329450C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294510: 916AE554  stw r11, -0x1aac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6828 as u32), ctx.r[11].u32 ) };
	// 83294514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329451C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294520: 4E800020  blr
	return;
}

pub fn sub_83294528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294528 size=56
	// 83294528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329452C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294534: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294538: 386BCACC  addi r3, r11, -0x3534
	ctx.r[3].s64 = ctx.r[11].s64 + -13620;
	// 8329453C: 4AEF4C05  bl 0x82189140
	ctx.lr = 0x83294540;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294540: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294544: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294548: 916AE558  stw r11, -0x1aa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6824 as u32), ctx.r[11].u32 ) };
	// 8329454C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294558: 4E800020  blr
	return;
}

pub fn sub_83294560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294560 size=56
	// 83294560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329456C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294570: 386BCAD4  addi r3, r11, -0x352c
	ctx.r[3].s64 = ctx.r[11].s64 + -13612;
	// 83294574: 4AEF4BCD  bl 0x82189140
	ctx.lr = 0x83294578;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294578: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329457C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294580: 916AE55C  stw r11, -0x1aa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6820 as u32), ctx.r[11].u32 ) };
	// 83294584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329458C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294590: 4E800020  blr
	return;
}

pub fn sub_83294598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294598 size=56
	// 83294598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832945A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832945A4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832945A8: 386BCAD8  addi r3, r11, -0x3528
	ctx.r[3].s64 = ctx.r[11].s64 + -13608;
	// 832945AC: 4AEF4B95  bl 0x82189140
	ctx.lr = 0x832945B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832945B0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832945B4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832945B8: 916AE560  stw r11, -0x1aa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6816 as u32), ctx.r[11].u32 ) };
	// 832945BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832945C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832945C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832945C8: 4E800020  blr
	return;
}

pub fn sub_832945D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832945D0 size=56
	// 832945D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832945D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832945D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832945DC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832945E0: 386BCAE0  addi r3, r11, -0x3520
	ctx.r[3].s64 = ctx.r[11].s64 + -13600;
	// 832945E4: 4AEF4B5D  bl 0x82189140
	ctx.lr = 0x832945E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832945E8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832945EC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832945F0: 916AE564  stw r11, -0x1a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6812 as u32), ctx.r[11].u32 ) };
	// 832945F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832945F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832945FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294600: 4E800020  blr
	return;
}

pub fn sub_83294608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294608 size=56
	// 83294608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329460C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294614: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294618: 386BCAE8  addi r3, r11, -0x3518
	ctx.r[3].s64 = ctx.r[11].s64 + -13592;
	// 8329461C: 4AEF4B25  bl 0x82189140
	ctx.lr = 0x83294620;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294620: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294624: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294628: 916AE568  stw r11, -0x1a98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6808 as u32), ctx.r[11].u32 ) };
	// 8329462C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294638: 4E800020  blr
	return;
}

pub fn sub_83294640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294640 size=56
	// 83294640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329464C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294650: 386BCAF4  addi r3, r11, -0x350c
	ctx.r[3].s64 = ctx.r[11].s64 + -13580;
	// 83294654: 4AEF4AED  bl 0x82189140
	ctx.lr = 0x83294658;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294658: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329465C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294660: 916AE56C  stw r11, -0x1a94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6804 as u32), ctx.r[11].u32 ) };
	// 83294664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329466C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294670: 4E800020  blr
	return;
}

pub fn sub_83294678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294678 size=56
	// 83294678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329467C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294684: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294688: 386BCB00  addi r3, r11, -0x3500
	ctx.r[3].s64 = ctx.r[11].s64 + -13568;
	// 8329468C: 4AEF4AB5  bl 0x82189140
	ctx.lr = 0x83294690;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294690: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294694: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294698: 916AE570  stw r11, -0x1a90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6800 as u32), ctx.r[11].u32 ) };
	// 8329469C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832946A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832946A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832946A8: 4E800020  blr
	return;
}

pub fn sub_832946B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832946B0 size=56
	// 832946B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832946B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832946B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832946BC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832946C0: 386BCB0C  addi r3, r11, -0x34f4
	ctx.r[3].s64 = ctx.r[11].s64 + -13556;
	// 832946C4: 4AEF4A7D  bl 0x82189140
	ctx.lr = 0x832946C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832946C8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832946CC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832946D0: 916AE574  stw r11, -0x1a8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6796 as u32), ctx.r[11].u32 ) };
	// 832946D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832946D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832946DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832946E0: 4E800020  blr
	return;
}

pub fn sub_832946E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832946E8 size=56
	// 832946E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832946EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832946F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832946F4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832946F8: 386BCB18  addi r3, r11, -0x34e8
	ctx.r[3].s64 = ctx.r[11].s64 + -13544;
	// 832946FC: 4AEF4A45  bl 0x82189140
	ctx.lr = 0x83294700;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294700: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294704: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294708: 916AE578  stw r11, -0x1a88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6792 as u32), ctx.r[11].u32 ) };
	// 8329470C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294718: 4E800020  blr
	return;
}

pub fn sub_83294720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294720 size=56
	// 83294720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329472C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83294730: 386B3874  addi r3, r11, 0x3874
	ctx.r[3].s64 = ctx.r[11].s64 + 14452;
	// 83294734: 4AEF4A0D  bl 0x82189140
	ctx.lr = 0x83294738;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294738: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329473C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294740: 916AE57C  stw r11, -0x1a84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6788 as u32), ctx.r[11].u32 ) };
	// 83294744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329474C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294750: 4E800020  blr
	return;
}

pub fn sub_83294758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294758 size=56
	// 83294758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329475C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294764: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294768: 386BCB28  addi r3, r11, -0x34d8
	ctx.r[3].s64 = ctx.r[11].s64 + -13528;
	// 8329476C: 4AEF49D5  bl 0x82189140
	ctx.lr = 0x83294770;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294770: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294774: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294778: 916AE580  stw r11, -0x1a80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6784 as u32), ctx.r[11].u32 ) };
	// 8329477C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294788: 4E800020  blr
	return;
}

pub fn sub_83294790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294790 size=56
	// 83294790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329479C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832947A0: 386BCB38  addi r3, r11, -0x34c8
	ctx.r[3].s64 = ctx.r[11].s64 + -13512;
	// 832947A4: 4AEF499D  bl 0x82189140
	ctx.lr = 0x832947A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832947A8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832947AC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832947B0: 916AE584  stw r11, -0x1a7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6780 as u32), ctx.r[11].u32 ) };
	// 832947B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832947B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832947BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832947C0: 4E800020  blr
	return;
}

pub fn sub_832947C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832947C8 size=56
	// 832947C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832947CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832947D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832947D4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832947D8: 386BCB44  addi r3, r11, -0x34bc
	ctx.r[3].s64 = ctx.r[11].s64 + -13500;
	// 832947DC: 4AEF4965  bl 0x82189140
	ctx.lr = 0x832947E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832947E0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832947E4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832947E8: 916AE588  stw r11, -0x1a78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6776 as u32), ctx.r[11].u32 ) };
	// 832947EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832947F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832947F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832947F8: 4E800020  blr
	return;
}

pub fn sub_83294800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294800 size=56
	// 83294800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329480C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294810: 386BCB50  addi r3, r11, -0x34b0
	ctx.r[3].s64 = ctx.r[11].s64 + -13488;
	// 83294814: 4AEF492D  bl 0x82189140
	ctx.lr = 0x83294818;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294818: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329481C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294820: 916AE58C  stw r11, -0x1a74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6772 as u32), ctx.r[11].u32 ) };
	// 83294824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329482C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294830: 4E800020  blr
	return;
}

pub fn sub_83294838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294838 size=56
	// 83294838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294844: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294848: 386BCB58  addi r3, r11, -0x34a8
	ctx.r[3].s64 = ctx.r[11].s64 + -13480;
	// 8329484C: 4AEF48F5  bl 0x82189140
	ctx.lr = 0x83294850;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294850: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294854: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294858: 916AE590  stw r11, -0x1a70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6768 as u32), ctx.r[11].u32 ) };
	// 8329485C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294868: 4E800020  blr
	return;
}

pub fn sub_83294870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294870 size=56
	// 83294870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329487C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294880: 386BCB68  addi r3, r11, -0x3498
	ctx.r[3].s64 = ctx.r[11].s64 + -13464;
	// 83294884: 4AEF48BD  bl 0x82189140
	ctx.lr = 0x83294888;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294888: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329488C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294890: 916AE594  stw r11, -0x1a6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6764 as u32), ctx.r[11].u32 ) };
	// 83294894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832948A0: 4E800020  blr
	return;
}

pub fn sub_832948A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832948A8 size=56
	// 832948A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832948AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832948B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832948B4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832948B8: 386BCB78  addi r3, r11, -0x3488
	ctx.r[3].s64 = ctx.r[11].s64 + -13448;
	// 832948BC: 4AEF4885  bl 0x82189140
	ctx.lr = 0x832948C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832948C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832948C4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832948C8: 916AE598  stw r11, -0x1a68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6760 as u32), ctx.r[11].u32 ) };
	// 832948CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832948D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832948D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832948D8: 4E800020  blr
	return;
}

pub fn sub_832948E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832948E0 size=56
	// 832948E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832948E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832948E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832948EC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832948F0: 386BCB88  addi r3, r11, -0x3478
	ctx.r[3].s64 = ctx.r[11].s64 + -13432;
	// 832948F4: 4AEF484D  bl 0x82189140
	ctx.lr = 0x832948F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832948F8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832948FC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294900: 916AE59C  stw r11, -0x1a64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6756 as u32), ctx.r[11].u32 ) };
	// 83294904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329490C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294910: 4E800020  blr
	return;
}

pub fn sub_83294918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294918 size=56
	// 83294918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294924: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294928: 386BCB98  addi r3, r11, -0x3468
	ctx.r[3].s64 = ctx.r[11].s64 + -13416;
	// 8329492C: 4AEF4815  bl 0x82189140
	ctx.lr = 0x83294930;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294930: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294934: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294938: 916AE5A0  stw r11, -0x1a60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6752 as u32), ctx.r[11].u32 ) };
	// 8329493C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294948: 4E800020  blr
	return;
}

pub fn sub_83294950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294950 size=56
	// 83294950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329495C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294960: 386BCBA8  addi r3, r11, -0x3458
	ctx.r[3].s64 = ctx.r[11].s64 + -13400;
	// 83294964: 4AEF47DD  bl 0x82189140
	ctx.lr = 0x83294968;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294968: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329496C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294970: 916AE5A4  stw r11, -0x1a5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6748 as u32), ctx.r[11].u32 ) };
	// 83294974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329497C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294980: 4E800020  blr
	return;
}

pub fn sub_83294988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294988 size=56
	// 83294988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294994: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294998: 386BCBB8  addi r3, r11, -0x3448
	ctx.r[3].s64 = ctx.r[11].s64 + -13384;
	// 8329499C: 4AEF47A5  bl 0x82189140
	ctx.lr = 0x832949A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832949A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832949A4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832949A8: 916AE5A8  stw r11, -0x1a58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6744 as u32), ctx.r[11].u32 ) };
	// 832949AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832949B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832949B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832949B8: 4E800020  blr
	return;
}

pub fn sub_832949C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832949C0 size=56
	// 832949C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832949C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832949C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832949CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832949D0: 386BCBC8  addi r3, r11, -0x3438
	ctx.r[3].s64 = ctx.r[11].s64 + -13368;
	// 832949D4: 4AEF476D  bl 0x82189140
	ctx.lr = 0x832949D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832949D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832949DC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832949E0: 916AE5AC  stw r11, -0x1a54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6740 as u32), ctx.r[11].u32 ) };
	// 832949E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832949E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832949EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832949F0: 4E800020  blr
	return;
}

pub fn sub_832949F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832949F8 size=56
	// 832949F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832949FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294A04: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294A08: 386BCBD8  addi r3, r11, -0x3428
	ctx.r[3].s64 = ctx.r[11].s64 + -13352;
	// 83294A0C: 4AEF4735  bl 0x82189140
	ctx.lr = 0x83294A10;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294A10: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294A14: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294A18: 916AE5B0  stw r11, -0x1a50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6736 as u32), ctx.r[11].u32 ) };
	// 83294A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294A28: 4E800020  blr
	return;
}

pub fn sub_83294A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294A30 size=56
	// 83294A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294A3C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294A40: 386BCBE8  addi r3, r11, -0x3418
	ctx.r[3].s64 = ctx.r[11].s64 + -13336;
	// 83294A44: 4AEF46FD  bl 0x82189140
	ctx.lr = 0x83294A48;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294A48: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294A4C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294A50: 916AE5B4  stw r11, -0x1a4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6732 as u32), ctx.r[11].u32 ) };
	// 83294A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294A60: 4E800020  blr
	return;
}

pub fn sub_83294A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294A68 size=56
	// 83294A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294A74: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294A78: 386BCBF8  addi r3, r11, -0x3408
	ctx.r[3].s64 = ctx.r[11].s64 + -13320;
	// 83294A7C: 4AEF46C5  bl 0x82189140
	ctx.lr = 0x83294A80;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294A80: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294A84: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294A88: 916AE5B8  stw r11, -0x1a48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6728 as u32), ctx.r[11].u32 ) };
	// 83294A8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294A98: 4E800020  blr
	return;
}

pub fn sub_83294AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294AA0 size=56
	// 83294AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294AAC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294AB0: 386BCC08  addi r3, r11, -0x33f8
	ctx.r[3].s64 = ctx.r[11].s64 + -13304;
	// 83294AB4: 4AEF468D  bl 0x82189140
	ctx.lr = 0x83294AB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294AB8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294ABC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294AC0: 916AE5BC  stw r11, -0x1a44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6724 as u32), ctx.r[11].u32 ) };
	// 83294AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294AD0: 4E800020  blr
	return;
}

pub fn sub_83294AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294AD8 size=56
	// 83294AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294AE4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294AE8: 386BCC14  addi r3, r11, -0x33ec
	ctx.r[3].s64 = ctx.r[11].s64 + -13292;
	// 83294AEC: 4AEF4655  bl 0x82189140
	ctx.lr = 0x83294AF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294AF0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294AF4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294AF8: 916AE5C0  stw r11, -0x1a40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6720 as u32), ctx.r[11].u32 ) };
	// 83294AFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294B08: 4E800020  blr
	return;
}

pub fn sub_83294B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294B10 size=56
	// 83294B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294B1C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294B20: 386BCC20  addi r3, r11, -0x33e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13280;
	// 83294B24: 4AEF461D  bl 0x82189140
	ctx.lr = 0x83294B28;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294B28: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294B2C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294B30: 916AE5C4  stw r11, -0x1a3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6716 as u32), ctx.r[11].u32 ) };
	// 83294B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294B40: 4E800020  blr
	return;
}

pub fn sub_83294B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294B48 size=56
	// 83294B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294B54: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294B58: 386BCC2C  addi r3, r11, -0x33d4
	ctx.r[3].s64 = ctx.r[11].s64 + -13268;
	// 83294B5C: 4AEF45E5  bl 0x82189140
	ctx.lr = 0x83294B60;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294B60: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294B64: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294B68: 916AE5C8  stw r11, -0x1a38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6712 as u32), ctx.r[11].u32 ) };
	// 83294B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294B78: 4E800020  blr
	return;
}

pub fn sub_83294B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294B80 size=56
	// 83294B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294B8C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294B90: 386BCC38  addi r3, r11, -0x33c8
	ctx.r[3].s64 = ctx.r[11].s64 + -13256;
	// 83294B94: 4AEF45AD  bl 0x82189140
	ctx.lr = 0x83294B98;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294B98: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294B9C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294BA0: 916AE5CC  stw r11, -0x1a34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6708 as u32), ctx.r[11].u32 ) };
	// 83294BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294BB0: 4E800020  blr
	return;
}

pub fn sub_83294BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294BB8 size=56
	// 83294BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294BC4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294BC8: 386BCC44  addi r3, r11, -0x33bc
	ctx.r[3].s64 = ctx.r[11].s64 + -13244;
	// 83294BCC: 4AEF4575  bl 0x82189140
	ctx.lr = 0x83294BD0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294BD0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294BD4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294BD8: 916AE5D0  stw r11, -0x1a30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6704 as u32), ctx.r[11].u32 ) };
	// 83294BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294BE8: 4E800020  blr
	return;
}

pub fn sub_83294BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294BF0 size=56
	// 83294BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294BFC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294C00: 386BCC50  addi r3, r11, -0x33b0
	ctx.r[3].s64 = ctx.r[11].s64 + -13232;
	// 83294C04: 4AEF453D  bl 0x82189140
	ctx.lr = 0x83294C08;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294C08: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294C0C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294C10: 916AE5D4  stw r11, -0x1a2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6700 as u32), ctx.r[11].u32 ) };
	// 83294C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294C20: 4E800020  blr
	return;
}

pub fn sub_83294C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294C28 size=56
	// 83294C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294C34: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294C38: 386BCC5C  addi r3, r11, -0x33a4
	ctx.r[3].s64 = ctx.r[11].s64 + -13220;
	// 83294C3C: 4AEF4505  bl 0x82189140
	ctx.lr = 0x83294C40;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294C40: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294C44: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294C48: 916AE5D8  stw r11, -0x1a28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6696 as u32), ctx.r[11].u32 ) };
	// 83294C4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294C58: 4E800020  blr
	return;
}

pub fn sub_83294C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294C60 size=56
	// 83294C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294C6C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294C70: 386BCC68  addi r3, r11, -0x3398
	ctx.r[3].s64 = ctx.r[11].s64 + -13208;
	// 83294C74: 4AEF44CD  bl 0x82189140
	ctx.lr = 0x83294C78;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294C78: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294C7C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294C80: 916AE5DC  stw r11, -0x1a24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6692 as u32), ctx.r[11].u32 ) };
	// 83294C84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294C90: 4E800020  blr
	return;
}

pub fn sub_83294C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294C98 size=56
	// 83294C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294CA4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294CA8: 386BCC74  addi r3, r11, -0x338c
	ctx.r[3].s64 = ctx.r[11].s64 + -13196;
	// 83294CAC: 4AEF4495  bl 0x82189140
	ctx.lr = 0x83294CB0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294CB0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294CB4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294CB8: 916AE5E0  stw r11, -0x1a20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6688 as u32), ctx.r[11].u32 ) };
	// 83294CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294CC8: 4E800020  blr
	return;
}

pub fn sub_83294CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294CD0 size=56
	// 83294CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294CDC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294CE0: 386BCC84  addi r3, r11, -0x337c
	ctx.r[3].s64 = ctx.r[11].s64 + -13180;
	// 83294CE4: 4AEF445D  bl 0x82189140
	ctx.lr = 0x83294CE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294CE8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294CEC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294CF0: 916AE5E4  stw r11, -0x1a1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6684 as u32), ctx.r[11].u32 ) };
	// 83294CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294D00: 4E800020  blr
	return;
}

pub fn sub_83294D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294D08 size=56
	// 83294D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294D14: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294D18: 386BCC90  addi r3, r11, -0x3370
	ctx.r[3].s64 = ctx.r[11].s64 + -13168;
	// 83294D1C: 4AEF4425  bl 0x82189140
	ctx.lr = 0x83294D20;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294D20: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294D24: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294D28: 916AE5E8  stw r11, -0x1a18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6680 as u32), ctx.r[11].u32 ) };
	// 83294D2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294D38: 4E800020  blr
	return;
}

pub fn sub_83294D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294D40 size=56
	// 83294D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294D4C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294D50: 386BCC9C  addi r3, r11, -0x3364
	ctx.r[3].s64 = ctx.r[11].s64 + -13156;
	// 83294D54: 4AEF43ED  bl 0x82189140
	ctx.lr = 0x83294D58;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294D58: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294D5C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294D60: 916AE5EC  stw r11, -0x1a14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6676 as u32), ctx.r[11].u32 ) };
	// 83294D64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294D70: 4E800020  blr
	return;
}

pub fn sub_83294D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294D78 size=56
	// 83294D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294D84: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294D88: 386BCCA8  addi r3, r11, -0x3358
	ctx.r[3].s64 = ctx.r[11].s64 + -13144;
	// 83294D8C: 4AEF43B5  bl 0x82189140
	ctx.lr = 0x83294D90;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294D90: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294D94: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294D98: 916AE5F0  stw r11, -0x1a10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6672 as u32), ctx.r[11].u32 ) };
	// 83294D9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294DA8: 4E800020  blr
	return;
}

pub fn sub_83294DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294DB0 size=56
	// 83294DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294DBC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294DC0: 386BCCB0  addi r3, r11, -0x3350
	ctx.r[3].s64 = ctx.r[11].s64 + -13136;
	// 83294DC4: 4AEF437D  bl 0x82189140
	ctx.lr = 0x83294DC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294DC8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294DCC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294DD0: 916AE5F4  stw r11, -0x1a0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6668 as u32), ctx.r[11].u32 ) };
	// 83294DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294DE0: 4E800020  blr
	return;
}

pub fn sub_83294DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294DE8 size=56
	// 83294DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294DF4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294DF8: 386BCCB8  addi r3, r11, -0x3348
	ctx.r[3].s64 = ctx.r[11].s64 + -13128;
	// 83294DFC: 4AEF4345  bl 0x82189140
	ctx.lr = 0x83294E00;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294E00: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294E04: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294E08: 916AE5F8  stw r11, -0x1a08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6664 as u32), ctx.r[11].u32 ) };
	// 83294E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294E18: 4E800020  blr
	return;
}

pub fn sub_83294E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294E20 size=56
	// 83294E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294E2C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294E30: 386BCCC0  addi r3, r11, -0x3340
	ctx.r[3].s64 = ctx.r[11].s64 + -13120;
	// 83294E34: 4AEF430D  bl 0x82189140
	ctx.lr = 0x83294E38;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294E38: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294E3C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294E40: 916AE5FC  stw r11, -0x1a04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6660 as u32), ctx.r[11].u32 ) };
	// 83294E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294E50: 4E800020  blr
	return;
}

pub fn sub_83294E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294E58 size=56
	// 83294E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294E64: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294E68: 386BCCC8  addi r3, r11, -0x3338
	ctx.r[3].s64 = ctx.r[11].s64 + -13112;
	// 83294E6C: 4AEF42D5  bl 0x82189140
	ctx.lr = 0x83294E70;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294E70: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294E74: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294E78: 916AE600  stw r11, -0x1a00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6656 as u32), ctx.r[11].u32 ) };
	// 83294E7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294E88: 4E800020  blr
	return;
}

pub fn sub_83294E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294E90 size=56
	// 83294E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294E9C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294EA0: 386BCCD8  addi r3, r11, -0x3328
	ctx.r[3].s64 = ctx.r[11].s64 + -13096;
	// 83294EA4: 4AEF429D  bl 0x82189140
	ctx.lr = 0x83294EA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294EA8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294EAC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294EB0: 916AE604  stw r11, -0x19fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6652 as u32), ctx.r[11].u32 ) };
	// 83294EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294EC0: 4E800020  blr
	return;
}

pub fn sub_83294EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294EC8 size=56
	// 83294EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294ED4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294ED8: 386BCCE8  addi r3, r11, -0x3318
	ctx.r[3].s64 = ctx.r[11].s64 + -13080;
	// 83294EDC: 4AEF4265  bl 0x82189140
	ctx.lr = 0x83294EE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294EE0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294EE4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294EE8: 916AE608  stw r11, -0x19f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6648 as u32), ctx.r[11].u32 ) };
	// 83294EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294EF8: 4E800020  blr
	return;
}

pub fn sub_83294F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294F00 size=56
	// 83294F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294F0C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294F10: 386BCCF8  addi r3, r11, -0x3308
	ctx.r[3].s64 = ctx.r[11].s64 + -13064;
	// 83294F14: 4AEF422D  bl 0x82189140
	ctx.lr = 0x83294F18;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294F18: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294F1C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294F20: 916AE60C  stw r11, -0x19f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6644 as u32), ctx.r[11].u32 ) };
	// 83294F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294F30: 4E800020  blr
	return;
}

pub fn sub_83294F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294F38 size=56
	// 83294F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294F44: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294F48: 386BCD04  addi r3, r11, -0x32fc
	ctx.r[3].s64 = ctx.r[11].s64 + -13052;
	// 83294F4C: 4AEF41F5  bl 0x82189140
	ctx.lr = 0x83294F50;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294F50: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294F54: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294F58: 916AE610  stw r11, -0x19f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6640 as u32), ctx.r[11].u32 ) };
	// 83294F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294F68: 4E800020  blr
	return;
}

pub fn sub_83294F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294F70 size=56
	// 83294F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294F7C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294F80: 386BCD10  addi r3, r11, -0x32f0
	ctx.r[3].s64 = ctx.r[11].s64 + -13040;
	// 83294F84: 4AEF41BD  bl 0x82189140
	ctx.lr = 0x83294F88;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294F88: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294F8C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294F90: 916AE614  stw r11, -0x19ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6636 as u32), ctx.r[11].u32 ) };
	// 83294F94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294FA0: 4E800020  blr
	return;
}

pub fn sub_83294FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294FA8 size=56
	// 83294FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294FB4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294FB8: 386BCD1C  addi r3, r11, -0x32e4
	ctx.r[3].s64 = ctx.r[11].s64 + -13028;
	// 83294FBC: 4AEF4185  bl 0x82189140
	ctx.lr = 0x83294FC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294FC0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294FC4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83294FC8: 916AE618  stw r11, -0x19e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6632 as u32), ctx.r[11].u32 ) };
	// 83294FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83294FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83294FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83294FD8: 4E800020  blr
	return;
}

pub fn sub_83294FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294FE0 size=56
	// 83294FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83294FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83294FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83294FEC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83294FF0: 386BCD2C  addi r3, r11, -0x32d4
	ctx.r[3].s64 = ctx.r[11].s64 + -13012;
	// 83294FF4: 4AEF414D  bl 0x82189140
	ctx.lr = 0x83294FF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83294FF8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83294FFC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295000: 916AE61C  stw r11, -0x19e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6628 as u32), ctx.r[11].u32 ) };
	// 83295004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329500C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295010: 4E800020  blr
	return;
}

pub fn sub_83295018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295018 size=56
	// 83295018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329501C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295024: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295028: 386BCD3C  addi r3, r11, -0x32c4
	ctx.r[3].s64 = ctx.r[11].s64 + -12996;
	// 8329502C: 4AEF4115  bl 0x82189140
	ctx.lr = 0x83295030;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295030: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295034: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295038: 916AE620  stw r11, -0x19e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6624 as u32), ctx.r[11].u32 ) };
	// 8329503C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295048: 4E800020  blr
	return;
}

pub fn sub_83295050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295050 size=56
	// 83295050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329505C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83295060: 386B0894  addi r3, r11, 0x894
	ctx.r[3].s64 = ctx.r[11].s64 + 2196;
	// 83295064: 4AEF40DD  bl 0x82189140
	ctx.lr = 0x83295068;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295068: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329506C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295070: 916AE624  stw r11, -0x19dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6620 as u32), ctx.r[11].u32 ) };
	// 83295074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329507C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295080: 4E800020  blr
	return;
}

pub fn sub_83295088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295088 size=56
	// 83295088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329508C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295094: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83295098: 386B08A0  addi r3, r11, 0x8a0
	ctx.r[3].s64 = ctx.r[11].s64 + 2208;
	// 8329509C: 4AEF40A5  bl 0x82189140
	ctx.lr = 0x832950A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832950A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832950A4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832950A8: 916AE628  stw r11, -0x19d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6616 as u32), ctx.r[11].u32 ) };
	// 832950AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832950B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832950B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832950B8: 4E800020  blr
	return;
}

pub fn sub_832950C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832950C0 size=56
	// 832950C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832950C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832950C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832950CC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 832950D0: 386B08AC  addi r3, r11, 0x8ac
	ctx.r[3].s64 = ctx.r[11].s64 + 2220;
	// 832950D4: 4AEF406D  bl 0x82189140
	ctx.lr = 0x832950D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832950D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832950DC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832950E0: 916AE62C  stw r11, -0x19d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6612 as u32), ctx.r[11].u32 ) };
	// 832950E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832950E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832950EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832950F0: 4E800020  blr
	return;
}

pub fn sub_832950F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832950F8 size=56
	// 832950F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832950FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295104: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295108: 386BCD48  addi r3, r11, -0x32b8
	ctx.r[3].s64 = ctx.r[11].s64 + -12984;
	// 8329510C: 4AEF4035  bl 0x82189140
	ctx.lr = 0x83295110;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295110: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295114: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295118: 916AE630  stw r11, -0x19d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6608 as u32), ctx.r[11].u32 ) };
	// 8329511C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295128: 4E800020  blr
	return;
}

pub fn sub_83295130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295130 size=56
	// 83295130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329513C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 83295140: 386B0880  addi r3, r11, 0x880
	ctx.r[3].s64 = ctx.r[11].s64 + 2176;
	// 83295144: 4AEF3FFD  bl 0x82189140
	ctx.lr = 0x83295148;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295148: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329514C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295150: 916AE634  stw r11, -0x19cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6604 as u32), ctx.r[11].u32 ) };
	// 83295154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329515C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295160: 4E800020  blr
	return;
}

pub fn sub_83295168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295168 size=56
	// 83295168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329516C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295174: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295178: 386BCD54  addi r3, r11, -0x32ac
	ctx.r[3].s64 = ctx.r[11].s64 + -12972;
	// 8329517C: 4AEF3FC5  bl 0x82189140
	ctx.lr = 0x83295180;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295180: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295184: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295188: 916AE638  stw r11, -0x19c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6600 as u32), ctx.r[11].u32 ) };
	// 8329518C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295198: 4E800020  blr
	return;
}

pub fn sub_832951A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832951A0 size=56
	// 832951A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832951A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832951A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832951AC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832951B0: 386BCD60  addi r3, r11, -0x32a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12960;
	// 832951B4: 4AEF3F8D  bl 0x82189140
	ctx.lr = 0x832951B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832951B8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832951BC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832951C0: 916AE63C  stw r11, -0x19c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6596 as u32), ctx.r[11].u32 ) };
	// 832951C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832951C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832951CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832951D0: 4E800020  blr
	return;
}

pub fn sub_832951D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832951D8 size=56
	// 832951D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832951DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832951E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832951E4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832951E8: 386BCD68  addi r3, r11, -0x3298
	ctx.r[3].s64 = ctx.r[11].s64 + -12952;
	// 832951EC: 4AEF3F55  bl 0x82189140
	ctx.lr = 0x832951F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832951F0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832951F4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832951F8: 916AE640  stw r11, -0x19c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6592 as u32), ctx.r[11].u32 ) };
	// 832951FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295208: 4E800020  blr
	return;
}

pub fn sub_83295210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295210 size=56
	// 83295210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329521C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295220: 386BCD70  addi r3, r11, -0x3290
	ctx.r[3].s64 = ctx.r[11].s64 + -12944;
	// 83295224: 4AEF3F1D  bl 0x82189140
	ctx.lr = 0x83295228;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295228: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329522C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295230: 916AE644  stw r11, -0x19bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6588 as u32), ctx.r[11].u32 ) };
	// 83295234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329523C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295240: 4E800020  blr
	return;
}

pub fn sub_83295248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295248 size=56
	// 83295248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329524C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295254: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295258: 386BCD7C  addi r3, r11, -0x3284
	ctx.r[3].s64 = ctx.r[11].s64 + -12932;
	// 8329525C: 4AEF3EE5  bl 0x82189140
	ctx.lr = 0x83295260;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295260: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295264: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295268: 916AE648  stw r11, -0x19b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6584 as u32), ctx.r[11].u32 ) };
	// 8329526C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295278: 4E800020  blr
	return;
}

pub fn sub_83295280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295280 size=56
	// 83295280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329528C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295290: 386BCD88  addi r3, r11, -0x3278
	ctx.r[3].s64 = ctx.r[11].s64 + -12920;
	// 83295294: 4AEF3EAD  bl 0x82189140
	ctx.lr = 0x83295298;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295298: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329529C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832952A0: 916AE64C  stw r11, -0x19b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6580 as u32), ctx.r[11].u32 ) };
	// 832952A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832952A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832952AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832952B0: 4E800020  blr
	return;
}

pub fn sub_832952B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832952B8 size=56
	// 832952B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832952BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832952C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832952C4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832952C8: 386BCD94  addi r3, r11, -0x326c
	ctx.r[3].s64 = ctx.r[11].s64 + -12908;
	// 832952CC: 4AEF3E75  bl 0x82189140
	ctx.lr = 0x832952D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832952D0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832952D4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832952D8: 916AE650  stw r11, -0x19b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6576 as u32), ctx.r[11].u32 ) };
	// 832952DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832952E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832952E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832952E8: 4E800020  blr
	return;
}

pub fn sub_832952F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832952F0 size=56
	// 832952F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832952F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832952F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832952FC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295300: 386BCDA0  addi r3, r11, -0x3260
	ctx.r[3].s64 = ctx.r[11].s64 + -12896;
	// 83295304: 4AEF3E3D  bl 0x82189140
	ctx.lr = 0x83295308;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295308: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329530C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295310: 916AE654  stw r11, -0x19ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6572 as u32), ctx.r[11].u32 ) };
	// 83295314: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329531C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295320: 4E800020  blr
	return;
}

pub fn sub_83295328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295328 size=56
	// 83295328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329532C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295334: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295338: 386BCDAC  addi r3, r11, -0x3254
	ctx.r[3].s64 = ctx.r[11].s64 + -12884;
	// 8329533C: 4AEF3E05  bl 0x82189140
	ctx.lr = 0x83295340;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295340: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295344: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295348: 916AE658  stw r11, -0x19a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6568 as u32), ctx.r[11].u32 ) };
	// 8329534C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295358: 4E800020  blr
	return;
}

pub fn sub_83295360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295360 size=56
	// 83295360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329536C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295370: 386BCDB8  addi r3, r11, -0x3248
	ctx.r[3].s64 = ctx.r[11].s64 + -12872;
	// 83295374: 4AEF3DCD  bl 0x82189140
	ctx.lr = 0x83295378;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295378: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329537C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295380: 916AE65C  stw r11, -0x19a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6564 as u32), ctx.r[11].u32 ) };
	// 83295384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329538C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295390: 4E800020  blr
	return;
}

pub fn sub_83295398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295398 size=56
	// 83295398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329539C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832953A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832953A4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832953A8: 386BCDC4  addi r3, r11, -0x323c
	ctx.r[3].s64 = ctx.r[11].s64 + -12860;
	// 832953AC: 4AEF3D95  bl 0x82189140
	ctx.lr = 0x832953B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832953B0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832953B4: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832953B8: 916AE660  stw r11, -0x19a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6560 as u32), ctx.r[11].u32 ) };
	// 832953BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832953C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832953C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832953C8: 4E800020  blr
	return;
}

pub fn sub_832953D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832953D0 size=56
	// 832953D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832953D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832953D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832953DC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832953E0: 386BCDD4  addi r3, r11, -0x322c
	ctx.r[3].s64 = ctx.r[11].s64 + -12844;
	// 832953E4: 4AEF3D5D  bl 0x82189140
	ctx.lr = 0x832953E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832953E8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832953EC: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 832953F0: 916AE664  stw r11, -0x199c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6556 as u32), ctx.r[11].u32 ) };
	// 832953F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832953F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832953FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295400: 4E800020  blr
	return;
}

pub fn sub_83295408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295408 size=56
	// 83295408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329540C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295414: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295418: 386BCDE0  addi r3, r11, -0x3220
	ctx.r[3].s64 = ctx.r[11].s64 + -12832;
	// 8329541C: 4AEF3D25  bl 0x82189140
	ctx.lr = 0x83295420;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295420: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295424: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295428: 916AE668  stw r11, -0x1998(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6552 as u32), ctx.r[11].u32 ) };
	// 8329542C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295438: 4E800020  blr
	return;
}

pub fn sub_83295440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295440 size=56
	// 83295440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329544C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295450: 386BCDEC  addi r3, r11, -0x3214
	ctx.r[3].s64 = ctx.r[11].s64 + -12820;
	// 83295454: 4AEF3CED  bl 0x82189140
	ctx.lr = 0x83295458;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295458: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329545C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295460: 916AE66C  stw r11, -0x1994(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6548 as u32), ctx.r[11].u32 ) };
	// 83295464: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329546C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295470: 4E800020  blr
	return;
}

pub fn sub_83295478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295478 size=56
	// 83295478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329547C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295484: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295488: 386BCDF8  addi r3, r11, -0x3208
	ctx.r[3].s64 = ctx.r[11].s64 + -12808;
	// 8329548C: 4AEF3CB5  bl 0x82189140
	ctx.lr = 0x83295490;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295490: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295494: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83295498: 916AE670  stw r11, -0x1990(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6544 as u32), ctx.r[11].u32 ) };
	// 8329549C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832954A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832954A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832954A8: 4E800020  blr
	return;
}

pub fn sub_832954B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832954B0 size=56
	// 832954B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832954B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832954B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832954BC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832954C0: 386BCE04  addi r3, r11, -0x31fc
	ctx.r[3].s64 = ctx.r[11].s64 + -12796;
	// 832954C4: 4AEF3C7D  bl 0x82189140
	ctx.lr = 0x832954C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832954C8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832954CC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832954D0: 916AE674  stw r11, -0x198c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6540 as u32), ctx.r[11].u32 ) };
	// 832954D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832954D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832954DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832954E0: 4E800020  blr
	return;
}

pub fn sub_832954E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832954E8 size=56
	// 832954E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832954EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832954F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832954F4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832954F8: 386BCE18  addi r3, r11, -0x31e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12776;
	// 832954FC: 4AEF3C45  bl 0x82189140
	ctx.lr = 0x83295500;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295500: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295504: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295508: 916AE678  stw r11, -0x1988(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6536 as u32), ctx.r[11].u32 ) };
	// 8329550C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295518: 4E800020  blr
	return;
}

pub fn sub_83295520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295520 size=56
	// 83295520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329552C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295530: 386BCE2C  addi r3, r11, -0x31d4
	ctx.r[3].s64 = ctx.r[11].s64 + -12756;
	// 83295534: 4AEF3C0D  bl 0x82189140
	ctx.lr = 0x83295538;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295538: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329553C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295540: 916AE67C  stw r11, -0x1984(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6532 as u32), ctx.r[11].u32 ) };
	// 83295544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329554C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295550: 4E800020  blr
	return;
}

pub fn sub_83295558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295558 size=56
	// 83295558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329555C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295564: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295568: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 8329556C: 4AEF3BD5  bl 0x82189140
	ctx.lr = 0x83295570;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295570: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295574: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295578: 916AE680  stw r11, -0x1980(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6528 as u32), ctx.r[11].u32 ) };
	// 8329557C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295588: 4E800020  blr
	return;
}

pub fn sub_83295590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295590 size=56
	// 83295590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329559C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832955A0: 386BCE50  addi r3, r11, -0x31b0
	ctx.r[3].s64 = ctx.r[11].s64 + -12720;
	// 832955A4: 4AEF3B9D  bl 0x82189140
	ctx.lr = 0x832955A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832955A8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832955AC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832955B0: 916AE684  stw r11, -0x197c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6524 as u32), ctx.r[11].u32 ) };
	// 832955B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832955B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832955BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832955C0: 4E800020  blr
	return;
}

pub fn sub_832955C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832955C8 size=56
	// 832955C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832955CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832955D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832955D4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832955D8: 386BCE60  addi r3, r11, -0x31a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12704;
	// 832955DC: 4AEF3B65  bl 0x82189140
	ctx.lr = 0x832955E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832955E0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832955E4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832955E8: 916AE688  stw r11, -0x1978(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6520 as u32), ctx.r[11].u32 ) };
	// 832955EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832955F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832955F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832955F8: 4E800020  blr
	return;
}

pub fn sub_83295600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295600 size=56
	// 83295600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329560C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295610: 386BCE74  addi r3, r11, -0x318c
	ctx.r[3].s64 = ctx.r[11].s64 + -12684;
	// 83295614: 4AEF3B2D  bl 0x82189140
	ctx.lr = 0x83295618;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295618: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329561C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295620: 916AE68C  stw r11, -0x1974(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6516 as u32), ctx.r[11].u32 ) };
	// 83295624: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329562C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295630: 4E800020  blr
	return;
}

pub fn sub_83295638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295638 size=56
	// 83295638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329563C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295644: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295648: 386BCE8C  addi r3, r11, -0x3174
	ctx.r[3].s64 = ctx.r[11].s64 + -12660;
	// 8329564C: 4AEF3AF5  bl 0x82189140
	ctx.lr = 0x83295650;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295650: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295654: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295658: 916AE690  stw r11, -0x1970(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6512 as u32), ctx.r[11].u32 ) };
	// 8329565C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295668: 4E800020  blr
	return;
}

pub fn sub_83295670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295670 size=56
	// 83295670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329567C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295680: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 83295684: 4AEF3ABD  bl 0x82189140
	ctx.lr = 0x83295688;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295688: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329568C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295690: 916AE694  stw r11, -0x196c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6508 as u32), ctx.r[11].u32 ) };
	// 83295694: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329569C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832956A0: 4E800020  blr
	return;
}

pub fn sub_832956A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832956A8 size=56
	// 832956A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832956AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832956B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832956B4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832956B8: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 832956BC: 4AEF3A85  bl 0x82189140
	ctx.lr = 0x832956C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832956C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832956C4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832956C8: 916AE698  stw r11, -0x1968(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6504 as u32), ctx.r[11].u32 ) };
	// 832956CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832956D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832956D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832956D8: 4E800020  blr
	return;
}

pub fn sub_832956E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832956E0 size=56
	// 832956E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832956E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832956E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832956EC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832956F0: 386BCED0  addi r3, r11, -0x3130
	ctx.r[3].s64 = ctx.r[11].s64 + -12592;
	// 832956F4: 4AEF3A4D  bl 0x82189140
	ctx.lr = 0x832956F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 832956F8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832956FC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295700: 916AE69C  stw r11, -0x1964(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6500 as u32), ctx.r[11].u32 ) };
	// 83295704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329570C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295710: 4E800020  blr
	return;
}

pub fn sub_83295718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295718 size=56
	// 83295718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329571C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295724: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295728: 386BCEEC  addi r3, r11, -0x3114
	ctx.r[3].s64 = ctx.r[11].s64 + -12564;
	// 8329572C: 4AEF3A15  bl 0x82189140
	ctx.lr = 0x83295730;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295730: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295734: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295738: 916AE6A0  stw r11, -0x1960(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6496 as u32), ctx.r[11].u32 ) };
	// 8329573C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295748: 4E800020  blr
	return;
}

pub fn sub_83295750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295750 size=56
	// 83295750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329575C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295760: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 83295764: 4AEF39DD  bl 0x82189140
	ctx.lr = 0x83295768;
	crate::recompiler::externs::call(&mut ctx, base, 0x82189140);
	// 83295768: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329576C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295770: 916AE6A4  stw r11, -0x195c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6492 as u32), ctx.r[11].u32 ) };
	// 83295774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329577C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295780: 4E800020  blr
	return;
}

