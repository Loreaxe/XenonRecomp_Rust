pub fn sub_83296120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296120 size=52
    let mut pc: u32 = 0x83296120;
    'dispatch: loop {
        match pc {
            0x83296120 => {
    //   block [0x83296120..0x83296154)
	// 83296120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329612C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296130: 386BDF9C  addi r3, r11, -0x2064
	ctx.r[3].s64 = ctx.r[11].s64 + -8292;
	// 83296134: 4AEF300D  bl 0x82189140
	ctx.lr = 0x83296138;
	sub_82189140(ctx, base);
	// 83296138: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329613C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296140: 916AE794  stw r11, -0x186c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6252 as u32), ctx.r[11].u32 ) };
	// 83296144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329614C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296158 size=52
    let mut pc: u32 = 0x83296158;
    'dispatch: loop {
        match pc {
            0x83296158 => {
    //   block [0x83296158..0x8329618C)
	// 83296158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329615C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296164: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83296168: 386B103C  addi r3, r11, 0x103c
	ctx.r[3].s64 = ctx.r[11].s64 + 4156;
	// 8329616C: 4AEF2FD5  bl 0x82189140
	ctx.lr = 0x83296170;
	sub_82189140(ctx, base);
	// 83296170: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296174: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296178: 916AE798  stw r11, -0x1868(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6248 as u32), ctx.r[11].u32 ) };
	// 8329617C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296190 size=52
    let mut pc: u32 = 0x83296190;
    'dispatch: loop {
        match pc {
            0x83296190 => {
    //   block [0x83296190..0x832961C4)
	// 83296190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329619C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832961A0: 386BCF28  addi r3, r11, -0x30d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12504;
	// 832961A4: 4AEF2F9D  bl 0x82189140
	ctx.lr = 0x832961A8;
	sub_82189140(ctx, base);
	// 832961A8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832961AC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832961B0: 916AE79C  stw r11, -0x1864(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6244 as u32), ctx.r[11].u32 ) };
	// 832961B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832961B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832961BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832961C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832961C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832961C8 size=52
    let mut pc: u32 = 0x832961C8;
    'dispatch: loop {
        match pc {
            0x832961C8 => {
    //   block [0x832961C8..0x832961FC)
	// 832961C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832961CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832961D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832961D4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832961D8: 386BCF18  addi r3, r11, -0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12520;
	// 832961DC: 4AEF2F65  bl 0x82189140
	ctx.lr = 0x832961E0;
	sub_82189140(ctx, base);
	// 832961E0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832961E4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832961E8: 916AE7A0  stw r11, -0x1860(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6240 as u32), ctx.r[11].u32 ) };
	// 832961EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832961F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832961F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832961F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296200 size=48
    let mut pc: u32 = 0x83296200;
    'dispatch: loop {
        match pc {
            0x83296200 => {
    //   block [0x83296200..0x83296230)
	// 83296200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329620C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296210: 386BDFE0  addi r3, r11, -0x2020
	ctx.r[3].s64 = ctx.r[11].s64 + -8224;
	// 83296214: 4AEF2F2D  bl 0x82189140
	ctx.lr = 0x83296218;
	sub_82189140(ctx, base);
	// 83296218: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329621C: 906AE7A4  stw r3, -0x185c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6236 as u32), ctx.r[3].u32 ) };
	// 83296220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329622C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296230 size=48
    let mut pc: u32 = 0x83296230;
    'dispatch: loop {
        match pc {
            0x83296230 => {
    //   block [0x83296230..0x83296260)
	// 83296230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329623C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296240: 386BDFE8  addi r3, r11, -0x2018
	ctx.r[3].s64 = ctx.r[11].s64 + -8216;
	// 83296244: 4AEF2EFD  bl 0x82189140
	ctx.lr = 0x83296248;
	sub_82189140(ctx, base);
	// 83296248: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329624C: 906AE7A8  stw r3, -0x1858(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6232 as u32), ctx.r[3].u32 ) };
	// 83296250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329625C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296260 size=48
    let mut pc: u32 = 0x83296260;
    'dispatch: loop {
        match pc {
            0x83296260 => {
    //   block [0x83296260..0x83296290)
	// 83296260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329626C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296270: 386BDFF4  addi r3, r11, -0x200c
	ctx.r[3].s64 = ctx.r[11].s64 + -8204;
	// 83296274: 4AEF2ECD  bl 0x82189140
	ctx.lr = 0x83296278;
	sub_82189140(ctx, base);
	// 83296278: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329627C: 906AE7AC  stw r3, -0x1854(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6228 as u32), ctx.r[3].u32 ) };
	// 83296280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329628C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296290 size=48
    let mut pc: u32 = 0x83296290;
    'dispatch: loop {
        match pc {
            0x83296290 => {
    //   block [0x83296290..0x832962C0)
	// 83296290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329629C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832962A0: 386BDFFC  addi r3, r11, -0x2004
	ctx.r[3].s64 = ctx.r[11].s64 + -8196;
	// 832962A4: 4AEF2E9D  bl 0x82189140
	ctx.lr = 0x832962A8;
	sub_82189140(ctx, base);
	// 832962A8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832962AC: 906AE7B0  stw r3, -0x1850(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6224 as u32), ctx.r[3].u32 ) };
	// 832962B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832962B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832962B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832962BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832962C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832962C0 size=48
    let mut pc: u32 = 0x832962C0;
    'dispatch: loop {
        match pc {
            0x832962C0 => {
    //   block [0x832962C0..0x832962F0)
	// 832962C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832962C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832962C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832962CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832962D0: 386BE008  addi r3, r11, -0x1ff8
	ctx.r[3].s64 = ctx.r[11].s64 + -8184;
	// 832962D4: 4AEF2E6D  bl 0x82189140
	ctx.lr = 0x832962D8;
	sub_82189140(ctx, base);
	// 832962D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832962DC: 906AE7B4  stw r3, -0x184c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6220 as u32), ctx.r[3].u32 ) };
	// 832962E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832962E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832962E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832962EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832962F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832962F0 size=48
    let mut pc: u32 = 0x832962F0;
    'dispatch: loop {
        match pc {
            0x832962F0 => {
    //   block [0x832962F0..0x83296320)
	// 832962F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832962F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832962F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832962FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83296300: 386B0D10  addi r3, r11, 0xd10
	ctx.r[3].s64 = ctx.r[11].s64 + 3344;
	// 83296304: 4AEF2E3D  bl 0x82189140
	ctx.lr = 0x83296308;
	sub_82189140(ctx, base);
	// 83296308: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329630C: 906AE7B8  stw r3, -0x1848(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6216 as u32), ctx.r[3].u32 ) };
	// 83296310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329631C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296320 size=52
    let mut pc: u32 = 0x83296320;
    'dispatch: loop {
        match pc {
            0x83296320 => {
    //   block [0x83296320..0x83296354)
	// 83296320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329632C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296330: 386BCE04  addi r3, r11, -0x31fc
	ctx.r[3].s64 = ctx.r[11].s64 + -12796;
	// 83296334: 4AEF2E0D  bl 0x82189140
	ctx.lr = 0x83296338;
	sub_82189140(ctx, base);
	// 83296338: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329633C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296340: 916AE7BC  stw r11, -0x1844(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6212 as u32), ctx.r[11].u32 ) };
	// 83296344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329634C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296358 size=52
    let mut pc: u32 = 0x83296358;
    'dispatch: loop {
        match pc {
            0x83296358 => {
    //   block [0x83296358..0x8329638C)
	// 83296358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329635C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296364: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296368: 386BCE18  addi r3, r11, -0x31e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12776;
	// 8329636C: 4AEF2DD5  bl 0x82189140
	ctx.lr = 0x83296370;
	sub_82189140(ctx, base);
	// 83296370: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296374: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296378: 916AE7C0  stw r11, -0x1840(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6208 as u32), ctx.r[11].u32 ) };
	// 8329637C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296390 size=52
    let mut pc: u32 = 0x83296390;
    'dispatch: loop {
        match pc {
            0x83296390 => {
    //   block [0x83296390..0x832963C4)
	// 83296390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329639C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832963A0: 386BCE2C  addi r3, r11, -0x31d4
	ctx.r[3].s64 = ctx.r[11].s64 + -12756;
	// 832963A4: 4AEF2D9D  bl 0x82189140
	ctx.lr = 0x832963A8;
	sub_82189140(ctx, base);
	// 832963A8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832963AC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832963B0: 916AE7C4  stw r11, -0x183c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6204 as u32), ctx.r[11].u32 ) };
	// 832963B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832963B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832963BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832963C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832963C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832963C8 size=52
    let mut pc: u32 = 0x832963C8;
    'dispatch: loop {
        match pc {
            0x832963C8 => {
    //   block [0x832963C8..0x832963FC)
	// 832963C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832963CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832963D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832963D4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832963D8: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 832963DC: 4AEF2D65  bl 0x82189140
	ctx.lr = 0x832963E0;
	sub_82189140(ctx, base);
	// 832963E0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832963E4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832963E8: 916AE7C8  stw r11, -0x1838(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6200 as u32), ctx.r[11].u32 ) };
	// 832963EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832963F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832963F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832963F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296400 size=52
    let mut pc: u32 = 0x83296400;
    'dispatch: loop {
        match pc {
            0x83296400 => {
    //   block [0x83296400..0x83296434)
	// 83296400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329640C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296410: 386BCE50  addi r3, r11, -0x31b0
	ctx.r[3].s64 = ctx.r[11].s64 + -12720;
	// 83296414: 4AEF2D2D  bl 0x82189140
	ctx.lr = 0x83296418;
	sub_82189140(ctx, base);
	// 83296418: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329641C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296420: 916AE7CC  stw r11, -0x1834(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6196 as u32), ctx.r[11].u32 ) };
	// 83296424: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329642C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296438 size=52
    let mut pc: u32 = 0x83296438;
    'dispatch: loop {
        match pc {
            0x83296438 => {
    //   block [0x83296438..0x8329646C)
	// 83296438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329643C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296444: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296448: 386BCE60  addi r3, r11, -0x31a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12704;
	// 8329644C: 4AEF2CF5  bl 0x82189140
	ctx.lr = 0x83296450;
	sub_82189140(ctx, base);
	// 83296450: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296454: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296458: 916AE7D0  stw r11, -0x1830(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6192 as u32), ctx.r[11].u32 ) };
	// 8329645C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296470 size=52
    let mut pc: u32 = 0x83296470;
    'dispatch: loop {
        match pc {
            0x83296470 => {
    //   block [0x83296470..0x832964A4)
	// 83296470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329647C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296480: 386BCE74  addi r3, r11, -0x318c
	ctx.r[3].s64 = ctx.r[11].s64 + -12684;
	// 83296484: 4AEF2CBD  bl 0x82189140
	ctx.lr = 0x83296488;
	sub_82189140(ctx, base);
	// 83296488: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329648C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296490: 916AE7D4  stw r11, -0x182c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6188 as u32), ctx.r[11].u32 ) };
	// 83296494: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329649C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832964A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832964A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832964A8 size=52
    let mut pc: u32 = 0x832964A8;
    'dispatch: loop {
        match pc {
            0x832964A8 => {
    //   block [0x832964A8..0x832964DC)
	// 832964A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832964AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832964B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832964B4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832964B8: 386BCE8C  addi r3, r11, -0x3174
	ctx.r[3].s64 = ctx.r[11].s64 + -12660;
	// 832964BC: 4AEF2C85  bl 0x82189140
	ctx.lr = 0x832964C0;
	sub_82189140(ctx, base);
	// 832964C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832964C4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832964C8: 916AE7D8  stw r11, -0x1828(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6184 as u32), ctx.r[11].u32 ) };
	// 832964CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832964D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832964D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832964D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832964E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832964E0 size=52
    let mut pc: u32 = 0x832964E0;
    'dispatch: loop {
        match pc {
            0x832964E0 => {
    //   block [0x832964E0..0x83296514)
	// 832964E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832964E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832964E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832964EC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832964F0: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 832964F4: 4AEF2C4D  bl 0x82189140
	ctx.lr = 0x832964F8;
	sub_82189140(ctx, base);
	// 832964F8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832964FC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296500: 916AE7DC  stw r11, -0x1824(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6180 as u32), ctx.r[11].u32 ) };
	// 83296504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329650C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296518 size=52
    let mut pc: u32 = 0x83296518;
    'dispatch: loop {
        match pc {
            0x83296518 => {
    //   block [0x83296518..0x8329654C)
	// 83296518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329651C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296524: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296528: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 8329652C: 4AEF2C15  bl 0x82189140
	ctx.lr = 0x83296530;
	sub_82189140(ctx, base);
	// 83296530: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296534: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296538: 916AE7E0  stw r11, -0x1820(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6176 as u32), ctx.r[11].u32 ) };
	// 8329653C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296550 size=52
    let mut pc: u32 = 0x83296550;
    'dispatch: loop {
        match pc {
            0x83296550 => {
    //   block [0x83296550..0x83296584)
	// 83296550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329655C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296560: 386BCED0  addi r3, r11, -0x3130
	ctx.r[3].s64 = ctx.r[11].s64 + -12592;
	// 83296564: 4AEF2BDD  bl 0x82189140
	ctx.lr = 0x83296568;
	sub_82189140(ctx, base);
	// 83296568: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329656C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296570: 916AE7E4  stw r11, -0x181c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6172 as u32), ctx.r[11].u32 ) };
	// 83296574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329657C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296588 size=52
    let mut pc: u32 = 0x83296588;
    'dispatch: loop {
        match pc {
            0x83296588 => {
    //   block [0x83296588..0x832965BC)
	// 83296588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329658C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296594: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296598: 386BCEEC  addi r3, r11, -0x3114
	ctx.r[3].s64 = ctx.r[11].s64 + -12564;
	// 8329659C: 4AEF2BA5  bl 0x82189140
	ctx.lr = 0x832965A0;
	sub_82189140(ctx, base);
	// 832965A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832965A4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832965A8: 916AE7E8  stw r11, -0x1818(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6168 as u32), ctx.r[11].u32 ) };
	// 832965AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832965B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832965B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832965B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832965C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832965C0 size=52
    let mut pc: u32 = 0x832965C0;
    'dispatch: loop {
        match pc {
            0x832965C0 => {
    //   block [0x832965C0..0x832965F4)
	// 832965C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832965C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832965C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832965CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832965D0: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 832965D4: 4AEF2B6D  bl 0x82189140
	ctx.lr = 0x832965D8;
	sub_82189140(ctx, base);
	// 832965D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832965DC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832965E0: 916AE7EC  stw r11, -0x1814(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6164 as u32), ctx.r[11].u32 ) };
	// 832965E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832965E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832965EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832965F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832965F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832965F8 size=52
    let mut pc: u32 = 0x832965F8;
    'dispatch: loop {
        match pc {
            0x832965F8 => {
    //   block [0x832965F8..0x8329662C)
	// 832965F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832965FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296604: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296608: 386BCF18  addi r3, r11, -0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12520;
	// 8329660C: 4AEF2B35  bl 0x82189140
	ctx.lr = 0x83296610;
	sub_82189140(ctx, base);
	// 83296610: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296614: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296618: 916AE7F0  stw r11, -0x1810(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6160 as u32), ctx.r[11].u32 ) };
	// 8329661C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296630 size=52
    let mut pc: u32 = 0x83296630;
    'dispatch: loop {
        match pc {
            0x83296630 => {
    //   block [0x83296630..0x83296664)
	// 83296630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329663C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296640: 386BCF28  addi r3, r11, -0x30d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12504;
	// 83296644: 4AEF2AFD  bl 0x82189140
	ctx.lr = 0x83296648;
	sub_82189140(ctx, base);
	// 83296648: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329664C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296650: 916AE7F4  stw r11, -0x180c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6156 as u32), ctx.r[11].u32 ) };
	// 83296654: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329665C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83296668 size=52
    let mut pc: u32 = 0x83296668;
    'dispatch: loop {
        match pc {
            0x83296668 => {
    //   block [0x83296668..0x83296684)
	// 83296668: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 8329666C: 3D408331  lis r10, -0x7ccf
	ctx.r[10].s64 = -2093940736;
	// 83296670: 392BE7F8  addi r9, r11, -0x1808
	ctx.r[9].s64 = ctx.r[11].s64 + -6152;
	// 83296674: 396A4B30  addi r11, r10, 0x4b30
	ctx.r[11].s64 = ctx.r[10].s64 + 19248;
	// 83296678: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8329667C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83296680: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x83296684; continue 'dispatch;
            }
            0x83296684 => {
    //   block [0x83296684..0x8329669C)
	// 83296684: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83296688: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8329668C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 83296690: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83296694: 4200FFF0  bdnz 0x83296684
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83296684; continue 'dispatch;
	}
	// 83296698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832966A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832966A0 size=48
    let mut pc: u32 = 0x832966A0;
    'dispatch: loop {
        match pc {
            0x832966A0 => {
    //   block [0x832966A0..0x832966D0)
	// 832966A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832966A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832966A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832966AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832966B0: 386B0E48  addi r3, r11, 0xe48
	ctx.r[3].s64 = ctx.r[11].s64 + 3656;
	// 832966B4: 4AEF2A8D  bl 0x82189140
	ctx.lr = 0x832966B8;
	sub_82189140(ctx, base);
	// 832966B8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832966BC: 906AE838  stw r3, -0x17c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6088 as u32), ctx.r[3].u32 ) };
	// 832966C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832966C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832966C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832966CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832966D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832966D0 size=48
    let mut pc: u32 = 0x832966D0;
    'dispatch: loop {
        match pc {
            0x832966D0 => {
    //   block [0x832966D0..0x83296700)
	// 832966D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832966D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832966D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832966DC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832966E0: 386BE504  addi r3, r11, -0x1afc
	ctx.r[3].s64 = ctx.r[11].s64 + -6908;
	// 832966E4: 4AEF2A5D  bl 0x82189140
	ctx.lr = 0x832966E8;
	sub_82189140(ctx, base);
	// 832966E8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832966EC: 906AE83C  stw r3, -0x17c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6084 as u32), ctx.r[3].u32 ) };
	// 832966F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832966F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832966F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832966FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296700 size=52
    let mut pc: u32 = 0x83296700;
    'dispatch: loop {
        match pc {
            0x83296700 => {
    //   block [0x83296700..0x83296734)
	// 83296700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329670C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296710: 386BE50C  addi r3, r11, -0x1af4
	ctx.r[3].s64 = ctx.r[11].s64 + -6900;
	// 83296714: 4AEF2A2D  bl 0x82189140
	ctx.lr = 0x83296718;
	sub_82189140(ctx, base);
	// 83296718: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329671C: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83296720: 916AE840  stw r11, -0x17c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6080 as u32), ctx.r[11].u32 ) };
	// 83296724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329672C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296738 size=52
    let mut pc: u32 = 0x83296738;
    'dispatch: loop {
        match pc {
            0x83296738 => {
    //   block [0x83296738..0x8329676C)
	// 83296738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329673C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296744: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296748: 386BE514  addi r3, r11, -0x1aec
	ctx.r[3].s64 = ctx.r[11].s64 + -6892;
	// 8329674C: 4AEF29F5  bl 0x82189140
	ctx.lr = 0x83296750;
	sub_82189140(ctx, base);
	// 83296750: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296754: 546B017E  clrlwi r11, r3, 5
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x07FFFFFFu64;
	// 83296758: 916AE844  stw r11, -0x17bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6076 as u32), ctx.r[11].u32 ) };
	// 8329675C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296770 size=52
    let mut pc: u32 = 0x83296770;
    'dispatch: loop {
        match pc {
            0x83296770 => {
    //   block [0x83296770..0x832967A4)
	// 83296770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329677C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296780: 386BCE04  addi r3, r11, -0x31fc
	ctx.r[3].s64 = ctx.r[11].s64 + -12796;
	// 83296784: 4AEF29BD  bl 0x82189140
	ctx.lr = 0x83296788;
	sub_82189140(ctx, base);
	// 83296788: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329678C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296790: 916AE848  stw r11, -0x17b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6072 as u32), ctx.r[11].u32 ) };
	// 83296794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329679C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832967A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832967A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832967A8 size=52
    let mut pc: u32 = 0x832967A8;
    'dispatch: loop {
        match pc {
            0x832967A8 => {
    //   block [0x832967A8..0x832967DC)
	// 832967A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832967AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832967B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832967B4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832967B8: 386BCE18  addi r3, r11, -0x31e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12776;
	// 832967BC: 4AEF2985  bl 0x82189140
	ctx.lr = 0x832967C0;
	sub_82189140(ctx, base);
	// 832967C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832967C4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832967C8: 916AE84C  stw r11, -0x17b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6068 as u32), ctx.r[11].u32 ) };
	// 832967CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832967D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832967D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832967D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832967E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832967E0 size=52
    let mut pc: u32 = 0x832967E0;
    'dispatch: loop {
        match pc {
            0x832967E0 => {
    //   block [0x832967E0..0x83296814)
	// 832967E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832967E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832967E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832967EC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832967F0: 386BCE2C  addi r3, r11, -0x31d4
	ctx.r[3].s64 = ctx.r[11].s64 + -12756;
	// 832967F4: 4AEF294D  bl 0x82189140
	ctx.lr = 0x832967F8;
	sub_82189140(ctx, base);
	// 832967F8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832967FC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296800: 916AE850  stw r11, -0x17b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6064 as u32), ctx.r[11].u32 ) };
	// 83296804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329680C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296818 size=52
    let mut pc: u32 = 0x83296818;
    'dispatch: loop {
        match pc {
            0x83296818 => {
    //   block [0x83296818..0x8329684C)
	// 83296818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329681C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296824: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296828: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 8329682C: 4AEF2915  bl 0x82189140
	ctx.lr = 0x83296830;
	sub_82189140(ctx, base);
	// 83296830: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296834: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296838: 916AE854  stw r11, -0x17ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6060 as u32), ctx.r[11].u32 ) };
	// 8329683C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296850 size=52
    let mut pc: u32 = 0x83296850;
    'dispatch: loop {
        match pc {
            0x83296850 => {
    //   block [0x83296850..0x83296884)
	// 83296850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329685C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296860: 386BCE50  addi r3, r11, -0x31b0
	ctx.r[3].s64 = ctx.r[11].s64 + -12720;
	// 83296864: 4AEF28DD  bl 0x82189140
	ctx.lr = 0x83296868;
	sub_82189140(ctx, base);
	// 83296868: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329686C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296870: 916AE858  stw r11, -0x17a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6056 as u32), ctx.r[11].u32 ) };
	// 83296874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329687C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296888 size=52
    let mut pc: u32 = 0x83296888;
    'dispatch: loop {
        match pc {
            0x83296888 => {
    //   block [0x83296888..0x832968BC)
	// 83296888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329688C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296894: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296898: 386BCE60  addi r3, r11, -0x31a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12704;
	// 8329689C: 4AEF28A5  bl 0x82189140
	ctx.lr = 0x832968A0;
	sub_82189140(ctx, base);
	// 832968A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832968A4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832968A8: 916AE85C  stw r11, -0x17a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6052 as u32), ctx.r[11].u32 ) };
	// 832968AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832968B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832968B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832968B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832968C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832968C0 size=52
    let mut pc: u32 = 0x832968C0;
    'dispatch: loop {
        match pc {
            0x832968C0 => {
    //   block [0x832968C0..0x832968F4)
	// 832968C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832968C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832968C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832968CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832968D0: 386BCE74  addi r3, r11, -0x318c
	ctx.r[3].s64 = ctx.r[11].s64 + -12684;
	// 832968D4: 4AEF286D  bl 0x82189140
	ctx.lr = 0x832968D8;
	sub_82189140(ctx, base);
	// 832968D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832968DC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832968E0: 916AE860  stw r11, -0x17a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6048 as u32), ctx.r[11].u32 ) };
	// 832968E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832968E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832968EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832968F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832968F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832968F8 size=52
    let mut pc: u32 = 0x832968F8;
    'dispatch: loop {
        match pc {
            0x832968F8 => {
    //   block [0x832968F8..0x8329692C)
	// 832968F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832968FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296904: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296908: 386BCE8C  addi r3, r11, -0x3174
	ctx.r[3].s64 = ctx.r[11].s64 + -12660;
	// 8329690C: 4AEF2835  bl 0x82189140
	ctx.lr = 0x83296910;
	sub_82189140(ctx, base);
	// 83296910: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296914: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296918: 916AE864  stw r11, -0x179c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6044 as u32), ctx.r[11].u32 ) };
	// 8329691C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296930 size=52
    let mut pc: u32 = 0x83296930;
    'dispatch: loop {
        match pc {
            0x83296930 => {
    //   block [0x83296930..0x83296964)
	// 83296930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329693C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296940: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 83296944: 4AEF27FD  bl 0x82189140
	ctx.lr = 0x83296948;
	sub_82189140(ctx, base);
	// 83296948: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329694C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296950: 916AE868  stw r11, -0x1798(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6040 as u32), ctx.r[11].u32 ) };
	// 83296954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329695C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296968 size=52
    let mut pc: u32 = 0x83296968;
    'dispatch: loop {
        match pc {
            0x83296968 => {
    //   block [0x83296968..0x8329699C)
	// 83296968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329696C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296974: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296978: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 8329697C: 4AEF27C5  bl 0x82189140
	ctx.lr = 0x83296980;
	sub_82189140(ctx, base);
	// 83296980: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296984: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296988: 916AE86C  stw r11, -0x1794(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6036 as u32), ctx.r[11].u32 ) };
	// 8329698C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832969A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832969A0 size=52
    let mut pc: u32 = 0x832969A0;
    'dispatch: loop {
        match pc {
            0x832969A0 => {
    //   block [0x832969A0..0x832969D4)
	// 832969A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832969A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832969A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832969AC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832969B0: 386BCED0  addi r3, r11, -0x3130
	ctx.r[3].s64 = ctx.r[11].s64 + -12592;
	// 832969B4: 4AEF278D  bl 0x82189140
	ctx.lr = 0x832969B8;
	sub_82189140(ctx, base);
	// 832969B8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832969BC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832969C0: 916AE870  stw r11, -0x1790(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6032 as u32), ctx.r[11].u32 ) };
	// 832969C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832969C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832969CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832969D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832969D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832969D8 size=52
    let mut pc: u32 = 0x832969D8;
    'dispatch: loop {
        match pc {
            0x832969D8 => {
    //   block [0x832969D8..0x83296A0C)
	// 832969D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832969DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832969E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832969E4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832969E8: 386BCEEC  addi r3, r11, -0x3114
	ctx.r[3].s64 = ctx.r[11].s64 + -12564;
	// 832969EC: 4AEF2755  bl 0x82189140
	ctx.lr = 0x832969F0;
	sub_82189140(ctx, base);
	// 832969F0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832969F4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832969F8: 916AE874  stw r11, -0x178c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6028 as u32), ctx.r[11].u32 ) };
	// 832969FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296A10 size=52
    let mut pc: u32 = 0x83296A10;
    'dispatch: loop {
        match pc {
            0x83296A10 => {
    //   block [0x83296A10..0x83296A44)
	// 83296A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296A18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296A1C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296A20: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 83296A24: 4AEF271D  bl 0x82189140
	ctx.lr = 0x83296A28;
	sub_82189140(ctx, base);
	// 83296A28: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296A2C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296A30: 916AE878  stw r11, -0x1788(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6024 as u32), ctx.r[11].u32 ) };
	// 83296A34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296A48 size=52
    let mut pc: u32 = 0x83296A48;
    'dispatch: loop {
        match pc {
            0x83296A48 => {
    //   block [0x83296A48..0x83296A7C)
	// 83296A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296A50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296A54: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296A58: 386BCF18  addi r3, r11, -0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12520;
	// 83296A5C: 4AEF26E5  bl 0x82189140
	ctx.lr = 0x83296A60;
	sub_82189140(ctx, base);
	// 83296A60: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296A64: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296A68: 916AE87C  stw r11, -0x1784(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6020 as u32), ctx.r[11].u32 ) };
	// 83296A6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296A80 size=52
    let mut pc: u32 = 0x83296A80;
    'dispatch: loop {
        match pc {
            0x83296A80 => {
    //   block [0x83296A80..0x83296AB4)
	// 83296A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296A8C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296A90: 386BCF28  addi r3, r11, -0x30d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12504;
	// 83296A94: 4AEF26AD  bl 0x82189140
	ctx.lr = 0x83296A98;
	sub_82189140(ctx, base);
	// 83296A98: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296A9C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296AA0: 916AE880  stw r11, -0x1780(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6016 as u32), ctx.r[11].u32 ) };
	// 83296AA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296AB8 size=52
    let mut pc: u32 = 0x83296AB8;
    'dispatch: loop {
        match pc {
            0x83296AB8 => {
    //   block [0x83296AB8..0x83296AEC)
	// 83296AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296AC4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296AC8: 386BCE04  addi r3, r11, -0x31fc
	ctx.r[3].s64 = ctx.r[11].s64 + -12796;
	// 83296ACC: 4AEF2675  bl 0x82189140
	ctx.lr = 0x83296AD0;
	sub_82189140(ctx, base);
	// 83296AD0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296AD4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296AD8: 916AE884  stw r11, -0x177c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6012 as u32), ctx.r[11].u32 ) };
	// 83296ADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296AF0 size=52
    let mut pc: u32 = 0x83296AF0;
    'dispatch: loop {
        match pc {
            0x83296AF0 => {
    //   block [0x83296AF0..0x83296B24)
	// 83296AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296AFC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296B00: 386BCE18  addi r3, r11, -0x31e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12776;
	// 83296B04: 4AEF263D  bl 0x82189140
	ctx.lr = 0x83296B08;
	sub_82189140(ctx, base);
	// 83296B08: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296B0C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296B10: 916AE888  stw r11, -0x1778(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6008 as u32), ctx.r[11].u32 ) };
	// 83296B14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296B28 size=52
    let mut pc: u32 = 0x83296B28;
    'dispatch: loop {
        match pc {
            0x83296B28 => {
    //   block [0x83296B28..0x83296B5C)
	// 83296B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296B34: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296B38: 386BCE2C  addi r3, r11, -0x31d4
	ctx.r[3].s64 = ctx.r[11].s64 + -12756;
	// 83296B3C: 4AEF2605  bl 0x82189140
	ctx.lr = 0x83296B40;
	sub_82189140(ctx, base);
	// 83296B40: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296B44: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296B48: 916AE88C  stw r11, -0x1774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6004 as u32), ctx.r[11].u32 ) };
	// 83296B4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296B60 size=52
    let mut pc: u32 = 0x83296B60;
    'dispatch: loop {
        match pc {
            0x83296B60 => {
    //   block [0x83296B60..0x83296B94)
	// 83296B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296B6C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296B70: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 83296B74: 4AEF25CD  bl 0x82189140
	ctx.lr = 0x83296B78;
	sub_82189140(ctx, base);
	// 83296B78: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296B7C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296B80: 916AE890  stw r11, -0x1770(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6000 as u32), ctx.r[11].u32 ) };
	// 83296B84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296B98 size=52
    let mut pc: u32 = 0x83296B98;
    'dispatch: loop {
        match pc {
            0x83296B98 => {
    //   block [0x83296B98..0x83296BCC)
	// 83296B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296BA4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296BA8: 386BCE50  addi r3, r11, -0x31b0
	ctx.r[3].s64 = ctx.r[11].s64 + -12720;
	// 83296BAC: 4AEF2595  bl 0x82189140
	ctx.lr = 0x83296BB0;
	sub_82189140(ctx, base);
	// 83296BB0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296BB4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296BB8: 916AE894  stw r11, -0x176c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5996 as u32), ctx.r[11].u32 ) };
	// 83296BBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296BD0 size=52
    let mut pc: u32 = 0x83296BD0;
    'dispatch: loop {
        match pc {
            0x83296BD0 => {
    //   block [0x83296BD0..0x83296C04)
	// 83296BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296BDC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296BE0: 386BCE60  addi r3, r11, -0x31a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12704;
	// 83296BE4: 4AEF255D  bl 0x82189140
	ctx.lr = 0x83296BE8;
	sub_82189140(ctx, base);
	// 83296BE8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296BEC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296BF0: 916AE898  stw r11, -0x1768(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5992 as u32), ctx.r[11].u32 ) };
	// 83296BF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296C08 size=52
    let mut pc: u32 = 0x83296C08;
    'dispatch: loop {
        match pc {
            0x83296C08 => {
    //   block [0x83296C08..0x83296C3C)
	// 83296C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296C14: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296C18: 386BCE74  addi r3, r11, -0x318c
	ctx.r[3].s64 = ctx.r[11].s64 + -12684;
	// 83296C1C: 4AEF2525  bl 0x82189140
	ctx.lr = 0x83296C20;
	sub_82189140(ctx, base);
	// 83296C20: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296C24: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296C28: 916AE89C  stw r11, -0x1764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5988 as u32), ctx.r[11].u32 ) };
	// 83296C2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296C40 size=52
    let mut pc: u32 = 0x83296C40;
    'dispatch: loop {
        match pc {
            0x83296C40 => {
    //   block [0x83296C40..0x83296C74)
	// 83296C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296C4C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296C50: 386BCE8C  addi r3, r11, -0x3174
	ctx.r[3].s64 = ctx.r[11].s64 + -12660;
	// 83296C54: 4AEF24ED  bl 0x82189140
	ctx.lr = 0x83296C58;
	sub_82189140(ctx, base);
	// 83296C58: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296C5C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296C60: 916AE8A0  stw r11, -0x1760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5984 as u32), ctx.r[11].u32 ) };
	// 83296C64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296C78 size=52
    let mut pc: u32 = 0x83296C78;
    'dispatch: loop {
        match pc {
            0x83296C78 => {
    //   block [0x83296C78..0x83296CAC)
	// 83296C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296C80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296C84: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296C88: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 83296C8C: 4AEF24B5  bl 0x82189140
	ctx.lr = 0x83296C90;
	sub_82189140(ctx, base);
	// 83296C90: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296C94: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296C98: 916AE8A4  stw r11, -0x175c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5980 as u32), ctx.r[11].u32 ) };
	// 83296C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296CB0 size=52
    let mut pc: u32 = 0x83296CB0;
    'dispatch: loop {
        match pc {
            0x83296CB0 => {
    //   block [0x83296CB0..0x83296CE4)
	// 83296CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296CBC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296CC0: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 83296CC4: 4AEF247D  bl 0x82189140
	ctx.lr = 0x83296CC8;
	sub_82189140(ctx, base);
	// 83296CC8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296CCC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296CD0: 916AE8A8  stw r11, -0x1758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5976 as u32), ctx.r[11].u32 ) };
	// 83296CD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296CE8 size=52
    let mut pc: u32 = 0x83296CE8;
    'dispatch: loop {
        match pc {
            0x83296CE8 => {
    //   block [0x83296CE8..0x83296D1C)
	// 83296CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296CF4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296CF8: 386BCED0  addi r3, r11, -0x3130
	ctx.r[3].s64 = ctx.r[11].s64 + -12592;
	// 83296CFC: 4AEF2445  bl 0x82189140
	ctx.lr = 0x83296D00;
	sub_82189140(ctx, base);
	// 83296D00: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296D04: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296D08: 916AE8AC  stw r11, -0x1754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5972 as u32), ctx.r[11].u32 ) };
	// 83296D0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296D20 size=52
    let mut pc: u32 = 0x83296D20;
    'dispatch: loop {
        match pc {
            0x83296D20 => {
    //   block [0x83296D20..0x83296D54)
	// 83296D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296D2C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296D30: 386BCEEC  addi r3, r11, -0x3114
	ctx.r[3].s64 = ctx.r[11].s64 + -12564;
	// 83296D34: 4AEF240D  bl 0x82189140
	ctx.lr = 0x83296D38;
	sub_82189140(ctx, base);
	// 83296D38: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296D3C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296D40: 916AE8B0  stw r11, -0x1750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5968 as u32), ctx.r[11].u32 ) };
	// 83296D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296D58 size=52
    let mut pc: u32 = 0x83296D58;
    'dispatch: loop {
        match pc {
            0x83296D58 => {
    //   block [0x83296D58..0x83296D8C)
	// 83296D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296D64: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296D68: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 83296D6C: 4AEF23D5  bl 0x82189140
	ctx.lr = 0x83296D70;
	sub_82189140(ctx, base);
	// 83296D70: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296D74: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296D78: 916AE8B4  stw r11, -0x174c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5964 as u32), ctx.r[11].u32 ) };
	// 83296D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296D90 size=52
    let mut pc: u32 = 0x83296D90;
    'dispatch: loop {
        match pc {
            0x83296D90 => {
    //   block [0x83296D90..0x83296DC4)
	// 83296D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296D98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296D9C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296DA0: 386BCF18  addi r3, r11, -0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12520;
	// 83296DA4: 4AEF239D  bl 0x82189140
	ctx.lr = 0x83296DA8;
	sub_82189140(ctx, base);
	// 83296DA8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296DAC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296DB0: 916AE8B8  stw r11, -0x1748(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5960 as u32), ctx.r[11].u32 ) };
	// 83296DB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296DC8 size=52
    let mut pc: u32 = 0x83296DC8;
    'dispatch: loop {
        match pc {
            0x83296DC8 => {
    //   block [0x83296DC8..0x83296DFC)
	// 83296DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296DD4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296DD8: 386BCF28  addi r3, r11, -0x30d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12504;
	// 83296DDC: 4AEF2365  bl 0x82189140
	ctx.lr = 0x83296DE0;
	sub_82189140(ctx, base);
	// 83296DE0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296DE4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296DE8: 916AE8BC  stw r11, -0x1744(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5956 as u32), ctx.r[11].u32 ) };
	// 83296DEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83296E00 size=12
    let mut pc: u32 = 0x83296E00;
    'dispatch: loop {
        match pc {
            0x83296E00 => {
    //   block [0x83296E00..0x83296E0C)
	// 83296E00: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83296E04: 386B70F8  addi r3, r11, 0x70f8
	ctx.r[3].s64 = ctx.r[11].s64 + 28920;
	// 83296E08: 4BA13118  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83296E10 size=12
    let mut pc: u32 = 0x83296E10;
    'dispatch: loop {
        match pc {
            0x83296E10 => {
    //   block [0x83296E10..0x83296E1C)
	// 83296E10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83296E14: 386B7108  addi r3, r11, 0x7108
	ctx.r[3].s64 = ctx.r[11].s64 + 28936;
	// 83296E18: 4BA13108  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83296E20 size=12
    let mut pc: u32 = 0x83296E20;
    'dispatch: loop {
        match pc {
            0x83296E20 => {
    //   block [0x83296E20..0x83296E2C)
	// 83296E20: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83296E24: 386B7118  addi r3, r11, 0x7118
	ctx.r[3].s64 = ctx.r[11].s64 + 28952;
	// 83296E28: 4BA130F8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83296E30 size=12
    let mut pc: u32 = 0x83296E30;
    'dispatch: loop {
        match pc {
            0x83296E30 => {
    //   block [0x83296E30..0x83296E3C)
	// 83296E30: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83296E34: 386B7128  addi r3, r11, 0x7128
	ctx.r[3].s64 = ctx.r[11].s64 + 28968;
	// 83296E38: 4BA130E8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296E40 size=52
    let mut pc: u32 = 0x83296E40;
    'dispatch: loop {
        match pc {
            0x83296E40 => {
    //   block [0x83296E40..0x83296E74)
	// 83296E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296E4C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296E50: 386BCF34  addi r3, r11, -0x30cc
	ctx.r[3].s64 = ctx.r[11].s64 + -12492;
	// 83296E54: 4AEF22ED  bl 0x82189140
	ctx.lr = 0x83296E58;
	sub_82189140(ctx, base);
	// 83296E58: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296E5C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296E60: 916AE8E8  stw r11, -0x1718(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5912 as u32), ctx.r[11].u32 ) };
	// 83296E64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296E78 size=52
    let mut pc: u32 = 0x83296E78;
    'dispatch: loop {
        match pc {
            0x83296E78 => {
    //   block [0x83296E78..0x83296EAC)
	// 83296E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296E84: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296E88: 386BCF40  addi r3, r11, -0x30c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12480;
	// 83296E8C: 4AEF22B5  bl 0x82189140
	ctx.lr = 0x83296E90;
	sub_82189140(ctx, base);
	// 83296E90: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296E94: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296E98: 916AE8EC  stw r11, -0x1714(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5908 as u32), ctx.r[11].u32 ) };
	// 83296E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296EB0 size=52
    let mut pc: u32 = 0x83296EB0;
    'dispatch: loop {
        match pc {
            0x83296EB0 => {
    //   block [0x83296EB0..0x83296EE4)
	// 83296EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296EBC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296EC0: 386BCF4C  addi r3, r11, -0x30b4
	ctx.r[3].s64 = ctx.r[11].s64 + -12468;
	// 83296EC4: 4AEF227D  bl 0x82189140
	ctx.lr = 0x83296EC8;
	sub_82189140(ctx, base);
	// 83296EC8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296ECC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296ED0: 916AE8F0  stw r11, -0x1710(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5904 as u32), ctx.r[11].u32 ) };
	// 83296ED4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296EE8 size=52
    let mut pc: u32 = 0x83296EE8;
    'dispatch: loop {
        match pc {
            0x83296EE8 => {
    //   block [0x83296EE8..0x83296F1C)
	// 83296EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296EF4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296EF8: 386BCF60  addi r3, r11, -0x30a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12448;
	// 83296EFC: 4AEF2245  bl 0x82189140
	ctx.lr = 0x83296F00;
	sub_82189140(ctx, base);
	// 83296F00: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296F04: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296F08: 916AE8F4  stw r11, -0x170c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5900 as u32), ctx.r[11].u32 ) };
	// 83296F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296F20 size=52
    let mut pc: u32 = 0x83296F20;
    'dispatch: loop {
        match pc {
            0x83296F20 => {
    //   block [0x83296F20..0x83296F54)
	// 83296F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296F2C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296F30: 386BCF74  addi r3, r11, -0x308c
	ctx.r[3].s64 = ctx.r[11].s64 + -12428;
	// 83296F34: 4AEF220D  bl 0x82189140
	ctx.lr = 0x83296F38;
	sub_82189140(ctx, base);
	// 83296F38: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296F3C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296F40: 916AE8F8  stw r11, -0x1708(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5896 as u32), ctx.r[11].u32 ) };
	// 83296F44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296F58 size=52
    let mut pc: u32 = 0x83296F58;
    'dispatch: loop {
        match pc {
            0x83296F58 => {
    //   block [0x83296F58..0x83296F8C)
	// 83296F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296F64: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296F68: 386BCF80  addi r3, r11, -0x3080
	ctx.r[3].s64 = ctx.r[11].s64 + -12416;
	// 83296F6C: 4AEF21D5  bl 0x82189140
	ctx.lr = 0x83296F70;
	sub_82189140(ctx, base);
	// 83296F70: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296F74: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296F78: 916AE8FC  stw r11, -0x1704(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5892 as u32), ctx.r[11].u32 ) };
	// 83296F7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296F90 size=52
    let mut pc: u32 = 0x83296F90;
    'dispatch: loop {
        match pc {
            0x83296F90 => {
    //   block [0x83296F90..0x83296FC4)
	// 83296F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296F9C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296FA0: 386BCF8C  addi r3, r11, -0x3074
	ctx.r[3].s64 = ctx.r[11].s64 + -12404;
	// 83296FA4: 4AEF219D  bl 0x82189140
	ctx.lr = 0x83296FA8;
	sub_82189140(ctx, base);
	// 83296FA8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296FAC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296FB0: 916AE900  stw r11, -0x1700(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5888 as u32), ctx.r[11].u32 ) };
	// 83296FB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296FC8 size=52
    let mut pc: u32 = 0x83296FC8;
    'dispatch: loop {
        match pc {
            0x83296FC8 => {
    //   block [0x83296FC8..0x83296FFC)
	// 83296FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296FD4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296FD8: 386BCF9C  addi r3, r11, -0x3064
	ctx.r[3].s64 = ctx.r[11].s64 + -12388;
	// 83296FDC: 4AEF2165  bl 0x82189140
	ctx.lr = 0x83296FE0;
	sub_82189140(ctx, base);
	// 83296FE0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296FE4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296FE8: 916AE904  stw r11, -0x16fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5884 as u32), ctx.r[11].u32 ) };
	// 83296FEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297000 size=52
    let mut pc: u32 = 0x83297000;
    'dispatch: loop {
        match pc {
            0x83297000 => {
    //   block [0x83297000..0x83297034)
	// 83297000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329700C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297010: 386BCFAC  addi r3, r11, -0x3054
	ctx.r[3].s64 = ctx.r[11].s64 + -12372;
	// 83297014: 4AEF212D  bl 0x82189140
	ctx.lr = 0x83297018;
	sub_82189140(ctx, base);
	// 83297018: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329701C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297020: 916AE908  stw r11, -0x16f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5880 as u32), ctx.r[11].u32 ) };
	// 83297024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329702C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297038 size=52
    let mut pc: u32 = 0x83297038;
    'dispatch: loop {
        match pc {
            0x83297038 => {
    //   block [0x83297038..0x8329706C)
	// 83297038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329703C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297044: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297048: 386BCFC0  addi r3, r11, -0x3040
	ctx.r[3].s64 = ctx.r[11].s64 + -12352;
	// 8329704C: 4AEF20F5  bl 0x82189140
	ctx.lr = 0x83297050;
	sub_82189140(ctx, base);
	// 83297050: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83297054: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297058: 916AE90C  stw r11, -0x16f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5876 as u32), ctx.r[11].u32 ) };
	// 8329705C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297070 size=52
    let mut pc: u32 = 0x83297070;
    'dispatch: loop {
        match pc {
            0x83297070 => {
    //   block [0x83297070..0x832970A4)
	// 83297070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329707C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297080: 386BCFD0  addi r3, r11, -0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + -12336;
	// 83297084: 4AEF20BD  bl 0x82189140
	ctx.lr = 0x83297088;
	sub_82189140(ctx, base);
	// 83297088: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329708C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297090: 916AE910  stw r11, -0x16f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5872 as u32), ctx.r[11].u32 ) };
	// 83297094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329709C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832970A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832970A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832970A8 size=52
    let mut pc: u32 = 0x832970A8;
    'dispatch: loop {
        match pc {
            0x832970A8 => {
    //   block [0x832970A8..0x832970DC)
	// 832970A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832970AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832970B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832970B4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832970B8: 386BCFE4  addi r3, r11, -0x301c
	ctx.r[3].s64 = ctx.r[11].s64 + -12316;
	// 832970BC: 4AEF2085  bl 0x82189140
	ctx.lr = 0x832970C0;
	sub_82189140(ctx, base);
	// 832970C0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832970C4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832970C8: 916AE914  stw r11, -0x16ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5868 as u32), ctx.r[11].u32 ) };
	// 832970CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832970D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832970D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832970D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832970E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832970E0 size=52
    let mut pc: u32 = 0x832970E0;
    'dispatch: loop {
        match pc {
            0x832970E0 => {
    //   block [0x832970E0..0x83297114)
	// 832970E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832970E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832970E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832970EC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832970F0: 386BCFFC  addi r3, r11, -0x3004
	ctx.r[3].s64 = ctx.r[11].s64 + -12292;
	// 832970F4: 4AEF204D  bl 0x82189140
	ctx.lr = 0x832970F8;
	sub_82189140(ctx, base);
	// 832970F8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832970FC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297100: 916AE918  stw r11, -0x16e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5864 as u32), ctx.r[11].u32 ) };
	// 83297104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329710C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297118 size=52
    let mut pc: u32 = 0x83297118;
    'dispatch: loop {
        match pc {
            0x83297118 => {
    //   block [0x83297118..0x8329714C)
	// 83297118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329711C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297124: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297128: 386BD010  addi r3, r11, -0x2ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -12272;
	// 8329712C: 4AEF2015  bl 0x82189140
	ctx.lr = 0x83297130;
	sub_82189140(ctx, base);
	// 83297130: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83297134: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297138: 916AE91C  stw r11, -0x16e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5860 as u32), ctx.r[11].u32 ) };
	// 8329713C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297150 size=52
    let mut pc: u32 = 0x83297150;
    'dispatch: loop {
        match pc {
            0x83297150 => {
    //   block [0x83297150..0x83297184)
	// 83297150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329715C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297160: 386BD020  addi r3, r11, -0x2fe0
	ctx.r[3].s64 = ctx.r[11].s64 + -12256;
	// 83297164: 4AEF1FDD  bl 0x82189140
	ctx.lr = 0x83297168;
	sub_82189140(ctx, base);
	// 83297168: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329716C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297170: 916AE920  stw r11, -0x16e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5856 as u32), ctx.r[11].u32 ) };
	// 83297174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329717C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297188 size=52
    let mut pc: u32 = 0x83297188;
    'dispatch: loop {
        match pc {
            0x83297188 => {
    //   block [0x83297188..0x832971BC)
	// 83297188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329718C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297194: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297198: 386BD034  addi r3, r11, -0x2fcc
	ctx.r[3].s64 = ctx.r[11].s64 + -12236;
	// 8329719C: 4AEF1FA5  bl 0x82189140
	ctx.lr = 0x832971A0;
	sub_82189140(ctx, base);
	// 832971A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832971A4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832971A8: 916AE924  stw r11, -0x16dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5852 as u32), ctx.r[11].u32 ) };
	// 832971AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832971B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832971B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832971B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832971C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832971C0 size=52
    let mut pc: u32 = 0x832971C0;
    'dispatch: loop {
        match pc {
            0x832971C0 => {
    //   block [0x832971C0..0x832971F4)
	// 832971C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832971C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832971C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832971CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832971D0: 386BD04C  addi r3, r11, -0x2fb4
	ctx.r[3].s64 = ctx.r[11].s64 + -12212;
	// 832971D4: 4AEF1F6D  bl 0x82189140
	ctx.lr = 0x832971D8;
	sub_82189140(ctx, base);
	// 832971D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832971DC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832971E0: 916AE928  stw r11, -0x16d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5848 as u32), ctx.r[11].u32 ) };
	// 832971E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832971E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832971EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832971F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832971F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832971F8 size=52
    let mut pc: u32 = 0x832971F8;
    'dispatch: loop {
        match pc {
            0x832971F8 => {
    //   block [0x832971F8..0x8329722C)
	// 832971F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832971FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297204: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297208: 386BD054  addi r3, r11, -0x2fac
	ctx.r[3].s64 = ctx.r[11].s64 + -12204;
	// 8329720C: 4AEF1F35  bl 0x82189140
	ctx.lr = 0x83297210;
	sub_82189140(ctx, base);
	// 83297210: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83297214: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297218: 916AE92C  stw r11, -0x16d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5844 as u32), ctx.r[11].u32 ) };
	// 8329721C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297230 size=52
    let mut pc: u32 = 0x83297230;
    'dispatch: loop {
        match pc {
            0x83297230 => {
    //   block [0x83297230..0x83297264)
	// 83297230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329723C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297240: 386BD060  addi r3, r11, -0x2fa0
	ctx.r[3].s64 = ctx.r[11].s64 + -12192;
	// 83297244: 4AEF1EFD  bl 0x82189140
	ctx.lr = 0x83297248;
	sub_82189140(ctx, base);
	// 83297248: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329724C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297250: 916AE930  stw r11, -0x16d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5840 as u32), ctx.r[11].u32 ) };
	// 83297254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329725C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297268 size=52
    let mut pc: u32 = 0x83297268;
    'dispatch: loop {
        match pc {
            0x83297268 => {
    //   block [0x83297268..0x8329729C)
	// 83297268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297274: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297278: 386BD068  addi r3, r11, -0x2f98
	ctx.r[3].s64 = ctx.r[11].s64 + -12184;
	// 8329727C: 4AEF1EC5  bl 0x82189140
	ctx.lr = 0x83297280;
	sub_82189140(ctx, base);
	// 83297280: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83297284: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297288: 916AE934  stw r11, -0x16cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5836 as u32), ctx.r[11].u32 ) };
	// 8329728C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832972A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832972A0 size=52
    let mut pc: u32 = 0x832972A0;
    'dispatch: loop {
        match pc {
            0x832972A0 => {
    //   block [0x832972A0..0x832972D4)
	// 832972A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832972A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832972A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832972AC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832972B0: 386BD070  addi r3, r11, -0x2f90
	ctx.r[3].s64 = ctx.r[11].s64 + -12176;
	// 832972B4: 4AEF1E8D  bl 0x82189140
	ctx.lr = 0x832972B8;
	sub_82189140(ctx, base);
	// 832972B8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832972BC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832972C0: 916AE938  stw r11, -0x16c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5832 as u32), ctx.r[11].u32 ) };
	// 832972C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832972C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832972CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832972D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832972D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832972D8 size=52
    let mut pc: u32 = 0x832972D8;
    'dispatch: loop {
        match pc {
            0x832972D8 => {
    //   block [0x832972D8..0x8329730C)
	// 832972D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832972DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832972E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832972E4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832972E8: 386BD078  addi r3, r11, -0x2f88
	ctx.r[3].s64 = ctx.r[11].s64 + -12168;
	// 832972EC: 4AEF1E55  bl 0x82189140
	ctx.lr = 0x832972F0;
	sub_82189140(ctx, base);
	// 832972F0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832972F4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832972F8: 916AE93C  stw r11, -0x16c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5828 as u32), ctx.r[11].u32 ) };
	// 832972FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297310 size=52
    let mut pc: u32 = 0x83297310;
    'dispatch: loop {
        match pc {
            0x83297310 => {
    //   block [0x83297310..0x83297344)
	// 83297310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329731C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297320: 386BD080  addi r3, r11, -0x2f80
	ctx.r[3].s64 = ctx.r[11].s64 + -12160;
	// 83297324: 4AEF1E1D  bl 0x82189140
	ctx.lr = 0x83297328;
	sub_82189140(ctx, base);
	// 83297328: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329732C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297330: 916AE940  stw r11, -0x16c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5824 as u32), ctx.r[11].u32 ) };
	// 83297334: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329733C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297348 size=52
    let mut pc: u32 = 0x83297348;
    'dispatch: loop {
        match pc {
            0x83297348 => {
    //   block [0x83297348..0x8329737C)
	// 83297348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297354: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83297358: 386BD088  addi r3, r11, -0x2f78
	ctx.r[3].s64 = ctx.r[11].s64 + -12152;
	// 8329735C: 4AEF1DE5  bl 0x82189140
	ctx.lr = 0x83297360;
	sub_82189140(ctx, base);
	// 83297360: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83297364: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83297368: 916AE944  stw r11, -0x16bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5820 as u32), ctx.r[11].u32 ) };
	// 8329736C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83297380 size=12
    let mut pc: u32 = 0x83297380;
    'dispatch: loop {
        match pc {
            0x83297380 => {
    //   block [0x83297380..0x8329738C)
	// 83297380: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83297384: 386B7180  addi r3, r11, 0x7180
	ctx.r[3].s64 = ctx.r[11].s64 + 29056;
	// 83297388: 4BA12B98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83297390 size=12
    let mut pc: u32 = 0x83297390;
    'dispatch: loop {
        match pc {
            0x83297390 => {
    //   block [0x83297390..0x8329739C)
	// 83297390: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83297394: 386B71A0  addi r3, r11, 0x71a0
	ctx.r[3].s64 = ctx.r[11].s64 + 29088;
	// 83297398: 4BA12B88  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832973A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832973A0 size=12
    let mut pc: u32 = 0x832973A0;
    'dispatch: loop {
        match pc {
            0x832973A0 => {
    //   block [0x832973A0..0x832973AC)
	// 832973A0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832973A4: 386B7190  addi r3, r11, 0x7190
	ctx.r[3].s64 = ctx.r[11].s64 + 29072;
	// 832973A8: 4BA12B78  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832973B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832973B0 size=12
    let mut pc: u32 = 0x832973B0;
    'dispatch: loop {
        match pc {
            0x832973B0 => {
    //   block [0x832973B0..0x832973BC)
	// 832973B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832973B4: 386B7208  addi r3, r11, 0x7208
	ctx.r[3].s64 = ctx.r[11].s64 + 29192;
	// 832973B8: 4BA12B68  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832973C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832973C0 size=12
    let mut pc: u32 = 0x832973C0;
    'dispatch: loop {
        match pc {
            0x832973C0 => {
    //   block [0x832973C0..0x832973CC)
	// 832973C0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832973C4: 386B71B0  addi r3, r11, 0x71b0
	ctx.r[3].s64 = ctx.r[11].s64 + 29104;
	// 832973C8: 4BA12B58  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832973D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832973D0 size=12
    let mut pc: u32 = 0x832973D0;
    'dispatch: loop {
        match pc {
            0x832973D0 => {
    //   block [0x832973D0..0x832973DC)
	// 832973D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832973D4: 386B7218  addi r3, r11, 0x7218
	ctx.r[3].s64 = ctx.r[11].s64 + 29208;
	// 832973D8: 4BA12B48  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832973E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832973E0 size=12
    let mut pc: u32 = 0x832973E0;
    'dispatch: loop {
        match pc {
            0x832973E0 => {
    //   block [0x832973E0..0x832973EC)
	// 832973E0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832973E4: 386B7228  addi r3, r11, 0x7228
	ctx.r[3].s64 = ctx.r[11].s64 + 29224;
	// 832973E8: 4BA12B38  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832973F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832973F0 size=52
    let mut pc: u32 = 0x832973F0;
    'dispatch: loop {
        match pc {
            0x832973F0 => {
    //   block [0x832973F0..0x83297424)
	// 832973F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832973F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832973F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832973FC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297400: 386B4B94  addi r3, r11, 0x4b94
	ctx.r[3].s64 = ctx.r[11].s64 + 19348;
	// 83297404: 4B96AF65  bl 0x82c02368
	ctx.lr = 0x83297408;
	sub_82C02368(ctx, base);
	// 83297408: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329740C: 386A7280  addi r3, r10, 0x7280
	ctx.r[3].s64 = ctx.r[10].s64 + 29312;
	// 83297410: 4BA12B11  bl 0x82ca9f20
	ctx.lr = 0x83297414;
	sub_82CA9F20(ctx, base);
	// 83297414: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329741C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83297428 size=12
    let mut pc: u32 = 0x83297428;
    'dispatch: loop {
        match pc {
            0x83297428 => {
    //   block [0x83297428..0x83297434)
	// 83297428: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329742C: 386B7320  addi r3, r11, 0x7320
	ctx.r[3].s64 = ctx.r[11].s64 + 29472;
	// 83297430: 4BA12AF0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297438 size=52
    let mut pc: u32 = 0x83297438;
    'dispatch: loop {
        match pc {
            0x83297438 => {
    //   block [0x83297438..0x8329746C)
	// 83297438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329743C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297444: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297448: 386B4BAC  addi r3, r11, 0x4bac
	ctx.r[3].s64 = ctx.r[11].s64 + 19372;
	// 8329744C: 4AF603B5  bl 0x821f7800
	ctx.lr = 0x83297450;
	sub_821F7800(ctx, base);
	// 83297450: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297454: 386A72F0  addi r3, r10, 0x72f0
	ctx.r[3].s64 = ctx.r[10].s64 + 29424;
	// 83297458: 4BA12AC9  bl 0x82ca9f20
	ctx.lr = 0x8329745C;
	sub_82CA9F20(ctx, base);
	// 8329745C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297470 size=52
    let mut pc: u32 = 0x83297470;
    'dispatch: loop {
        match pc {
            0x83297470 => {
    //   block [0x83297470..0x832974A4)
	// 83297470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329747C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297480: 386B4BB0  addi r3, r11, 0x4bb0
	ctx.r[3].s64 = ctx.r[11].s64 + 19376;
	// 83297484: 4B08AB25  bl 0x82321fa8
	ctx.lr = 0x83297488;
	sub_82321FA8(ctx, base);
	// 83297488: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329748C: 386A7348  addi r3, r10, 0x7348
	ctx.r[3].s64 = ctx.r[10].s64 + 29512;
	// 83297490: 4BA12A91  bl 0x82ca9f20
	ctx.lr = 0x83297494;
	sub_82CA9F20(ctx, base);
	// 83297494: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329749C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832974A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832974A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832974A8 size=52
    let mut pc: u32 = 0x832974A8;
    'dispatch: loop {
        match pc {
            0x832974A8 => {
    //   block [0x832974A8..0x832974DC)
	// 832974A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832974AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832974B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832974B4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832974B8: 386B4BBC  addi r3, r11, 0x4bbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19388;
	// 832974BC: 4AF60345  bl 0x821f7800
	ctx.lr = 0x832974C0;
	sub_821F7800(ctx, base);
	// 832974C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832974C4: 386A7300  addi r3, r10, 0x7300
	ctx.r[3].s64 = ctx.r[10].s64 + 29440;
	// 832974C8: 4BA12A59  bl 0x82ca9f20
	ctx.lr = 0x832974CC;
	sub_82CA9F20(ctx, base);
	// 832974CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832974D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832974D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832974D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832974E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832974E0 size=52
    let mut pc: u32 = 0x832974E0;
    'dispatch: loop {
        match pc {
            0x832974E0 => {
    //   block [0x832974E0..0x83297514)
	// 832974E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832974E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832974E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832974EC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832974F0: 386B4BC0  addi r3, r11, 0x4bc0
	ctx.r[3].s64 = ctx.r[11].s64 + 19392;
	// 832974F4: 4AF6030D  bl 0x821f7800
	ctx.lr = 0x832974F8;
	sub_821F7800(ctx, base);
	// 832974F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832974FC: 386A7310  addi r3, r10, 0x7310
	ctx.r[3].s64 = ctx.r[10].s64 + 29456;
	// 83297500: 4BA12A21  bl 0x82ca9f20
	ctx.lr = 0x83297504;
	sub_82CA9F20(ctx, base);
	// 83297504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329750C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297518 size=52
    let mut pc: u32 = 0x83297518;
    'dispatch: loop {
        match pc {
            0x83297518 => {
    //   block [0x83297518..0x8329754C)
	// 83297518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329751C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297524: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297528: 386B4BD8  addi r3, r11, 0x4bd8
	ctx.r[3].s64 = ctx.r[11].s64 + 19416;
	// 8329752C: 4B972685  bl 0x82c09bb0
	ctx.lr = 0x83297530;
	sub_82C09BB0(ctx, base);
	// 83297530: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297534: 386A73B8  addi r3, r10, 0x73b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29624;
	// 83297538: 4BA129E9  bl 0x82ca9f20
	ctx.lr = 0x8329753C;
	sub_82CA9F20(ctx, base);
	// 8329753C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83297550 size=84
    let mut pc: u32 = 0x83297550;
    'dispatch: loop {
        match pc {
            0x83297550 => {
    //   block [0x83297550..0x83297584)
	// 83297550: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297554: 3D602323  lis r11, 0x2323
	ctx.r[11].s64 = 589496320;
	// 83297558: 390A4C38  addi r8, r10, 0x4c38
	ctx.r[8].s64 = ctx.r[10].s64 + 19512;
	// 8329755C: 616B2323  ori r11, r11, 0x2323
	ctx.r[11].u64 = ctx.r[11].u64 | 8995;
	// 83297560: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 83297564: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83297568: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8329756C: 394AA8DC  addi r10, r10, -0x5724
	ctx.r[10].s64 = ctx.r[10].s64 + -22308;
	// 83297570: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 83297574: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83297578: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8329757C: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83297580: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	pc = 0x83297584; continue 'dispatch;
            }
            0x83297584 => {
    //   block [0x83297584..0x832975A4)
	// 83297584: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83297588: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8329758C: 99690000  stb r11, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83297590: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83297594: 4200FFF0  bdnz 0x83297584
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83297584; continue 'dispatch;
	}
	// 83297598: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8329759C: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832975A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832975A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832975A8 size=96
    let mut pc: u32 = 0x832975A8;
    'dispatch: loop {
        match pc {
            0x832975A8 => {
    //   block [0x832975A8..0x83297608)
	// 832975A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832975AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832975B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832975B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832975B8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832975BC: 3D602323  lis r11, 0x2323
	ctx.r[11].s64 = 589496320;
	// 832975C0: 3BEA4C48  addi r31, r10, 0x4c48
	ctx.r[31].s64 = ctx.r[10].s64 + 19528;
	// 832975C4: 616B2323  ori r11, r11, 0x2323
	ctx.r[11].u64 = ctx.r[11].u64 | 8995;
	// 832975C8: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 832975CC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 832975D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832975D4: 3888A8E8  addi r4, r8, -0x5718
	ctx.r[4].s64 = ctx.r[8].s64 + -22296;
	// 832975D8: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 832975DC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832975E0: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832975E4: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832975E8: 4BA11E99  bl 0x82ca9480
	ctx.lr = 0x832975EC;
	sub_82CA9480(ctx, base);
	// 832975EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832975F0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832975F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832975F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832975FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83297604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297608 size=52
    let mut pc: u32 = 0x83297608;
    'dispatch: loop {
        match pc {
            0x83297608 => {
    //   block [0x83297608..0x8329763C)
	// 83297608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329760C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297614: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297618: 386B4C60  addi r3, r11, 0x4c60
	ctx.r[3].s64 = ctx.r[11].s64 + 19552;
	// 8329761C: 4B972595  bl 0x82c09bb0
	ctx.lr = 0x83297620;
	sub_82C09BB0(ctx, base);
	// 83297620: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297624: 386A7408  addi r3, r10, 0x7408
	ctx.r[3].s64 = ctx.r[10].s64 + 29704;
	// 83297628: 4BA128F9  bl 0x82ca9f20
	ctx.lr = 0x8329762C;
	sub_82CA9F20(ctx, base);
	// 8329762C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297640 size=64
    let mut pc: u32 = 0x83297640;
    'dispatch: loop {
        match pc {
            0x83297640 => {
    //   block [0x83297640..0x83297680)
	// 83297640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329764C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83297650: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297654: 388B4A24  addi r4, r11, 0x4a24
	ctx.r[4].s64 = ctx.r[11].s64 + 18980;
	// 83297658: 386A4CB8  addi r3, r10, 0x4cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 19640;
	// 8329765C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297660: 4AF95871  bl 0x8222ced0
	ctx.lr = 0x83297664;
	sub_8222CED0(ctx, base);
	// 83297664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297668: 38697418  addi r3, r9, 0x7418
	ctx.r[3].s64 = ctx.r[9].s64 + 29720;
	// 8329766C: 4BA128B5  bl 0x82ca9f20
	ctx.lr = 0x83297670;
	sub_82CA9F20(ctx, base);
	// 83297670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329767C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297680 size=52
    let mut pc: u32 = 0x83297680;
    'dispatch: loop {
        match pc {
            0x83297680 => {
    //   block [0x83297680..0x832976B4)
	// 83297680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329768C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297690: 386B4CBC  addi r3, r11, 0x4cbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19644;
	// 83297694: 4B08A915  bl 0x82321fa8
	ctx.lr = 0x83297698;
	sub_82321FA8(ctx, base);
	// 83297698: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329769C: 386A7428  addi r3, r10, 0x7428
	ctx.r[3].s64 = ctx.r[10].s64 + 29736;
	// 832976A0: 4BA12881  bl 0x82ca9f20
	ctx.lr = 0x832976A4;
	sub_82CA9F20(ctx, base);
	// 832976A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832976A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832976AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832976B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832976B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832976B8 size=52
    let mut pc: u32 = 0x832976B8;
    'dispatch: loop {
        match pc {
            0x832976B8 => {
    //   block [0x832976B8..0x832976EC)
	// 832976B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832976BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832976C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832976C4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832976C8: 386B4CC8  addi r3, r11, 0x4cc8
	ctx.r[3].s64 = ctx.r[11].s64 + 19656;
	// 832976CC: 4B780855  bl 0x82a17f20
	ctx.lr = 0x832976D0;
	sub_82A17F20(ctx, base);
	// 832976D0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832976D4: 386A7498  addi r3, r10, 0x7498
	ctx.r[3].s64 = ctx.r[10].s64 + 29848;
	// 832976D8: 4BA12849  bl 0x82ca9f20
	ctx.lr = 0x832976DC;
	sub_82CA9F20(ctx, base);
	// 832976DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832976E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832976E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832976E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832976F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832976F0 size=52
    let mut pc: u32 = 0x832976F0;
    'dispatch: loop {
        match pc {
            0x832976F0 => {
    //   block [0x832976F0..0x83297724)
	// 832976F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832976F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832976F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832976FC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297700: 386B4CD4  addi r3, r11, 0x4cd4
	ctx.r[3].s64 = ctx.r[11].s64 + 19668;
	// 83297704: 4B9737D5  bl 0x82c0aed8
	ctx.lr = 0x83297708;
	sub_82C0AED8(ctx, base);
	// 83297708: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329770C: 386A7508  addi r3, r10, 0x7508
	ctx.r[3].s64 = ctx.r[10].s64 + 29960;
	// 83297710: 4BA12811  bl 0x82ca9f20
	ctx.lr = 0x83297714;
	sub_82CA9F20(ctx, base);
	// 83297714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329771C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297728 size=52
    let mut pc: u32 = 0x83297728;
    'dispatch: loop {
        match pc {
            0x83297728 => {
    //   block [0x83297728..0x8329775C)
	// 83297728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297734: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297738: 386B4CE0  addi r3, r11, 0x4ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 19680;
	// 8329773C: 4B216F25  bl 0x824ae660
	ctx.lr = 0x83297740;
	sub_824AE660(ctx, base);
	// 83297740: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297744: 386A7578  addi r3, r10, 0x7578
	ctx.r[3].s64 = ctx.r[10].s64 + 30072;
	// 83297748: 4BA127D9  bl 0x82ca9f20
	ctx.lr = 0x8329774C;
	sub_82CA9F20(ctx, base);
	// 8329774C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297760 size=104
    let mut pc: u32 = 0x83297760;
    'dispatch: loop {
        match pc {
            0x83297760 => {
    //   block [0x83297760..0x83297780)
	// 83297760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297764: 4BA11CA9  bl 0x82ca940c
	ctx.lr = 0x83297768;
	sub_82CA93D0(ctx, base);
	// 83297768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329776C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297770: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 83297774: 396B4CF4  addi r11, r11, 0x4cf4
	ctx.r[11].s64 = ctx.r[11].s64 + 19700;
	// 83297778: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8329777C: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	pc = 0x83297780; continue 'dispatch;
            }
            0x83297780 => {
    //   block [0x83297780..0x83297794)
	// 83297780: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83297784: 4AF87AD5  bl 0x8221f258
	ctx.lr = 0x83297788;
	sub_8221F258(ctx, base);
	// 83297788: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329778C: 419A0008  beq cr6, 0x83297794
	if ctx.cr[6].eq {
	pc = 0x83297794; continue 'dispatch;
	}
	// 83297790: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83297794; continue 'dispatch;
            }
            0x83297794 => {
    //   block [0x83297794..0x832977A0)
	// 83297794: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83297798: 41820008  beq 0x832977a0
	if ctx.cr[0].eq {
	pc = 0x832977A0; continue 'dispatch;
	}
	// 8329779C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x832977A0; continue 'dispatch;
            }
            0x832977A0 => {
    //   block [0x832977A0..0x832977C8)
	// 832977A0: 907FFFFC  stw r3, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[3].u32 ) };
	// 832977A4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832977A8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832977AC: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 832977B0: 4080FFD0  bge 0x83297780
	if !ctx.cr[0].lt {
	pc = 0x83297780; continue 'dispatch;
	}
	// 832977B4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832977B8: 386B7688  addi r3, r11, 0x7688
	ctx.r[3].s64 = ctx.r[11].s64 + 30344;
	// 832977BC: 4BA12765  bl 0x82ca9f20
	ctx.lr = 0x832977C0;
	sub_82CA9F20(ctx, base);
	// 832977C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832977C4: 4BA11C98  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832977C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832977C8 size=88
    let mut pc: u32 = 0x832977C8;
    'dispatch: loop {
        match pc {
            0x832977C8 => {
    //   block [0x832977C8..0x832977E8)
	// 832977C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832977CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832977D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832977D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832977D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832977DC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832977E0: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 832977E4: 3BCB4D30  addi r30, r11, 0x4d30
	ctx.r[30].s64 = ctx.r[11].s64 + 19760;
	pc = 0x832977E8; continue 'dispatch;
            }
            0x832977E8 => {
    //   block [0x832977E8..0x83297820)
	// 832977E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832977EC: 4B8A60C5  bl 0x82b3d8b0
	ctx.lr = 0x832977F0;
	sub_82B3D8B0(ctx, base);
	// 832977F0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832977F4: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 832977F8: 4080FFF0  bge 0x832977e8
	if !ctx.cr[0].lt {
	pc = 0x832977E8; continue 'dispatch;
	}
	// 832977FC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83297800: 386B75E8  addi r3, r11, 0x75e8
	ctx.r[3].s64 = ctx.r[11].s64 + 30184;
	// 83297804: 4BA1271D  bl 0x82ca9f20
	ctx.lr = 0x83297808;
	sub_82CA9F20(ctx, base);
	// 83297808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8329780C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83297818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8329781C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297820 size=108
    let mut pc: u32 = 0x83297820;
    'dispatch: loop {
        match pc {
            0x83297820 => {
    //   block [0x83297820..0x8329784C)
	// 83297820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297824: 4BA11BE1  bl 0x82ca9404
	ctx.lr = 0x83297828;
	sub_82CA93D0(ctx, base);
	// 83297828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329782C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297830: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 83297834: 396B4DBC  addi r11, r11, 0x4dbc
	ctx.r[11].s64 = ctx.r[11].s64 + 19900;
	// 83297838: 3B600005  li r27, 5
	ctx.r[27].s64 = 5;
	// 8329783C: 3BEB0018  addi r31, r11, 0x18
	ctx.r[31].s64 = ctx.r[11].s64 + 24;
	// 83297840: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297844: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83297848: 3B8BA954  addi r28, r11, -0x56ac
	ctx.r[28].s64 = ctx.r[11].s64 + -22188;
	pc = 0x8329784C; continue 'dispatch;
            }
            0x8329784C => {
    //   block [0x8329784C..0x8329788C)
	// 8329784C: 387FFFE8  addi r3, r31, -0x18
	ctx.r[3].s64 = ctx.r[31].s64 + -24;
	// 83297850: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 83297854: 4B8A60E5  bl 0x82b3d938
	ctx.lr = 0x83297858;
	sub_82B3D938(ctx, base);
	// 83297858: 939FFFE8  stw r28, -0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-24 as u32), ctx.r[28].u32 ) };
	// 8329785C: 937FFFFC  stw r27, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[27].u32 ) };
	// 83297860: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83297864: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83297868: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8329786C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83297870: 3BFF0024  addi r31, r31, 0x24
	ctx.r[31].s64 = ctx.r[31].s64 + 36;
	// 83297874: 4080FFD8  bge 0x8329784c
	if !ctx.cr[0].lt {
	pc = 0x8329784C; continue 'dispatch;
	}
	// 83297878: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329787C: 386B7638  addi r3, r11, 0x7638
	ctx.r[3].s64 = ctx.r[11].s64 + 30264;
	// 83297880: 4BA126A1  bl 0x82ca9f20
	ctx.lr = 0x83297884;
	sub_82CA9F20(ctx, base);
	// 83297884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83297888: 4BA11BCC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297890 size=80
    let mut pc: u32 = 0x83297890;
    'dispatch: loop {
        match pc {
            0x83297890 => {
    //   block [0x83297890..0x832978E0)
	// 83297890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8329789C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832978A0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832978A4: 3BEB4E74  addi r31, r11, 0x4e74
	ctx.r[31].s64 = ctx.r[11].s64 + 20084;
	// 832978A8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832978AC: 4B08A6FD  bl 0x82321fa8
	ctx.lr = 0x832978B0;
	sub_82321FA8(ctx, base);
	// 832978B0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832978B4: 4B8A5FFD  bl 0x82b3d8b0
	ctx.lr = 0x832978B8;
	sub_82B3D8B0(ctx, base);
	// 832978B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832978BC: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832978C0: 997F0038  stb r11, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 832978C4: 386A7708  addi r3, r10, 0x7708
	ctx.r[3].s64 = ctx.r[10].s64 + 30472;
	// 832978C8: 4BA12659  bl 0x82ca9f20
	ctx.lr = 0x832978CC;
	sub_82CA9F20(ctx, base);
	// 832978CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832978D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832978D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832978D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832978DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832978E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832978E0 size=80
    let mut pc: u32 = 0x832978E0;
    'dispatch: loop {
        match pc {
            0x832978E0 => {
    //   block [0x832978E0..0x83297930)
	// 832978E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832978E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832978E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832978EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832978F0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832978F4: 3BEB4EB0  addi r31, r11, 0x4eb0
	ctx.r[31].s64 = ctx.r[11].s64 + 20144;
	// 832978F8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832978FC: 4B08A6AD  bl 0x82321fa8
	ctx.lr = 0x83297900;
	sub_82321FA8(ctx, base);
	// 83297900: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83297904: 4B8A5FAD  bl 0x82b3d8b0
	ctx.lr = 0x83297908;
	sub_82B3D8B0(ctx, base);
	// 83297908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329790C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297910: 997F0038  stb r11, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 83297914: 386A7718  addi r3, r10, 0x7718
	ctx.r[3].s64 = ctx.r[10].s64 + 30488;
	// 83297918: 4BA12609  bl 0x82ca9f20
	ctx.lr = 0x8329791C;
	sub_82CA9F20(ctx, base);
	// 8329791C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8329792C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297930 size=64
    let mut pc: u32 = 0x83297930;
    'dispatch: loop {
        match pc {
            0x83297930 => {
    //   block [0x83297930..0x83297970)
	// 83297930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329793C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297940: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297944: 388BB3D8  addi r4, r11, -0x4c28
	ctx.r[4].s64 = ctx.r[11].s64 + -19496;
	// 83297948: 386A4EF8  addi r3, r10, 0x4ef8
	ctx.r[3].s64 = ctx.r[10].s64 + 20216;
	// 8329794C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297950: 4AF95581  bl 0x8222ced0
	ctx.lr = 0x83297954;
	sub_8222CED0(ctx, base);
	// 83297954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297958: 38697728  addi r3, r9, 0x7728
	ctx.r[3].s64 = ctx.r[9].s64 + 30504;
	// 8329795C: 4BA125C5  bl 0x82ca9f20
	ctx.lr = 0x83297960;
	sub_82CA9F20(ctx, base);
	// 83297960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329796C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297970 size=64
    let mut pc: u32 = 0x83297970;
    'dispatch: loop {
        match pc {
            0x83297970 => {
    //   block [0x83297970..0x832979B0)
	// 83297970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329797C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297980: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297984: 388BB3E8  addi r4, r11, -0x4c18
	ctx.r[4].s64 = ctx.r[11].s64 + -19480;
	// 83297988: 386A4EFC  addi r3, r10, 0x4efc
	ctx.r[3].s64 = ctx.r[10].s64 + 20220;
	// 8329798C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297990: 4AF95541  bl 0x8222ced0
	ctx.lr = 0x83297994;
	sub_8222CED0(ctx, base);
	// 83297994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297998: 38697738  addi r3, r9, 0x7738
	ctx.r[3].s64 = ctx.r[9].s64 + 30520;
	// 8329799C: 4BA12585  bl 0x82ca9f20
	ctx.lr = 0x832979A0;
	sub_82CA9F20(ctx, base);
	// 832979A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832979A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832979A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832979AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832979B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832979B0 size=64
    let mut pc: u32 = 0x832979B0;
    'dispatch: loop {
        match pc {
            0x832979B0 => {
    //   block [0x832979B0..0x832979F0)
	// 832979B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832979B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832979B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832979BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832979C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832979C4: 388BB534  addi r4, r11, -0x4acc
	ctx.r[4].s64 = ctx.r[11].s64 + -19148;
	// 832979C8: 386A4F00  addi r3, r10, 0x4f00
	ctx.r[3].s64 = ctx.r[10].s64 + 20224;
	// 832979CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832979D0: 4AF95501  bl 0x8222ced0
	ctx.lr = 0x832979D4;
	sub_8222CED0(ctx, base);
	// 832979D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832979D8: 38697748  addi r3, r9, 0x7748
	ctx.r[3].s64 = ctx.r[9].s64 + 30536;
	// 832979DC: 4BA12545  bl 0x82ca9f20
	ctx.lr = 0x832979E0;
	sub_82CA9F20(ctx, base);
	// 832979E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832979E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832979E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832979EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832979F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832979F0 size=64
    let mut pc: u32 = 0x832979F0;
    'dispatch: loop {
        match pc {
            0x832979F0 => {
    //   block [0x832979F0..0x83297A30)
	// 832979F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832979F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832979F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832979FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297A00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297A04: 388BB540  addi r4, r11, -0x4ac0
	ctx.r[4].s64 = ctx.r[11].s64 + -19136;
	// 83297A08: 386A4F04  addi r3, r10, 0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + 20228;
	// 83297A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297A10: 4AF954C1  bl 0x8222ced0
	ctx.lr = 0x83297A14;
	sub_8222CED0(ctx, base);
	// 83297A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297A18: 38697758  addi r3, r9, 0x7758
	ctx.r[3].s64 = ctx.r[9].s64 + 30552;
	// 83297A1C: 4BA12505  bl 0x82ca9f20
	ctx.lr = 0x83297A20;
	sub_82CA9F20(ctx, base);
	// 83297A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297A30 size=64
    let mut pc: u32 = 0x83297A30;
    'dispatch: loop {
        match pc {
            0x83297A30 => {
    //   block [0x83297A30..0x83297A70)
	// 83297A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297A3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297A40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297A44: 388BB780  addi r4, r11, -0x4880
	ctx.r[4].s64 = ctx.r[11].s64 + -18560;
	// 83297A48: 386A4F08  addi r3, r10, 0x4f08
	ctx.r[3].s64 = ctx.r[10].s64 + 20232;
	// 83297A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297A50: 4AF95481  bl 0x8222ced0
	ctx.lr = 0x83297A54;
	sub_8222CED0(ctx, base);
	// 83297A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297A58: 38697768  addi r3, r9, 0x7768
	ctx.r[3].s64 = ctx.r[9].s64 + 30568;
	// 83297A5C: 4BA124C5  bl 0x82ca9f20
	ctx.lr = 0x83297A60;
	sub_82CA9F20(ctx, base);
	// 83297A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297A70 size=64
    let mut pc: u32 = 0x83297A70;
    'dispatch: loop {
        match pc {
            0x83297A70 => {
    //   block [0x83297A70..0x83297AB0)
	// 83297A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297A7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297A80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297A84: 388BB790  addi r4, r11, -0x4870
	ctx.r[4].s64 = ctx.r[11].s64 + -18544;
	// 83297A88: 386A4F0C  addi r3, r10, 0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 20236;
	// 83297A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297A90: 4AF95441  bl 0x8222ced0
	ctx.lr = 0x83297A94;
	sub_8222CED0(ctx, base);
	// 83297A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297A98: 38697778  addi r3, r9, 0x7778
	ctx.r[3].s64 = ctx.r[9].s64 + 30584;
	// 83297A9C: 4BA12485  bl 0x82ca9f20
	ctx.lr = 0x83297AA0;
	sub_82CA9F20(ctx, base);
	// 83297AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297AB0 size=64
    let mut pc: u32 = 0x83297AB0;
    'dispatch: loop {
        match pc {
            0x83297AB0 => {
    //   block [0x83297AB0..0x83297AF0)
	// 83297AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297ABC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297AC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297AC4: 388BB79C  addi r4, r11, -0x4864
	ctx.r[4].s64 = ctx.r[11].s64 + -18532;
	// 83297AC8: 386A4F10  addi r3, r10, 0x4f10
	ctx.r[3].s64 = ctx.r[10].s64 + 20240;
	// 83297ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297AD0: 4AF95401  bl 0x8222ced0
	ctx.lr = 0x83297AD4;
	sub_8222CED0(ctx, base);
	// 83297AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297AD8: 38697788  addi r3, r9, 0x7788
	ctx.r[3].s64 = ctx.r[9].s64 + 30600;
	// 83297ADC: 4BA12445  bl 0x82ca9f20
	ctx.lr = 0x83297AE0;
	sub_82CA9F20(ctx, base);
	// 83297AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297AF0 size=64
    let mut pc: u32 = 0x83297AF0;
    'dispatch: loop {
        match pc {
            0x83297AF0 => {
    //   block [0x83297AF0..0x83297B30)
	// 83297AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297AFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297B00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297B04: 388BB884  addi r4, r11, -0x477c
	ctx.r[4].s64 = ctx.r[11].s64 + -18300;
	// 83297B08: 386A4F14  addi r3, r10, 0x4f14
	ctx.r[3].s64 = ctx.r[10].s64 + 20244;
	// 83297B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297B10: 4AF953C1  bl 0x8222ced0
	ctx.lr = 0x83297B14;
	sub_8222CED0(ctx, base);
	// 83297B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297B18: 38697798  addi r3, r9, 0x7798
	ctx.r[3].s64 = ctx.r[9].s64 + 30616;
	// 83297B1C: 4BA12405  bl 0x82ca9f20
	ctx.lr = 0x83297B20;
	sub_82CA9F20(ctx, base);
	// 83297B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297B30 size=64
    let mut pc: u32 = 0x83297B30;
    'dispatch: loop {
        match pc {
            0x83297B30 => {
    //   block [0x83297B30..0x83297B70)
	// 83297B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297B3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297B40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297B44: 388BB89C  addi r4, r11, -0x4764
	ctx.r[4].s64 = ctx.r[11].s64 + -18276;
	// 83297B48: 386A4F18  addi r3, r10, 0x4f18
	ctx.r[3].s64 = ctx.r[10].s64 + 20248;
	// 83297B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297B50: 4AF95381  bl 0x8222ced0
	ctx.lr = 0x83297B54;
	sub_8222CED0(ctx, base);
	// 83297B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297B58: 386977A8  addi r3, r9, 0x77a8
	ctx.r[3].s64 = ctx.r[9].s64 + 30632;
	// 83297B5C: 4BA123C5  bl 0x82ca9f20
	ctx.lr = 0x83297B60;
	sub_82CA9F20(ctx, base);
	// 83297B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297B70 size=64
    let mut pc: u32 = 0x83297B70;
    'dispatch: loop {
        match pc {
            0x83297B70 => {
    //   block [0x83297B70..0x83297BB0)
	// 83297B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297B7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297B80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297B84: 388BB8AC  addi r4, r11, -0x4754
	ctx.r[4].s64 = ctx.r[11].s64 + -18260;
	// 83297B88: 386A4F1C  addi r3, r10, 0x4f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 20252;
	// 83297B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297B90: 4AF95341  bl 0x8222ced0
	ctx.lr = 0x83297B94;
	sub_8222CED0(ctx, base);
	// 83297B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297B98: 386977B8  addi r3, r9, 0x77b8
	ctx.r[3].s64 = ctx.r[9].s64 + 30648;
	// 83297B9C: 4BA12385  bl 0x82ca9f20
	ctx.lr = 0x83297BA0;
	sub_82CA9F20(ctx, base);
	// 83297BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297BB0 size=64
    let mut pc: u32 = 0x83297BB0;
    'dispatch: loop {
        match pc {
            0x83297BB0 => {
    //   block [0x83297BB0..0x83297BF0)
	// 83297BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297BBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297BC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297BC4: 388BB9F8  addi r4, r11, -0x4608
	ctx.r[4].s64 = ctx.r[11].s64 + -17928;
	// 83297BC8: 386A4F20  addi r3, r10, 0x4f20
	ctx.r[3].s64 = ctx.r[10].s64 + 20256;
	// 83297BCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297BD0: 4AF95301  bl 0x8222ced0
	ctx.lr = 0x83297BD4;
	sub_8222CED0(ctx, base);
	// 83297BD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297BD8: 386977C8  addi r3, r9, 0x77c8
	ctx.r[3].s64 = ctx.r[9].s64 + 30664;
	// 83297BDC: 4BA12345  bl 0x82ca9f20
	ctx.lr = 0x83297BE0;
	sub_82CA9F20(ctx, base);
	// 83297BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297BF0 size=64
    let mut pc: u32 = 0x83297BF0;
    'dispatch: loop {
        match pc {
            0x83297BF0 => {
    //   block [0x83297BF0..0x83297C30)
	// 83297BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297BFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297C00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297C04: 388BBA10  addi r4, r11, -0x45f0
	ctx.r[4].s64 = ctx.r[11].s64 + -17904;
	// 83297C08: 386A4F24  addi r3, r10, 0x4f24
	ctx.r[3].s64 = ctx.r[10].s64 + 20260;
	// 83297C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297C10: 4AF952C1  bl 0x8222ced0
	ctx.lr = 0x83297C14;
	sub_8222CED0(ctx, base);
	// 83297C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297C18: 386977D8  addi r3, r9, 0x77d8
	ctx.r[3].s64 = ctx.r[9].s64 + 30680;
	// 83297C1C: 4BA12305  bl 0x82ca9f20
	ctx.lr = 0x83297C20;
	sub_82CA9F20(ctx, base);
	// 83297C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297C30 size=64
    let mut pc: u32 = 0x83297C30;
    'dispatch: loop {
        match pc {
            0x83297C30 => {
    //   block [0x83297C30..0x83297C70)
	// 83297C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297C3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297C40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297C44: 388BBA20  addi r4, r11, -0x45e0
	ctx.r[4].s64 = ctx.r[11].s64 + -17888;
	// 83297C48: 386A4F28  addi r3, r10, 0x4f28
	ctx.r[3].s64 = ctx.r[10].s64 + 20264;
	// 83297C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297C50: 4AF95281  bl 0x8222ced0
	ctx.lr = 0x83297C54;
	sub_8222CED0(ctx, base);
	// 83297C54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297C58: 386977E8  addi r3, r9, 0x77e8
	ctx.r[3].s64 = ctx.r[9].s64 + 30696;
	// 83297C5C: 4BA122C5  bl 0x82ca9f20
	ctx.lr = 0x83297C60;
	sub_82CA9F20(ctx, base);
	// 83297C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297C70 size=64
    let mut pc: u32 = 0x83297C70;
    'dispatch: loop {
        match pc {
            0x83297C70 => {
    //   block [0x83297C70..0x83297CB0)
	// 83297C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297C7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297C80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297C84: 388BBA6C  addi r4, r11, -0x4594
	ctx.r[4].s64 = ctx.r[11].s64 + -17812;
	// 83297C88: 386A4F2C  addi r3, r10, 0x4f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 20268;
	// 83297C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297C90: 4AF95241  bl 0x8222ced0
	ctx.lr = 0x83297C94;
	sub_8222CED0(ctx, base);
	// 83297C94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297C98: 386977F8  addi r3, r9, 0x77f8
	ctx.r[3].s64 = ctx.r[9].s64 + 30712;
	// 83297C9C: 4BA12285  bl 0x82ca9f20
	ctx.lr = 0x83297CA0;
	sub_82CA9F20(ctx, base);
	// 83297CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297CB0 size=64
    let mut pc: u32 = 0x83297CB0;
    'dispatch: loop {
        match pc {
            0x83297CB0 => {
    //   block [0x83297CB0..0x83297CF0)
	// 83297CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297CBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297CC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297CC4: 388BBA84  addi r4, r11, -0x457c
	ctx.r[4].s64 = ctx.r[11].s64 + -17788;
	// 83297CC8: 386A4F30  addi r3, r10, 0x4f30
	ctx.r[3].s64 = ctx.r[10].s64 + 20272;
	// 83297CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297CD0: 4AF95201  bl 0x8222ced0
	ctx.lr = 0x83297CD4;
	sub_8222CED0(ctx, base);
	// 83297CD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297CD8: 38697808  addi r3, r9, 0x7808
	ctx.r[3].s64 = ctx.r[9].s64 + 30728;
	// 83297CDC: 4BA12245  bl 0x82ca9f20
	ctx.lr = 0x83297CE0;
	sub_82CA9F20(ctx, base);
	// 83297CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297CF0 size=64
    let mut pc: u32 = 0x83297CF0;
    'dispatch: loop {
        match pc {
            0x83297CF0 => {
    //   block [0x83297CF0..0x83297D30)
	// 83297CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297CFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297D00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297D04: 388BBA98  addi r4, r11, -0x4568
	ctx.r[4].s64 = ctx.r[11].s64 + -17768;
	// 83297D08: 386A4F34  addi r3, r10, 0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + 20276;
	// 83297D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297D10: 4AF951C1  bl 0x8222ced0
	ctx.lr = 0x83297D14;
	sub_8222CED0(ctx, base);
	// 83297D14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297D18: 38697818  addi r3, r9, 0x7818
	ctx.r[3].s64 = ctx.r[9].s64 + 30744;
	// 83297D1C: 4BA12205  bl 0x82ca9f20
	ctx.lr = 0x83297D20;
	sub_82CA9F20(ctx, base);
	// 83297D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297D30 size=64
    let mut pc: u32 = 0x83297D30;
    'dispatch: loop {
        match pc {
            0x83297D30 => {
    //   block [0x83297D30..0x83297D70)
	// 83297D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297D3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297D40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297D44: 388BBB60  addi r4, r11, -0x44a0
	ctx.r[4].s64 = ctx.r[11].s64 + -17568;
	// 83297D48: 386A4F38  addi r3, r10, 0x4f38
	ctx.r[3].s64 = ctx.r[10].s64 + 20280;
	// 83297D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297D50: 4AF95181  bl 0x8222ced0
	ctx.lr = 0x83297D54;
	sub_8222CED0(ctx, base);
	// 83297D54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297D58: 38697828  addi r3, r9, 0x7828
	ctx.r[3].s64 = ctx.r[9].s64 + 30760;
	// 83297D5C: 4BA121C5  bl 0x82ca9f20
	ctx.lr = 0x83297D60;
	sub_82CA9F20(ctx, base);
	// 83297D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297D70 size=64
    let mut pc: u32 = 0x83297D70;
    'dispatch: loop {
        match pc {
            0x83297D70 => {
    //   block [0x83297D70..0x83297DB0)
	// 83297D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297D7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297D80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297D84: 388BBB70  addi r4, r11, -0x4490
	ctx.r[4].s64 = ctx.r[11].s64 + -17552;
	// 83297D88: 386A4F3C  addi r3, r10, 0x4f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 20284;
	// 83297D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297D90: 4AF95141  bl 0x8222ced0
	ctx.lr = 0x83297D94;
	sub_8222CED0(ctx, base);
	// 83297D94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297D98: 38697838  addi r3, r9, 0x7838
	ctx.r[3].s64 = ctx.r[9].s64 + 30776;
	// 83297D9C: 4BA12185  bl 0x82ca9f20
	ctx.lr = 0x83297DA0;
	sub_82CA9F20(ctx, base);
	// 83297DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297DB0 size=64
    let mut pc: u32 = 0x83297DB0;
    'dispatch: loop {
        match pc {
            0x83297DB0 => {
    //   block [0x83297DB0..0x83297DF0)
	// 83297DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297DBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297DC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297DC4: 388BBB7C  addi r4, r11, -0x4484
	ctx.r[4].s64 = ctx.r[11].s64 + -17540;
	// 83297DC8: 386A4F40  addi r3, r10, 0x4f40
	ctx.r[3].s64 = ctx.r[10].s64 + 20288;
	// 83297DCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297DD0: 4AF95101  bl 0x8222ced0
	ctx.lr = 0x83297DD4;
	sub_8222CED0(ctx, base);
	// 83297DD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297DD8: 38697848  addi r3, r9, 0x7848
	ctx.r[3].s64 = ctx.r[9].s64 + 30792;
	// 83297DDC: 4BA12145  bl 0x82ca9f20
	ctx.lr = 0x83297DE0;
	sub_82CA9F20(ctx, base);
	// 83297DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297DF0 size=64
    let mut pc: u32 = 0x83297DF0;
    'dispatch: loop {
        match pc {
            0x83297DF0 => {
    //   block [0x83297DF0..0x83297E30)
	// 83297DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297DFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297E00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297E04: 388BBC24  addi r4, r11, -0x43dc
	ctx.r[4].s64 = ctx.r[11].s64 + -17372;
	// 83297E08: 386A4F44  addi r3, r10, 0x4f44
	ctx.r[3].s64 = ctx.r[10].s64 + 20292;
	// 83297E0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297E10: 4AF950C1  bl 0x8222ced0
	ctx.lr = 0x83297E14;
	sub_8222CED0(ctx, base);
	// 83297E14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297E18: 38697858  addi r3, r9, 0x7858
	ctx.r[3].s64 = ctx.r[9].s64 + 30808;
	// 83297E1C: 4BA12105  bl 0x82ca9f20
	ctx.lr = 0x83297E20;
	sub_82CA9F20(ctx, base);
	// 83297E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297E30 size=64
    let mut pc: u32 = 0x83297E30;
    'dispatch: loop {
        match pc {
            0x83297E30 => {
    //   block [0x83297E30..0x83297E70)
	// 83297E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297E3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297E40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297E44: 388BBC38  addi r4, r11, -0x43c8
	ctx.r[4].s64 = ctx.r[11].s64 + -17352;
	// 83297E48: 386A4F48  addi r3, r10, 0x4f48
	ctx.r[3].s64 = ctx.r[10].s64 + 20296;
	// 83297E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297E50: 4AF95081  bl 0x8222ced0
	ctx.lr = 0x83297E54;
	sub_8222CED0(ctx, base);
	// 83297E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297E58: 38697868  addi r3, r9, 0x7868
	ctx.r[3].s64 = ctx.r[9].s64 + 30824;
	// 83297E5C: 4BA120C5  bl 0x82ca9f20
	ctx.lr = 0x83297E60;
	sub_82CA9F20(ctx, base);
	// 83297E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297E70 size=64
    let mut pc: u32 = 0x83297E70;
    'dispatch: loop {
        match pc {
            0x83297E70 => {
    //   block [0x83297E70..0x83297EB0)
	// 83297E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297E7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297E80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297E84: 388BBC48  addi r4, r11, -0x43b8
	ctx.r[4].s64 = ctx.r[11].s64 + -17336;
	// 83297E88: 386A4F4C  addi r3, r10, 0x4f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 20300;
	// 83297E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297E90: 4AF95041  bl 0x8222ced0
	ctx.lr = 0x83297E94;
	sub_8222CED0(ctx, base);
	// 83297E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297E98: 38697878  addi r3, r9, 0x7878
	ctx.r[3].s64 = ctx.r[9].s64 + 30840;
	// 83297E9C: 4BA12085  bl 0x82ca9f20
	ctx.lr = 0x83297EA0;
	sub_82CA9F20(ctx, base);
	// 83297EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297EB0 size=64
    let mut pc: u32 = 0x83297EB0;
    'dispatch: loop {
        match pc {
            0x83297EB0 => {
    //   block [0x83297EB0..0x83297EF0)
	// 83297EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297EBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297EC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297EC4: 388BBD4C  addi r4, r11, -0x42b4
	ctx.r[4].s64 = ctx.r[11].s64 + -17076;
	// 83297EC8: 386A4F50  addi r3, r10, 0x4f50
	ctx.r[3].s64 = ctx.r[10].s64 + 20304;
	// 83297ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297ED0: 4AF95001  bl 0x8222ced0
	ctx.lr = 0x83297ED4;
	sub_8222CED0(ctx, base);
	// 83297ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297ED8: 38697888  addi r3, r9, 0x7888
	ctx.r[3].s64 = ctx.r[9].s64 + 30856;
	// 83297EDC: 4BA12045  bl 0x82ca9f20
	ctx.lr = 0x83297EE0;
	sub_82CA9F20(ctx, base);
	// 83297EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297EF0 size=64
    let mut pc: u32 = 0x83297EF0;
    'dispatch: loop {
        match pc {
            0x83297EF0 => {
    //   block [0x83297EF0..0x83297F30)
	// 83297EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297EFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297F00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297F04: 388BBD64  addi r4, r11, -0x429c
	ctx.r[4].s64 = ctx.r[11].s64 + -17052;
	// 83297F08: 386A4F54  addi r3, r10, 0x4f54
	ctx.r[3].s64 = ctx.r[10].s64 + 20308;
	// 83297F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297F10: 4AF94FC1  bl 0x8222ced0
	ctx.lr = 0x83297F14;
	sub_8222CED0(ctx, base);
	// 83297F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297F18: 38697898  addi r3, r9, 0x7898
	ctx.r[3].s64 = ctx.r[9].s64 + 30872;
	// 83297F1C: 4BA12005  bl 0x82ca9f20
	ctx.lr = 0x83297F20;
	sub_82CA9F20(ctx, base);
	// 83297F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297F30 size=64
    let mut pc: u32 = 0x83297F30;
    'dispatch: loop {
        match pc {
            0x83297F30 => {
    //   block [0x83297F30..0x83297F70)
	// 83297F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297F3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297F40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297F44: 388BBD64  addi r4, r11, -0x429c
	ctx.r[4].s64 = ctx.r[11].s64 + -17052;
	// 83297F48: 386A4F58  addi r3, r10, 0x4f58
	ctx.r[3].s64 = ctx.r[10].s64 + 20312;
	// 83297F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297F50: 4AF94F81  bl 0x8222ced0
	ctx.lr = 0x83297F54;
	sub_8222CED0(ctx, base);
	// 83297F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297F58: 386978A8  addi r3, r9, 0x78a8
	ctx.r[3].s64 = ctx.r[9].s64 + 30888;
	// 83297F5C: 4BA11FC5  bl 0x82ca9f20
	ctx.lr = 0x83297F60;
	sub_82CA9F20(ctx, base);
	// 83297F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297F70 size=64
    let mut pc: u32 = 0x83297F70;
    'dispatch: loop {
        match pc {
            0x83297F70 => {
    //   block [0x83297F70..0x83297FB0)
	// 83297F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297F7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297F80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297F84: 388BBEC4  addi r4, r11, -0x413c
	ctx.r[4].s64 = ctx.r[11].s64 + -16700;
	// 83297F88: 386A4F5C  addi r3, r10, 0x4f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 20316;
	// 83297F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297F90: 4AF94F41  bl 0x8222ced0
	ctx.lr = 0x83297F94;
	sub_8222CED0(ctx, base);
	// 83297F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297F98: 386978B8  addi r3, r9, 0x78b8
	ctx.r[3].s64 = ctx.r[9].s64 + 30904;
	// 83297F9C: 4BA11F85  bl 0x82ca9f20
	ctx.lr = 0x83297FA0;
	sub_82CA9F20(ctx, base);
	// 83297FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297FB0 size=64
    let mut pc: u32 = 0x83297FB0;
    'dispatch: loop {
        match pc {
            0x83297FB0 => {
    //   block [0x83297FB0..0x83297FF0)
	// 83297FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297FBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297FC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297FC4: 388BBED4  addi r4, r11, -0x412c
	ctx.r[4].s64 = ctx.r[11].s64 + -16684;
	// 83297FC8: 386A4F60  addi r3, r10, 0x4f60
	ctx.r[3].s64 = ctx.r[10].s64 + 20320;
	// 83297FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297FD0: 4AF94F01  bl 0x8222ced0
	ctx.lr = 0x83297FD4;
	sub_8222CED0(ctx, base);
	// 83297FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297FD8: 386978C8  addi r3, r9, 0x78c8
	ctx.r[3].s64 = ctx.r[9].s64 + 30920;
	// 83297FDC: 4BA11F45  bl 0x82ca9f20
	ctx.lr = 0x83297FE0;
	sub_82CA9F20(ctx, base);
	// 83297FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297FF0 size=64
    let mut pc: u32 = 0x83297FF0;
    'dispatch: loop {
        match pc {
            0x83297FF0 => {
    //   block [0x83297FF0..0x83298030)
	// 83297FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297FFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298000: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298004: 388BC07C  addi r4, r11, -0x3f84
	ctx.r[4].s64 = ctx.r[11].s64 + -16260;
	// 83298008: 386A4F64  addi r3, r10, 0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + 20324;
	// 8329800C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298010: 4AF94EC1  bl 0x8222ced0
	ctx.lr = 0x83298014;
	sub_8222CED0(ctx, base);
	// 83298014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298018: 386978D8  addi r3, r9, 0x78d8
	ctx.r[3].s64 = ctx.r[9].s64 + 30936;
	// 8329801C: 4BA11F05  bl 0x82ca9f20
	ctx.lr = 0x83298020;
	sub_82CA9F20(ctx, base);
	// 83298020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298030 size=64
    let mut pc: u32 = 0x83298030;
    'dispatch: loop {
        match pc {
            0x83298030 => {
    //   block [0x83298030..0x83298070)
	// 83298030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329803C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298040: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298044: 388BC094  addi r4, r11, -0x3f6c
	ctx.r[4].s64 = ctx.r[11].s64 + -16236;
	// 83298048: 386A4F68  addi r3, r10, 0x4f68
	ctx.r[3].s64 = ctx.r[10].s64 + 20328;
	// 8329804C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298050: 4AF94E81  bl 0x8222ced0
	ctx.lr = 0x83298054;
	sub_8222CED0(ctx, base);
	// 83298054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298058: 386978E8  addi r3, r9, 0x78e8
	ctx.r[3].s64 = ctx.r[9].s64 + 30952;
	// 8329805C: 4BA11EC5  bl 0x82ca9f20
	ctx.lr = 0x83298060;
	sub_82CA9F20(ctx, base);
	// 83298060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298070 size=64
    let mut pc: u32 = 0x83298070;
    'dispatch: loop {
        match pc {
            0x83298070 => {
    //   block [0x83298070..0x832980B0)
	// 83298070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329807C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298080: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298084: 388BC0A4  addi r4, r11, -0x3f5c
	ctx.r[4].s64 = ctx.r[11].s64 + -16220;
	// 83298088: 386A4F6C  addi r3, r10, 0x4f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 20332;
	// 8329808C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298090: 4AF94E41  bl 0x8222ced0
	ctx.lr = 0x83298094;
	sub_8222CED0(ctx, base);
	// 83298094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298098: 386978F8  addi r3, r9, 0x78f8
	ctx.r[3].s64 = ctx.r[9].s64 + 30968;
	// 8329809C: 4BA11E85  bl 0x82ca9f20
	ctx.lr = 0x832980A0;
	sub_82CA9F20(ctx, base);
	// 832980A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832980A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832980A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832980AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832980B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832980B0 size=64
    let mut pc: u32 = 0x832980B0;
    'dispatch: loop {
        match pc {
            0x832980B0 => {
    //   block [0x832980B0..0x832980F0)
	// 832980B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832980B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832980B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832980BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832980C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832980C4: 388BC180  addi r4, r11, -0x3e80
	ctx.r[4].s64 = ctx.r[11].s64 + -16000;
	// 832980C8: 386A4F70  addi r3, r10, 0x4f70
	ctx.r[3].s64 = ctx.r[10].s64 + 20336;
	// 832980CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832980D0: 4AF94E01  bl 0x8222ced0
	ctx.lr = 0x832980D4;
	sub_8222CED0(ctx, base);
	// 832980D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832980D8: 38697908  addi r3, r9, 0x7908
	ctx.r[3].s64 = ctx.r[9].s64 + 30984;
	// 832980DC: 4BA11E45  bl 0x82ca9f20
	ctx.lr = 0x832980E0;
	sub_82CA9F20(ctx, base);
	// 832980E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832980E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832980E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832980EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832980F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832980F0 size=64
    let mut pc: u32 = 0x832980F0;
    'dispatch: loop {
        match pc {
            0x832980F0 => {
    //   block [0x832980F0..0x83298130)
	// 832980F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832980F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832980F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832980FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298100: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298104: 388BC190  addi r4, r11, -0x3e70
	ctx.r[4].s64 = ctx.r[11].s64 + -15984;
	// 83298108: 386A4F74  addi r3, r10, 0x4f74
	ctx.r[3].s64 = ctx.r[10].s64 + 20340;
	// 8329810C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298110: 4AF94DC1  bl 0x8222ced0
	ctx.lr = 0x83298114;
	sub_8222CED0(ctx, base);
	// 83298114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298118: 38697918  addi r3, r9, 0x7918
	ctx.r[3].s64 = ctx.r[9].s64 + 31000;
	// 8329811C: 4BA11E05  bl 0x82ca9f20
	ctx.lr = 0x83298120;
	sub_82CA9F20(ctx, base);
	// 83298120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298130 size=64
    let mut pc: u32 = 0x83298130;
    'dispatch: loop {
        match pc {
            0x83298130 => {
    //   block [0x83298130..0x83298170)
	// 83298130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329813C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298140: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298144: 388BC1A4  addi r4, r11, -0x3e5c
	ctx.r[4].s64 = ctx.r[11].s64 + -15964;
	// 83298148: 386A4F78  addi r3, r10, 0x4f78
	ctx.r[3].s64 = ctx.r[10].s64 + 20344;
	// 8329814C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298150: 4AF94D81  bl 0x8222ced0
	ctx.lr = 0x83298154;
	sub_8222CED0(ctx, base);
	// 83298154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298158: 38697928  addi r3, r9, 0x7928
	ctx.r[3].s64 = ctx.r[9].s64 + 31016;
	// 8329815C: 4BA11DC5  bl 0x82ca9f20
	ctx.lr = 0x83298160;
	sub_82CA9F20(ctx, base);
	// 83298160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329816C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298170 size=64
    let mut pc: u32 = 0x83298170;
    'dispatch: loop {
        match pc {
            0x83298170 => {
    //   block [0x83298170..0x832981B0)
	// 83298170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329817C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298180: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298184: 388BC24C  addi r4, r11, -0x3db4
	ctx.r[4].s64 = ctx.r[11].s64 + -15796;
	// 83298188: 386A4F7C  addi r3, r10, 0x4f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 20348;
	// 8329818C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298190: 4AF94D41  bl 0x8222ced0
	ctx.lr = 0x83298194;
	sub_8222CED0(ctx, base);
	// 83298194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298198: 38697938  addi r3, r9, 0x7938
	ctx.r[3].s64 = ctx.r[9].s64 + 31032;
	// 8329819C: 4BA11D85  bl 0x82ca9f20
	ctx.lr = 0x832981A0;
	sub_82CA9F20(ctx, base);
	// 832981A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832981A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832981A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832981AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832981B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832981B0 size=64
    let mut pc: u32 = 0x832981B0;
    'dispatch: loop {
        match pc {
            0x832981B0 => {
    //   block [0x832981B0..0x832981F0)
	// 832981B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832981B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832981B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832981BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832981C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832981C4: 388BC258  addi r4, r11, -0x3da8
	ctx.r[4].s64 = ctx.r[11].s64 + -15784;
	// 832981C8: 386A4F80  addi r3, r10, 0x4f80
	ctx.r[3].s64 = ctx.r[10].s64 + 20352;
	// 832981CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832981D0: 4AF94D01  bl 0x8222ced0
	ctx.lr = 0x832981D4;
	sub_8222CED0(ctx, base);
	// 832981D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832981D8: 38697948  addi r3, r9, 0x7948
	ctx.r[3].s64 = ctx.r[9].s64 + 31048;
	// 832981DC: 4BA11D45  bl 0x82ca9f20
	ctx.lr = 0x832981E0;
	sub_82CA9F20(ctx, base);
	// 832981E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832981E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832981E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832981EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832981F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832981F0 size=64
    let mut pc: u32 = 0x832981F0;
    'dispatch: loop {
        match pc {
            0x832981F0 => {
    //   block [0x832981F0..0x83298230)
	// 832981F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832981F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832981F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832981FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298200: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298204: 388BC26C  addi r4, r11, -0x3d94
	ctx.r[4].s64 = ctx.r[11].s64 + -15764;
	// 83298208: 386A4F84  addi r3, r10, 0x4f84
	ctx.r[3].s64 = ctx.r[10].s64 + 20356;
	// 8329820C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298210: 4AF94CC1  bl 0x8222ced0
	ctx.lr = 0x83298214;
	sub_8222CED0(ctx, base);
	// 83298214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298218: 38697958  addi r3, r9, 0x7958
	ctx.r[3].s64 = ctx.r[9].s64 + 31064;
	// 8329821C: 4BA11D05  bl 0x82ca9f20
	ctx.lr = 0x83298220;
	sub_82CA9F20(ctx, base);
	// 83298220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329822C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298230 size=64
    let mut pc: u32 = 0x83298230;
    'dispatch: loop {
        match pc {
            0x83298230 => {
    //   block [0x83298230..0x83298270)
	// 83298230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329823C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298240: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298244: 388BC324  addi r4, r11, -0x3cdc
	ctx.r[4].s64 = ctx.r[11].s64 + -15580;
	// 83298248: 386A4F94  addi r3, r10, 0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + 20372;
	// 8329824C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298250: 4AF94C81  bl 0x8222ced0
	ctx.lr = 0x83298254;
	sub_8222CED0(ctx, base);
	// 83298254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298258: 38697968  addi r3, r9, 0x7968
	ctx.r[3].s64 = ctx.r[9].s64 + 31080;
	// 8329825C: 4BA11CC5  bl 0x82ca9f20
	ctx.lr = 0x83298260;
	sub_82CA9F20(ctx, base);
	// 83298260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329826C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298270 size=64
    let mut pc: u32 = 0x83298270;
    'dispatch: loop {
        match pc {
            0x83298270 => {
    //   block [0x83298270..0x832982B0)
	// 83298270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329827C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83298280: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298284: 388B8CC0  addi r4, r11, -0x7340
	ctx.r[4].s64 = ctx.r[11].s64 + -29504;
	// 83298288: 386A4F98  addi r3, r10, 0x4f98
	ctx.r[3].s64 = ctx.r[10].s64 + 20376;
	// 8329828C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298290: 4AF94C41  bl 0x8222ced0
	ctx.lr = 0x83298294;
	sub_8222CED0(ctx, base);
	// 83298294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298298: 38697978  addi r3, r9, 0x7978
	ctx.r[3].s64 = ctx.r[9].s64 + 31096;
	// 8329829C: 4BA11C85  bl 0x82ca9f20
	ctx.lr = 0x832982A0;
	sub_82CA9F20(ctx, base);
	// 832982A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832982A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832982A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832982AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832982B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832982B0 size=64
    let mut pc: u32 = 0x832982B0;
    'dispatch: loop {
        match pc {
            0x832982B0 => {
    //   block [0x832982B0..0x832982F0)
	// 832982B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832982B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832982B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832982BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832982C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832982C4: 388BC3B8  addi r4, r11, -0x3c48
	ctx.r[4].s64 = ctx.r[11].s64 + -15432;
	// 832982C8: 386A4F9C  addi r3, r10, 0x4f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 20380;
	// 832982CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832982D0: 4AF94C01  bl 0x8222ced0
	ctx.lr = 0x832982D4;
	sub_8222CED0(ctx, base);
	// 832982D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832982D8: 38697988  addi r3, r9, 0x7988
	ctx.r[3].s64 = ctx.r[9].s64 + 31112;
	// 832982DC: 4BA11C45  bl 0x82ca9f20
	ctx.lr = 0x832982E0;
	sub_82CA9F20(ctx, base);
	// 832982E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832982E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832982E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832982EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832982F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832982F0 size=64
    let mut pc: u32 = 0x832982F0;
    'dispatch: loop {
        match pc {
            0x832982F0 => {
    //   block [0x832982F0..0x83298330)
	// 832982F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832982F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832982F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832982FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298300: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298304: 388BC3C0  addi r4, r11, -0x3c40
	ctx.r[4].s64 = ctx.r[11].s64 + -15424;
	// 83298308: 386A4FA0  addi r3, r10, 0x4fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 20384;
	// 8329830C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298310: 4AF94BC1  bl 0x8222ced0
	ctx.lr = 0x83298314;
	sub_8222CED0(ctx, base);
	// 83298314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298318: 38697998  addi r3, r9, 0x7998
	ctx.r[3].s64 = ctx.r[9].s64 + 31128;
	// 8329831C: 4BA11C05  bl 0x82ca9f20
	ctx.lr = 0x83298320;
	sub_82CA9F20(ctx, base);
	// 83298320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329832C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298330 size=64
    let mut pc: u32 = 0x83298330;
    'dispatch: loop {
        match pc {
            0x83298330 => {
    //   block [0x83298330..0x83298370)
	// 83298330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329833C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298340: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298344: 388BC3CC  addi r4, r11, -0x3c34
	ctx.r[4].s64 = ctx.r[11].s64 + -15412;
	// 83298348: 386A4FA4  addi r3, r10, 0x4fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 20388;
	// 8329834C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298350: 4AF94B81  bl 0x8222ced0
	ctx.lr = 0x83298354;
	sub_8222CED0(ctx, base);
	// 83298354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298358: 386979A8  addi r3, r9, 0x79a8
	ctx.r[3].s64 = ctx.r[9].s64 + 31144;
	// 8329835C: 4BA11BC5  bl 0x82ca9f20
	ctx.lr = 0x83298360;
	sub_82CA9F20(ctx, base);
	// 83298360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329836C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298370 size=64
    let mut pc: u32 = 0x83298370;
    'dispatch: loop {
        match pc {
            0x83298370 => {
    //   block [0x83298370..0x832983B0)
	// 83298370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329837C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298380: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298384: 388BC55C  addi r4, r11, -0x3aa4
	ctx.r[4].s64 = ctx.r[11].s64 + -15012;
	// 83298388: 386A4FA8  addi r3, r10, 0x4fa8
	ctx.r[3].s64 = ctx.r[10].s64 + 20392;
	// 8329838C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298390: 4AF94B41  bl 0x8222ced0
	ctx.lr = 0x83298394;
	sub_8222CED0(ctx, base);
	// 83298394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298398: 386979B8  addi r3, r9, 0x79b8
	ctx.r[3].s64 = ctx.r[9].s64 + 31160;
	// 8329839C: 4BA11B85  bl 0x82ca9f20
	ctx.lr = 0x832983A0;
	sub_82CA9F20(ctx, base);
	// 832983A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832983A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832983A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832983AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832983B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832983B0 size=64
    let mut pc: u32 = 0x832983B0;
    'dispatch: loop {
        match pc {
            0x832983B0 => {
    //   block [0x832983B0..0x832983F0)
	// 832983B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832983B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832983B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832983BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832983C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832983C4: 388BC574  addi r4, r11, -0x3a8c
	ctx.r[4].s64 = ctx.r[11].s64 + -14988;
	// 832983C8: 386A4FAC  addi r3, r10, 0x4fac
	ctx.r[3].s64 = ctx.r[10].s64 + 20396;
	// 832983CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832983D0: 4AF94B01  bl 0x8222ced0
	ctx.lr = 0x832983D4;
	sub_8222CED0(ctx, base);
	// 832983D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832983D8: 386979C8  addi r3, r9, 0x79c8
	ctx.r[3].s64 = ctx.r[9].s64 + 31176;
	// 832983DC: 4BA11B45  bl 0x82ca9f20
	ctx.lr = 0x832983E0;
	sub_82CA9F20(ctx, base);
	// 832983E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832983E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832983E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832983EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832983F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832983F0 size=64
    let mut pc: u32 = 0x832983F0;
    'dispatch: loop {
        match pc {
            0x832983F0 => {
    //   block [0x832983F0..0x83298430)
	// 832983F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832983F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832983F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832983FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298400: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298404: 388BC674  addi r4, r11, -0x398c
	ctx.r[4].s64 = ctx.r[11].s64 + -14732;
	// 83298408: 386A4FB0  addi r3, r10, 0x4fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 20400;
	// 8329840C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298410: 4AF94AC1  bl 0x8222ced0
	ctx.lr = 0x83298414;
	sub_8222CED0(ctx, base);
	// 83298414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298418: 386979D8  addi r3, r9, 0x79d8
	ctx.r[3].s64 = ctx.r[9].s64 + 31192;
	// 8329841C: 4BA11B05  bl 0x82ca9f20
	ctx.lr = 0x83298420;
	sub_82CA9F20(ctx, base);
	// 83298420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298430 size=64
    let mut pc: u32 = 0x83298430;
    'dispatch: loop {
        match pc {
            0x83298430 => {
    //   block [0x83298430..0x83298470)
	// 83298430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329843C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298440: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298444: 388BC684  addi r4, r11, -0x397c
	ctx.r[4].s64 = ctx.r[11].s64 + -14716;
	// 83298448: 386A4FB4  addi r3, r10, 0x4fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 20404;
	// 8329844C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298450: 4AF94A81  bl 0x8222ced0
	ctx.lr = 0x83298454;
	sub_8222CED0(ctx, base);
	// 83298454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298458: 386979E8  addi r3, r9, 0x79e8
	ctx.r[3].s64 = ctx.r[9].s64 + 31208;
	// 8329845C: 4BA11AC5  bl 0x82ca9f20
	ctx.lr = 0x83298460;
	sub_82CA9F20(ctx, base);
	// 83298460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329846C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298470 size=64
    let mut pc: u32 = 0x83298470;
    'dispatch: loop {
        match pc {
            0x83298470 => {
    //   block [0x83298470..0x832984B0)
	// 83298470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329847C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298480: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298484: 388BC7D4  addi r4, r11, -0x382c
	ctx.r[4].s64 = ctx.r[11].s64 + -14380;
	// 83298488: 386A4FB8  addi r3, r10, 0x4fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 20408;
	// 8329848C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298490: 4AF94A41  bl 0x8222ced0
	ctx.lr = 0x83298494;
	sub_8222CED0(ctx, base);
	// 83298494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298498: 386979F8  addi r3, r9, 0x79f8
	ctx.r[3].s64 = ctx.r[9].s64 + 31224;
	// 8329849C: 4BA11A85  bl 0x82ca9f20
	ctx.lr = 0x832984A0;
	sub_82CA9F20(ctx, base);
	// 832984A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832984A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832984A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832984AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832984B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832984B0 size=64
    let mut pc: u32 = 0x832984B0;
    'dispatch: loop {
        match pc {
            0x832984B0 => {
    //   block [0x832984B0..0x832984F0)
	// 832984B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832984B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832984B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832984BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832984C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832984C4: 388BC7E8  addi r4, r11, -0x3818
	ctx.r[4].s64 = ctx.r[11].s64 + -14360;
	// 832984C8: 386A4FBC  addi r3, r10, 0x4fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 20412;
	// 832984CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832984D0: 4AF94A01  bl 0x8222ced0
	ctx.lr = 0x832984D4;
	sub_8222CED0(ctx, base);
	// 832984D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832984D8: 38697A08  addi r3, r9, 0x7a08
	ctx.r[3].s64 = ctx.r[9].s64 + 31240;
	// 832984DC: 4BA11A45  bl 0x82ca9f20
	ctx.lr = 0x832984E0;
	sub_82CA9F20(ctx, base);
	// 832984E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832984E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832984E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832984EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832984F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832984F0 size=64
    let mut pc: u32 = 0x832984F0;
    'dispatch: loop {
        match pc {
            0x832984F0 => {
    //   block [0x832984F0..0x83298530)
	// 832984F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832984F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832984F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832984FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298500: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298504: 388BC8D8  addi r4, r11, -0x3728
	ctx.r[4].s64 = ctx.r[11].s64 + -14120;
	// 83298508: 386A4FC0  addi r3, r10, 0x4fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 20416;
	// 8329850C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298510: 4AF949C1  bl 0x8222ced0
	ctx.lr = 0x83298514;
	sub_8222CED0(ctx, base);
	// 83298514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298518: 38697A18  addi r3, r9, 0x7a18
	ctx.r[3].s64 = ctx.r[9].s64 + 31256;
	// 8329851C: 4BA11A05  bl 0x82ca9f20
	ctx.lr = 0x83298520;
	sub_82CA9F20(ctx, base);
	// 83298520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298530 size=64
    let mut pc: u32 = 0x83298530;
    'dispatch: loop {
        match pc {
            0x83298530 => {
    //   block [0x83298530..0x83298570)
	// 83298530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329853C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298540: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298544: 388BC8E8  addi r4, r11, -0x3718
	ctx.r[4].s64 = ctx.r[11].s64 + -14104;
	// 83298548: 386A4FC4  addi r3, r10, 0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 20420;
	// 8329854C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298550: 4AF94981  bl 0x8222ced0
	ctx.lr = 0x83298554;
	sub_8222CED0(ctx, base);
	// 83298554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298558: 38697A28  addi r3, r9, 0x7a28
	ctx.r[3].s64 = ctx.r[9].s64 + 31272;
	// 8329855C: 4BA119C5  bl 0x82ca9f20
	ctx.lr = 0x83298560;
	sub_82CA9F20(ctx, base);
	// 83298560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329856C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298570 size=64
    let mut pc: u32 = 0x83298570;
    'dispatch: loop {
        match pc {
            0x83298570 => {
    //   block [0x83298570..0x832985B0)
	// 83298570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329857C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298580: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298584: 388BC914  addi r4, r11, -0x36ec
	ctx.r[4].s64 = ctx.r[11].s64 + -14060;
	// 83298588: 386A4FC8  addi r3, r10, 0x4fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 20424;
	// 8329858C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298590: 4AF94941  bl 0x8222ced0
	ctx.lr = 0x83298594;
	sub_8222CED0(ctx, base);
	// 83298594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298598: 38697A38  addi r3, r9, 0x7a38
	ctx.r[3].s64 = ctx.r[9].s64 + 31288;
	// 8329859C: 4BA11985  bl 0x82ca9f20
	ctx.lr = 0x832985A0;
	sub_82CA9F20(ctx, base);
	// 832985A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832985A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832985A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832985AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832985B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832985B0 size=64
    let mut pc: u32 = 0x832985B0;
    'dispatch: loop {
        match pc {
            0x832985B0 => {
    //   block [0x832985B0..0x832985F0)
	// 832985B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832985B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832985B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832985BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832985C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832985C4: 388BC9D0  addi r4, r11, -0x3630
	ctx.r[4].s64 = ctx.r[11].s64 + -13872;
	// 832985C8: 386A4FD8  addi r3, r10, 0x4fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 20440;
	// 832985CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832985D0: 4AF94901  bl 0x8222ced0
	ctx.lr = 0x832985D4;
	sub_8222CED0(ctx, base);
	// 832985D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832985D8: 38697A48  addi r3, r9, 0x7a48
	ctx.r[3].s64 = ctx.r[9].s64 + 31304;
	// 832985DC: 4BA11945  bl 0x82ca9f20
	ctx.lr = 0x832985E0;
	sub_82CA9F20(ctx, base);
	// 832985E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832985E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832985E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832985EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832985F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832985F0 size=64
    let mut pc: u32 = 0x832985F0;
    'dispatch: loop {
        match pc {
            0x832985F0 => {
    //   block [0x832985F0..0x83298630)
	// 832985F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832985F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832985F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832985FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298600: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298604: 388BA950  addi r4, r11, -0x56b0
	ctx.r[4].s64 = ctx.r[11].s64 + -22192;
	// 83298608: 386A4FDC  addi r3, r10, 0x4fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 20444;
	// 8329860C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298610: 4AF948C1  bl 0x8222ced0
	ctx.lr = 0x83298614;
	sub_8222CED0(ctx, base);
	// 83298614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298618: 38697A58  addi r3, r9, 0x7a58
	ctx.r[3].s64 = ctx.r[9].s64 + 31320;
	// 8329861C: 4BA11905  bl 0x82ca9f20
	ctx.lr = 0x83298620;
	sub_82CA9F20(ctx, base);
	// 83298620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329862C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298630 size=132
    let mut pc: u32 = 0x83298630;
    'dispatch: loop {
        match pc {
            0x83298630 => {
    //   block [0x83298630..0x83298688)
	// 83298630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298634: 4BA10DD9  bl 0x82ca940c
	ctx.lr = 0x83298638;
	sub_82CA93D0(ctx, base);
	// 83298638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329863C: 4B8BC6BD  bl 0x82b54cf8
	ctx.lr = 0x83298640;
	sub_82B54CF8(ctx, base);
	// 83298640: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298644: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83298648: 3BEA4FE0  addi r31, r10, 0x4fe0
	ctx.r[31].s64 = ctx.r[10].s64 + 20448;
	// 8329864C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83298650: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 83298654: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83298658: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8329865C: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83298660: F95F0008  std r10, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83298664: 4BD68A8D  bl 0x830010f0
	ctx.lr = 0x83298668;
	sub_830010F0(ctx, base);
	// 83298668: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8329866C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83298670: 4AF86BE9  bl 0x8221f258
	ctx.lr = 0x83298674;
	sub_8221F258(ctx, base);
	// 83298674: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83298678: 419A0010  beq cr6, 0x83298688
	if ctx.cr[6].eq {
	pc = 0x83298688; continue 'dispatch;
	}
	// 8329867C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83298680: FBA30008  std r29, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 83298684: 48000008  b 0x8329868c
	pc = 0x8329868C; continue 'dispatch;
            }
            0x83298688 => {
    //   block [0x83298688..0x8329868C)
	// 83298688: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8329868C; continue 'dispatch;
            }
            0x8329868C => {
    //   block [0x8329868C..0x832986B4)
	// 8329868C: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83298690: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298694: 93AB000C  stw r29, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83298698: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8329869C: 386A7A68  addi r3, r10, 0x7a68
	ctx.r[3].s64 = ctx.r[10].s64 + 31336;
	// 832986A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832986A4: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 832986A8: 4BA11879  bl 0x82ca9f20
	ctx.lr = 0x832986AC;
	sub_82CA9F20(ctx, base);
	// 832986AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832986B0: 4BA10DAC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832986B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832986B8 size=64
    let mut pc: u32 = 0x832986B8;
    'dispatch: loop {
        match pc {
            0x832986B8 => {
    //   block [0x832986B8..0x832986F8)
	// 832986B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832986BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832986C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832986C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832986C8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832986CC: 388BCA58  addi r4, r11, -0x35a8
	ctx.r[4].s64 = ctx.r[11].s64 + -13736;
	// 832986D0: 386A5008  addi r3, r10, 0x5008
	ctx.r[3].s64 = ctx.r[10].s64 + 20488;
	// 832986D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832986D8: 4AF947F9  bl 0x8222ced0
	ctx.lr = 0x832986DC;
	sub_8222CED0(ctx, base);
	// 832986DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832986E0: 38697A78  addi r3, r9, 0x7a78
	ctx.r[3].s64 = ctx.r[9].s64 + 31352;
	// 832986E4: 4BA1183D  bl 0x82ca9f20
	ctx.lr = 0x832986E8;
	sub_82CA9F20(ctx, base);
	// 832986E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832986EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832986F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832986F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832986F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832986F8 size=64
    let mut pc: u32 = 0x832986F8;
    'dispatch: loop {
        match pc {
            0x832986F8 => {
    //   block [0x832986F8..0x83298738)
	// 832986F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832986FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298704: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298708: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329870C: 388BCA64  addi r4, r11, -0x359c
	ctx.r[4].s64 = ctx.r[11].s64 + -13724;
	// 83298710: 386A500C  addi r3, r10, 0x500c
	ctx.r[3].s64 = ctx.r[10].s64 + 20492;
	// 83298714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298718: 4AF947B9  bl 0x8222ced0
	ctx.lr = 0x8329871C;
	sub_8222CED0(ctx, base);
	// 8329871C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298720: 38697A88  addi r3, r9, 0x7a88
	ctx.r[3].s64 = ctx.r[9].s64 + 31368;
	// 83298724: 4BA117FD  bl 0x82ca9f20
	ctx.lr = 0x83298728;
	sub_82CA9F20(ctx, base);
	// 83298728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329872C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298738 size=52
    let mut pc: u32 = 0x83298738;
    'dispatch: loop {
        match pc {
            0x83298738 => {
    //   block [0x83298738..0x8329876C)
	// 83298738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329873C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298744: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298748: 386B5010  addi r3, r11, 0x5010
	ctx.r[3].s64 = ctx.r[11].s64 + 20496;
	// 8329874C: 4B9B73BD  bl 0x82c4fb08
	ctx.lr = 0x83298750;
	sub_82C4FB08(ctx, base);
	// 83298750: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298754: 386A7A98  addi r3, r10, 0x7a98
	ctx.r[3].s64 = ctx.r[10].s64 + 31384;
	// 83298758: 4BA117C9  bl 0x82ca9f20
	ctx.lr = 0x8329875C;
	sub_82CA9F20(ctx, base);
	// 8329875C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298770 size=52
    let mut pc: u32 = 0x83298770;
    'dispatch: loop {
        match pc {
            0x83298770 => {
    //   block [0x83298770..0x832987A4)
	// 83298770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329877C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298780: 386B5088  addi r3, r11, 0x5088
	ctx.r[3].s64 = ctx.r[11].s64 + 20616;
	// 83298784: 4B9B7385  bl 0x82c4fb08
	ctx.lr = 0x83298788;
	sub_82C4FB08(ctx, base);
	// 83298788: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329878C: 386A7AF0  addi r3, r10, 0x7af0
	ctx.r[3].s64 = ctx.r[10].s64 + 31472;
	// 83298790: 4BA11791  bl 0x82ca9f20
	ctx.lr = 0x83298794;
	sub_82CA9F20(ctx, base);
	// 83298794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329879C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832987A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832987A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832987A8 size=52
    let mut pc: u32 = 0x832987A8;
    'dispatch: loop {
        match pc {
            0x832987A8 => {
    //   block [0x832987A8..0x832987DC)
	// 832987A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832987AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832987B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832987B4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832987B8: 386B5108  addi r3, r11, 0x5108
	ctx.r[3].s64 = ctx.r[11].s64 + 20744;
	// 832987BC: 4B9713F5  bl 0x82c09bb0
	ctx.lr = 0x832987C0;
	sub_82C09BB0(ctx, base);
	// 832987C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832987C4: 386A7B48  addi r3, r10, 0x7b48
	ctx.r[3].s64 = ctx.r[10].s64 + 31560;
	// 832987C8: 4BA11759  bl 0x82ca9f20
	ctx.lr = 0x832987CC;
	sub_82CA9F20(ctx, base);
	// 832987CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832987D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832987D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832987D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832987E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832987E0 size=16
    let mut pc: u32 = 0x832987E0;
    'dispatch: loop {
        match pc {
            0x832987E0 => {
    //   block [0x832987E0..0x832987F0)
	// 832987E0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832987E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832987E8: 914B515C  stw r10, 0x515c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20828 as u32), ctx.r[10].u32 ) };
	// 832987EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832987F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832987F0 size=16
    let mut pc: u32 = 0x832987F0;
    'dispatch: loop {
        match pc {
            0x832987F0 => {
    //   block [0x832987F0..0x83298800)
	// 832987F0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832987F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832987F8: 914B5160  stw r10, 0x5160(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20832 as u32), ctx.r[10].u32 ) };
	// 832987FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298800 size=52
    let mut pc: u32 = 0x83298800;
    'dispatch: loop {
        match pc {
            0x83298800 => {
    //   block [0x83298800..0x83298834)
	// 83298800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329880C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298810: 386B5184  addi r3, r11, 0x5184
	ctx.r[3].s64 = ctx.r[11].s64 + 20868;
	// 83298814: 4B215E4D  bl 0x824ae660
	ctx.lr = 0x83298818;
	sub_824AE660(ctx, base);
	// 83298818: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329881C: 386A7B58  addi r3, r10, 0x7b58
	ctx.r[3].s64 = ctx.r[10].s64 + 31576;
	// 83298820: 4BA11701  bl 0x82ca9f20
	ctx.lr = 0x83298824;
	sub_82CA9F20(ctx, base);
	// 83298824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329882C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83298838 size=524
    let mut pc: u32 = 0x83298838;
    'dispatch: loop {
        match pc {
            0x83298838 => {
    //   block [0x83298838..0x832988E4)
	// 83298838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329883C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83298844: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 83298848: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8329884C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298850: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 83298854: 38A00045  li r5, 0x45
	ctx.r[5].s64 = 69;
	// 83298858: 3BEB62D0  addi r31, r11, 0x62d0
	ctx.r[31].s64 = ctx.r[11].s64 + 25296;
	// 8329885C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298860: 387F00CF  addi r3, r31, 0xcf
	ctx.r[3].s64 = ctx.r[31].s64 + 207;
	// 83298864: 4BA1114D  bl 0x82ca99b0
	ctx.lr = 0x83298868;
	sub_82CA99B0(ctx, base);
	// 83298868: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8329886C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298870: 392BD53C  addi r9, r11, -0x2ac4
	ctx.r[9].s64 = ctx.r[11].s64 + -10948;
	// 83298874: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 83298878: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329887C: 810BD53C  lwz r8, -0x2ac4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10948 as u32) ) } as u64;
	// 83298880: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 83298884: C3EA0C14  lfs f31, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83298888: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8329888C: D3FF0114  stfs f31, 0x114(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 83298890: 80C90008  lwz r6, 8(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 83298894: D3FF0118  stfs f31, 0x118(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 83298898: D3FF011C  stfs f31, 0x11c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 8329889C: 911F0120  stw r8, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[8].u32 ) };
	// 832988A0: 90FF0124  stw r7, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[7].u32 ) };
	// 832988A4: 90DF0128  stw r6, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[6].u32 ) };
	// 832988A8: 4BA11109  bl 0x82ca99b0
	ctx.lr = 0x832988AC;
	sub_82CA99B0(ctx, base);
	// 832988AC: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 832988B0: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 832988B4: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 832988B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832988BC: 395F017C  addi r10, r31, 0x17c
	ctx.r[10].s64 = ctx.r[31].s64 + 380;
	// 832988C0: C005D308  lfs f0, -0x2cf8(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-11512 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832988C4: 396BD520  addi r11, r11, -0x2ae0
	ctx.r[11].s64 = ctx.r[11].s64 + -10976;
	// 832988C8: C1A4D300  lfs f13, -0x2d00(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-11520 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832988CC: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 832988D0: C183D304  lfs f12, -0x2cfc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-11516 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832988D4: D01F0170  stfs f0, 0x170(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), tmp.u32 ) };
	// 832988D8: D1BF0174  stfs f13, 0x174(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 832988DC: D19F0178  stfs f12, 0x178(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), tmp.u32 ) };
	// 832988E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x832988E4; continue 'dispatch;
            }
            0x832988E4 => {
    //   block [0x832988E4..0x8329893C)
	// 832988E4: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832988E8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 832988EC: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 832988F0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832988F4: 4200FFF0  bdnz 0x832988e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x832988E4; continue 'dispatch;
	}
	// 832988F8: 38A00036  li r5, 0x36
	ctx.r[5].s64 = 54;
	// 832988FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298900: 387F0196  addi r3, r31, 0x196
	ctx.r[3].s64 = ctx.r[31].s64 + 406;
	// 83298904: 4BA110AD  bl 0x82ca99b0
	ctx.lr = 0x83298908;
	sub_82CA99B0(ctx, base);
	// 83298908: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8329890C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 83298910: D3FF01CC  stfs f31, 0x1cc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), tmp.u32 ) };
	// 83298914: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298918: 395F01D8  addi r10, r31, 0x1d8
	ctx.r[10].s64 = ctx.r[31].s64 + 472;
	// 8329891C: 38E8D508  addi r7, r8, -0x2af8
	ctx.r[7].s64 = ctx.r[8].s64 + -11000;
	// 83298920: C3CB0C18  lfs f30, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 83298924: C0090A54  lfs f0, 0xa54(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2644 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83298928: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8329892C: D3DF01D0  stfs f30, 0x1d0(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), tmp.u32 ) };
	// 83298930: 39200015  li r9, 0x15
	ctx.r[9].s64 = 21;
	// 83298934: D01F01D4  stfs f0, 0x1d4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), tmp.u32 ) };
	// 83298938: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8329893C; continue 'dispatch;
            }
            0x8329893C => {
    //   block [0x8329893C..0x832989A0)
	// 8329893C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83298940: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83298944: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83298948: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8329894C: 4200FFF0  bdnz 0x8329893c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8329893C; continue 'dispatch;
	}
	// 83298950: 38A0003B  li r5, 0x3b
	ctx.r[5].s64 = 59;
	// 83298954: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298958: 387F01ED  addi r3, r31, 0x1ed
	ctx.r[3].s64 = ctx.r[31].s64 + 493;
	// 8329895C: 4BA11055  bl 0x82ca99b0
	ctx.lr = 0x83298960;
	sub_82CA99B0(ctx, base);
	// 83298960: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298964: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298968: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 8329896C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298970: 38FF0234  addi r7, r31, 0x234
	ctx.r[7].s64 = ctx.r[31].s64 + 564;
	// 83298974: 38C8D4F4  addi r6, r8, -0x2b0c
	ctx.r[6].s64 = ctx.r[8].s64 + -11020;
	// 83298978: C00BD310  lfs f0, -0x2cf0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8329897C: C1AA0BF8  lfs f13, 0xbf8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3064 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83298980: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83298984: C1899404  lfs f12, -0x6bfc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27644 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83298988: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8329898C: D01F0228  stfs f0, 0x228(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), tmp.u32 ) };
	// 83298990: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 83298994: D1BF022C  stfs f13, 0x22c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(556 as u32), tmp.u32 ) };
	// 83298998: D19F0230  stfs f12, 0x230(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), tmp.u32 ) };
	// 8329899C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x832989A0; continue 'dispatch;
            }
            0x832989A0 => {
    //   block [0x832989A0..0x83298A0C)
	// 832989A0: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832989A4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 832989A8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 832989AC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832989B0: 4200FFF0  bdnz 0x832989a0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x832989A0; continue 'dispatch;
	}
	// 832989B4: 38A0003E  li r5, 0x3e
	ctx.r[5].s64 = 62;
	// 832989B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832989BC: 387F0246  addi r3, r31, 0x246
	ctx.r[3].s64 = ctx.r[31].s64 + 582;
	// 832989C0: 4BA10FF1  bl 0x82ca99b0
	ctx.lr = 0x832989C4;
	sub_82CA99B0(ctx, base);
	// 832989C4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 832989C8: D3DF0284  stfs f30, 0x284(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), tmp.u32 ) };
	// 832989CC: 391F0290  addi r8, r31, 0x290
	ctx.r[8].s64 = ctx.r[31].s64 + 656;
	// 832989D0: 38E9D4E4  addi r7, r9, -0x2b1c
	ctx.r[7].s64 = ctx.r[9].s64 + -11036;
	// 832989D4: D3DF0288  stfs f30, 0x288(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), tmp.u32 ) };
	// 832989D8: D3FF028C  stfs f31, 0x28c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), tmp.u32 ) };
	// 832989DC: 397F02A0  addi r11, r31, 0x2a0
	ctx.r[11].s64 = ctx.r[31].s64 + 672;
	// 832989E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832989E4: 80C9D4E4  lwz r6, -0x2b1c(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-11036 as u32) ) } as u64;
	// 832989E8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 832989EC: 80A70004  lwz r5, 4(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 832989F0: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 832989F4: 8067000C  lwz r3, 0xc(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 832989F8: 90DF0290  stw r6, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[6].u32 ) };
	// 832989FC: 90BF0294  stw r5, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[5].u32 ) };
	// 83298A00: 909F0298  stw r4, 0x298(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[4].u32 ) };
	// 83298A04: 907F029C  stw r3, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[3].u32 ) };
	// 83298A08: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x83298A0C; continue 'dispatch;
            }
            0x83298A0C => {
    //   block [0x83298A0C..0x83298A44)
	// 83298A0C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83298A10: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83298A14: 4200FFF8  bdnz 0x83298a0c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83298A0C; continue 'dispatch;
	}
	// 83298A18: 38A001CC  li r5, 0x1cc
	ctx.r[5].s64 = 460;
	// 83298A1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298A20: 387F02E0  addi r3, r31, 0x2e0
	ctx.r[3].s64 = ctx.r[31].s64 + 736;
	// 83298A24: 4BA10F8D  bl 0x82ca99b0
	ctx.lr = 0x83298A28;
	sub_82CA99B0(ctx, base);
	// 83298A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83298A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298A34: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 83298A38: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83298A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83298A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298A48 size=44
    let mut pc: u32 = 0x83298A48;
    'dispatch: loop {
        match pc {
            0x83298A48 => {
    //   block [0x83298A48..0x83298A74)
	// 83298A48: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298A4C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298A50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298A54: 386951AC  addi r3, r9, 0x51ac
	ctx.r[3].s64 = ctx.r[9].s64 + 20908;
	// 83298A58: 388AD6F8  addi r4, r10, -0x2908
	ctx.r[4].s64 = ctx.r[10].s64 + -10504;
	// 83298A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298A60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298A64: 38EBFB28  addi r7, r11, -0x4d8
	ctx.r[7].s64 = ctx.r[11].s64 + -1240;
	// 83298A68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83298A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298A70: 4B9CB448  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298A78 size=44
    let mut pc: u32 = 0x83298A78;
    'dispatch: loop {
        match pc {
            0x83298A78 => {
    //   block [0x83298A78..0x83298AA4)
	// 83298A78: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298A7C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298A80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298A84: 38695190  addi r3, r9, 0x5190
	ctx.r[3].s64 = ctx.r[9].s64 + 20880;
	// 83298A88: 388AD700  addi r4, r10, -0x2900
	ctx.r[4].s64 = ctx.r[10].s64 + -10496;
	// 83298A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298A90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298A94: 38EBFB68  addi r7, r11, -0x498
	ctx.r[7].s64 = ctx.r[11].s64 + -1176;
	// 83298A98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83298A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298AA0: 4B9CB418  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298AA8 size=44
    let mut pc: u32 = 0x83298AA8;
    'dispatch: loop {
        match pc {
            0x83298AA8 => {
    //   block [0x83298AA8..0x83298AD4)
	// 83298AA8: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298AAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298AB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298AB4: 38695858  addi r3, r9, 0x5858
	ctx.r[3].s64 = ctx.r[9].s64 + 22616;
	// 83298AB8: 38CBDA98  addi r6, r11, -0x2568
	ctx.r[6].s64 = ctx.r[11].s64 + -9576;
	// 83298ABC: 388ADBC0  addi r4, r10, -0x2440
	ctx.r[4].s64 = ctx.r[10].s64 + -9280;
	// 83298AC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298AC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298AD0: 4B9CB3E8  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298AD8 size=48
    let mut pc: u32 = 0x83298AD8;
    'dispatch: loop {
        match pc {
            0x83298AD8 => {
    //   block [0x83298AD8..0x83298B08)
	// 83298AD8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298ADC: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298AE0: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298AE4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298AE8: 3889DBC0  addi r4, r9, -0x2440
	ctx.r[4].s64 = ctx.r[9].s64 + -9280;
	// 83298AEC: 38685804  addi r3, r8, 0x5804
	ctx.r[3].s64 = ctx.r[8].s64 + 22532;
	// 83298AF0: 38CADA98  addi r6, r10, -0x2568
	ctx.r[6].s64 = ctx.r[10].s64 + -9576;
	// 83298AF4: 392B2A90  addi r9, r11, 0x2a90
	ctx.r[9].s64 = ctx.r[11].s64 + 10896;
	// 83298AF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298B00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298B04: 4B9CB3B4  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B08 size=48
    let mut pc: u32 = 0x83298B08;
    'dispatch: loop {
        match pc {
            0x83298B08 => {
    //   block [0x83298B08..0x83298B38)
	// 83298B08: 3C808333  lis r4, -0x7ccd
	ctx.r[4].s64 = -2093809664;
	// 83298B0C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298B10: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298B14: 3CA082C6  lis r5, -0x7d3a
	ctx.r[5].s64 = -2100953088;
	// 83298B18: 38645794  addi r3, r4, 0x5794
	ctx.r[3].s64 = ctx.r[4].s64 + 22420;
	// 83298B1C: 392B2868  addi r9, r11, 0x2868
	ctx.r[9].s64 = ctx.r[11].s64 + 10344;
	// 83298B20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298B24: 38EA2590  addi r7, r10, 0x2590
	ctx.r[7].s64 = ctx.r[10].s64 + 9616;
	// 83298B28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83298B2C: 38A52560  addi r5, r5, 0x2560
	ctx.r[5].s64 = ctx.r[5].s64 + 9568;
	// 83298B30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298B34: 4B9CB384  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B38 size=48
    let mut pc: u32 = 0x83298B38;
    'dispatch: loop {
        match pc {
            0x83298B38 => {
    //   block [0x83298B38..0x83298B68)
	// 83298B38: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298B3C: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298B40: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298B44: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298B48: 3889DA94  addi r4, r9, -0x256c
	ctx.r[4].s64 = ctx.r[9].s64 + -9580;
	// 83298B4C: 38685900  addi r3, r8, 0x5900
	ctx.r[3].s64 = ctx.r[8].s64 + 22784;
	// 83298B50: 38CADBB8  addi r6, r10, -0x2448
	ctx.r[6].s64 = ctx.r[10].s64 + -9288;
	// 83298B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298B58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298B5C: 38EB2648  addi r7, r11, 0x2648
	ctx.r[7].s64 = ctx.r[11].s64 + 9800;
	// 83298B60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298B64: 4B9CB354  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B68 size=48
    let mut pc: u32 = 0x83298B68;
    'dispatch: loop {
        match pc {
            0x83298B68 => {
    //   block [0x83298B68..0x83298B98)
	// 83298B68: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 83298B6C: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298B70: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298B74: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298B78: 3889EBB8  addi r4, r9, -0x1448
	ctx.r[4].s64 = ctx.r[9].s64 + -5192;
	// 83298B7C: 386858E4  addi r3, r8, 0x58e4
	ctx.r[3].s64 = ctx.r[8].s64 + 22756;
	// 83298B80: 38CADAA0  addi r6, r10, -0x2560
	ctx.r[6].s64 = ctx.r[10].s64 + -9568;
	// 83298B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298B88: 390B20E8  addi r8, r11, 0x20e8
	ctx.r[8].s64 = ctx.r[11].s64 + 8424;
	// 83298B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298B90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298B94: 4B9CB324  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B98 size=52
    let mut pc: u32 = 0x83298B98;
    'dispatch: loop {
        match pc {
            0x83298B98 => {
    //   block [0x83298B98..0x83298BCC)
	// 83298B98: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298B9C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83298BA0: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298BA4: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298BA8: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298BAC: 38C9DAA8  addi r6, r9, -0x2558
	ctx.r[6].s64 = ctx.r[9].s64 + -9560;
	// 83298BB0: 38883D28  addi r4, r8, 0x3d28
	ctx.r[4].s64 = ctx.r[8].s64 + 15656;
	// 83298BB4: 38675890  addi r3, r7, 0x5890
	ctx.r[3].s64 = ctx.r[7].s64 + 22672;
	// 83298BB8: 392B21E0  addi r9, r11, 0x21e0
	ctx.r[9].s64 = ctx.r[11].s64 + 8672;
	// 83298BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298BC0: 38EA2170  addi r7, r10, 0x2170
	ctx.r[7].s64 = ctx.r[10].s64 + 8560;
	// 83298BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298BC8: 4B9CB2F0  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298BD0 size=44
    let mut pc: u32 = 0x83298BD0;
    'dispatch: loop {
        match pc {
            0x83298BD0 => {
    //   block [0x83298BD0..0x83298BFC)
	// 83298BD0: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298BD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298BD8: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83298BDC: 3869591C  addi r3, r9, 0x591c
	ctx.r[3].s64 = ctx.r[9].s64 + 22812;
	// 83298BE0: 38CBDAA8  addi r6, r11, -0x2558
	ctx.r[6].s64 = ctx.r[11].s64 + -9560;
	// 83298BE4: 388A3D28  addi r4, r10, 0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + 15656;
	// 83298BE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298BF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298BF8: 4B9CB2C0  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C00 size=44
    let mut pc: u32 = 0x83298C00;
    'dispatch: loop {
        match pc {
            0x83298C00 => {
    //   block [0x83298C00..0x83298C2C)
	// 83298C00: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298C04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298C08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298C0C: 386957CC  addi r3, r9, 0x57cc
	ctx.r[3].s64 = ctx.r[9].s64 + 22476;
	// 83298C10: 38CBDA98  addi r6, r11, -0x2568
	ctx.r[6].s64 = ctx.r[11].s64 + -9576;
	// 83298C14: 388ADBB0  addi r4, r10, -0x2450
	ctx.r[4].s64 = ctx.r[10].s64 + -9296;
	// 83298C18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298C20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298C28: 4B9CB290  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C30 size=44
    let mut pc: u32 = 0x83298C30;
    'dispatch: loop {
        match pc {
            0x83298C30 => {
    //   block [0x83298C30..0x83298C5C)
	// 83298C30: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298C34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298C38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298C3C: 38695874  addi r3, r9, 0x5874
	ctx.r[3].s64 = ctx.r[9].s64 + 22644;
	// 83298C40: 38CBDBB8  addi r6, r11, -0x2448
	ctx.r[6].s64 = ctx.r[11].s64 + -9288;
	// 83298C44: 388ADBAC  addi r4, r10, -0x2454
	ctx.r[4].s64 = ctx.r[10].s64 + -9300;
	// 83298C48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298C50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298C58: 4B9CB260  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C60 size=48
    let mut pc: u32 = 0x83298C60;
    'dispatch: loop {
        match pc {
            0x83298C60 => {
    //   block [0x83298C60..0x83298C90)
	// 83298C60: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298C64: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298C68: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298C6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298C70: 3889DBA8  addi r4, r9, -0x2458
	ctx.r[4].s64 = ctx.r[9].s64 + -9304;
	// 83298C74: 38685778  addi r3, r8, 0x5778
	ctx.r[3].s64 = ctx.r[8].s64 + 22392;
	// 83298C78: 38CADAA8  addi r6, r10, -0x2558
	ctx.r[6].s64 = ctx.r[10].s64 + -9560;
	// 83298C7C: 392B23C8  addi r9, r11, 0x23c8
	ctx.r[9].s64 = ctx.r[11].s64 + 9160;
	// 83298C80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298C88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298C8C: 4B9CB22C  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C90 size=52
    let mut pc: u32 = 0x83298C90;
    'dispatch: loop {
        match pc {
            0x83298C90 => {
    //   block [0x83298C90..0x83298CC4)
	// 83298C90: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298C94: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298C98: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298C9C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298CA0: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298CA4: 38C9DAA8  addi r6, r9, -0x2558
	ctx.r[6].s64 = ctx.r[9].s64 + -9560;
	// 83298CA8: 3888DBA8  addi r4, r8, -0x2458
	ctx.r[4].s64 = ctx.r[8].s64 + -9304;
	// 83298CAC: 38675954  addi r3, r7, 0x5954
	ctx.r[3].s64 = ctx.r[7].s64 + 22868;
	// 83298CB0: 392B23C8  addi r9, r11, 0x23c8
	ctx.r[9].s64 = ctx.r[11].s64 + 9160;
	// 83298CB4: 390A24B0  addi r8, r10, 0x24b0
	ctx.r[8].s64 = ctx.r[10].s64 + 9392;
	// 83298CB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298CC0: 4B9CB1F8  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298CC8 size=44
    let mut pc: u32 = 0x83298CC8;
    'dispatch: loop {
        match pc {
            0x83298CC8 => {
    //   block [0x83298CC8..0x83298CF4)
	// 83298CC8: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298CCC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298CD0: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83298CD4: 386957E8  addi r3, r9, 0x57e8
	ctx.r[3].s64 = ctx.r[9].s64 + 22504;
	// 83298CD8: 38CBDBB8  addi r6, r11, -0x2448
	ctx.r[6].s64 = ctx.r[11].s64 + -9288;
	// 83298CDC: 388AFCA8  addi r4, r10, -0x358
	ctx.r[4].s64 = ctx.r[10].s64 + -856;
	// 83298CE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298CE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298CF0: 4B9CB1C8  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298CF8 size=44
    let mut pc: u32 = 0x83298CF8;
    'dispatch: loop {
        match pc {
            0x83298CF8 => {
    //   block [0x83298CF8..0x83298D24)
	// 83298CF8: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298CFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298D00: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83298D04: 386958AC  addi r3, r9, 0x58ac
	ctx.r[3].s64 = ctx.r[9].s64 + 22700;
	// 83298D08: 38CBDA98  addi r6, r11, -0x2568
	ctx.r[6].s64 = ctx.r[11].s64 + -9576;
	// 83298D0C: 388AFCA8  addi r4, r10, -0x358
	ctx.r[4].s64 = ctx.r[10].s64 + -856;
	// 83298D10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298D18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298D20: 4B9CB198  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298D28 size=52
    let mut pc: u32 = 0x83298D28;
    'dispatch: loop {
        match pc {
            0x83298D28 => {
    //   block [0x83298D28..0x83298D5C)
	// 83298D28: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298D2C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298D30: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298D34: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298D38: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298D3C: 38C9DAA8  addi r6, r9, -0x2558
	ctx.r[6].s64 = ctx.r[9].s64 + -9560;
	// 83298D40: 3888DBA8  addi r4, r8, -0x2458
	ctx.r[4].s64 = ctx.r[8].s64 + -9304;
	// 83298D44: 3867583C  addi r3, r7, 0x583c
	ctx.r[3].s64 = ctx.r[7].s64 + 22588;
	// 83298D48: 392B2308  addi r9, r11, 0x2308
	ctx.r[9].s64 = ctx.r[11].s64 + 8968;
	// 83298D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298D50: 38EA2298  addi r7, r10, 0x2298
	ctx.r[7].s64 = ctx.r[10].s64 + 8856;
	// 83298D54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298D58: 4B9CB160  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298D60 size=44
    let mut pc: u32 = 0x83298D60;
    'dispatch: loop {
        match pc {
            0x83298D60 => {
    //   block [0x83298D60..0x83298D8C)
	// 83298D60: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298D68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298D6C: 38695740  addi r3, r9, 0x5740
	ctx.r[3].s64 = ctx.r[9].s64 + 22336;
	// 83298D70: 38CBDBB8  addi r6, r11, -0x2448
	ctx.r[6].s64 = ctx.r[11].s64 + -9288;
	// 83298D74: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83298D78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298D80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298D88: 4B9CB130  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298D90 size=48
    let mut pc: u32 = 0x83298D90;
    'dispatch: loop {
        match pc {
            0x83298D90 => {
    //   block [0x83298D90..0x83298DC0)
	// 83298D90: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83298D94: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298D98: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298D9C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83298DA0: 3D0082C6  lis r8, -0x7d3a
	ctx.r[8].s64 = -2100953088;
	// 83298DA4: 386757B0  addi r3, r7, 0x57b0
	ctx.r[3].s64 = ctx.r[7].s64 + 22448;
	// 83298DA8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83298DAC: 392A27D0  addi r9, r10, 0x27d0
	ctx.r[9].s64 = ctx.r[10].s64 + 10192;
	// 83298DB0: 390826F8  addi r8, r8, 0x26f8
	ctx.r[8].s64 = ctx.r[8].s64 + 9976;
	// 83298DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298DB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298DBC: 4B9CB0FC  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298DC0 size=44
    let mut pc: u32 = 0x83298DC0;
    'dispatch: loop {
        match pc {
            0x83298DC0 => {
    //   block [0x83298DC0..0x83298DEC)
	// 83298DC0: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298DC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298DC8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298DCC: 38695938  addi r3, r9, 0x5938
	ctx.r[3].s64 = ctx.r[9].s64 + 22840;
	// 83298DD0: 38CBDBA0  addi r6, r11, -0x2460
	ctx.r[6].s64 = ctx.r[11].s64 + -9312;
	// 83298DD4: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83298DD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298DE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298DE8: 4B9CB0D0  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298DF0 size=48
    let mut pc: u32 = 0x83298DF0;
    'dispatch: loop {
        match pc {
            0x83298DF0 => {
    //   block [0x83298DF0..0x83298E20)
	// 83298DF0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 83298DF4: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298DF8: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298DFC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298E00: 38890CA0  addi r4, r9, 0xca0
	ctx.r[4].s64 = ctx.r[9].s64 + 3232;
	// 83298E04: 3868575C  addi r3, r8, 0x575c
	ctx.r[3].s64 = ctx.r[8].s64 + 22364;
	// 83298E08: 38CADB98  addi r6, r10, -0x2468
	ctx.r[6].s64 = ctx.r[10].s64 + -9320;
	// 83298E0C: 392B2980  addi r9, r11, 0x2980
	ctx.r[9].s64 = ctx.r[11].s64 + 10624;
	// 83298E10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298E18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298E1C: 4B9CB09C  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298E20 size=44
    let mut pc: u32 = 0x83298E20;
    'dispatch: loop {
        match pc {
            0x83298E20 => {
    //   block [0x83298E20..0x83298E4C)
	// 83298E20: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298E24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298E28: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83298E2C: 386958C8  addi r3, r9, 0x58c8
	ctx.r[3].s64 = ctx.r[9].s64 + 22728;
	// 83298E30: 38CBDAA8  addi r6, r11, -0x2558
	ctx.r[6].s64 = ctx.r[11].s64 + -9560;
	// 83298E34: 388A34A8  addi r4, r10, 0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + 13480;
	// 83298E38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298E40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298E48: 4B9CB070  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298E50 size=44
    let mut pc: u32 = 0x83298E50;
    'dispatch: loop {
        match pc {
            0x83298E50 => {
    //   block [0x83298E50..0x83298E7C)
	// 83298E50: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298E54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298E58: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298E5C: 38695820  addi r3, r9, 0x5820
	ctx.r[3].s64 = ctx.r[9].s64 + 22560;
	// 83298E60: 38CBDAA8  addi r6, r11, -0x2558
	ctx.r[6].s64 = ctx.r[11].s64 + -9560;
	// 83298E64: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83298E68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298E70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298E78: 4B9CB040  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298E80 size=56
    let mut pc: u32 = 0x83298E80;
    'dispatch: loop {
        match pc {
            0x83298E80 => {
    //   block [0x83298E80..0x83298EB8)
	// 83298E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298E8C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298E90: 3C800010  lis r4, 0x10
	ctx.r[4].s64 = 1048576;
	// 83298E94: 386B597C  addi r3, r11, 0x597c
	ctx.r[3].s64 = ctx.r[11].s64 + 22908;
	// 83298E98: 4B9D18E1  bl 0x82c6a778
	ctx.lr = 0x83298E9C;
	sub_82C6A778(ctx, base);
	// 83298E9C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298EA0: 386A7BF0  addi r3, r10, 0x7bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 31728;
	// 83298EA4: 4BA1107D  bl 0x82ca9f20
	ctx.lr = 0x83298EA8;
	sub_82CA9F20(ctx, base);
	// 83298EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298EB8 size=52
    let mut pc: u32 = 0x83298EB8;
    'dispatch: loop {
        match pc {
            0x83298EB8 => {
    //   block [0x83298EB8..0x83298EEC)
	// 83298EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298EC4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298EC8: 386B5990  addi r3, r11, 0x5990
	ctx.r[3].s64 = ctx.r[11].s64 + 22928;
	// 83298ECC: 4B9D6865  bl 0x82c6f730
	ctx.lr = 0x83298ED0;
	sub_82C6F730(ctx, base);
	// 83298ED0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298ED4: 386A7C88  addi r3, r10, 0x7c88
	ctx.r[3].s64 = ctx.r[10].s64 + 31880;
	// 83298ED8: 4BA11049  bl 0x82ca9f20
	ctx.lr = 0x83298EDC;
	sub_82CA9F20(ctx, base);
	// 83298EDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298EF0 size=52
    let mut pc: u32 = 0x83298EF0;
    'dispatch: loop {
        match pc {
            0x83298EF0 => {
    //   block [0x83298EF0..0x83298F24)
	// 83298EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298EFC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298F00: 386B5998  addi r3, r11, 0x5998
	ctx.r[3].s64 = ctx.r[11].s64 + 22936;
	// 83298F04: 4AF5E8FD  bl 0x821f7800
	ctx.lr = 0x83298F08;
	sub_821F7800(ctx, base);
	// 83298F08: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298F0C: 386A7C18  addi r3, r10, 0x7c18
	ctx.r[3].s64 = ctx.r[10].s64 + 31768;
	// 83298F10: 4BA11011  bl 0x82ca9f20
	ctx.lr = 0x83298F14;
	sub_82CA9F20(ctx, base);
	// 83298F14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298F28 size=52
    let mut pc: u32 = 0x83298F28;
    'dispatch: loop {
        match pc {
            0x83298F28 => {
    //   block [0x83298F28..0x83298F5C)
	// 83298F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298F34: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298F38: 386B5994  addi r3, r11, 0x5994
	ctx.r[3].s64 = ctx.r[11].s64 + 22932;
	// 83298F3C: 4AF5E8C5  bl 0x821f7800
	ctx.lr = 0x83298F40;
	sub_821F7800(ctx, base);
	// 83298F40: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298F44: 386A7C28  addi r3, r10, 0x7c28
	ctx.r[3].s64 = ctx.r[10].s64 + 31784;
	// 83298F48: 4BA10FD9  bl 0x82ca9f20
	ctx.lr = 0x83298F4C;
	sub_82CA9F20(ctx, base);
	// 83298F4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298F60 size=148
    let mut pc: u32 = 0x83298F60;
    'dispatch: loop {
        match pc {
            0x83298F60 => {
    //   block [0x83298F60..0x83298FB4)
	// 83298F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298F68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83298F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298F70: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83298F74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83298F78: 3BEBE8D4  addi r31, r11, -0x172c
	ctx.r[31].s64 = ctx.r[11].s64 + -5932;
	// 83298F7C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83298F80: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83298F84: 4B8A492D  bl 0x82b3d8b0
	ctx.lr = 0x83298F88;
	sub_82B3D8B0(ctx, base);
	// 83298F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83298F8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83298F90: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83298F94: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83298F98: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83298F9C: 4B9DDB5D  bl 0x82c76af8
	ctx.lr = 0x83298FA0;
	sub_82C76AF8(ctx, base);
	// 83298FA0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83298FA4: 4AF862B5  bl 0x8221f258
	ctx.lr = 0x83298FA8;
	sub_8221F258(ctx, base);
	// 83298FA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83298FAC: 419A0008  beq cr6, 0x83298fb4
	if ctx.cr[6].eq {
	pc = 0x83298FB4; continue 'dispatch;
	}
	// 83298FB0: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83298FB4; continue 'dispatch;
            }
            0x83298FB4 => {
    //   block [0x83298FB4..0x83298FC0)
	// 83298FB4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83298FB8: 41820008  beq 0x83298fc0
	if ctx.cr[0].eq {
	pc = 0x83298FC0; continue 'dispatch;
	}
	// 83298FBC: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83298FC0; continue 'dispatch;
            }
            0x83298FC0 => {
    //   block [0x83298FC0..0x83298FF4)
	// 83298FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83298FC4: 907F0044  stw r3, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 83298FC8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83298FCC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83298FD0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83298FD4: 915F004C  stw r10, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83298FD8: 386B7CC0  addi r3, r11, 0x7cc0
	ctx.r[3].s64 = ctx.r[11].s64 + 31936;
	// 83298FDC: 4BA10F45  bl 0x82ca9f20
	ctx.lr = 0x83298FE0;
	sub_82CA9F20(ctx, base);
	// 83298FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83298FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298FF8 size=52
    let mut pc: u32 = 0x83298FF8;
    'dispatch: loop {
        match pc {
            0x83298FF8 => {
    //   block [0x83298FF8..0x8329902C)
	// 83298FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299004: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83299008: 386B59AC  addi r3, r11, 0x59ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22956;
	// 8329900C: 4B8A48A5  bl 0x82b3d8b0
	ctx.lr = 0x83299010;
	sub_82B3D8B0(ctx, base);
	// 83299010: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83299014: 386A7CD0  addi r3, r10, 0x7cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 31952;
	// 83299018: 4BA10F09  bl 0x82ca9f20
	ctx.lr = 0x8329901C;
	sub_82CA9F20(ctx, base);
	// 8329901C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83299020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299030 size=4
    let mut pc: u32 = 0x83299030;
    'dispatch: loop {
        match pc {
            0x83299030 => {
    //   block [0x83299030..0x83299034)
	// 83299030: 4B9EEFC0  b 0x82c87ff0
	sub_82C87FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299038 size=12
    let mut pc: u32 = 0x83299038;
    'dispatch: loop {
        match pc {
            0x83299038 => {
    //   block [0x83299038..0x83299044)
	// 83299038: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329903C: 386B7D90  addi r3, r11, 0x7d90
	ctx.r[3].s64 = ctx.r[11].s64 + 32144;
	// 83299040: 4BA10EE0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299048 size=12
    let mut pc: u32 = 0x83299048;
    'dispatch: loop {
        match pc {
            0x83299048 => {
    //   block [0x83299048..0x83299054)
	// 83299048: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329904C: 386B7D38  addi r3, r11, 0x7d38
	ctx.r[3].s64 = ctx.r[11].s64 + 32056;
	// 83299050: 4BA10ED0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299058 size=12
    let mut pc: u32 = 0x83299058;
    'dispatch: loop {
        match pc {
            0x83299058 => {
    //   block [0x83299058..0x83299064)
	// 83299058: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329905C: 386B7CE0  addi r3, r11, 0x7ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 31968;
	// 83299060: 4BA10EC0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299068 size=12
    let mut pc: u32 = 0x83299068;
    'dispatch: loop {
        match pc {
            0x83299068 => {
    //   block [0x83299068..0x83299074)
	// 83299068: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329906C: 386B7E08  addi r3, r11, 0x7e08
	ctx.r[3].s64 = ctx.r[11].s64 + 32264;
	// 83299070: 4BA10EB0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299078 size=52
    let mut pc: u32 = 0x83299078;
    'dispatch: loop {
        match pc {
            0x83299078 => {
    //   block [0x83299078..0x832990AC)
	// 83299078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299084: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83299088: 386B71D0  addi r3, r11, 0x71d0
	ctx.r[3].s64 = ctx.r[11].s64 + 29136;
	// 8329908C: 4BA3F495  bl 0x82cd8520
	ctx.lr = 0x83299090;
	sub_82CD8520(ctx, base);
	// 83299090: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83299094: 386B7E58  addi r3, r11, 0x7e58
	ctx.r[3].s64 = ctx.r[11].s64 + 32344;
	// 83299098: 4BA10E89  bl 0x82ca9f20
	ctx.lr = 0x8329909C;
	sub_82CA9F20(ctx, base);
	// 8329909C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832990A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832990A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832990A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832990B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832990B0 size=52
    let mut pc: u32 = 0x832990B0;
    'dispatch: loop {
        match pc {
            0x832990B0 => {
    //   block [0x832990B0..0x832990E4)
	// 832990B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832990B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832990B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832990BC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832990C0: 386B721C  addi r3, r11, 0x721c
	ctx.r[3].s64 = ctx.r[11].s64 + 29212;
	// 832990C4: 4BA3F45D  bl 0x82cd8520
	ctx.lr = 0x832990C8;
	sub_82CD8520(ctx, base);
	// 832990C8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832990CC: 386B7E68  addi r3, r11, 0x7e68
	ctx.r[3].s64 = ctx.r[11].s64 + 32360;
	// 832990D0: 4BA10E51  bl 0x82ca9f20
	ctx.lr = 0x832990D4;
	sub_82CA9F20(ctx, base);
	// 832990D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832990D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832990DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832990E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832990E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832990E8 size=12
    let mut pc: u32 = 0x832990E8;
    'dispatch: loop {
        match pc {
            0x832990E8 => {
    //   block [0x832990E8..0x832990F4)
	// 832990E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832990EC: 386B7E78  addi r3, r11, 0x7e78
	ctx.r[3].s64 = ctx.r[11].s64 + 32376;
	// 832990F0: 4BA10E30  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832990F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832990F8 size=56
    let mut pc: u32 = 0x832990F8;
    'dispatch: loop {
        match pc {
            0x832990F8 => {
    //   block [0x832990F8..0x83299130)
	// 832990F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832990FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299104: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299108: 396B0440  addi r11, r11, 0x440
	ctx.r[11].s64 = ctx.r[11].s64 + 1088;
	// 8329910C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83299110: 48020B75  bl 0x832b9c84
	ctx.lr = 0x83299114;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83299114: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83299118: 386A7E88  addi r3, r10, 0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + 32392;
	// 8329911C: 4BA10E05  bl 0x82ca9f20
	ctx.lr = 0x83299120;
	sub_82CA9F20(ctx, base);
	// 83299120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83299124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329912C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299130 size=56
    let mut pc: u32 = 0x83299130;
    'dispatch: loop {
        match pc {
            0x83299130 => {
    //   block [0x83299130..0x83299168)
	// 83299130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329913C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299140: 396B0460  addi r11, r11, 0x460
	ctx.r[11].s64 = ctx.r[11].s64 + 1120;
	// 83299144: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83299148: 48020B3D  bl 0x832b9c84
	ctx.lr = 0x8329914C;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8329914C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83299150: 386A7EA0  addi r3, r10, 0x7ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 32416;
	// 83299154: 4BA10DCD  bl 0x82ca9f20
	ctx.lr = 0x83299158;
	sub_82CA9F20(ctx, base);
	// 83299158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329915C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299168 size=56
    let mut pc: u32 = 0x83299168;
    'dispatch: loop {
        match pc {
            0x83299168 => {
    //   block [0x83299168..0x832991A0)
	// 83299168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329916C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299174: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299178: 396B04AC  addi r11, r11, 0x4ac
	ctx.r[11].s64 = ctx.r[11].s64 + 1196;
	// 8329917C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83299180: 48020B05  bl 0x832b9c84
	ctx.lr = 0x83299184;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83299184: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83299188: 386B7EB8  addi r3, r11, 0x7eb8
	ctx.r[3].s64 = ctx.r[11].s64 + 32440;
	// 8329918C: 4BA10D95  bl 0x82ca9f20
	ctx.lr = 0x83299190;
	sub_82CA9F20(ctx, base);
	// 83299190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83299194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329919C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832991A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832991A0 size=20
    let mut pc: u32 = 0x832991A0;
    'dispatch: loop {
        match pc {
            0x832991A0 => {
    //   block [0x832991A0..0x832991B4)
	// 832991A0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832991A4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 832991A8: 386B72F8  addi r3, r11, 0x72f8
	ctx.r[3].s64 = ctx.r[11].s64 + 29432;
	// 832991AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832991B0: 4BA10800  b 0x82ca99b0
	sub_82CA99B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832991B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832991B8 size=28
    let mut pc: u32 = 0x832991B8;
    'dispatch: loop {
        match pc {
            0x832991B8 => {
    //   block [0x832991B8..0x832991D4)
	// 832991B8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832991BC: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832991C0: 396B6E4C  addi r11, r11, 0x6e4c
	ctx.r[11].s64 = ctx.r[11].s64 + 28236;
	// 832991C4: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832991C8: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832991CC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832991D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832991D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832991D8 size=56
    let mut pc: u32 = 0x832991D8;
    'dispatch: loop {
        match pc {
            0x832991D8 => {
    //   block [0x832991D8..0x83299210)
	// 832991D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832991DC: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832991E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832991E4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832991E8: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832991EC: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832991F0: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832991F4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832991F8: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832991FC: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83299200: 396B7420  addi r11, r11, 0x7420
	ctx.r[11].s64 = ctx.r[11].s64 + 29728;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299210 size=528
    let mut pc: u32 = 0x83299210;
    'dispatch: loop {
        match pc {
            0x83299210 => {
    //   block [0x83299210..0x83299420)
	// 83299210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299214: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83299218: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 8329921C: 39297430  addi r9, r9, 0x7430
	ctx.r[9].s64 = ctx.r[9].s64 + 29744;
	// 83299220: 9141FF20  stw r10, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[10].u32 ) };
	// 83299224: 9141FF24  stw r10, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[10].u32 ) };
	// 83299228: 9141FF28  stw r10, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[10].u32 ) };
	// 8329922C: 9141FF2C  stw r10, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[10].u32 ) };
	// 83299230: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 83299234: 9141FF30  stw r10, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[10].u32 ) };
	// 83299238: 9141FF34  stw r10, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[10].u32 ) };
	// 8329923C: 9141FF38  stw r10, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[10].u32 ) };
	// 83299240: 9161FF3C  stw r11, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299420 size=528
    let mut pc: u32 = 0x83299420;
    'dispatch: loop {
        match pc {
            0x83299420 => {
    //   block [0x83299420..0x83299630)
	// 83299420: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83299424: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299428: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 8329942C: 39297530  addi r9, r9, 0x7530
	ctx.r[9].s64 = ctx.r[9].s64 + 30000;
	// 83299430: 9161FF20  stw r11, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[11].u32 ) };
	// 83299434: 9161FF24  stw r11, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[11].u32 ) };
	// 83299438: 9161FF28  stw r11, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[11].u32 ) };
	// 8329943C: 9161FF2C  stw r11, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[11].u32 ) };
	// 83299440: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 83299444: 9161FF30  stw r11, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[11].u32 ) };
	// 83299448: 9161FF34  stw r11, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[11].u32 ) };
	// 8329944C: 9161FF38  stw r11, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[11].u32 ) };
	// 83299450: 9141FF3C  stw r10, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299630 size=24
    let mut pc: u32 = 0x83299630;
    'dispatch: loop {
        match pc {
            0x83299630 => {
    //   block [0x83299630..0x83299648)
	// 83299630: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299634: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83299638: 394A6F38  addi r10, r10, 0x6f38
	ctx.r[10].s64 = ctx.r[10].s64 + 28472;
	// 8329963C: 816B6F24  lwz r11, 0x6f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28452 as u32) ) } as u64;
	// 83299640: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 83299644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299648 size=112
    let mut pc: u32 = 0x83299648;
    'dispatch: loop {
        match pc {
            0x83299648 => {
    //   block [0x83299648..0x832996B8)
	// 83299648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329964C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299658: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329965C: 392A3CB0  addi r9, r10, 0x3cb0
	ctx.r[9].s64 = ctx.r[10].s64 + 15536;
	// 83299660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299664: 390B6F38  addi r8, r11, 0x6f38
	ctx.r[8].s64 = ctx.r[11].s64 + 28472;
	// 83299668: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8329966C: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 83299670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299674: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329967C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83299680: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 83299684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299688: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329968C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329969C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832996A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832996A4: 4BABC59D  bl 0x82d55c40
	ctx.lr = 0x832996A8;
	sub_82D55C40(ctx, base);
	// 832996A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832996AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832996B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832996B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832996B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832996B8 size=108
    let mut pc: u32 = 0x832996B8;
    'dispatch: loop {
        match pc {
            0x832996B8 => {
    //   block [0x832996B8..0x83299724)
	// 832996B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832996BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832996C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832996C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 832996C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832996CC: 38EB3D18  addi r7, r11, 0x3d18
	ctx.r[7].s64 = ctx.r[11].s64 + 15640;
	// 832996D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832996D4: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832996D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832996DC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832996E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832996E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832996E8: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 832996EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832996F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832996F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832996F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832996FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299704: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83299708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329970C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299710: 4BABC531  bl 0x82d55c40
	ctx.lr = 0x83299714;
	sub_82D55C40(ctx, base);
	// 83299714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329971C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299728 size=24
    let mut pc: u32 = 0x83299728;
    'dispatch: loop {
        match pc {
            0x83299728 => {
    //   block [0x83299728..0x83299740)
	// 83299728: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329972C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83299730: 394A7050  addi r10, r10, 0x7050
	ctx.r[10].s64 = ctx.r[10].s64 + 28752;
	// 83299734: 816B7028  lwz r11, 0x7028(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28712 as u32) ) } as u64;
	// 83299738: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329973C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299740 size=112
    let mut pc: u32 = 0x83299740;
    'dispatch: loop {
        match pc {
            0x83299740 => {
    //   block [0x83299740..0x832997B0)
	// 83299740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329974C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299750: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299754: 392A3D04  addi r9, r10, 0x3d04
	ctx.r[9].s64 = ctx.r[10].s64 + 15620;
	// 83299758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329975C: 390B7050  addi r8, r11, 0x7050
	ctx.r[8].s64 = ctx.r[11].s64 + 28752;
	// 83299760: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83299764: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 83299768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329976C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299774: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83299778: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 8329977C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299780: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83299784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329978C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329979C: 4BABC4A5  bl 0x82d55c40
	ctx.lr = 0x832997A0;
	sub_82D55C40(ctx, base);
	// 832997A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832997A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832997A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832997AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832997B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832997B0 size=40
    let mut pc: u32 = 0x832997B0;
    'dispatch: loop {
        match pc {
            0x832997B0 => {
    //   block [0x832997B0..0x832997D8)
	// 832997B0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832997B4: 814B70B0  lwz r10, 0x70b0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28848 as u32) ) } as u64;
	// 832997B8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832997BC: 396B70D0  addi r11, r11, 0x70d0
	ctx.r[11].s64 = ctx.r[11].s64 + 28880;
	// 832997C0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 832997C4: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 832997C8: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 832997CC: 814A70B4  lwz r10, 0x70b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28852 as u32) ) } as u64;
	// 832997D0: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 832997D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832997D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832997D8 size=112
    let mut pc: u32 = 0x832997D8;
    'dispatch: loop {
        match pc {
            0x832997D8 => {
    //   block [0x832997D8..0x83299848)
	// 832997D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832997DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832997E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832997E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832997E8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832997EC: 392A4178  addi r9, r10, 0x4178
	ctx.r[9].s64 = ctx.r[10].s64 + 16760;
	// 832997F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832997F4: 390B70D0  addi r8, r11, 0x70d0
	ctx.r[8].s64 = ctx.r[11].s64 + 28880;
	// 832997F8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832997FC: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 83299800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299804: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329980C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83299810: 386A76C4  addi r3, r10, 0x76c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30404;
	// 83299814: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299818: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8329981C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329982C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299834: 4BABC40D  bl 0x82d55c40
	ctx.lr = 0x83299838;
	sub_82D55C40(ctx, base);
	// 83299838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329983C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299848 size=28
    let mut pc: u32 = 0x83299848;
    'dispatch: loop {
        match pc {
            0x83299848 => {
    //   block [0x83299848..0x83299864)
	// 83299848: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329984C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299850: 396B7368  addi r11, r11, 0x7368
	ctx.r[11].s64 = ctx.r[11].s64 + 29544;
	// 83299854: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 83299858: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329985C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83299860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299868 size=108
    let mut pc: u32 = 0x83299868;
    'dispatch: loop {
        match pc {
            0x83299868 => {
    //   block [0x83299868..0x832998D4)
	// 83299868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329986C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299874: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329987C: 38EB4F8C  addi r7, r11, 0x4f8c
	ctx.r[7].s64 = ctx.r[11].s64 + 20364;
	// 83299880: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 83299884: 388A4FD4  addi r4, r10, 0x4fd4
	ctx.r[4].s64 = ctx.r[10].s64 + 20436;
	// 83299888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329988C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299890: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299898: 386A7704  addi r3, r10, 0x7704
	ctx.r[3].s64 = ctx.r[10].s64 + 30468;
	// 8329989C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832998A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832998A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832998A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832998AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832998B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832998B4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832998B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832998BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832998C0: 4BABC381  bl 0x82d55c40
	ctx.lr = 0x832998C4;
	sub_82D55C40(ctx, base);
	// 832998C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832998C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832998CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832998D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832998D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832998D8 size=108
    let mut pc: u32 = 0x832998D8;
    'dispatch: loop {
        match pc {
            0x832998D8 => {
    //   block [0x832998D8..0x83299944)
	// 832998D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832998DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832998E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832998E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 832998E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832998EC: 38EB4FBC  addi r7, r11, 0x4fbc
	ctx.r[7].s64 = ctx.r[11].s64 + 20412;
	// 832998F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832998F4: 388A4FF0  addi r4, r10, 0x4ff0
	ctx.r[4].s64 = ctx.r[10].s64 + 20464;
	// 832998F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832998FC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299908: 386A7734  addi r3, r10, 0x7734
	ctx.r[3].s64 = ctx.r[10].s64 + 30516;
	// 8329990C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329991C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299924: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83299928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329992C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299930: 4BABC311  bl 0x82d55c40
	ctx.lr = 0x83299934;
	sub_82D55C40(ctx, base);
	// 83299934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329993C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299948 size=28
    let mut pc: u32 = 0x83299948;
    'dispatch: loop {
        match pc {
            0x83299948 => {
    //   block [0x83299948..0x83299964)
	// 83299948: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329994C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299950: 396B7718  addi r11, r11, 0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + 30488;
	// 83299954: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 83299958: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329995C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83299960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299968 size=28
    let mut pc: u32 = 0x83299968;
    'dispatch: loop {
        match pc {
            0x83299968 => {
    //   block [0x83299968..0x83299984)
	// 83299968: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329996C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299970: 396B772C  addi r11, r11, 0x772c
	ctx.r[11].s64 = ctx.r[11].s64 + 30508;
	// 83299974: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 83299978: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329997C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83299980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299988 size=108
    let mut pc: u32 = 0x83299988;
    'dispatch: loop {
        match pc {
            0x83299988 => {
    //   block [0x83299988..0x832999F4)
	// 83299988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299994: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329999C: 38EB5240  addi r7, r11, 0x5240
	ctx.r[7].s64 = ctx.r[11].s64 + 21056;
	// 832999A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832999A4: 388A52B8  addi r4, r10, 0x52b8
	ctx.r[4].s64 = ctx.r[10].s64 + 21176;
	// 832999A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832999AC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832999B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832999B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832999B8: 386A7774  addi r3, r10, 0x7774
	ctx.r[3].s64 = ctx.r[10].s64 + 30580;
	// 832999BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832999C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832999C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832999C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832999CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832999D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832999D4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832999D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832999DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832999E0: 4BABC261  bl 0x82d55c40
	ctx.lr = 0x832999E4;
	sub_82D55C40(ctx, base);
	// 832999E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832999E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832999EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832999F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832999F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832999F8 size=108
    let mut pc: u32 = 0x832999F8;
    'dispatch: loop {
        match pc {
            0x832999F8 => {
    //   block [0x832999F8..0x83299A64)
	// 832999F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832999FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299A04: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299A0C: 38EB5258  addi r7, r11, 0x5258
	ctx.r[7].s64 = ctx.r[11].s64 + 21080;
	// 83299A10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 83299A14: 388A52D0  addi r4, r10, 0x52d0
	ctx.r[4].s64 = ctx.r[10].s64 + 21200;
	// 83299A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299A1C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299A28: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 83299A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299A44: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83299A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299A50: 4BABC1F1  bl 0x82d55c40
	ctx.lr = 0x83299A54;
	sub_82D55C40(ctx, base);
	// 83299A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299A68 size=108
    let mut pc: u32 = 0x83299A68;
    'dispatch: loop {
        match pc {
            0x83299A68 => {
    //   block [0x83299A68..0x83299AD4)
	// 83299A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299A74: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299A7C: 38EB5308  addi r7, r11, 0x5308
	ctx.r[7].s64 = ctx.r[11].s64 + 21256;
	// 83299A80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 83299A84: 388A5368  addi r4, r10, 0x5368
	ctx.r[4].s64 = ctx.r[10].s64 + 21352;
	// 83299A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299A8C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299A98: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 83299A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299AB4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 83299AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299AC0: 4BABC181  bl 0x82d55c40
	ctx.lr = 0x83299AC4;
	sub_82D55C40(ctx, base);
	// 83299AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299AD8 size=108
    let mut pc: u32 = 0x83299AD8;
    'dispatch: loop {
        match pc {
            0x83299AD8 => {
    //   block [0x83299AD8..0x83299B44)
	// 83299AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299AE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299AEC: 38EB5450  addi r7, r11, 0x5450
	ctx.r[7].s64 = ctx.r[11].s64 + 21584;
	// 83299AF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 83299AF4: 388A55B8  addi r4, r10, 0x55b8
	ctx.r[4].s64 = ctx.r[10].s64 + 21944;
	// 83299AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299AFC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299B08: 386A7804  addi r3, r10, 0x7804
	ctx.r[3].s64 = ctx.r[10].s64 + 30724;
	// 83299B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299B24: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83299B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299B30: 4BABC111  bl 0x82d55c40
	ctx.lr = 0x83299B34;
	sub_82D55C40(ctx, base);
	// 83299B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299B48 size=108
    let mut pc: u32 = 0x83299B48;
    'dispatch: loop {
        match pc {
            0x83299B48 => {
    //   block [0x83299B48..0x83299BB4)
	// 83299B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299B54: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299B58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299B5C: 38EB5498  addi r7, r11, 0x5498
	ctx.r[7].s64 = ctx.r[11].s64 + 21656;
	// 83299B60: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299B64: 388A55DC  addi r4, r10, 0x55dc
	ctx.r[4].s64 = ctx.r[10].s64 + 21980;
	// 83299B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299B6C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299B70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299B78: 386A7834  addi r3, r10, 0x7834
	ctx.r[3].s64 = ctx.r[10].s64 + 30772;
	// 83299B7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299B94: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83299B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299BA0: 4BABC0A1  bl 0x82d55c40
	ctx.lr = 0x83299BA4;
	sub_82D55C40(ctx, base);
	// 83299BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299BB8 size=108
    let mut pc: u32 = 0x83299BB8;
    'dispatch: loop {
        match pc {
            0x83299BB8 => {
    //   block [0x83299BB8..0x83299C24)
	// 83299BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299BC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299BCC: 38EB5528  addi r7, r11, 0x5528
	ctx.r[7].s64 = ctx.r[11].s64 + 21800;
	// 83299BD0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299BD4: 388A5600  addi r4, r10, 0x5600
	ctx.r[4].s64 = ctx.r[10].s64 + 22016;
	// 83299BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299BDC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299BE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299BE8: 386A7864  addi r3, r10, 0x7864
	ctx.r[3].s64 = ctx.r[10].s64 + 30820;
	// 83299BEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299C04: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83299C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299C10: 4BABC031  bl 0x82d55c40
	ctx.lr = 0x83299C14;
	sub_82D55C40(ctx, base);
	// 83299C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299C28 size=108
    let mut pc: u32 = 0x83299C28;
    'dispatch: loop {
        match pc {
            0x83299C28 => {
    //   block [0x83299C28..0x83299C94)
	// 83299C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299C34: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299C3C: 38EB5798  addi r7, r11, 0x5798
	ctx.r[7].s64 = ctx.r[11].s64 + 22424;
	// 83299C40: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 83299C44: 388A5A20  addi r4, r10, 0x5a20
	ctx.r[4].s64 = ctx.r[10].s64 + 23072;
	// 83299C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299C4C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299C58: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 83299C5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299C74: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83299C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299C80: 4BABBFC1  bl 0x82d55c40
	ctx.lr = 0x83299C84;
	sub_82D55C40(ctx, base);
	// 83299C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299C98 size=108
    let mut pc: u32 = 0x83299C98;
    'dispatch: loop {
        match pc {
            0x83299C98 => {
    //   block [0x83299C98..0x83299D04)
	// 83299C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299CA4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299CAC: 38EB5870  addi r7, r11, 0x5870
	ctx.r[7].s64 = ctx.r[11].s64 + 22640;
	// 83299CB0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299CB4: 388A5A50  addi r4, r10, 0x5a50
	ctx.r[4].s64 = ctx.r[10].s64 + 23120;
	// 83299CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299CBC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299CC8: 386A78C4  addi r3, r10, 0x78c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30916;
	// 83299CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299CE4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83299CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299CF0: 4BABBF51  bl 0x82d55c40
	ctx.lr = 0x83299CF4;
	sub_82D55C40(ctx, base);
	// 83299CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299D08 size=112
    let mut pc: u32 = 0x83299D08;
    'dispatch: loop {
        match pc {
            0x83299D08 => {
    //   block [0x83299D08..0x83299D78)
	// 83299D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299D14: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299D18: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299D1C: 38AA7B64  addi r5, r10, 0x7b64
	ctx.r[5].s64 = ctx.r[10].s64 + 31588;
	// 83299D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299D24: 390B5900  addi r8, r11, 0x5900
	ctx.r[8].s64 = ctx.r[11].s64 + 22784;
	// 83299D28: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 83299D2C: 388A5A80  addi r4, r10, 0x5a80
	ctx.r[4].s64 = ctx.r[10].s64 + 23168;
	// 83299D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299D34: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299D40: 386A78F4  addi r3, r10, 0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30964;
	// 83299D44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299D5C: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 83299D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299D64: 4BABBEDD  bl 0x82d55c40
	ctx.lr = 0x83299D68;
	sub_82D55C40(ctx, base);
	// 83299D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299D78 size=112
    let mut pc: u32 = 0x83299D78;
    'dispatch: loop {
        match pc {
            0x83299D78 => {
    //   block [0x83299D78..0x83299DE8)
	// 83299D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299D84: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299D88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299D8C: 38AA7B34  addi r5, r10, 0x7b34
	ctx.r[5].s64 = ctx.r[10].s64 + 31540;
	// 83299D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299D94: 390B5AD0  addi r8, r11, 0x5ad0
	ctx.r[8].s64 = ctx.r[11].s64 + 23248;
	// 83299D98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 83299D9C: 388A5B4C  addi r4, r10, 0x5b4c
	ctx.r[4].s64 = ctx.r[10].s64 + 23372;
	// 83299DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299DA4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299DA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299DB0: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 83299DB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299DCC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83299DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299DD4: 4BABBE6D  bl 0x82d55c40
	ctx.lr = 0x83299DD8;
	sub_82D55C40(ctx, base);
	// 83299DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299DE8 size=108
    let mut pc: u32 = 0x83299DE8;
    'dispatch: loop {
        match pc {
            0x83299DE8 => {
    //   block [0x83299DE8..0x83299E54)
	// 83299DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299DF4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299DFC: 38EB5BB0  addi r7, r11, 0x5bb0
	ctx.r[7].s64 = ctx.r[11].s64 + 23472;
	// 83299E00: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299E04: 388A5D90  addi r4, r10, 0x5d90
	ctx.r[4].s64 = ctx.r[10].s64 + 23952;
	// 83299E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299E0C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299E10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299E18: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 83299E1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299E34: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83299E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299E3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299E40: 4BABBE01  bl 0x82d55c40
	ctx.lr = 0x83299E44;
	sub_82D55C40(ctx, base);
	// 83299E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299E58 size=112
    let mut pc: u32 = 0x83299E58;
    'dispatch: loop {
        match pc {
            0x83299E58 => {
    //   block [0x83299E58..0x83299EC8)
	// 83299E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299E64: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299E68: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299E6C: 38AA7B64  addi r5, r10, 0x7b64
	ctx.r[5].s64 = ctx.r[10].s64 + 31588;
	// 83299E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299E74: 390B5C40  addi r8, r11, 0x5c40
	ctx.r[8].s64 = ctx.r[11].s64 + 23616;
	// 83299E78: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 83299E7C: 388A5DC8  addi r4, r10, 0x5dc8
	ctx.r[4].s64 = ctx.r[10].s64 + 24008;
	// 83299E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299E84: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299E90: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 83299E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299EAC: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 83299EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299EB4: 4BABBD8D  bl 0x82d55c40
	ctx.lr = 0x83299EB8;
	sub_82D55C40(ctx, base);
	// 83299EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299EC8 size=112
    let mut pc: u32 = 0x83299EC8;
    'dispatch: loop {
        match pc {
            0x83299EC8 => {
    //   block [0x83299EC8..0x83299F38)
	// 83299EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299ED4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299ED8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299EDC: 38AA7B64  addi r5, r10, 0x7b64
	ctx.r[5].s64 = ctx.r[10].s64 + 31588;
	// 83299EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299EE4: 390B5DF8  addi r8, r11, 0x5df8
	ctx.r[8].s64 = ctx.r[11].s64 + 24056;
	// 83299EE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83299EEC: 388A5E40  addi r4, r10, 0x5e40
	ctx.r[4].s64 = ctx.r[10].s64 + 24128;
	// 83299EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299EF4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299F00: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 83299F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299F1C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 83299F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299F24: 4BABBD1D  bl 0x82d55c40
	ctx.lr = 0x83299F28;
	sub_82D55C40(ctx, base);
	// 83299F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299F38 size=112
    let mut pc: u32 = 0x83299F38;
    'dispatch: loop {
        match pc {
            0x83299F38 => {
    //   block [0x83299F38..0x83299FA8)
	// 83299F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299F44: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299F48: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 83299F4C: 392B5EBC  addi r9, r11, 0x5ebc
	ctx.r[9].s64 = ctx.r[11].s64 + 24252;
	// 83299F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299F54: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 83299F58: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83299F5C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 83299F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299F64: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299F68: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 83299F6C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299F70: 388A5EF8  addi r4, r10, 0x5ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 24312;
	// 83299F74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299F78: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83299F7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299F80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83299F84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299F88: 386B79E4  addi r3, r11, 0x79e4
	ctx.r[3].s64 = ctx.r[11].s64 + 31204;
	// 83299F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299F90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299F94: 4BABBCAD  bl 0x82d55c40
	ctx.lr = 0x83299F98;
	sub_82D55C40(ctx, base);
	// 83299F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299FA8 size=24
    let mut pc: u32 = 0x83299FA8;
    'dispatch: loop {
        match pc {
            0x83299FA8 => {
    //   block [0x83299FA8..0x83299FC0)
	// 83299FA8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299FAC: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83299FB0: 394A79F8  addi r10, r10, 0x79f8
	ctx.r[10].s64 = ctx.r[10].s64 + 31224;
	// 83299FB4: 816B79E0  lwz r11, 0x79e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31200 as u32) ) } as u64;
	// 83299FB8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83299FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299FC0 size=112
    let mut pc: u32 = 0x83299FC0;
    'dispatch: loop {
        match pc {
            0x83299FC0 => {
    //   block [0x83299FC0..0x8329A030)
	// 83299FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299FCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299FD0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299FD4: 392A5F68  addi r9, r10, 0x5f68
	ctx.r[9].s64 = ctx.r[10].s64 + 24424;
	// 83299FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299FDC: 390B79F8  addi r8, r11, 0x79f8
	ctx.r[8].s64 = ctx.r[11].s64 + 31224;
	// 83299FE0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 83299FE4: 388A5F7C  addi r4, r10, 0x5f7c
	ctx.r[4].s64 = ctx.r[10].s64 + 24444;
	// 83299FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299FEC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299FF4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83299FF8: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 83299FFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329A000: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329A004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A01C: 4BABBC25  bl 0x82d55c40
	ctx.lr = 0x8329A020;
	sub_82D55C40(ctx, base);
	// 8329A020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A030 size=108
    let mut pc: u32 = 0x8329A030;
    'dispatch: loop {
        match pc {
            0x8329A030 => {
    //   block [0x8329A030..0x8329A09C)
	// 8329A030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A03C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A044: 38EB5FC8  addi r7, r11, 0x5fc8
	ctx.r[7].s64 = ctx.r[11].s64 + 24520;
	// 8329A048: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8329A04C: 388A6040  addi r4, r10, 0x6040
	ctx.r[4].s64 = ctx.r[10].s64 + 24640;
	// 8329A050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A054: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A060: 386A7A44  addi r3, r10, 0x7a44
	ctx.r[3].s64 = ctx.r[10].s64 + 31300;
	// 8329A064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A07C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329A080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A088: 4BABBBB9  bl 0x82d55c40
	ctx.lr = 0x8329A08C;
	sub_82D55C40(ctx, base);
	// 8329A08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


