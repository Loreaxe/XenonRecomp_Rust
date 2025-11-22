pub fn sub_8326C210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C210 size=12
	// 8326C210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C250 size=12
	// 8326C250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C290 size=12
	// 8326C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C2D0 size=12
	// 8326C2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C310 size=12
	// 8326C310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C380 size=12
	// 8326C380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C3C0 size=12
	// 8326C3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C400 size=12
	// 8326C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326C460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C460 size=56
	// 8326C460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C46C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C474: 386B2DF8  addi r3, r11, 0x2df8
	ctx.r[3].s64 = ctx.r[11].s64 + 11768;
	// 8326C478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C47C: 4AF878DD  bl 0x821f3d58
	ctx.lr = 0x8326C480;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C484: 906AC680  stw r3, -0x3980(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14720 as u32), ctx.r[3].u32 ) };
	// 8326C488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C494: 4E800020  blr
	return;
}

pub fn sub_8326C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C498 size=56
	// 8326C498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C4A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326C4A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C4AC: 386B0F7C  addi r3, r11, 0xf7c
	ctx.r[3].s64 = ctx.r[11].s64 + 3964;
	// 8326C4B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C4B4: 4AF878A5  bl 0x821f3d58
	ctx.lr = 0x8326C4B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C4B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C4BC: 906AC684  stw r3, -0x397c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14716 as u32), ctx.r[3].u32 ) };
	// 8326C4C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C4CC: 4E800020  blr
	return;
}

pub fn sub_8326C4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C4D0 size=56
	// 8326C4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C4D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C4DC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C4E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C4E4: 386B70CC  addi r3, r11, 0x70cc
	ctx.r[3].s64 = ctx.r[11].s64 + 28876;
	// 8326C4E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C4EC: 4AF8786D  bl 0x821f3d58
	ctx.lr = 0x8326C4F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C4F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C4F4: 906AC688  stw r3, -0x3978(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14712 as u32), ctx.r[3].u32 ) };
	// 8326C4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C504: 4E800020  blr
	return;
}

pub fn sub_8326C508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C508 size=56
	// 8326C508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C514: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C51C: 386B70E8  addi r3, r11, 0x70e8
	ctx.r[3].s64 = ctx.r[11].s64 + 28904;
	// 8326C520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C524: 4AF87835  bl 0x821f3d58
	ctx.lr = 0x8326C528;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C52C: 906AC68C  stw r3, -0x3974(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14708 as u32), ctx.r[3].u32 ) };
	// 8326C530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C53C: 4E800020  blr
	return;
}

pub fn sub_8326C540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C540 size=56
	// 8326C540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C54C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C550: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C554: 386B70F4  addi r3, r11, 0x70f4
	ctx.r[3].s64 = ctx.r[11].s64 + 28916;
	// 8326C558: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C55C: 4AF877FD  bl 0x821f3d58
	ctx.lr = 0x8326C560;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C564: 906AC690  stw r3, -0x3970(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14704 as u32), ctx.r[3].u32 ) };
	// 8326C568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C56C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C574: 4E800020  blr
	return;
}

pub fn sub_8326C578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C578 size=56
	// 8326C578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C584: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C588: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C58C: 386B7100  addi r3, r11, 0x7100
	ctx.r[3].s64 = ctx.r[11].s64 + 28928;
	// 8326C590: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C594: 4AF877C5  bl 0x821f3d58
	ctx.lr = 0x8326C598;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C598: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C59C: 906AC694  stw r3, -0x396c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14700 as u32), ctx.r[3].u32 ) };
	// 8326C5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C5AC: 4E800020  blr
	return;
}

pub fn sub_8326C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C5B0 size=56
	// 8326C5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C5B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C5BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C5C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C5C4: 386B710C  addi r3, r11, 0x710c
	ctx.r[3].s64 = ctx.r[11].s64 + 28940;
	// 8326C5C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C5CC: 4AF8778D  bl 0x821f3d58
	ctx.lr = 0x8326C5D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C5D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C5D4: 906AC698  stw r3, -0x3968(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14696 as u32), ctx.r[3].u32 ) };
	// 8326C5D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C5E4: 4E800020  blr
	return;
}

pub fn sub_8326C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C5E8 size=56
	// 8326C5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C5F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C5F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C5F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C5FC: 386B712C  addi r3, r11, 0x712c
	ctx.r[3].s64 = ctx.r[11].s64 + 28972;
	// 8326C600: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C604: 4AF87755  bl 0x821f3d58
	ctx.lr = 0x8326C608;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C60C: 906AC69C  stw r3, -0x3964(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14692 as u32), ctx.r[3].u32 ) };
	// 8326C610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C61C: 4E800020  blr
	return;
}

pub fn sub_8326C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C620 size=56
	// 8326C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C62C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C630: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C634: 386B7158  addi r3, r11, 0x7158
	ctx.r[3].s64 = ctx.r[11].s64 + 29016;
	// 8326C638: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C63C: 4AF8771D  bl 0x821f3d58
	ctx.lr = 0x8326C640;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C644: 906AC6A0  stw r3, -0x3960(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14688 as u32), ctx.r[3].u32 ) };
	// 8326C648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C654: 4E800020  blr
	return;
}

pub fn sub_8326C658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C658 size=56
	// 8326C658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C664: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C668: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C66C: 386B7164  addi r3, r11, 0x7164
	ctx.r[3].s64 = ctx.r[11].s64 + 29028;
	// 8326C670: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C674: 4AF876E5  bl 0x821f3d58
	ctx.lr = 0x8326C678;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C67C: 906AC6A4  stw r3, -0x395c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14684 as u32), ctx.r[3].u32 ) };
	// 8326C680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C68C: 4E800020  blr
	return;
}

pub fn sub_8326C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C690 size=56
	// 8326C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C69C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C6A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C6A4: 386B7174  addi r3, r11, 0x7174
	ctx.r[3].s64 = ctx.r[11].s64 + 29044;
	// 8326C6A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C6AC: 4AF876AD  bl 0x821f3d58
	ctx.lr = 0x8326C6B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C6B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C6B4: 906AC6A8  stw r3, -0x3958(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14680 as u32), ctx.r[3].u32 ) };
	// 8326C6B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C6C4: 4E800020  blr
	return;
}

pub fn sub_8326C6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C6C8 size=56
	// 8326C6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C6D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326C6D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C6DC: 386B7140  addi r3, r11, 0x7140
	ctx.r[3].s64 = ctx.r[11].s64 + 28992;
	// 8326C6E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C6E4: 4AF87675  bl 0x821f3d58
	ctx.lr = 0x8326C6E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C6E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C6EC: 906AC6AC  stw r3, -0x3954(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14676 as u32), ctx.r[3].u32 ) };
	// 8326C6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C6FC: 4E800020  blr
	return;
}

pub fn sub_8326C700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C700 size=56
	// 8326C700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C70C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C710: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C714: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326C718: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C71C: 4AF8763D  bl 0x821f3d58
	ctx.lr = 0x8326C720;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C720: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C724: 906AC6B0  stw r3, -0x3950(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14672 as u32), ctx.r[3].u32 ) };
	// 8326C728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C734: 4E800020  blr
	return;
}

pub fn sub_8326C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C738 size=56
	// 8326C738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C748: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C74C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326C750: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C754: 4AF87605  bl 0x821f3d58
	ctx.lr = 0x8326C758;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C758: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C75C: 906AC6B4  stw r3, -0x394c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14668 as u32), ctx.r[3].u32 ) };
	// 8326C760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C76C: 4E800020  blr
	return;
}

pub fn sub_8326C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C770 size=56
	// 8326C770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C77C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C780: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C784: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326C788: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C78C: 4AF875CD  bl 0x821f3d58
	ctx.lr = 0x8326C790;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C794: 906AC6B8  stw r3, -0x3948(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14664 as u32), ctx.r[3].u32 ) };
	// 8326C798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C7A4: 4E800020  blr
	return;
}

pub fn sub_8326C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C7A8 size=56
	// 8326C7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C7B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C7B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C7B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C7BC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326C7C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C7C4: 4AF87595  bl 0x821f3d58
	ctx.lr = 0x8326C7C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C7C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C7CC: 906AC6BC  stw r3, -0x3944(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14660 as u32), ctx.r[3].u32 ) };
	// 8326C7D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C7DC: 4E800020  blr
	return;
}

pub fn sub_8326C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C7E0 size=56
	// 8326C7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C7E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C7EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C7F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C7F4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326C7F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C7FC: 4AF8755D  bl 0x821f3d58
	ctx.lr = 0x8326C800;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C804: 906AC6C0  stw r3, -0x3940(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14656 as u32), ctx.r[3].u32 ) };
	// 8326C808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C814: 4E800020  blr
	return;
}

pub fn sub_8326C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C818 size=56
	// 8326C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C824: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C828: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C82C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326C830: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C834: 4AF87525  bl 0x821f3d58
	ctx.lr = 0x8326C838;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C838: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C83C: 906AC6C4  stw r3, -0x393c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14652 as u32), ctx.r[3].u32 ) };
	// 8326C840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C84C: 4E800020  blr
	return;
}

pub fn sub_8326C850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C850 size=56
	// 8326C850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C85C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C860: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C864: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326C868: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C86C: 4AF874ED  bl 0x821f3d58
	ctx.lr = 0x8326C870;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C870: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C874: 906AC6C8  stw r3, -0x3938(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14648 as u32), ctx.r[3].u32 ) };
	// 8326C878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C884: 4E800020  blr
	return;
}

pub fn sub_8326C888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C888 size=56
	// 8326C888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C894: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C898: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C89C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326C8A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C8A4: 4AF874B5  bl 0x821f3d58
	ctx.lr = 0x8326C8A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C8A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C8AC: 906AC6CC  stw r3, -0x3934(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14644 as u32), ctx.r[3].u32 ) };
	// 8326C8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C8BC: 4E800020  blr
	return;
}

pub fn sub_8326C8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C8C0 size=56
	// 8326C8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C8CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C8D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C8D4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326C8D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C8DC: 4AF8747D  bl 0x821f3d58
	ctx.lr = 0x8326C8E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C8E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C8E4: 906AC6D0  stw r3, -0x3930(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14640 as u32), ctx.r[3].u32 ) };
	// 8326C8E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C8F4: 4E800020  blr
	return;
}

pub fn sub_8326C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C8F8 size=56
	// 8326C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C908: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C90C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326C910: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C914: 4AF87445  bl 0x821f3d58
	ctx.lr = 0x8326C918;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C918: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C91C: 906AC6D4  stw r3, -0x392c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14636 as u32), ctx.r[3].u32 ) };
	// 8326C920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C92C: 4E800020  blr
	return;
}

pub fn sub_8326C930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C930 size=56
	// 8326C930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C93C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C940: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C944: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326C948: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C94C: 4AF8740D  bl 0x821f3d58
	ctx.lr = 0x8326C950;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C954: 906AC6D8  stw r3, -0x3928(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14632 as u32), ctx.r[3].u32 ) };
	// 8326C958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C95C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C964: 4E800020  blr
	return;
}

pub fn sub_8326C968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C968 size=56
	// 8326C968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C974: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C978: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C97C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326C980: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C984: 4AF873D5  bl 0x821f3d58
	ctx.lr = 0x8326C988;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C98C: 906AC6DC  stw r3, -0x3924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14628 as u32), ctx.r[3].u32 ) };
	// 8326C990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C99C: 4E800020  blr
	return;
}

pub fn sub_8326C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C9A0 size=56
	// 8326C9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C9A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C9AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C9B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C9B4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326C9B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C9BC: 4AF8739D  bl 0x821f3d58
	ctx.lr = 0x8326C9C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C9C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C9C4: 906AC6E0  stw r3, -0x3920(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14624 as u32), ctx.r[3].u32 ) };
	// 8326C9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326C9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326C9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326C9D4: 4E800020  blr
	return;
}

pub fn sub_8326C9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326C9D8 size=56
	// 8326C9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326C9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326C9E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326C9E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326C9E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326C9EC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326C9F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326C9F4: 4AF87365  bl 0x821f3d58
	ctx.lr = 0x8326C9F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326C9F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326C9FC: 906AC6E4  stw r3, -0x391c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14620 as u32), ctx.r[3].u32 ) };
	// 8326CA00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CA0C: 4E800020  blr
	return;
}

pub fn sub_8326CA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CA10 size=56
	// 8326CA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CA18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CA1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CA20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CA24: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326CA28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CA2C: 4AF8732D  bl 0x821f3d58
	ctx.lr = 0x8326CA30;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326CA30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CA34: 906AC6E8  stw r3, -0x3918(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14616 as u32), ctx.r[3].u32 ) };
	// 8326CA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CA44: 4E800020  blr
	return;
}

pub fn sub_8326CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CA48 size=56
	// 8326CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CA54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CA58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CA5C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326CA60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CA64: 4AF872F5  bl 0x821f3d58
	ctx.lr = 0x8326CA68;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326CA68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CA6C: 906AC6EC  stw r3, -0x3914(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14612 as u32), ctx.r[3].u32 ) };
	// 8326CA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CA7C: 4E800020  blr
	return;
}

pub fn sub_8326CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CA80 size=56
	// 8326CA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CA90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CA94: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326CA98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CA9C: 4AF872BD  bl 0x821f3d58
	ctx.lr = 0x8326CAA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326CAA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CAA4: 906AC6F0  stw r3, -0x3910(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14608 as u32), ctx.r[3].u32 ) };
	// 8326CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CAB4: 4E800020  blr
	return;
}

pub fn sub_8326CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CAB8 size=56
	// 8326CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CACC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326CAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CAD4: 4AF87285  bl 0x821f3d58
	ctx.lr = 0x8326CAD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326CAD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CADC: 906AC6F4  stw r3, -0x390c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14604 as u32), ctx.r[3].u32 ) };
	// 8326CAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CAEC: 4E800020  blr
	return;
}

pub fn sub_8326CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CAF0 size=56
	// 8326CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CB04: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326CB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CB0C: 4AF8724D  bl 0x821f3d58
	ctx.lr = 0x8326CB10;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326CB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CB14: 906AC6F8  stw r3, -0x3908(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14600 as u32), ctx.r[3].u32 ) };
	// 8326CB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CB24: 4E800020  blr
	return;
}

pub fn sub_8326CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CB28 size=56
	// 8326CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CB3C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326CB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CB44: 4AF87215  bl 0x821f3d58
	ctx.lr = 0x8326CB48;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326CB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CB4C: 906AC6FC  stw r3, -0x3904(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14596 as u32), ctx.r[3].u32 ) };
	// 8326CB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CB5C: 4E800020  blr
	return;
}

pub fn sub_8326CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CB60 size=56
	// 8326CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326CB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326CB74: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326CB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326CB7C: 4AF871DD  bl 0x821f3d58
	ctx.lr = 0x8326CB80;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326CB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326CB84: 906AC700  stw r3, -0x3900(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14592 as u32), ctx.r[3].u32 ) };
	// 8326CB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326CB94: 4E800020  blr
	return;
}

pub fn sub_8326CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326CB98 size=1344
	// 8326CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326CBA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326CBA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326CBA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326CBAC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8326CBB0: 3BEBC708  addi r31, r11, -0x38f8
	ctx.r[31].s64 = ctx.r[11].s64 + -14584;
	// 8326CBB4: 388AA530  addi r4, r10, -0x5ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -23248;
	// 8326CBB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326CBBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBC0: 4AFC0311  bl 0x8222ced0
	ctx.lr = 0x8326CBC4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CBC4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CBC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBCC: 38892668  addi r4, r9, 0x2668
	ctx.r[4].s64 = ctx.r[9].s64 + 9832;
	// 8326CBD0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8326CBD4: 4AFC02FD  bl 0x8222ced0
	ctx.lr = 0x8326CBD8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CBD8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CBDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBE0: 38882660  addi r4, r8, 0x2660
	ctx.r[4].s64 = ctx.r[8].s64 + 9824;
	// 8326CBE4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8326CBE8: 4AFC02E9  bl 0x8222ced0
	ctx.lr = 0x8326CBEC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CBEC: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CBF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CBF4: 3887265C  addi r4, r7, 0x265c
	ctx.r[4].s64 = ctx.r[7].s64 + 9820;
	// 8326CBF8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8326CBFC: 4AFC02D5  bl 0x8222ced0
	ctx.lr = 0x8326CC00;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC00: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC08: 38862658  addi r4, r6, 0x2658
	ctx.r[4].s64 = ctx.r[6].s64 + 9816;
	// 8326CC0C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8326CC10: 4AFC02C1  bl 0x8222ced0
	ctx.lr = 0x8326CC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC14: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CC18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC1C: 3884264C  addi r4, r4, 0x264c
	ctx.r[4].s64 = ctx.r[4].s64 + 9804;
	// 8326CC20: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8326CC24: 4AFC02AD  bl 0x8222ced0
	ctx.lr = 0x8326CC28;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC28: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CC2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC30: 38832640  addi r4, r3, 0x2640
	ctx.r[4].s64 = ctx.r[3].s64 + 9792;
	// 8326CC34: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8326CC38: 4AFC0299  bl 0x8222ced0
	ctx.lr = 0x8326CC3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CC40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC44: 388B2638  addi r4, r11, 0x2638
	ctx.r[4].s64 = ctx.r[11].s64 + 9784;
	// 8326CC48: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8326CC4C: 4AFC0285  bl 0x8222ced0
	ctx.lr = 0x8326CC50;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC50: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CC54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC58: 388A2634  addi r4, r10, 0x2634
	ctx.r[4].s64 = ctx.r[10].s64 + 9780;
	// 8326CC5C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8326CC60: 4AFC0271  bl 0x8222ced0
	ctx.lr = 0x8326CC64;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC64: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CC68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC6C: 38892624  addi r4, r9, 0x2624
	ctx.r[4].s64 = ctx.r[9].s64 + 9764;
	// 8326CC70: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8326CC74: 4AFC025D  bl 0x8222ced0
	ctx.lr = 0x8326CC78;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC78: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CC7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC80: 38882618  addi r4, r8, 0x2618
	ctx.r[4].s64 = ctx.r[8].s64 + 9752;
	// 8326CC84: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8326CC88: 4AFC0249  bl 0x8222ced0
	ctx.lr = 0x8326CC8C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CC8C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CC90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CC94: 38872610  addi r4, r7, 0x2610
	ctx.r[4].s64 = ctx.r[7].s64 + 9744;
	// 8326CC98: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8326CC9C: 4AFC0235  bl 0x8222ced0
	ctx.lr = 0x8326CCA0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CCA0: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CCA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCA8: 38862608  addi r4, r6, 0x2608
	ctx.r[4].s64 = ctx.r[6].s64 + 9736;
	// 8326CCAC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8326CCB0: 4AFC0221  bl 0x8222ced0
	ctx.lr = 0x8326CCB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CCB4: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CCB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCBC: 388425FC  addi r4, r4, 0x25fc
	ctx.r[4].s64 = ctx.r[4].s64 + 9724;
	// 8326CCC0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8326CCC4: 4AFC020D  bl 0x8222ced0
	ctx.lr = 0x8326CCC8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CCC8: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CCCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCD0: 388325F4  addi r4, r3, 0x25f4
	ctx.r[4].s64 = ctx.r[3].s64 + 9716;
	// 8326CCD4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8326CCD8: 4AFC01F9  bl 0x8222ced0
	ctx.lr = 0x8326CCDC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CCDC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326CCE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCE4: 388B3AB0  addi r4, r11, 0x3ab0
	ctx.r[4].s64 = ctx.r[11].s64 + 15024;
	// 8326CCE8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8326CCEC: 4AFC01E5  bl 0x8222ced0
	ctx.lr = 0x8326CCF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CCF0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CCF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CCF8: 388A25E8  addi r4, r10, 0x25e8
	ctx.r[4].s64 = ctx.r[10].s64 + 9704;
	// 8326CCFC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8326CD00: 4AFC01D1  bl 0x8222ced0
	ctx.lr = 0x8326CD04;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD04: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CD08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD0C: 388908B8  addi r4, r9, 0x8b8
	ctx.r[4].s64 = ctx.r[9].s64 + 2232;
	// 8326CD10: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 8326CD14: 4AFC01BD  bl 0x8222ced0
	ctx.lr = 0x8326CD18;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD18: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CD1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD20: 388825E0  addi r4, r8, 0x25e0
	ctx.r[4].s64 = ctx.r[8].s64 + 9696;
	// 8326CD24: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 8326CD28: 4AFC01A9  bl 0x8222ced0
	ctx.lr = 0x8326CD2C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD2C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CD30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD34: 388725D0  addi r4, r7, 0x25d0
	ctx.r[4].s64 = ctx.r[7].s64 + 9680;
	// 8326CD38: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 8326CD3C: 4AFC0195  bl 0x8222ced0
	ctx.lr = 0x8326CD40;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD40: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CD44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD48: 388625C4  addi r4, r6, 0x25c4
	ctx.r[4].s64 = ctx.r[6].s64 + 9668;
	// 8326CD4C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8326CD50: 4AFC0181  bl 0x8222ced0
	ctx.lr = 0x8326CD54;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD54: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CD58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD5C: 388425B8  addi r4, r4, 0x25b8
	ctx.r[4].s64 = ctx.r[4].s64 + 9656;
	// 8326CD60: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8326CD64: 4AFC016D  bl 0x8222ced0
	ctx.lr = 0x8326CD68;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD68: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CD6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD70: 388325B0  addi r4, r3, 0x25b0
	ctx.r[4].s64 = ctx.r[3].s64 + 9648;
	// 8326CD74: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8326CD78: 4AFC0159  bl 0x8222ced0
	ctx.lr = 0x8326CD7C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD7C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CD80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD84: 388B25A8  addi r4, r11, 0x25a8
	ctx.r[4].s64 = ctx.r[11].s64 + 9640;
	// 8326CD88: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 8326CD8C: 4AFC0145  bl 0x8222ced0
	ctx.lr = 0x8326CD90;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CD90: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CD94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CD98: 388A25A0  addi r4, r10, 0x25a0
	ctx.r[4].s64 = ctx.r[10].s64 + 9632;
	// 8326CD9C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8326CDA0: 4AFC0131  bl 0x8222ced0
	ctx.lr = 0x8326CDA4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CDA4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CDA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDAC: 38892590  addi r4, r9, 0x2590
	ctx.r[4].s64 = ctx.r[9].s64 + 9616;
	// 8326CDB0: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 8326CDB4: 4AFC011D  bl 0x8222ced0
	ctx.lr = 0x8326CDB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CDB8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CDBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDC0: 3888257C  addi r4, r8, 0x257c
	ctx.r[4].s64 = ctx.r[8].s64 + 9596;
	// 8326CDC4: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8326CDC8: 4AFC0109  bl 0x8222ced0
	ctx.lr = 0x8326CDCC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CDCC: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CDD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDD4: 3887256C  addi r4, r7, 0x256c
	ctx.r[4].s64 = ctx.r[7].s64 + 9580;
	// 8326CDD8: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 8326CDDC: 4AFC00F5  bl 0x8222ced0
	ctx.lr = 0x8326CDE0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CDE0: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CDE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDE8: 38862560  addi r4, r6, 0x2560
	ctx.r[4].s64 = ctx.r[6].s64 + 9568;
	// 8326CDEC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8326CDF0: 4AFC00E1  bl 0x8222ced0
	ctx.lr = 0x8326CDF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CDF4: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CDF8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CDFC: 3884254C  addi r4, r4, 0x254c
	ctx.r[4].s64 = ctx.r[4].s64 + 9548;
	// 8326CE00: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 8326CE04: 4AFC00CD  bl 0x8222ced0
	ctx.lr = 0x8326CE08;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE08: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CE0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE10: 38832544  addi r4, r3, 0x2544
	ctx.r[4].s64 = ctx.r[3].s64 + 9540;
	// 8326CE14: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8326CE18: 4AFC00B9  bl 0x8222ced0
	ctx.lr = 0x8326CE1C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE1C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CE20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE24: 388B253C  addi r4, r11, 0x253c
	ctx.r[4].s64 = ctx.r[11].s64 + 9532;
	// 8326CE28: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8326CE2C: 4AFC00A5  bl 0x8222ced0
	ctx.lr = 0x8326CE30;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE30: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE38: 388A2530  addi r4, r10, 0x2530
	ctx.r[4].s64 = ctx.r[10].s64 + 9520;
	// 8326CE3C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8326CE40: 4AFC0091  bl 0x8222ced0
	ctx.lr = 0x8326CE44;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE44: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CE48: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE4C: 38892524  addi r4, r9, 0x2524
	ctx.r[4].s64 = ctx.r[9].s64 + 9508;
	// 8326CE50: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 8326CE54: 4AFC007D  bl 0x8222ced0
	ctx.lr = 0x8326CE58;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE58: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CE5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE60: 38882520  addi r4, r8, 0x2520
	ctx.r[4].s64 = ctx.r[8].s64 + 9504;
	// 8326CE64: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 8326CE68: 4AFC0069  bl 0x8222ced0
	ctx.lr = 0x8326CE6C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE6C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CE70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE74: 38872514  addi r4, r7, 0x2514
	ctx.r[4].s64 = ctx.r[7].s64 + 9492;
	// 8326CE78: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 8326CE7C: 4AFC0055  bl 0x8222ced0
	ctx.lr = 0x8326CE80;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE80: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CE84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE88: 38862508  addi r4, r6, 0x2508
	ctx.r[4].s64 = ctx.r[6].s64 + 9480;
	// 8326CE8C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8326CE90: 4AFC0041  bl 0x8222ced0
	ctx.lr = 0x8326CE94;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CE94: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CE98: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CE9C: 38842500  addi r4, r4, 0x2500
	ctx.r[4].s64 = ctx.r[4].s64 + 9472;
	// 8326CEA0: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 8326CEA4: 4AFC002D  bl 0x8222ced0
	ctx.lr = 0x8326CEA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CEA8: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CEAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CEB0: 388324F0  addi r4, r3, 0x24f0
	ctx.r[4].s64 = ctx.r[3].s64 + 9456;
	// 8326CEB4: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 8326CEB8: 4AFC0019  bl 0x8222ced0
	ctx.lr = 0x8326CEBC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CEBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CEC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CEC4: 388B24E8  addi r4, r11, 0x24e8
	ctx.r[4].s64 = ctx.r[11].s64 + 9448;
	// 8326CEC8: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 8326CECC: 4AFC0005  bl 0x8222ced0
	ctx.lr = 0x8326CED0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CED0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CED8: 388A24DC  addi r4, r10, 0x24dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9436;
	// 8326CEDC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8326CEE0: 4AFBFFF1  bl 0x8222ced0
	ctx.lr = 0x8326CEE4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CEE4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CEE8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CEEC: 388924D0  addi r4, r9, 0x24d0
	ctx.r[4].s64 = ctx.r[9].s64 + 9424;
	// 8326CEF0: 387F00A4  addi r3, r31, 0xa4
	ctx.r[3].s64 = ctx.r[31].s64 + 164;
	// 8326CEF4: 4AFBFFDD  bl 0x8222ced0
	ctx.lr = 0x8326CEF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CEF8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CEFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF00: 388824BC  addi r4, r8, 0x24bc
	ctx.r[4].s64 = ctx.r[8].s64 + 9404;
	// 8326CF04: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 8326CF08: 4AFBFFC9  bl 0x8222ced0
	ctx.lr = 0x8326CF0C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF0C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CF10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF14: 388724B4  addi r4, r7, 0x24b4
	ctx.r[4].s64 = ctx.r[7].s64 + 9396;
	// 8326CF18: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 8326CF1C: 4AFBFFB5  bl 0x8222ced0
	ctx.lr = 0x8326CF20;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF20: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CF24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF28: 388624A0  addi r4, r6, 0x24a0
	ctx.r[4].s64 = ctx.r[6].s64 + 9376;
	// 8326CF2C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 8326CF30: 4AFBFFA1  bl 0x8222ced0
	ctx.lr = 0x8326CF34;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF34: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CF38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF3C: 38842494  addi r4, r4, 0x2494
	ctx.r[4].s64 = ctx.r[4].s64 + 9364;
	// 8326CF40: 387F00B4  addi r3, r31, 0xb4
	ctx.r[3].s64 = ctx.r[31].s64 + 180;
	// 8326CF44: 4AFBFF8D  bl 0x8222ced0
	ctx.lr = 0x8326CF48;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF48: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CF4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF50: 3883248C  addi r4, r3, 0x248c
	ctx.r[4].s64 = ctx.r[3].s64 + 9356;
	// 8326CF54: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 8326CF58: 4AFBFF79  bl 0x8222ced0
	ctx.lr = 0x8326CF5C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF5C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326CF60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF64: 388B2484  addi r4, r11, 0x2484
	ctx.r[4].s64 = ctx.r[11].s64 + 9348;
	// 8326CF68: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 8326CF6C: 4AFBFF65  bl 0x8222ced0
	ctx.lr = 0x8326CF70;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF70: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326CF74: 388A2480  addi r4, r10, 0x2480
	ctx.r[4].s64 = ctx.r[10].s64 + 9344;
	// 8326CF78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF7C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8326CF80: 4AFBFF51  bl 0x8222ced0
	ctx.lr = 0x8326CF84;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF84: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326CF88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CF8C: 3889247C  addi r4, r9, 0x247c
	ctx.r[4].s64 = ctx.r[9].s64 + 9340;
	// 8326CF90: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 8326CF94: 4AFBFF3D  bl 0x8222ced0
	ctx.lr = 0x8326CF98;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CF98: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326CF9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFA0: 3888246C  addi r4, r8, 0x246c
	ctx.r[4].s64 = ctx.r[8].s64 + 9324;
	// 8326CFA4: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 8326CFA8: 4AFBFF29  bl 0x8222ced0
	ctx.lr = 0x8326CFAC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CFAC: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326CFB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFB4: 38872460  addi r4, r7, 0x2460
	ctx.r[4].s64 = ctx.r[7].s64 + 9312;
	// 8326CFB8: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 8326CFBC: 4AFBFF15  bl 0x8222ced0
	ctx.lr = 0x8326CFC0;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CFC0: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326CFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFC8: 38862454  addi r4, r6, 0x2454
	ctx.r[4].s64 = ctx.r[6].s64 + 9300;
	// 8326CFCC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8326CFD0: 4AFBFF01  bl 0x8222ced0
	ctx.lr = 0x8326CFD4;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CFD4: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326CFD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFDC: 38842444  addi r4, r4, 0x2444
	ctx.r[4].s64 = ctx.r[4].s64 + 9284;
	// 8326CFE0: 387F00D4  addi r3, r31, 0xd4
	ctx.r[3].s64 = ctx.r[31].s64 + 212;
	// 8326CFE4: 4AFBFEED  bl 0x8222ced0
	ctx.lr = 0x8326CFE8;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CFE8: 3C60820E  lis r3, -0x7df2
	ctx.r[3].s64 = -2113011712;
	// 8326CFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326CFF0: 3883243C  addi r4, r3, 0x243c
	ctx.r[4].s64 = ctx.r[3].s64 + 9276;
	// 8326CFF4: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 8326CFF8: 4AFBFED9  bl 0x8222ced0
	ctx.lr = 0x8326CFFC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326CFFC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D000: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D004: 388B2434  addi r4, r11, 0x2434
	ctx.r[4].s64 = ctx.r[11].s64 + 9268;
	// 8326D008: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 8326D00C: 4AFBFEC5  bl 0x8222ced0
	ctx.lr = 0x8326D010;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326D010: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326D014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D018: 388A2428  addi r4, r10, 0x2428
	ctx.r[4].s64 = ctx.r[10].s64 + 9256;
	// 8326D01C: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8326D020: 4AFBFEB1  bl 0x8222ced0
	ctx.lr = 0x8326D024;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326D024: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326D028: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D02C: 38892420  addi r4, r9, 0x2420
	ctx.r[4].s64 = ctx.r[9].s64 + 9248;
	// 8326D030: 387F00E4  addi r3, r31, 0xe4
	ctx.r[3].s64 = ctx.r[31].s64 + 228;
	// 8326D034: 4AFBFE9D  bl 0x8222ced0
	ctx.lr = 0x8326D038;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326D038: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326D03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D040: 3888240C  addi r4, r8, 0x240c
	ctx.r[4].s64 = ctx.r[8].s64 + 9228;
	// 8326D044: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 8326D048: 4AFBFE89  bl 0x8222ced0
	ctx.lr = 0x8326D04C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326D04C: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326D050: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D054: 38872400  addi r4, r7, 0x2400
	ctx.r[4].s64 = ctx.r[7].s64 + 9216;
	// 8326D058: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 8326D05C: 4AFBFE75  bl 0x8222ced0
	ctx.lr = 0x8326D060;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326D060: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326D064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D068: 388623F4  addi r4, r6, 0x23f4
	ctx.r[4].s64 = ctx.r[6].s64 + 9204;
	// 8326D06C: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 8326D070: 4AFBFE61  bl 0x8222ced0
	ctx.lr = 0x8326D074;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326D074: 3C80820E  lis r4, -0x7df2
	ctx.r[4].s64 = -2113011712;
	// 8326D078: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D07C: 388423E8  addi r4, r4, 0x23e8
	ctx.r[4].s64 = ctx.r[4].s64 + 9192;
	// 8326D080: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 8326D084: 4AFBFE4D  bl 0x8222ced0
	ctx.lr = 0x8326D088;
	crate::recompiler::externs::call(&mut ctx, base, 0x8222CED0);
	// 8326D088: 3C608349  lis r3, -0x7cb7
	ctx.r[3].s64 = -2092367872;
	// 8326D08C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8326D090: 39037088  addi r8, r3, 0x7088
	ctx.r[8].s64 = ctx.r[3].s64 + 28808;
	// 8326D094: 917F00F8  stw r11, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 8326D098: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8326D09C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326D0A0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8326D0A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8326D0A8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326D0AC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326D0B0: 4082FFE8  bne 0x8326d098
	if !ctx.cr[0].eq {
	pc = 0x8326D098; continue 'dispatch;
	}
	// 8326D0B4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D0B8: 3867EAA0  addi r3, r7, -0x1560
	ctx.r[3].s64 = ctx.r[7].s64 + -5472;
	// 8326D0BC: 4BA3CE65  bl 0x82ca9f20
	ctx.lr = 0x8326D0C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8326D0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326D0D0: 4E800020  blr
	return;
}

pub fn sub_8326D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D0D8 size=56
	// 8326D0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D0E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D0E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D0EC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326D0F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D0F4: 4AF86C65  bl 0x821f3d58
	ctx.lr = 0x8326D0F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D0F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D0FC: 906AC804  stw r3, -0x37fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14332 as u32), ctx.r[3].u32 ) };
	// 8326D100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D10C: 4E800020  blr
	return;
}

pub fn sub_8326D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D110 size=56
	// 8326D110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D11C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D120: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D124: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326D128: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D12C: 4AF86C2D  bl 0x821f3d58
	ctx.lr = 0x8326D130;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D134: 906AC808  stw r3, -0x37f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14328 as u32), ctx.r[3].u32 ) };
	// 8326D138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D144: 4E800020  blr
	return;
}

pub fn sub_8326D148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D148 size=56
	// 8326D148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D154: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D158: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D15C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326D160: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D164: 4AF86BF5  bl 0x821f3d58
	ctx.lr = 0x8326D168;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D16C: 906AC80C  stw r3, -0x37f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14324 as u32), ctx.r[3].u32 ) };
	// 8326D170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D17C: 4E800020  blr
	return;
}

pub fn sub_8326D180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D180 size=56
	// 8326D180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D18C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D190: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D194: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326D198: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D19C: 4AF86BBD  bl 0x821f3d58
	ctx.lr = 0x8326D1A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D1A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D1A4: 906AC810  stw r3, -0x37f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14320 as u32), ctx.r[3].u32 ) };
	// 8326D1A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D1B4: 4E800020  blr
	return;
}

pub fn sub_8326D1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D1B8 size=56
	// 8326D1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D1C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D1C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326D1C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D1CC: 386B40C8  addi r3, r11, 0x40c8
	ctx.r[3].s64 = ctx.r[11].s64 + 16584;
	// 8326D1D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D1D4: 4AF86B85  bl 0x821f3d58
	ctx.lr = 0x8326D1D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D1D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D1DC: 906AC814  stw r3, -0x37ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14316 as u32), ctx.r[3].u32 ) };
	// 8326D1E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D1EC: 4E800020  blr
	return;
}

pub fn sub_8326D1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D1F0 size=56
	// 8326D1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D1F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D1FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D200: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D204: 386B2A90  addi r3, r11, 0x2a90
	ctx.r[3].s64 = ctx.r[11].s64 + 10896;
	// 8326D208: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D20C: 4AF86B4D  bl 0x821f3d58
	ctx.lr = 0x8326D210;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D214: 906AC818  stw r3, -0x37e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14312 as u32), ctx.r[3].u32 ) };
	// 8326D218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D224: 4E800020  blr
	return;
}

pub fn sub_8326D228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D228 size=56
	// 8326D228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D234: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D238: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D23C: 386B2B0C  addi r3, r11, 0x2b0c
	ctx.r[3].s64 = ctx.r[11].s64 + 11020;
	// 8326D240: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D244: 4AF86B15  bl 0x821f3d58
	ctx.lr = 0x8326D248;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D24C: 906AC81C  stw r3, -0x37e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14308 as u32), ctx.r[3].u32 ) };
	// 8326D250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D25C: 4E800020  blr
	return;
}

pub fn sub_8326D260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D260 size=56
	// 8326D260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D26C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D270: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D274: 386B2B20  addi r3, r11, 0x2b20
	ctx.r[3].s64 = ctx.r[11].s64 + 11040;
	// 8326D278: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D27C: 4AF86ADD  bl 0x821f3d58
	ctx.lr = 0x8326D280;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D284: 906AC820  stw r3, -0x37e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14304 as u32), ctx.r[3].u32 ) };
	// 8326D288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D294: 4E800020  blr
	return;
}

pub fn sub_8326D298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D298 size=56
	// 8326D298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D2A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D2A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D2AC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326D2B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D2B4: 4AF86AA5  bl 0x821f3d58
	ctx.lr = 0x8326D2B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D2B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D2BC: 906AC824  stw r3, -0x37dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14300 as u32), ctx.r[3].u32 ) };
	// 8326D2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D2CC: 4E800020  blr
	return;
}

pub fn sub_8326D2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D2D0 size=56
	// 8326D2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D2DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D2E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D2E4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326D2E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D2EC: 4AF86A6D  bl 0x821f3d58
	ctx.lr = 0x8326D2F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D2F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D2F4: 906AC828  stw r3, -0x37d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14296 as u32), ctx.r[3].u32 ) };
	// 8326D2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D304: 4E800020  blr
	return;
}

pub fn sub_8326D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D308 size=56
	// 8326D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D318: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D31C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326D320: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D324: 4AF86A35  bl 0x821f3d58
	ctx.lr = 0x8326D328;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D32C: 906AC82C  stw r3, -0x37d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14292 as u32), ctx.r[3].u32 ) };
	// 8326D330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D33C: 4E800020  blr
	return;
}

pub fn sub_8326D340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D340 size=56
	// 8326D340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D34C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D350: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D354: 386B2B2C  addi r3, r11, 0x2b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11052;
	// 8326D358: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D35C: 4AF869FD  bl 0x821f3d58
	ctx.lr = 0x8326D360;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D364: 906AC830  stw r3, -0x37d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14288 as u32), ctx.r[3].u32 ) };
	// 8326D368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D374: 4E800020  blr
	return;
}

pub fn sub_8326D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D378 size=56
	// 8326D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D384: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D388: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D38C: 386B2B40  addi r3, r11, 0x2b40
	ctx.r[3].s64 = ctx.r[11].s64 + 11072;
	// 8326D390: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D394: 4AF869C5  bl 0x821f3d58
	ctx.lr = 0x8326D398;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D39C: 906AC834  stw r3, -0x37cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14284 as u32), ctx.r[3].u32 ) };
	// 8326D3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D3AC: 4E800020  blr
	return;
}

pub fn sub_8326D3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D3B0 size=56
	// 8326D3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D3BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D3C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D3C4: 386B2B54  addi r3, r11, 0x2b54
	ctx.r[3].s64 = ctx.r[11].s64 + 11092;
	// 8326D3C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D3CC: 4AF8698D  bl 0x821f3d58
	ctx.lr = 0x8326D3D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D3D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D3D4: 906AC838  stw r3, -0x37c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14280 as u32), ctx.r[3].u32 ) };
	// 8326D3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D3E4: 4E800020  blr
	return;
}

pub fn sub_8326D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D3E8 size=56
	// 8326D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D3F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D3F4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D3F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D3FC: 386B2B68  addi r3, r11, 0x2b68
	ctx.r[3].s64 = ctx.r[11].s64 + 11112;
	// 8326D400: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D404: 4AF86955  bl 0x821f3d58
	ctx.lr = 0x8326D408;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D40C: 906AC83C  stw r3, -0x37c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14276 as u32), ctx.r[3].u32 ) };
	// 8326D410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D41C: 4E800020  blr
	return;
}

pub fn sub_8326D420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D420 size=56
	// 8326D420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D42C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D430: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D434: 386B2B7C  addi r3, r11, 0x2b7c
	ctx.r[3].s64 = ctx.r[11].s64 + 11132;
	// 8326D438: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D43C: 4AF8691D  bl 0x821f3d58
	ctx.lr = 0x8326D440;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D444: 906AC840  stw r3, -0x37c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14272 as u32), ctx.r[3].u32 ) };
	// 8326D448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D454: 4E800020  blr
	return;
}

pub fn sub_8326D458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D458 size=12
	// 8326D458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D498 size=12
	// 8326D498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D4D8 size=12
	// 8326D4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D518 size=12
	// 8326D518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D558 size=12
	// 8326D558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D598 size=12
	// 8326D598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D5D8 size=12
	// 8326D5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D618 size=12
	// 8326D618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326D658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D658 size=144
	// 8326D658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D664: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8326D668: 4AFB1BF1  bl 0x8221f258
	ctx.lr = 0x8326D66C;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8326D66C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326D670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8326D674: 419A0008  beq cr6, 0x8326d67c
	if ctx.cr[6].eq {
	pc = 0x8326D67C; continue 'dispatch;
	}
	// 8326D678: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8326D67C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D680: 41820008  beq 0x8326d688
	if ctx.cr[0].eq {
	pc = 0x8326D688; continue 'dispatch;
	}
	// 8326D684: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8326D688: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D68C: 41820008  beq 0x8326d694
	if ctx.cr[0].eq {
	pc = 0x8326D694; continue 'dispatch;
	}
	// 8326D690: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8326D694: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326D698: 99430029  stb r10, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 8326D69C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8326D6A0: 3909C864  addi r8, r9, -0x379c
	ctx.r[8].s64 = ctx.r[9].s64 + -14236;
	// 8326D6A4: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 8326D6A8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D6AC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326D6B0: 99630029  stb r11, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 8326D6B4: 3867ECD0  addi r3, r7, -0x1330
	ctx.r[3].s64 = ctx.r[7].s64 + -4912;
	// 8326D6B8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6BC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326D6C0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6C4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326D6C8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6CC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326D6D0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326D6D4: 4BA3C84D  bl 0x82ca9f20
	ctx.lr = 0x8326D6D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8326D6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D6E4: 4E800020  blr
	return;
}

pub fn sub_8326D6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D6E8 size=160
	// 8326D6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D6F4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8326D6F8: 4AFB1B61  bl 0x8221f258
	ctx.lr = 0x8326D6FC;
	crate::recompiler::externs::call(&mut ctx, base, 0x8221F258);
	// 8326D6FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326D700: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8326D704: 419A0008  beq cr6, 0x8326d70c
	if ctx.cr[6].eq {
	pc = 0x8326D70C; continue 'dispatch;
	}
	// 8326D708: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8326D70C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D710: 41820008  beq 0x8326d718
	if ctx.cr[0].eq {
	pc = 0x8326D718; continue 'dispatch;
	}
	// 8326D714: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8326D718: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D71C: 41820008  beq 0x8326d724
	if ctx.cr[0].eq {
	pc = 0x8326D724; continue 'dispatch;
	}
	// 8326D720: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8326D724: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326D728: 99430021  stb r10, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 8326D72C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8326D730: 3909C870  addi r8, r9, -0x3790
	ctx.r[8].s64 = ctx.r[9].s64 + -14224;
	// 8326D734: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8326D738: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D73C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326D740: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8326D744: 3867ECE0  addi r3, r7, -0x1320
	ctx.r[3].s64 = ctx.r[7].s64 + -4896;
	// 8326D748: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D74C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326D750: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D754: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326D758: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D75C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326D760: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326D764: 4BA3C7BD  bl 0x82ca9f20
	ctx.lr = 0x8326D768;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8326D768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D774: 4E800020  blr
	return;
	// 8326D778: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326D77C: 386BECF0  addi r3, r11, -0x1310
	ctx.r[3].s64 = ctx.r[11].s64 + -4880;
	// 8326D780: 4BA3C7A0  b 0x82ca9f20
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	return;
}

pub fn sub_8326D788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D788 size=56
	// 8326D788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D794: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D79C: 386B4024  addi r3, r11, 0x4024
	ctx.r[3].s64 = ctx.r[11].s64 + 16420;
	// 8326D7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D7A4: 4AF865B5  bl 0x821f3d58
	ctx.lr = 0x8326D7A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D7AC: 906AC88C  stw r3, -0x3774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14196 as u32), ctx.r[3].u32 ) };
	// 8326D7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D7BC: 4E800020  blr
	return;
}

pub fn sub_8326D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D7C0 size=56
	// 8326D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D7CC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D7D4: 386B4034  addi r3, r11, 0x4034
	ctx.r[3].s64 = ctx.r[11].s64 + 16436;
	// 8326D7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D7DC: 4AF8657D  bl 0x821f3d58
	ctx.lr = 0x8326D7E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D7E4: 906AC890  stw r3, -0x3770(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14192 as u32), ctx.r[3].u32 ) };
	// 8326D7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D7F4: 4E800020  blr
	return;
}

pub fn sub_8326D7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D7F8 size=56
	// 8326D7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D804: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D80C: 386B4048  addi r3, r11, 0x4048
	ctx.r[3].s64 = ctx.r[11].s64 + 16456;
	// 8326D810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D814: 4AF86545  bl 0x821f3d58
	ctx.lr = 0x8326D818;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D81C: 906AC894  stw r3, -0x376c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14188 as u32), ctx.r[3].u32 ) };
	// 8326D820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D82C: 4E800020  blr
	return;
}

pub fn sub_8326D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D830 size=56
	// 8326D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D83C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D844: 386B405C  addi r3, r11, 0x405c
	ctx.r[3].s64 = ctx.r[11].s64 + 16476;
	// 8326D848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D84C: 4AF8650D  bl 0x821f3d58
	ctx.lr = 0x8326D850;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D854: 906AC898  stw r3, -0x3768(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14184 as u32), ctx.r[3].u32 ) };
	// 8326D858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D864: 4E800020  blr
	return;
}

pub fn sub_8326D868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D868 size=56
	// 8326D868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D874: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D87C: 386B406C  addi r3, r11, 0x406c
	ctx.r[3].s64 = ctx.r[11].s64 + 16492;
	// 8326D880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D884: 4AF864D5  bl 0x821f3d58
	ctx.lr = 0x8326D888;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D88C: 906AC89C  stw r3, -0x3764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14180 as u32), ctx.r[3].u32 ) };
	// 8326D890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D89C: 4E800020  blr
	return;
}

pub fn sub_8326D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D8A0 size=56
	// 8326D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D8AC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D8B4: 386B407C  addi r3, r11, 0x407c
	ctx.r[3].s64 = ctx.r[11].s64 + 16508;
	// 8326D8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D8BC: 4AF8649D  bl 0x821f3d58
	ctx.lr = 0x8326D8C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D8C4: 906AC8A0  stw r3, -0x3760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14176 as u32), ctx.r[3].u32 ) };
	// 8326D8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D8D4: 4E800020  blr
	return;
}

pub fn sub_8326D8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D8D8 size=56
	// 8326D8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D8E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D8EC: 386B408C  addi r3, r11, 0x408c
	ctx.r[3].s64 = ctx.r[11].s64 + 16524;
	// 8326D8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D8F4: 4AF86465  bl 0x821f3d58
	ctx.lr = 0x8326D8F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D8FC: 906AC8A4  stw r3, -0x375c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14172 as u32), ctx.r[3].u32 ) };
	// 8326D900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D90C: 4E800020  blr
	return;
}

pub fn sub_8326D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D910 size=56
	// 8326D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D91C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D924: 386B2B20  addi r3, r11, 0x2b20
	ctx.r[3].s64 = ctx.r[11].s64 + 11040;
	// 8326D928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D92C: 4AF8642D  bl 0x821f3d58
	ctx.lr = 0x8326D930;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D934: 906AC8A8  stw r3, -0x3758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14168 as u32), ctx.r[3].u32 ) };
	// 8326D938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D944: 4E800020  blr
	return;
}

pub fn sub_8326D948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D948 size=56
	// 8326D948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D954: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D95C: 386B2B2C  addi r3, r11, 0x2b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11052;
	// 8326D960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D964: 4AF863F5  bl 0x821f3d58
	ctx.lr = 0x8326D968;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D96C: 906AC8AC  stw r3, -0x3754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14164 as u32), ctx.r[3].u32 ) };
	// 8326D970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D97C: 4E800020  blr
	return;
}

pub fn sub_8326D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D980 size=56
	// 8326D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D98C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D994: 386B40A0  addi r3, r11, 0x40a0
	ctx.r[3].s64 = ctx.r[11].s64 + 16544;
	// 8326D998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D99C: 4AF863BD  bl 0x821f3d58
	ctx.lr = 0x8326D9A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D9A4: 906AC8B0  stw r3, -0x3750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14160 as u32), ctx.r[3].u32 ) };
	// 8326D9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D9B4: 4E800020  blr
	return;
}

pub fn sub_8326D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D9B8 size=56
	// 8326D9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D9C4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D9CC: 386B40B8  addi r3, r11, 0x40b8
	ctx.r[3].s64 = ctx.r[11].s64 + 16568;
	// 8326D9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D9D4: 4AF86385  bl 0x821f3d58
	ctx.lr = 0x8326D9D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326D9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D9DC: 906AC8B4  stw r3, -0x374c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14156 as u32), ctx.r[3].u32 ) };
	// 8326D9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D9EC: 4E800020  blr
	return;
}

pub fn sub_8326D9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D9F0 size=56
	// 8326D9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D9FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA04: 386B40C8  addi r3, r11, 0x40c8
	ctx.r[3].s64 = ctx.r[11].s64 + 16584;
	// 8326DA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA0C: 4AF8634D  bl 0x821f3d58
	ctx.lr = 0x8326DA10;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA14: 906AC8B8  stw r3, -0x3748(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14152 as u32), ctx.r[3].u32 ) };
	// 8326DA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA24: 4E800020  blr
	return;
}

pub fn sub_8326DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA28 size=56
	// 8326DA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DA34: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA3C: 386B40DC  addi r3, r11, 0x40dc
	ctx.r[3].s64 = ctx.r[11].s64 + 16604;
	// 8326DA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA44: 4AF86315  bl 0x821f3d58
	ctx.lr = 0x8326DA48;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA4C: 906AC8BC  stw r3, -0x3744(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14148 as u32), ctx.r[3].u32 ) };
	// 8326DA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA5C: 4E800020  blr
	return;
}

pub fn sub_8326DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA60 size=56
	// 8326DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DA6C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA74: 386B40F4  addi r3, r11, 0x40f4
	ctx.r[3].s64 = ctx.r[11].s64 + 16628;
	// 8326DA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA7C: 4AF862DD  bl 0x821f3d58
	ctx.lr = 0x8326DA80;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA84: 906AC8C0  stw r3, -0x3740(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14144 as u32), ctx.r[3].u32 ) };
	// 8326DA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA94: 4E800020  blr
	return;
}

pub fn sub_8326DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA98 size=56
	// 8326DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DAA4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DAA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DAAC: 386B410C  addi r3, r11, 0x410c
	ctx.r[3].s64 = ctx.r[11].s64 + 16652;
	// 8326DAB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DAB4: 4AF862A5  bl 0x821f3d58
	ctx.lr = 0x8326DAB8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DABC: 906AC8C4  stw r3, -0x373c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14140 as u32), ctx.r[3].u32 ) };
	// 8326DAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DACC: 4E800020  blr
	return;
}

pub fn sub_8326DAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DAD0 size=56
	// 8326DAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DADC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DAE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DAE4: 386B4128  addi r3, r11, 0x4128
	ctx.r[3].s64 = ctx.r[11].s64 + 16680;
	// 8326DAE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DAEC: 4AF8626D  bl 0x821f3d58
	ctx.lr = 0x8326DAF0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DAF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DAF4: 906AC8C8  stw r3, -0x3738(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14136 as u32), ctx.r[3].u32 ) };
	// 8326DAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB04: 4E800020  blr
	return;
}

pub fn sub_8326DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DB08 size=56
	// 8326DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DB14: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DB18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DB1C: 386B4144  addi r3, r11, 0x4144
	ctx.r[3].s64 = ctx.r[11].s64 + 16708;
	// 8326DB20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DB24: 4AF86235  bl 0x821f3d58
	ctx.lr = 0x8326DB28;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB2C: 906AC8CC  stw r3, -0x3734(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14132 as u32), ctx.r[3].u32 ) };
	// 8326DB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB3C: 4E800020  blr
	return;
}

pub fn sub_8326DB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DB40 size=136
	// 8326DB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DB4C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DB54: 386B4154  addi r3, r11, 0x4154
	ctx.r[3].s64 = ctx.r[11].s64 + 16724;
	// 8326DB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DB5C: 4AF861FD  bl 0x821f3d58
	ctx.lr = 0x8326DB60;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB64: 906AC8D0  stw r3, -0x3730(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14128 as u32), ctx.r[3].u32 ) };
	// 8326DB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB74: 4E800020  blr
	return;
	// 8326DB78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326DB80: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8326DB84: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8326DB88: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8326DB8C: 812AC8B4  lwz r9, -0x374c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-14156 as u32) ) } as u64;
	// 8326DB90: 816BC89C  lwz r11, -0x3764(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14180 as u32) ) } as u64;
	// 8326DB94: 38A6C8D4  addi r5, r6, -0x372c
	ctx.r[5].s64 = ctx.r[6].s64 + -14124;
	// 8326DB98: 8108C8B8  lwz r8, -0x3748(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-14152 as u32) ) } as u64;
	// 8326DB9C: 8147C8A4  lwz r10, -0x375c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-14172 as u32) ) } as u64;
	// 8326DBA0: 9166C8D4  stw r11, -0x372c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-14124 as u32), ctx.r[11].u32 ) };
	// 8326DBA4: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8326DBA8: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326DBAC: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8326DBB0: 91450010  stw r10, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8326DBB4: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8326DBB8: 91450018  stw r10, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8326DBBC: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8326DBC0: 4E800020  blr
	return;
}

pub fn sub_8326DBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DBC8 size=184
	// 8326DBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DBD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8326DBD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326DBD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DBDC: 3D60811C  lis r11, -0x7ee4
	ctx.r[11].s64 = -2128871424;
	// 8326DBE0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326DBE4: 617F9DC5  ori r31, r11, 0x9dc5
	ctx.r[31].u64 = ctx.r[11].u64 | 40389;
	// 8326DBE8: 386A418C  addi r3, r10, 0x418c
	ctx.r[3].s64 = ctx.r[10].s64 + 16780;
	// 8326DBEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DBF0: 4AF86169  bl 0x821f3d58
	ctx.lr = 0x8326DBF4;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DBF4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326DBF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326DBFC: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326DC00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC04: 38684184  addi r3, r8, 0x4184
	ctx.r[3].s64 = ctx.r[8].s64 + 16772;
	// 8326DC08: 9169C8F4  stw r11, -0x370c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-14092 as u32), ctx.r[11].u32 ) };
	// 8326DC0C: 3BC9C8F4  addi r30, r9, -0x370c
	ctx.r[30].s64 = ctx.r[9].s64 + -14092;
	// 8326DC10: 4AF86149  bl 0x821f3d58
	ctx.lr = 0x8326DC14;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DC14: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326DC18: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326DC1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC20: 3867417C  addi r3, r7, 0x417c
	ctx.r[3].s64 = ctx.r[7].s64 + 16764;
	// 8326DC24: 4AF86135  bl 0x821f3d58
	ctx.lr = 0x8326DC28;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DC28: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8326DC2C: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326DC30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC34: 38664174  addi r3, r6, 0x4174
	ctx.r[3].s64 = ctx.r[6].s64 + 16756;
	// 8326DC38: 4AF86121  bl 0x821f3d58
	ctx.lr = 0x8326DC3C;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DC3C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8326DC40: 3CA0820E  lis r5, -0x7df2
	ctx.r[5].s64 = -2113011712;
	// 8326DC44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC48: 3865416C  addi r3, r5, 0x416c
	ctx.r[3].s64 = ctx.r[5].s64 + 16748;
	// 8326DC4C: 4AF8610D  bl 0x821f3d58
	ctx.lr = 0x8326DC50;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DC50: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8326DC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326DC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DC60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8326DC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326DC68: 4E800020  blr
	return;
}

pub fn sub_8326DC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DC80 size=72
	// 8326DC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DC8C: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 8326DC90: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326DC94: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326DC98: 388A4D9C  addi r4, r10, 0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + 19868;
	// 8326DC9C: 3869C918  addi r3, r9, -0x36e8
	ctx.r[3].s64 = ctx.r[9].s64 + -14056;
	// 8326DCA0: 38ABCA90  addi r5, r11, -0x3570
	ctx.r[5].s64 = ctx.r[11].s64 + -13680;
	// 8326DCA4: 4BC2AB3D  bl 0x82e987e0
	ctx.lr = 0x8326DCA8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E987E0);
	// 8326DCA8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326DCAC: 3868ED68  addi r3, r8, -0x1298
	ctx.r[3].s64 = ctx.r[8].s64 + -4760;
	// 8326DCB0: 4BA3C271  bl 0x82ca9f20
	ctx.lr = 0x8326DCB4;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8326DCB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DCC0: 4E800020  blr
	return;
}

pub fn sub_8326DCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DCC8 size=104
	// 8326DCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DCD4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8326DCD8: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 8326DCDC: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326DCE0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8326DCE4: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 8326DCE8: 38894DC8  addi r4, r9, 0x4dc8
	ctx.r[4].s64 = ctx.r[9].s64 + 19912;
	// 8326DCEC: 3868CA2C  addi r3, r8, -0x35d4
	ctx.r[3].s64 = ctx.r[8].s64 + -13780;
	// 8326DCF0: 38AAD860  addi r5, r10, -0x27a0
	ctx.r[5].s64 = ctx.r[10].s64 + -10144;
	// 8326DCF4: 4BC17F05  bl 0x82e85bf8
	ctx.lr = 0x8326DCF8;
	crate::recompiler::externs::call(&mut ctx, base, 0x82E85BF8);
	// 8326DCF8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326DCFC: 3867ED80  addi r3, r7, -0x1280
	ctx.r[3].s64 = ctx.r[7].s64 + -4736;
	// 8326DD00: 4BA3C221  bl 0x82ca9f20
	ctx.lr = 0x8326DD04;
	crate::recompiler::externs::call(&mut ctx, base, 0x82CA9F20);
	// 8326DD04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DD10: 4E800020  blr
	return;
}

pub fn sub_8326DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DD30 size=12
	// 8326DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DD38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326DD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DD80 size=12
	// 8326DD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DDC0 size=12
	// 8326DDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326DE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE00 size=12
	// 8326DE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE40 size=56
	// 8326DE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE4C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DE54: 386B5CC8  addi r3, r11, 0x5cc8
	ctx.r[3].s64 = ctx.r[11].s64 + 23752;
	// 8326DE58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DE5C: 4AF85EFD  bl 0x821f3d58
	ctx.lr = 0x8326DE60;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DE60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE64: 906ACB64  stw r3, -0x349c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13468 as u32), ctx.r[3].u32 ) };
	// 8326DE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DE74: 4E800020  blr
	return;
}

pub fn sub_8326DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE78 size=56
	// 8326DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE84: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DE8C: 386B5CDC  addi r3, r11, 0x5cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 23772;
	// 8326DE90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DE94: 4AF85EC5  bl 0x821f3d58
	ctx.lr = 0x8326DE98;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DE98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE9C: 906ACB68  stw r3, -0x3498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13464 as u32), ctx.r[3].u32 ) };
	// 8326DEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DEAC: 4E800020  blr
	return;
}

pub fn sub_8326DEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DEB0 size=56
	// 8326DEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DEBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DEC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DEC4: 386B5FA0  addi r3, r11, 0x5fa0
	ctx.r[3].s64 = ctx.r[11].s64 + 24480;
	// 8326DEC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DECC: 4AF85E8D  bl 0x821f3d58
	ctx.lr = 0x8326DED0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DED4: 906ACB6C  stw r3, -0x3494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13460 as u32), ctx.r[3].u32 ) };
	// 8326DED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DEE4: 4E800020  blr
	return;
}

pub fn sub_8326DEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DEE8 size=56
	// 8326DEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DEF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DEF4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DEF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DEFC: 386B5FB8  addi r3, r11, 0x5fb8
	ctx.r[3].s64 = ctx.r[11].s64 + 24504;
	// 8326DF00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DF04: 4AF85E55  bl 0x821f3d58
	ctx.lr = 0x8326DF08;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DF08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF0C: 906ACB70  stw r3, -0x3490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13456 as u32), ctx.r[3].u32 ) };
	// 8326DF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF1C: 4E800020  blr
	return;
}

pub fn sub_8326DF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF20 size=56
	// 8326DF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DF2C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DF30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DF34: 386B6428  addi r3, r11, 0x6428
	ctx.r[3].s64 = ctx.r[11].s64 + 25640;
	// 8326DF38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DF3C: 4AF85E1D  bl 0x821f3d58
	ctx.lr = 0x8326DF40;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326DF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF44: 906ACB74  stw r3, -0x348c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13452 as u32), ctx.r[3].u32 ) };
	// 8326DF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF54: 4E800020  blr
	return;
}

pub fn sub_8326DF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF58 size=12
	// 8326DF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF98 size=12
	// 8326DF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326DFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DFD8 size=12
	// 8326DFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E018 size=12
	// 8326E018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E058 size=12
	// 8326E058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E098 size=12
	// 8326E098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E0F8 size=12
	// 8326E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E138 size=12
	// 8326E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E178 size=12
	// 8326E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E1B8 size=12
	// 8326E1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E1C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E1F8 size=12
	// 8326E1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E238 size=12
	// 8326E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E278 size=12
	// 8326E278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326E2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E2B8 size=56
	// 8326E2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E2C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E2C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E2C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E2CC: 386B123C  addi r3, r11, 0x123c
	ctx.r[3].s64 = ctx.r[11].s64 + 4668;
	// 8326E2D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E2D4: 4AF85A85  bl 0x821f3d58
	ctx.lr = 0x8326E2D8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E2D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E2DC: 906ACBB0  stw r3, -0x3450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13392 as u32), ctx.r[3].u32 ) };
	// 8326E2E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E2EC: 4E800020  blr
	return;
}

pub fn sub_8326E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E2F0 size=56
	// 8326E2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E2F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E2FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E300: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E304: 386B92EC  addi r3, r11, -0x6d14
	ctx.r[3].s64 = ctx.r[11].s64 + -27924;
	// 8326E308: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E30C: 4AF85A4D  bl 0x821f3d58
	ctx.lr = 0x8326E310;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E314: 906ACBB4  stw r3, -0x344c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13388 as u32), ctx.r[3].u32 ) };
	// 8326E318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E324: 4E800020  blr
	return;
}

pub fn sub_8326E328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E328 size=56
	// 8326E328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E334: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E338: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E33C: 386B92F8  addi r3, r11, -0x6d08
	ctx.r[3].s64 = ctx.r[11].s64 + -27912;
	// 8326E340: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E344: 4AF85A15  bl 0x821f3d58
	ctx.lr = 0x8326E348;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E34C: 906ACBB8  stw r3, -0x3448(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13384 as u32), ctx.r[3].u32 ) };
	// 8326E350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E35C: 4E800020  blr
	return;
}

pub fn sub_8326E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E360 size=56
	// 8326E360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E36C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E370: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E374: 386B9428  addi r3, r11, -0x6bd8
	ctx.r[3].s64 = ctx.r[11].s64 + -27608;
	// 8326E378: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E37C: 4AF859DD  bl 0x821f3d58
	ctx.lr = 0x8326E380;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E384: 906ACBBC  stw r3, -0x3444(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13380 as u32), ctx.r[3].u32 ) };
	// 8326E388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E394: 4E800020  blr
	return;
}

pub fn sub_8326E398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E398 size=56
	// 8326E398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E3A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E3A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E3AC: 386B9430  addi r3, r11, -0x6bd0
	ctx.r[3].s64 = ctx.r[11].s64 + -27600;
	// 8326E3B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E3B4: 4AF859A5  bl 0x821f3d58
	ctx.lr = 0x8326E3B8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E3B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E3BC: 906ACBC0  stw r3, -0x3440(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13376 as u32), ctx.r[3].u32 ) };
	// 8326E3C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E3CC: 4E800020  blr
	return;
}

pub fn sub_8326E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E3D0 size=56
	// 8326E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E3DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E3E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E3E4: 386B943C  addi r3, r11, -0x6bc4
	ctx.r[3].s64 = ctx.r[11].s64 + -27588;
	// 8326E3E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E3EC: 4AF8596D  bl 0x821f3d58
	ctx.lr = 0x8326E3F0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E3F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E3F4: 906ACBC4  stw r3, -0x343c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13372 as u32), ctx.r[3].u32 ) };
	// 8326E3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E404: 4E800020  blr
	return;
}

pub fn sub_8326E408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E408 size=56
	// 8326E408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E414: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E41C: 386B123C  addi r3, r11, 0x123c
	ctx.r[3].s64 = ctx.r[11].s64 + 4668;
	// 8326E420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E424: 4AF85935  bl 0x821f3d58
	ctx.lr = 0x8326E428;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E42C: 906ACBC8  stw r3, -0x3438(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13368 as u32), ctx.r[3].u32 ) };
	// 8326E430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E43C: 4E800020  blr
	return;
}

pub fn sub_8326E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E440 size=56
	// 8326E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E44C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E454: 386B944C  addi r3, r11, -0x6bb4
	ctx.r[3].s64 = ctx.r[11].s64 + -27572;
	// 8326E458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E45C: 4AF858FD  bl 0x821f3d58
	ctx.lr = 0x8326E460;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E464: 906ACBCC  stw r3, -0x3434(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13364 as u32), ctx.r[3].u32 ) };
	// 8326E468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E474: 4E800020  blr
	return;
}

pub fn sub_8326E478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E478 size=56
	// 8326E478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E484: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E48C: 386B9458  addi r3, r11, -0x6ba8
	ctx.r[3].s64 = ctx.r[11].s64 + -27560;
	// 8326E490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E494: 4AF858C5  bl 0x821f3d58
	ctx.lr = 0x8326E498;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E49C: 906ACBD0  stw r3, -0x3430(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13360 as u32), ctx.r[3].u32 ) };
	// 8326E4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E4AC: 4E800020  blr
	return;
}

pub fn sub_8326E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E4B0 size=56
	// 8326E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E4BC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E4C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E4C4: 386B9464  addi r3, r11, -0x6b9c
	ctx.r[3].s64 = ctx.r[11].s64 + -27548;
	// 8326E4C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E4CC: 4AF8588D  bl 0x821f3d58
	ctx.lr = 0x8326E4D0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E4D4: 906ACBD4  stw r3, -0x342c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13356 as u32), ctx.r[3].u32 ) };
	// 8326E4D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E4E4: 4E800020  blr
	return;
}

pub fn sub_8326E4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E4E8 size=56
	// 8326E4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E4F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E4F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E4FC: 386B92EC  addi r3, r11, -0x6d14
	ctx.r[3].s64 = ctx.r[11].s64 + -27924;
	// 8326E500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E504: 4AF85855  bl 0x821f3d58
	ctx.lr = 0x8326E508;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E50C: 906ACBD8  stw r3, -0x3428(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13352 as u32), ctx.r[3].u32 ) };
	// 8326E510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E51C: 4E800020  blr
	return;
}

pub fn sub_8326E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E520 size=56
	// 8326E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E52C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E534: 386B92F8  addi r3, r11, -0x6d08
	ctx.r[3].s64 = ctx.r[11].s64 + -27912;
	// 8326E538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E53C: 4AF8581D  bl 0x821f3d58
	ctx.lr = 0x8326E540;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E544: 906ACBDC  stw r3, -0x3424(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13348 as u32), ctx.r[3].u32 ) };
	// 8326E548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E554: 4E800020  blr
	return;
}

pub fn sub_8326E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E558 size=56
	// 8326E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E564: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E56C: 386B9474  addi r3, r11, -0x6b8c
	ctx.r[3].s64 = ctx.r[11].s64 + -27532;
	// 8326E570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E574: 4AF857E5  bl 0x821f3d58
	ctx.lr = 0x8326E578;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E57C: 906ACBE0  stw r3, -0x3420(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13344 as u32), ctx.r[3].u32 ) };
	// 8326E580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E58C: 4E800020  blr
	return;
}

pub fn sub_8326E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E590 size=56
	// 8326E590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E59C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E5A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E5A4: 386B9480  addi r3, r11, -0x6b80
	ctx.r[3].s64 = ctx.r[11].s64 + -27520;
	// 8326E5A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E5AC: 4AF857AD  bl 0x821f3d58
	ctx.lr = 0x8326E5B0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E5B4: 906ACBE4  stw r3, -0x341c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13340 as u32), ctx.r[3].u32 ) };
	// 8326E5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E5C4: 4E800020  blr
	return;
}

pub fn sub_8326E5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E5C8 size=56
	// 8326E5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E5D4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326E5D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E5DC: 386BA994  addi r3, r11, -0x566c
	ctx.r[3].s64 = ctx.r[11].s64 + -22124;
	// 8326E5E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E5E4: 4AF85775  bl 0x821f3d58
	ctx.lr = 0x8326E5E8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E5EC: 906ACBE8  stw r3, -0x3418(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13336 as u32), ctx.r[3].u32 ) };
	// 8326E5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E5FC: 4E800020  blr
	return;
}

pub fn sub_8326E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E600 size=56
	// 8326E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E60C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E614: 386B9490  addi r3, r11, -0x6b70
	ctx.r[3].s64 = ctx.r[11].s64 + -27504;
	// 8326E618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E61C: 4AF8573D  bl 0x821f3d58
	ctx.lr = 0x8326E620;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E624: 906ACBEC  stw r3, -0x3414(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13332 as u32), ctx.r[3].u32 ) };
	// 8326E628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E634: 4E800020  blr
	return;
}

pub fn sub_8326E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E638 size=56
	// 8326E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E644: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326E648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E64C: 386BA9A0  addi r3, r11, -0x5660
	ctx.r[3].s64 = ctx.r[11].s64 + -22112;
	// 8326E650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E654: 4AF85705  bl 0x821f3d58
	ctx.lr = 0x8326E658;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E65C: 906ACBF0  stw r3, -0x3410(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13328 as u32), ctx.r[3].u32 ) };
	// 8326E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E66C: 4E800020  blr
	return;
}

pub fn sub_8326E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E670 size=56
	// 8326E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E67C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E684: 386B94A0  addi r3, r11, -0x6b60
	ctx.r[3].s64 = ctx.r[11].s64 + -27488;
	// 8326E688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E68C: 4AF856CD  bl 0x821f3d58
	ctx.lr = 0x8326E690;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E694: 906ACBF4  stw r3, -0x340c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13324 as u32), ctx.r[3].u32 ) };
	// 8326E698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E6A4: 4E800020  blr
	return;
}

pub fn sub_8326E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E6A8 size=56
	// 8326E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E6B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E6BC: 386B94B8  addi r3, r11, -0x6b48
	ctx.r[3].s64 = ctx.r[11].s64 + -27464;
	// 8326E6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E6C4: 4AF85695  bl 0x821f3d58
	ctx.lr = 0x8326E6C8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E6CC: 906ACBF8  stw r3, -0x3408(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13320 as u32), ctx.r[3].u32 ) };
	// 8326E6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E6DC: 4E800020  blr
	return;
}

pub fn sub_8326E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E6E0 size=56
	// 8326E6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E6EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E6F4: 386B94C8  addi r3, r11, -0x6b38
	ctx.r[3].s64 = ctx.r[11].s64 + -27448;
	// 8326E6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E6FC: 4AF8565D  bl 0x821f3d58
	ctx.lr = 0x8326E700;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E704: 906ACBFC  stw r3, -0x3404(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13316 as u32), ctx.r[3].u32 ) };
	// 8326E708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E714: 4E800020  blr
	return;
}

pub fn sub_8326E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E718 size=56
	// 8326E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E724: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E72C: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 8326E730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E734: 4AF85625  bl 0x821f3d58
	ctx.lr = 0x8326E738;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E73C: 906ACC00  stw r3, -0x3400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13312 as u32), ctx.r[3].u32 ) };
	// 8326E740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E74C: 4E800020  blr
	return;
}

pub fn sub_8326E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E750 size=56
	// 8326E750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E75C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326E760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E764: 386B1034  addi r3, r11, 0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + 4148;
	// 8326E768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E76C: 4AF855ED  bl 0x821f3d58
	ctx.lr = 0x8326E770;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E774: 906ACC04  stw r3, -0x33fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13308 as u32), ctx.r[3].u32 ) };
	// 8326E778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E784: 4E800020  blr
	return;
}

pub fn sub_8326E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E788 size=56
	// 8326E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E794: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E79C: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 8326E7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E7A4: 4AF855B5  bl 0x821f3d58
	ctx.lr = 0x8326E7A8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E7AC: 906ACC08  stw r3, -0x33f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13304 as u32), ctx.r[3].u32 ) };
	// 8326E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E7BC: 4E800020  blr
	return;
}

pub fn sub_8326E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E7C0 size=56
	// 8326E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E7CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E7D4: 386B94F4  addi r3, r11, -0x6b0c
	ctx.r[3].s64 = ctx.r[11].s64 + -27404;
	// 8326E7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E7DC: 4AF8557D  bl 0x821f3d58
	ctx.lr = 0x8326E7E0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E7E4: 906ACC0C  stw r3, -0x33f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13300 as u32), ctx.r[3].u32 ) };
	// 8326E7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E7F4: 4E800020  blr
	return;
}

pub fn sub_8326E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E7F8 size=56
	// 8326E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E804: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E80C: 386B9508  addi r3, r11, -0x6af8
	ctx.r[3].s64 = ctx.r[11].s64 + -27384;
	// 8326E810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E814: 4AF85545  bl 0x821f3d58
	ctx.lr = 0x8326E818;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E81C: 906ACC10  stw r3, -0x33f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13296 as u32), ctx.r[3].u32 ) };
	// 8326E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E82C: 4E800020  blr
	return;
}

pub fn sub_8326E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E830 size=56
	// 8326E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E83C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E844: 386B9514  addi r3, r11, -0x6aec
	ctx.r[3].s64 = ctx.r[11].s64 + -27372;
	// 8326E848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E84C: 4AF8550D  bl 0x821f3d58
	ctx.lr = 0x8326E850;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E854: 906ACC14  stw r3, -0x33ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13292 as u32), ctx.r[3].u32 ) };
	// 8326E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E864: 4E800020  blr
	return;
}

pub fn sub_8326E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E868 size=56
	// 8326E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E874: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E87C: 386B9520  addi r3, r11, -0x6ae0
	ctx.r[3].s64 = ctx.r[11].s64 + -27360;
	// 8326E880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E884: 4AF854D5  bl 0x821f3d58
	ctx.lr = 0x8326E888;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E88C: 906ACC18  stw r3, -0x33e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13288 as u32), ctx.r[3].u32 ) };
	// 8326E890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E89C: 4E800020  blr
	return;
}

pub fn sub_8326E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E8A0 size=56
	// 8326E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E8AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E8B4: 386B9528  addi r3, r11, -0x6ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -27352;
	// 8326E8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E8BC: 4AF8549D  bl 0x821f3d58
	ctx.lr = 0x8326E8C0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E8C4: 906ACC1C  stw r3, -0x33e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13284 as u32), ctx.r[3].u32 ) };
	// 8326E8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E8D4: 4E800020  blr
	return;
}

pub fn sub_8326E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E8D8 size=56
	// 8326E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E8E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E8EC: 386B2C84  addi r3, r11, 0x2c84
	ctx.r[3].s64 = ctx.r[11].s64 + 11396;
	// 8326E8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E8F4: 4AF85465  bl 0x821f3d58
	ctx.lr = 0x8326E8F8;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E8FC: 906ACC20  stw r3, -0x33e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13280 as u32), ctx.r[3].u32 ) };
	// 8326E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E90C: 4E800020  blr
	return;
}

pub fn sub_8326E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E910 size=56
	// 8326E910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E91C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E924: 386B9608  addi r3, r11, -0x69f8
	ctx.r[3].s64 = ctx.r[11].s64 + -27128;
	// 8326E928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E92C: 4AF8542D  bl 0x821f3d58
	ctx.lr = 0x8326E930;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E934: 906ACC24  stw r3, -0x33dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13276 as u32), ctx.r[3].u32 ) };
	// 8326E938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E944: 4E800020  blr
	return;
}

pub fn sub_8326E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E948 size=56
	// 8326E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E954: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E95C: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 8326E960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E964: 4AF853F5  bl 0x821f3d58
	ctx.lr = 0x8326E968;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E96C: 906ACC28  stw r3, -0x33d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13272 as u32), ctx.r[3].u32 ) };
	// 8326E970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E97C: 4E800020  blr
	return;
}

pub fn sub_8326E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E980 size=56
	// 8326E980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E98C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326E990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E994: 386B1034  addi r3, r11, 0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + 4148;
	// 8326E998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E99C: 4AF853BD  bl 0x821f3d58
	ctx.lr = 0x8326E9A0;
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
	// 8326E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E9A4: 906ACC2C  stw r3, -0x33d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13268 as u32), ctx.r[3].u32 ) };
	// 8326E9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E9B4: 4E800020  blr
	return;
}

pub fn sub_8326E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E9B8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E9F0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA28 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA60 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA98 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EAD0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB08 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB40 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB78 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EBB0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EBE8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC20 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC58 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326EC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC90 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326ECC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ECC8 size=12
	// 8326ECC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ECCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ECD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326ED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED08 size=12
	// 8326ED08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED48 size=12
	// 8326ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326ED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED88 size=12
	// 8326ED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EDC8 size=12
	// 8326EDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE08 size=12
	// 8326EE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE48 size=12
	// 8326EE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE88 size=12
	// 8326EE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EEC8 size=12
	// 8326EEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF08 size=12
	// 8326EF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF48 size=12
	// 8326EF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF88 size=12
	// 8326EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EFC8 size=12
	// 8326EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F008 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F040 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F078 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F0B0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F0E8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F120 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F158 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F190 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F1C8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F200 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F238 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F270 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F2A8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F2E0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F318 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F350 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F388 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F3C0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F3F8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F430 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F468 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F4A0 size=12
	// 8326F4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326F4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F4E0 size=12
	// 8326F4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F4E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326F520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F520 size=12
	// 8326F520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F560 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F598 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F5D0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F608 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F640 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F678 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F6B0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F6E8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F720 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F758 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F790 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F7C8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F800 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F838 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F870 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F8A8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F8E0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F918 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F950 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F988 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F9C0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326F9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F9F8 size=12
	// 8326F9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326FA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FA38 size=12
	// 8326FA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326FA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FA78 size=12
	// 8326FA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FAB8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FAF0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB28 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB60 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB98 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FBD0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC08 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC40 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC78 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FCB0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FCE8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD20 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD58 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD90 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FDC8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE00 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE38 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE70 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FEA8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FEE0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF18 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_8326FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF50 size=12
	// 8326FF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF90 size=12
	// 8326FF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_8326FFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FFD0 size=12
	// 8326FFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FFD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
}

pub fn sub_83270010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270010 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83270048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270048 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83270080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270080 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832700B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832700B8 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_832700F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832700F0 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83270128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270128 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83270160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270160 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

pub fn sub_83270198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270198 size=56
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
	crate::recompiler::externs::call(&mut ctx, base, 0x821F3D58);
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

