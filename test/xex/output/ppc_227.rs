pub fn sub_832940E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832940E0 size=12
    let mut pc: u32 = 0x832940E0;
    'dispatch: loop {
        match pc {
            0x832940E0 => {
    //   block [0x832940E0..0x832940EC)
	// 832940E0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832940E4: 386B70A8  addi r3, r11, 0x70a8
	ctx.r[3].s64 = ctx.r[11].s64 + 28840;
	// 832940E8: 4BA15E38  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832940F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832940F0 size=12
    let mut pc: u32 = 0x832940F0;
    'dispatch: loop {
        match pc {
            0x832940F0 => {
    //   block [0x832940F0..0x832940FC)
	// 832940F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832940F4: 386B70B8  addi r3, r11, 0x70b8
	ctx.r[3].s64 = ctx.r[11].s64 + 28856;
	// 832940F8: 4BA15E28  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294100 size=52
    let mut pc: u32 = 0x83294100;
    'dispatch: loop {
        match pc {
            0x83294100 => {
    //   block [0x83294100..0x83294134)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294138 size=52
    let mut pc: u32 = 0x83294138;
    'dispatch: loop {
        match pc {
            0x83294138 => {
    //   block [0x83294138..0x8329416C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294170 size=52
    let mut pc: u32 = 0x83294170;
    'dispatch: loop {
        match pc {
            0x83294170 => {
    //   block [0x83294170..0x832941A4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832941A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832941A8 size=52
    let mut pc: u32 = 0x832941A8;
    'dispatch: loop {
        match pc {
            0x832941A8 => {
    //   block [0x832941A8..0x832941DC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832941E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832941E0 size=52
    let mut pc: u32 = 0x832941E0;
    'dispatch: loop {
        match pc {
            0x832941E0 => {
    //   block [0x832941E0..0x83294214)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294218 size=52
    let mut pc: u32 = 0x83294218;
    'dispatch: loop {
        match pc {
            0x83294218 => {
    //   block [0x83294218..0x8329424C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294250 size=52
    let mut pc: u32 = 0x83294250;
    'dispatch: loop {
        match pc {
            0x83294250 => {
    //   block [0x83294250..0x83294284)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294288 size=52
    let mut pc: u32 = 0x83294288;
    'dispatch: loop {
        match pc {
            0x83294288 => {
    //   block [0x83294288..0x832942BC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832942C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832942C0 size=52
    let mut pc: u32 = 0x832942C0;
    'dispatch: loop {
        match pc {
            0x832942C0 => {
    //   block [0x832942C0..0x832942F4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832942F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832942F8 size=52
    let mut pc: u32 = 0x832942F8;
    'dispatch: loop {
        match pc {
            0x832942F8 => {
    //   block [0x832942F8..0x8329432C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294330 size=52
    let mut pc: u32 = 0x83294330;
    'dispatch: loop {
        match pc {
            0x83294330 => {
    //   block [0x83294330..0x83294364)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294368 size=52
    let mut pc: u32 = 0x83294368;
    'dispatch: loop {
        match pc {
            0x83294368 => {
    //   block [0x83294368..0x8329439C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832943A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832943A0 size=52
    let mut pc: u32 = 0x832943A0;
    'dispatch: loop {
        match pc {
            0x832943A0 => {
    //   block [0x832943A0..0x832943D4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832943D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832943D8 size=52
    let mut pc: u32 = 0x832943D8;
    'dispatch: loop {
        match pc {
            0x832943D8 => {
    //   block [0x832943D8..0x8329440C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294410 size=52
    let mut pc: u32 = 0x83294410;
    'dispatch: loop {
        match pc {
            0x83294410 => {
    //   block [0x83294410..0x83294444)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294448 size=52
    let mut pc: u32 = 0x83294448;
    'dispatch: loop {
        match pc {
            0x83294448 => {
    //   block [0x83294448..0x8329447C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294480 size=52
    let mut pc: u32 = 0x83294480;
    'dispatch: loop {
        match pc {
            0x83294480 => {
    //   block [0x83294480..0x832944B4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832944B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832944B8 size=52
    let mut pc: u32 = 0x832944B8;
    'dispatch: loop {
        match pc {
            0x832944B8 => {
    //   block [0x832944B8..0x832944EC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832944F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832944F0 size=52
    let mut pc: u32 = 0x832944F0;
    'dispatch: loop {
        match pc {
            0x832944F0 => {
    //   block [0x832944F0..0x83294524)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294528 size=52
    let mut pc: u32 = 0x83294528;
    'dispatch: loop {
        match pc {
            0x83294528 => {
    //   block [0x83294528..0x8329455C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294560 size=52
    let mut pc: u32 = 0x83294560;
    'dispatch: loop {
        match pc {
            0x83294560 => {
    //   block [0x83294560..0x83294594)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294598 size=52
    let mut pc: u32 = 0x83294598;
    'dispatch: loop {
        match pc {
            0x83294598 => {
    //   block [0x83294598..0x832945CC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832945D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832945D0 size=52
    let mut pc: u32 = 0x832945D0;
    'dispatch: loop {
        match pc {
            0x832945D0 => {
    //   block [0x832945D0..0x83294604)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294608 size=52
    let mut pc: u32 = 0x83294608;
    'dispatch: loop {
        match pc {
            0x83294608 => {
    //   block [0x83294608..0x8329463C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294640 size=52
    let mut pc: u32 = 0x83294640;
    'dispatch: loop {
        match pc {
            0x83294640 => {
    //   block [0x83294640..0x83294674)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294678 size=52
    let mut pc: u32 = 0x83294678;
    'dispatch: loop {
        match pc {
            0x83294678 => {
    //   block [0x83294678..0x832946AC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832946B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832946B0 size=52
    let mut pc: u32 = 0x832946B0;
    'dispatch: loop {
        match pc {
            0x832946B0 => {
    //   block [0x832946B0..0x832946E4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832946E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832946E8 size=52
    let mut pc: u32 = 0x832946E8;
    'dispatch: loop {
        match pc {
            0x832946E8 => {
    //   block [0x832946E8..0x8329471C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294720 size=52
    let mut pc: u32 = 0x83294720;
    'dispatch: loop {
        match pc {
            0x83294720 => {
    //   block [0x83294720..0x83294754)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294758 size=52
    let mut pc: u32 = 0x83294758;
    'dispatch: loop {
        match pc {
            0x83294758 => {
    //   block [0x83294758..0x8329478C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294790 size=52
    let mut pc: u32 = 0x83294790;
    'dispatch: loop {
        match pc {
            0x83294790 => {
    //   block [0x83294790..0x832947C4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832947C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832947C8 size=52
    let mut pc: u32 = 0x832947C8;
    'dispatch: loop {
        match pc {
            0x832947C8 => {
    //   block [0x832947C8..0x832947FC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294800 size=52
    let mut pc: u32 = 0x83294800;
    'dispatch: loop {
        match pc {
            0x83294800 => {
    //   block [0x83294800..0x83294834)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294838 size=52
    let mut pc: u32 = 0x83294838;
    'dispatch: loop {
        match pc {
            0x83294838 => {
    //   block [0x83294838..0x8329486C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294870 size=52
    let mut pc: u32 = 0x83294870;
    'dispatch: loop {
        match pc {
            0x83294870 => {
    //   block [0x83294870..0x832948A4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832948A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832948A8 size=52
    let mut pc: u32 = 0x832948A8;
    'dispatch: loop {
        match pc {
            0x832948A8 => {
    //   block [0x832948A8..0x832948DC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832948E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832948E0 size=52
    let mut pc: u32 = 0x832948E0;
    'dispatch: loop {
        match pc {
            0x832948E0 => {
    //   block [0x832948E0..0x83294914)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294918 size=52
    let mut pc: u32 = 0x83294918;
    'dispatch: loop {
        match pc {
            0x83294918 => {
    //   block [0x83294918..0x8329494C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294950 size=52
    let mut pc: u32 = 0x83294950;
    'dispatch: loop {
        match pc {
            0x83294950 => {
    //   block [0x83294950..0x83294984)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294988 size=52
    let mut pc: u32 = 0x83294988;
    'dispatch: loop {
        match pc {
            0x83294988 => {
    //   block [0x83294988..0x832949BC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832949C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832949C0 size=52
    let mut pc: u32 = 0x832949C0;
    'dispatch: loop {
        match pc {
            0x832949C0 => {
    //   block [0x832949C0..0x832949F4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832949F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832949F8 size=52
    let mut pc: u32 = 0x832949F8;
    'dispatch: loop {
        match pc {
            0x832949F8 => {
    //   block [0x832949F8..0x83294A2C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294A30 size=52
    let mut pc: u32 = 0x83294A30;
    'dispatch: loop {
        match pc {
            0x83294A30 => {
    //   block [0x83294A30..0x83294A64)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294A68 size=52
    let mut pc: u32 = 0x83294A68;
    'dispatch: loop {
        match pc {
            0x83294A68 => {
    //   block [0x83294A68..0x83294A9C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294AA0 size=52
    let mut pc: u32 = 0x83294AA0;
    'dispatch: loop {
        match pc {
            0x83294AA0 => {
    //   block [0x83294AA0..0x83294AD4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294AD8 size=52
    let mut pc: u32 = 0x83294AD8;
    'dispatch: loop {
        match pc {
            0x83294AD8 => {
    //   block [0x83294AD8..0x83294B0C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294B10 size=52
    let mut pc: u32 = 0x83294B10;
    'dispatch: loop {
        match pc {
            0x83294B10 => {
    //   block [0x83294B10..0x83294B44)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294B48 size=52
    let mut pc: u32 = 0x83294B48;
    'dispatch: loop {
        match pc {
            0x83294B48 => {
    //   block [0x83294B48..0x83294B7C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294B80 size=52
    let mut pc: u32 = 0x83294B80;
    'dispatch: loop {
        match pc {
            0x83294B80 => {
    //   block [0x83294B80..0x83294BB4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294BB8 size=52
    let mut pc: u32 = 0x83294BB8;
    'dispatch: loop {
        match pc {
            0x83294BB8 => {
    //   block [0x83294BB8..0x83294BEC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294BF0 size=52
    let mut pc: u32 = 0x83294BF0;
    'dispatch: loop {
        match pc {
            0x83294BF0 => {
    //   block [0x83294BF0..0x83294C24)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294C28 size=52
    let mut pc: u32 = 0x83294C28;
    'dispatch: loop {
        match pc {
            0x83294C28 => {
    //   block [0x83294C28..0x83294C5C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294C60 size=52
    let mut pc: u32 = 0x83294C60;
    'dispatch: loop {
        match pc {
            0x83294C60 => {
    //   block [0x83294C60..0x83294C94)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294C98 size=52
    let mut pc: u32 = 0x83294C98;
    'dispatch: loop {
        match pc {
            0x83294C98 => {
    //   block [0x83294C98..0x83294CCC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294CD0 size=52
    let mut pc: u32 = 0x83294CD0;
    'dispatch: loop {
        match pc {
            0x83294CD0 => {
    //   block [0x83294CD0..0x83294D04)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294D08 size=52
    let mut pc: u32 = 0x83294D08;
    'dispatch: loop {
        match pc {
            0x83294D08 => {
    //   block [0x83294D08..0x83294D3C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294D40 size=52
    let mut pc: u32 = 0x83294D40;
    'dispatch: loop {
        match pc {
            0x83294D40 => {
    //   block [0x83294D40..0x83294D74)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294D78 size=52
    let mut pc: u32 = 0x83294D78;
    'dispatch: loop {
        match pc {
            0x83294D78 => {
    //   block [0x83294D78..0x83294DAC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294DB0 size=52
    let mut pc: u32 = 0x83294DB0;
    'dispatch: loop {
        match pc {
            0x83294DB0 => {
    //   block [0x83294DB0..0x83294DE4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294DE8 size=52
    let mut pc: u32 = 0x83294DE8;
    'dispatch: loop {
        match pc {
            0x83294DE8 => {
    //   block [0x83294DE8..0x83294E1C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294E20 size=52
    let mut pc: u32 = 0x83294E20;
    'dispatch: loop {
        match pc {
            0x83294E20 => {
    //   block [0x83294E20..0x83294E54)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294E58 size=52
    let mut pc: u32 = 0x83294E58;
    'dispatch: loop {
        match pc {
            0x83294E58 => {
    //   block [0x83294E58..0x83294E8C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294E90 size=52
    let mut pc: u32 = 0x83294E90;
    'dispatch: loop {
        match pc {
            0x83294E90 => {
    //   block [0x83294E90..0x83294EC4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294EC8 size=52
    let mut pc: u32 = 0x83294EC8;
    'dispatch: loop {
        match pc {
            0x83294EC8 => {
    //   block [0x83294EC8..0x83294EFC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294F00 size=52
    let mut pc: u32 = 0x83294F00;
    'dispatch: loop {
        match pc {
            0x83294F00 => {
    //   block [0x83294F00..0x83294F34)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294F38 size=52
    let mut pc: u32 = 0x83294F38;
    'dispatch: loop {
        match pc {
            0x83294F38 => {
    //   block [0x83294F38..0x83294F6C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294F70 size=52
    let mut pc: u32 = 0x83294F70;
    'dispatch: loop {
        match pc {
            0x83294F70 => {
    //   block [0x83294F70..0x83294FA4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294FA8 size=52
    let mut pc: u32 = 0x83294FA8;
    'dispatch: loop {
        match pc {
            0x83294FA8 => {
    //   block [0x83294FA8..0x83294FDC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83294FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83294FE0 size=52
    let mut pc: u32 = 0x83294FE0;
    'dispatch: loop {
        match pc {
            0x83294FE0 => {
    //   block [0x83294FE0..0x83295014)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295018 size=52
    let mut pc: u32 = 0x83295018;
    'dispatch: loop {
        match pc {
            0x83295018 => {
    //   block [0x83295018..0x8329504C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295050 size=52
    let mut pc: u32 = 0x83295050;
    'dispatch: loop {
        match pc {
            0x83295050 => {
    //   block [0x83295050..0x83295084)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295088 size=52
    let mut pc: u32 = 0x83295088;
    'dispatch: loop {
        match pc {
            0x83295088 => {
    //   block [0x83295088..0x832950BC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832950C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832950C0 size=52
    let mut pc: u32 = 0x832950C0;
    'dispatch: loop {
        match pc {
            0x832950C0 => {
    //   block [0x832950C0..0x832950F4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832950F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832950F8 size=52
    let mut pc: u32 = 0x832950F8;
    'dispatch: loop {
        match pc {
            0x832950F8 => {
    //   block [0x832950F8..0x8329512C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295130 size=52
    let mut pc: u32 = 0x83295130;
    'dispatch: loop {
        match pc {
            0x83295130 => {
    //   block [0x83295130..0x83295164)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295168 size=52
    let mut pc: u32 = 0x83295168;
    'dispatch: loop {
        match pc {
            0x83295168 => {
    //   block [0x83295168..0x8329519C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832951A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832951A0 size=52
    let mut pc: u32 = 0x832951A0;
    'dispatch: loop {
        match pc {
            0x832951A0 => {
    //   block [0x832951A0..0x832951D4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832951D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832951D8 size=52
    let mut pc: u32 = 0x832951D8;
    'dispatch: loop {
        match pc {
            0x832951D8 => {
    //   block [0x832951D8..0x8329520C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295210 size=52
    let mut pc: u32 = 0x83295210;
    'dispatch: loop {
        match pc {
            0x83295210 => {
    //   block [0x83295210..0x83295244)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295248 size=52
    let mut pc: u32 = 0x83295248;
    'dispatch: loop {
        match pc {
            0x83295248 => {
    //   block [0x83295248..0x8329527C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295280 size=52
    let mut pc: u32 = 0x83295280;
    'dispatch: loop {
        match pc {
            0x83295280 => {
    //   block [0x83295280..0x832952B4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832952B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832952B8 size=52
    let mut pc: u32 = 0x832952B8;
    'dispatch: loop {
        match pc {
            0x832952B8 => {
    //   block [0x832952B8..0x832952EC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832952F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832952F0 size=52
    let mut pc: u32 = 0x832952F0;
    'dispatch: loop {
        match pc {
            0x832952F0 => {
    //   block [0x832952F0..0x83295324)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295328 size=52
    let mut pc: u32 = 0x83295328;
    'dispatch: loop {
        match pc {
            0x83295328 => {
    //   block [0x83295328..0x8329535C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295360 size=52
    let mut pc: u32 = 0x83295360;
    'dispatch: loop {
        match pc {
            0x83295360 => {
    //   block [0x83295360..0x83295394)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295398 size=52
    let mut pc: u32 = 0x83295398;
    'dispatch: loop {
        match pc {
            0x83295398 => {
    //   block [0x83295398..0x832953CC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832953D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832953D0 size=52
    let mut pc: u32 = 0x832953D0;
    'dispatch: loop {
        match pc {
            0x832953D0 => {
    //   block [0x832953D0..0x83295404)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295408 size=52
    let mut pc: u32 = 0x83295408;
    'dispatch: loop {
        match pc {
            0x83295408 => {
    //   block [0x83295408..0x8329543C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295440 size=52
    let mut pc: u32 = 0x83295440;
    'dispatch: loop {
        match pc {
            0x83295440 => {
    //   block [0x83295440..0x83295474)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295478 size=52
    let mut pc: u32 = 0x83295478;
    'dispatch: loop {
        match pc {
            0x83295478 => {
    //   block [0x83295478..0x832954AC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832954B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832954B0 size=52
    let mut pc: u32 = 0x832954B0;
    'dispatch: loop {
        match pc {
            0x832954B0 => {
    //   block [0x832954B0..0x832954E4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832954E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832954E8 size=52
    let mut pc: u32 = 0x832954E8;
    'dispatch: loop {
        match pc {
            0x832954E8 => {
    //   block [0x832954E8..0x8329551C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295520 size=52
    let mut pc: u32 = 0x83295520;
    'dispatch: loop {
        match pc {
            0x83295520 => {
    //   block [0x83295520..0x83295554)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295558 size=52
    let mut pc: u32 = 0x83295558;
    'dispatch: loop {
        match pc {
            0x83295558 => {
    //   block [0x83295558..0x8329558C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295590 size=52
    let mut pc: u32 = 0x83295590;
    'dispatch: loop {
        match pc {
            0x83295590 => {
    //   block [0x83295590..0x832955C4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832955C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832955C8 size=52
    let mut pc: u32 = 0x832955C8;
    'dispatch: loop {
        match pc {
            0x832955C8 => {
    //   block [0x832955C8..0x832955FC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295600 size=52
    let mut pc: u32 = 0x83295600;
    'dispatch: loop {
        match pc {
            0x83295600 => {
    //   block [0x83295600..0x83295634)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295638 size=52
    let mut pc: u32 = 0x83295638;
    'dispatch: loop {
        match pc {
            0x83295638 => {
    //   block [0x83295638..0x8329566C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295670 size=52
    let mut pc: u32 = 0x83295670;
    'dispatch: loop {
        match pc {
            0x83295670 => {
    //   block [0x83295670..0x832956A4)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832956A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832956A8 size=52
    let mut pc: u32 = 0x832956A8;
    'dispatch: loop {
        match pc {
            0x832956A8 => {
    //   block [0x832956A8..0x832956DC)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832956E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832956E0 size=52
    let mut pc: u32 = 0x832956E0;
    'dispatch: loop {
        match pc {
            0x832956E0 => {
    //   block [0x832956E0..0x83295714)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295718 size=52
    let mut pc: u32 = 0x83295718;
    'dispatch: loop {
        match pc {
            0x83295718 => {
    //   block [0x83295718..0x8329574C)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295750 size=52
    let mut pc: u32 = 0x83295750;
    'dispatch: loop {
        match pc {
            0x83295750 => {
    //   block [0x83295750..0x83295784)
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
	sub_82189140(ctx, base);
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295788 size=52
    let mut pc: u32 = 0x83295788;
    'dispatch: loop {
        match pc {
            0x83295788 => {
    //   block [0x83295788..0x832957BC)
	// 83295788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329578C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295794: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295798: 386BCF18  addi r3, r11, -0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12520;
	// 8329579C: 4AEF39A5  bl 0x82189140
	ctx.lr = 0x832957A0;
	sub_82189140(ctx, base);
	// 832957A0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832957A4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832957A8: 916AE6A8  stw r11, -0x1958(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6488 as u32), ctx.r[11].u32 ) };
	// 832957AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832957B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832957B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832957B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832957C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832957C0 size=52
    let mut pc: u32 = 0x832957C0;
    'dispatch: loop {
        match pc {
            0x832957C0 => {
    //   block [0x832957C0..0x832957F4)
	// 832957C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832957C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832957C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832957CC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832957D0: 386BCF28  addi r3, r11, -0x30d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12504;
	// 832957D4: 4AEF396D  bl 0x82189140
	ctx.lr = 0x832957D8;
	sub_82189140(ctx, base);
	// 832957D8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832957DC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832957E0: 916AE6AC  stw r11, -0x1954(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6484 as u32), ctx.r[11].u32 ) };
	// 832957E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832957E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832957EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832957F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832957F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832957F8 size=52
    let mut pc: u32 = 0x832957F8;
    'dispatch: loop {
        match pc {
            0x832957F8 => {
    //   block [0x832957F8..0x8329582C)
	// 832957F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832957FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295804: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295808: 386BCF34  addi r3, r11, -0x30cc
	ctx.r[3].s64 = ctx.r[11].s64 + -12492;
	// 8329580C: 4AEF3935  bl 0x82189140
	ctx.lr = 0x83295810;
	sub_82189140(ctx, base);
	// 83295810: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295814: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295818: 916AE6B0  stw r11, -0x1950(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6480 as u32), ctx.r[11].u32 ) };
	// 8329581C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295830 size=52
    let mut pc: u32 = 0x83295830;
    'dispatch: loop {
        match pc {
            0x83295830 => {
    //   block [0x83295830..0x83295864)
	// 83295830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329583C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295840: 386BCF40  addi r3, r11, -0x30c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12480;
	// 83295844: 4AEF38FD  bl 0x82189140
	ctx.lr = 0x83295848;
	sub_82189140(ctx, base);
	// 83295848: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329584C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295850: 916AE6B4  stw r11, -0x194c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6476 as u32), ctx.r[11].u32 ) };
	// 83295854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329585C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295868 size=52
    let mut pc: u32 = 0x83295868;
    'dispatch: loop {
        match pc {
            0x83295868 => {
    //   block [0x83295868..0x8329589C)
	// 83295868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329586C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295874: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295878: 386BCF4C  addi r3, r11, -0x30b4
	ctx.r[3].s64 = ctx.r[11].s64 + -12468;
	// 8329587C: 4AEF38C5  bl 0x82189140
	ctx.lr = 0x83295880;
	sub_82189140(ctx, base);
	// 83295880: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295884: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295888: 916AE6B8  stw r11, -0x1948(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6472 as u32), ctx.r[11].u32 ) };
	// 8329588C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832958A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832958A0 size=52
    let mut pc: u32 = 0x832958A0;
    'dispatch: loop {
        match pc {
            0x832958A0 => {
    //   block [0x832958A0..0x832958D4)
	// 832958A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832958A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832958A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832958AC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832958B0: 386BCF60  addi r3, r11, -0x30a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12448;
	// 832958B4: 4AEF388D  bl 0x82189140
	ctx.lr = 0x832958B8;
	sub_82189140(ctx, base);
	// 832958B8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832958BC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832958C0: 916AE6BC  stw r11, -0x1944(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6468 as u32), ctx.r[11].u32 ) };
	// 832958C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832958C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832958CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832958D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832958D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832958D8 size=52
    let mut pc: u32 = 0x832958D8;
    'dispatch: loop {
        match pc {
            0x832958D8 => {
    //   block [0x832958D8..0x8329590C)
	// 832958D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832958DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832958E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832958E4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832958E8: 386BCF74  addi r3, r11, -0x308c
	ctx.r[3].s64 = ctx.r[11].s64 + -12428;
	// 832958EC: 4AEF3855  bl 0x82189140
	ctx.lr = 0x832958F0;
	sub_82189140(ctx, base);
	// 832958F0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832958F4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832958F8: 916AE6C0  stw r11, -0x1940(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6464 as u32), ctx.r[11].u32 ) };
	// 832958FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295910 size=52
    let mut pc: u32 = 0x83295910;
    'dispatch: loop {
        match pc {
            0x83295910 => {
    //   block [0x83295910..0x83295944)
	// 83295910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329591C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295920: 386BCF80  addi r3, r11, -0x3080
	ctx.r[3].s64 = ctx.r[11].s64 + -12416;
	// 83295924: 4AEF381D  bl 0x82189140
	ctx.lr = 0x83295928;
	sub_82189140(ctx, base);
	// 83295928: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329592C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295930: 916AE6C4  stw r11, -0x193c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6460 as u32), ctx.r[11].u32 ) };
	// 83295934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329593C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295948 size=52
    let mut pc: u32 = 0x83295948;
    'dispatch: loop {
        match pc {
            0x83295948 => {
    //   block [0x83295948..0x8329597C)
	// 83295948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329594C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295954: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295958: 386BCF8C  addi r3, r11, -0x3074
	ctx.r[3].s64 = ctx.r[11].s64 + -12404;
	// 8329595C: 4AEF37E5  bl 0x82189140
	ctx.lr = 0x83295960;
	sub_82189140(ctx, base);
	// 83295960: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295964: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295968: 916AE6C8  stw r11, -0x1938(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6456 as u32), ctx.r[11].u32 ) };
	// 8329596C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295980 size=52
    let mut pc: u32 = 0x83295980;
    'dispatch: loop {
        match pc {
            0x83295980 => {
    //   block [0x83295980..0x832959B4)
	// 83295980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329598C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295990: 386BCF9C  addi r3, r11, -0x3064
	ctx.r[3].s64 = ctx.r[11].s64 + -12388;
	// 83295994: 4AEF37AD  bl 0x82189140
	ctx.lr = 0x83295998;
	sub_82189140(ctx, base);
	// 83295998: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329599C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832959A0: 916AE6CC  stw r11, -0x1934(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6452 as u32), ctx.r[11].u32 ) };
	// 832959A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832959A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832959AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832959B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832959B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832959B8 size=52
    let mut pc: u32 = 0x832959B8;
    'dispatch: loop {
        match pc {
            0x832959B8 => {
    //   block [0x832959B8..0x832959EC)
	// 832959B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832959BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832959C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832959C4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832959C8: 386BCFAC  addi r3, r11, -0x3054
	ctx.r[3].s64 = ctx.r[11].s64 + -12372;
	// 832959CC: 4AEF3775  bl 0x82189140
	ctx.lr = 0x832959D0;
	sub_82189140(ctx, base);
	// 832959D0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832959D4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832959D8: 916AE6D0  stw r11, -0x1930(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6448 as u32), ctx.r[11].u32 ) };
	// 832959DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832959E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832959E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832959E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832959F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832959F0 size=52
    let mut pc: u32 = 0x832959F0;
    'dispatch: loop {
        match pc {
            0x832959F0 => {
    //   block [0x832959F0..0x83295A24)
	// 832959F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832959F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832959F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832959FC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295A00: 386BCFC0  addi r3, r11, -0x3040
	ctx.r[3].s64 = ctx.r[11].s64 + -12352;
	// 83295A04: 4AEF373D  bl 0x82189140
	ctx.lr = 0x83295A08;
	sub_82189140(ctx, base);
	// 83295A08: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295A0C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295A10: 916AE6D4  stw r11, -0x192c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6444 as u32), ctx.r[11].u32 ) };
	// 83295A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295A28 size=52
    let mut pc: u32 = 0x83295A28;
    'dispatch: loop {
        match pc {
            0x83295A28 => {
    //   block [0x83295A28..0x83295A5C)
	// 83295A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295A34: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295A38: 386BCFD0  addi r3, r11, -0x3030
	ctx.r[3].s64 = ctx.r[11].s64 + -12336;
	// 83295A3C: 4AEF3705  bl 0x82189140
	ctx.lr = 0x83295A40;
	sub_82189140(ctx, base);
	// 83295A40: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295A44: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295A48: 916AE6D8  stw r11, -0x1928(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6440 as u32), ctx.r[11].u32 ) };
	// 83295A4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295A60 size=52
    let mut pc: u32 = 0x83295A60;
    'dispatch: loop {
        match pc {
            0x83295A60 => {
    //   block [0x83295A60..0x83295A94)
	// 83295A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295A6C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295A70: 386BCFE4  addi r3, r11, -0x301c
	ctx.r[3].s64 = ctx.r[11].s64 + -12316;
	// 83295A74: 4AEF36CD  bl 0x82189140
	ctx.lr = 0x83295A78;
	sub_82189140(ctx, base);
	// 83295A78: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295A7C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295A80: 916AE6DC  stw r11, -0x1924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6436 as u32), ctx.r[11].u32 ) };
	// 83295A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295A98 size=52
    let mut pc: u32 = 0x83295A98;
    'dispatch: loop {
        match pc {
            0x83295A98 => {
    //   block [0x83295A98..0x83295ACC)
	// 83295A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295AA4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295AA8: 386BCFFC  addi r3, r11, -0x3004
	ctx.r[3].s64 = ctx.r[11].s64 + -12292;
	// 83295AAC: 4AEF3695  bl 0x82189140
	ctx.lr = 0x83295AB0;
	sub_82189140(ctx, base);
	// 83295AB0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295AB4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295AB8: 916AE6E0  stw r11, -0x1920(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6432 as u32), ctx.r[11].u32 ) };
	// 83295ABC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295AD0 size=52
    let mut pc: u32 = 0x83295AD0;
    'dispatch: loop {
        match pc {
            0x83295AD0 => {
    //   block [0x83295AD0..0x83295B04)
	// 83295AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295ADC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295AE0: 386BD010  addi r3, r11, -0x2ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -12272;
	// 83295AE4: 4AEF365D  bl 0x82189140
	ctx.lr = 0x83295AE8;
	sub_82189140(ctx, base);
	// 83295AE8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295AEC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295AF0: 916AE6E4  stw r11, -0x191c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6428 as u32), ctx.r[11].u32 ) };
	// 83295AF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295B08 size=52
    let mut pc: u32 = 0x83295B08;
    'dispatch: loop {
        match pc {
            0x83295B08 => {
    //   block [0x83295B08..0x83295B3C)
	// 83295B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295B10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295B14: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295B18: 386BD020  addi r3, r11, -0x2fe0
	ctx.r[3].s64 = ctx.r[11].s64 + -12256;
	// 83295B1C: 4AEF3625  bl 0x82189140
	ctx.lr = 0x83295B20;
	sub_82189140(ctx, base);
	// 83295B20: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295B24: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295B28: 916AE6E8  stw r11, -0x1918(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6424 as u32), ctx.r[11].u32 ) };
	// 83295B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295B40 size=52
    let mut pc: u32 = 0x83295B40;
    'dispatch: loop {
        match pc {
            0x83295B40 => {
    //   block [0x83295B40..0x83295B74)
	// 83295B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295B4C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295B50: 386BD034  addi r3, r11, -0x2fcc
	ctx.r[3].s64 = ctx.r[11].s64 + -12236;
	// 83295B54: 4AEF35ED  bl 0x82189140
	ctx.lr = 0x83295B58;
	sub_82189140(ctx, base);
	// 83295B58: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295B5C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295B60: 916AE6EC  stw r11, -0x1914(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6420 as u32), ctx.r[11].u32 ) };
	// 83295B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295B78 size=52
    let mut pc: u32 = 0x83295B78;
    'dispatch: loop {
        match pc {
            0x83295B78 => {
    //   block [0x83295B78..0x83295BAC)
	// 83295B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295B84: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295B88: 386BD04C  addi r3, r11, -0x2fb4
	ctx.r[3].s64 = ctx.r[11].s64 + -12212;
	// 83295B8C: 4AEF35B5  bl 0x82189140
	ctx.lr = 0x83295B90;
	sub_82189140(ctx, base);
	// 83295B90: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295B94: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295B98: 916AE6F0  stw r11, -0x1910(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6416 as u32), ctx.r[11].u32 ) };
	// 83295B9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295BB0 size=52
    let mut pc: u32 = 0x83295BB0;
    'dispatch: loop {
        match pc {
            0x83295BB0 => {
    //   block [0x83295BB0..0x83295BE4)
	// 83295BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295BBC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295BC0: 386BD054  addi r3, r11, -0x2fac
	ctx.r[3].s64 = ctx.r[11].s64 + -12204;
	// 83295BC4: 4AEF357D  bl 0x82189140
	ctx.lr = 0x83295BC8;
	sub_82189140(ctx, base);
	// 83295BC8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295BCC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295BD0: 916AE6F4  stw r11, -0x190c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6412 as u32), ctx.r[11].u32 ) };
	// 83295BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295BE8 size=52
    let mut pc: u32 = 0x83295BE8;
    'dispatch: loop {
        match pc {
            0x83295BE8 => {
    //   block [0x83295BE8..0x83295C1C)
	// 83295BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295BF4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295BF8: 386BD060  addi r3, r11, -0x2fa0
	ctx.r[3].s64 = ctx.r[11].s64 + -12192;
	// 83295BFC: 4AEF3545  bl 0x82189140
	ctx.lr = 0x83295C00;
	sub_82189140(ctx, base);
	// 83295C00: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295C04: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295C08: 916AE6F8  stw r11, -0x1908(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6408 as u32), ctx.r[11].u32 ) };
	// 83295C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295C20 size=52
    let mut pc: u32 = 0x83295C20;
    'dispatch: loop {
        match pc {
            0x83295C20 => {
    //   block [0x83295C20..0x83295C54)
	// 83295C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295C2C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295C30: 386BD068  addi r3, r11, -0x2f98
	ctx.r[3].s64 = ctx.r[11].s64 + -12184;
	// 83295C34: 4AEF350D  bl 0x82189140
	ctx.lr = 0x83295C38;
	sub_82189140(ctx, base);
	// 83295C38: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295C3C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295C40: 916AE6FC  stw r11, -0x1904(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6404 as u32), ctx.r[11].u32 ) };
	// 83295C44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295C58 size=52
    let mut pc: u32 = 0x83295C58;
    'dispatch: loop {
        match pc {
            0x83295C58 => {
    //   block [0x83295C58..0x83295C8C)
	// 83295C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295C64: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295C68: 386BD070  addi r3, r11, -0x2f90
	ctx.r[3].s64 = ctx.r[11].s64 + -12176;
	// 83295C6C: 4AEF34D5  bl 0x82189140
	ctx.lr = 0x83295C70;
	sub_82189140(ctx, base);
	// 83295C70: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295C74: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295C78: 916AE700  stw r11, -0x1900(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6400 as u32), ctx.r[11].u32 ) };
	// 83295C7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295C90 size=52
    let mut pc: u32 = 0x83295C90;
    'dispatch: loop {
        match pc {
            0x83295C90 => {
    //   block [0x83295C90..0x83295CC4)
	// 83295C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295C9C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295CA0: 386BD078  addi r3, r11, -0x2f88
	ctx.r[3].s64 = ctx.r[11].s64 + -12168;
	// 83295CA4: 4AEF349D  bl 0x82189140
	ctx.lr = 0x83295CA8;
	sub_82189140(ctx, base);
	// 83295CA8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295CAC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295CB0: 916AE704  stw r11, -0x18fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6396 as u32), ctx.r[11].u32 ) };
	// 83295CB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295CC8 size=52
    let mut pc: u32 = 0x83295CC8;
    'dispatch: loop {
        match pc {
            0x83295CC8 => {
    //   block [0x83295CC8..0x83295CFC)
	// 83295CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295CD4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295CD8: 386BD080  addi r3, r11, -0x2f80
	ctx.r[3].s64 = ctx.r[11].s64 + -12160;
	// 83295CDC: 4AEF3465  bl 0x82189140
	ctx.lr = 0x83295CE0;
	sub_82189140(ctx, base);
	// 83295CE0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295CE4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295CE8: 916AE708  stw r11, -0x18f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6392 as u32), ctx.r[11].u32 ) };
	// 83295CEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295D00 size=52
    let mut pc: u32 = 0x83295D00;
    'dispatch: loop {
        match pc {
            0x83295D00 => {
    //   block [0x83295D00..0x83295D34)
	// 83295D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295D0C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295D10: 386BD088  addi r3, r11, -0x2f78
	ctx.r[3].s64 = ctx.r[11].s64 + -12152;
	// 83295D14: 4AEF342D  bl 0x82189140
	ctx.lr = 0x83295D18;
	sub_82189140(ctx, base);
	// 83295D18: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295D1C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295D20: 916AE70C  stw r11, -0x18f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6388 as u32), ctx.r[11].u32 ) };
	// 83295D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83295D38 size=28
    let mut pc: u32 = 0x83295D38;
    'dispatch: loop {
        match pc {
            0x83295D38 => {
    //   block [0x83295D38..0x83295D54)
	// 83295D38: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83295D3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83295D40: 386970D8  addi r3, r9, 0x70d8
	ctx.r[3].s64 = ctx.r[9].s64 + 28888;
	// 83295D44: 816A716C  lwz r11, 0x716c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29036 as u32) ) } as u64;
	// 83295D48: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 83295D4C: 916A716C  stw r11, 0x716c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29036 as u32), ctx.r[11].u32 ) };
	// 83295D50: 4BA141D0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83295D58 size=12
    let mut pc: u32 = 0x83295D58;
    'dispatch: loop {
        match pc {
            0x83295D58 => {
    //   block [0x83295D58..0x83295D64)
	// 83295D58: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83295D5C: 386B70F0  addi r3, r11, 0x70f0
	ctx.r[3].s64 = ctx.r[11].s64 + 28912;
	// 83295D60: 4BA141C0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295D68 size=52
    let mut pc: u32 = 0x83295D68;
    'dispatch: loop {
        match pc {
            0x83295D68 => {
    //   block [0x83295D68..0x83295D9C)
	// 83295D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295D74: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295D78: 386BCE04  addi r3, r11, -0x31fc
	ctx.r[3].s64 = ctx.r[11].s64 + -12796;
	// 83295D7C: 4AEF33C5  bl 0x82189140
	ctx.lr = 0x83295D80;
	sub_82189140(ctx, base);
	// 83295D80: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295D84: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295D88: 916AE750  stw r11, -0x18b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6320 as u32), ctx.r[11].u32 ) };
	// 83295D8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295DA0 size=52
    let mut pc: u32 = 0x83295DA0;
    'dispatch: loop {
        match pc {
            0x83295DA0 => {
    //   block [0x83295DA0..0x83295DD4)
	// 83295DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295DAC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295DB0: 386BCE18  addi r3, r11, -0x31e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12776;
	// 83295DB4: 4AEF338D  bl 0x82189140
	ctx.lr = 0x83295DB8;
	sub_82189140(ctx, base);
	// 83295DB8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295DBC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295DC0: 916AE754  stw r11, -0x18ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6316 as u32), ctx.r[11].u32 ) };
	// 83295DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295DD8 size=52
    let mut pc: u32 = 0x83295DD8;
    'dispatch: loop {
        match pc {
            0x83295DD8 => {
    //   block [0x83295DD8..0x83295E0C)
	// 83295DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295DE4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295DE8: 386BCE2C  addi r3, r11, -0x31d4
	ctx.r[3].s64 = ctx.r[11].s64 + -12756;
	// 83295DEC: 4AEF3355  bl 0x82189140
	ctx.lr = 0x83295DF0;
	sub_82189140(ctx, base);
	// 83295DF0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295DF4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295DF8: 916AE758  stw r11, -0x18a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6312 as u32), ctx.r[11].u32 ) };
	// 83295DFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295E10 size=52
    let mut pc: u32 = 0x83295E10;
    'dispatch: loop {
        match pc {
            0x83295E10 => {
    //   block [0x83295E10..0x83295E44)
	// 83295E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295E1C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295E20: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 83295E24: 4AEF331D  bl 0x82189140
	ctx.lr = 0x83295E28;
	sub_82189140(ctx, base);
	// 83295E28: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295E2C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295E30: 916AE75C  stw r11, -0x18a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6308 as u32), ctx.r[11].u32 ) };
	// 83295E34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295E48 size=52
    let mut pc: u32 = 0x83295E48;
    'dispatch: loop {
        match pc {
            0x83295E48 => {
    //   block [0x83295E48..0x83295E7C)
	// 83295E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295E54: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295E58: 386BCE50  addi r3, r11, -0x31b0
	ctx.r[3].s64 = ctx.r[11].s64 + -12720;
	// 83295E5C: 4AEF32E5  bl 0x82189140
	ctx.lr = 0x83295E60;
	sub_82189140(ctx, base);
	// 83295E60: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295E64: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295E68: 916AE760  stw r11, -0x18a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6304 as u32), ctx.r[11].u32 ) };
	// 83295E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295E80 size=52
    let mut pc: u32 = 0x83295E80;
    'dispatch: loop {
        match pc {
            0x83295E80 => {
    //   block [0x83295E80..0x83295EB4)
	// 83295E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295E8C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295E90: 386BCE60  addi r3, r11, -0x31a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12704;
	// 83295E94: 4AEF32AD  bl 0x82189140
	ctx.lr = 0x83295E98;
	sub_82189140(ctx, base);
	// 83295E98: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295E9C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295EA0: 916AE764  stw r11, -0x189c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6300 as u32), ctx.r[11].u32 ) };
	// 83295EA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295EB8 size=52
    let mut pc: u32 = 0x83295EB8;
    'dispatch: loop {
        match pc {
            0x83295EB8 => {
    //   block [0x83295EB8..0x83295EEC)
	// 83295EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295EC4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295EC8: 386BCE74  addi r3, r11, -0x318c
	ctx.r[3].s64 = ctx.r[11].s64 + -12684;
	// 83295ECC: 4AEF3275  bl 0x82189140
	ctx.lr = 0x83295ED0;
	sub_82189140(ctx, base);
	// 83295ED0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295ED4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295ED8: 916AE768  stw r11, -0x1898(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6296 as u32), ctx.r[11].u32 ) };
	// 83295EDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295EF0 size=52
    let mut pc: u32 = 0x83295EF0;
    'dispatch: loop {
        match pc {
            0x83295EF0 => {
    //   block [0x83295EF0..0x83295F24)
	// 83295EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295EFC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295F00: 386BCE8C  addi r3, r11, -0x3174
	ctx.r[3].s64 = ctx.r[11].s64 + -12660;
	// 83295F04: 4AEF323D  bl 0x82189140
	ctx.lr = 0x83295F08;
	sub_82189140(ctx, base);
	// 83295F08: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295F0C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295F10: 916AE76C  stw r11, -0x1894(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6292 as u32), ctx.r[11].u32 ) };
	// 83295F14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295F28 size=52
    let mut pc: u32 = 0x83295F28;
    'dispatch: loop {
        match pc {
            0x83295F28 => {
    //   block [0x83295F28..0x83295F5C)
	// 83295F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295F34: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295F38: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 83295F3C: 4AEF3205  bl 0x82189140
	ctx.lr = 0x83295F40;
	sub_82189140(ctx, base);
	// 83295F40: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295F44: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295F48: 916AE770  stw r11, -0x1890(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6288 as u32), ctx.r[11].u32 ) };
	// 83295F4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295F60 size=52
    let mut pc: u32 = 0x83295F60;
    'dispatch: loop {
        match pc {
            0x83295F60 => {
    //   block [0x83295F60..0x83295F94)
	// 83295F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295F6C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295F70: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 83295F74: 4AEF31CD  bl 0x82189140
	ctx.lr = 0x83295F78;
	sub_82189140(ctx, base);
	// 83295F78: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295F7C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295F80: 916AE774  stw r11, -0x188c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6284 as u32), ctx.r[11].u32 ) };
	// 83295F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295F98 size=52
    let mut pc: u32 = 0x83295F98;
    'dispatch: loop {
        match pc {
            0x83295F98 => {
    //   block [0x83295F98..0x83295FCC)
	// 83295F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295FA4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295FA8: 386BCED0  addi r3, r11, -0x3130
	ctx.r[3].s64 = ctx.r[11].s64 + -12592;
	// 83295FAC: 4AEF3195  bl 0x82189140
	ctx.lr = 0x83295FB0;
	sub_82189140(ctx, base);
	// 83295FB0: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295FB4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295FB8: 916AE778  stw r11, -0x1888(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6280 as u32), ctx.r[11].u32 ) };
	// 83295FBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83295FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83295FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83295FD0 size=52
    let mut pc: u32 = 0x83295FD0;
    'dispatch: loop {
        match pc {
            0x83295FD0 => {
    //   block [0x83295FD0..0x83296004)
	// 83295FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83295FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83295FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83295FDC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83295FE0: 386BCEEC  addi r3, r11, -0x3114
	ctx.r[3].s64 = ctx.r[11].s64 + -12564;
	// 83295FE4: 4AEF315D  bl 0x82189140
	ctx.lr = 0x83295FE8;
	sub_82189140(ctx, base);
	// 83295FE8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83295FEC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83295FF0: 916AE77C  stw r11, -0x1884(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6276 as u32), ctx.r[11].u32 ) };
	// 83295FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83295FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83295FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296008 size=52
    let mut pc: u32 = 0x83296008;
    'dispatch: loop {
        match pc {
            0x83296008 => {
    //   block [0x83296008..0x8329603C)
	// 83296008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329600C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296014: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296018: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 8329601C: 4AEF3125  bl 0x82189140
	ctx.lr = 0x83296020;
	sub_82189140(ctx, base);
	// 83296020: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296024: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296028: 916AE780  stw r11, -0x1880(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6272 as u32), ctx.r[11].u32 ) };
	// 8329602C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296040 size=52
    let mut pc: u32 = 0x83296040;
    'dispatch: loop {
        match pc {
            0x83296040 => {
    //   block [0x83296040..0x83296074)
	// 83296040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83296044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329604C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296050: 386BCF18  addi r3, r11, -0x30e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12520;
	// 83296054: 4AEF30ED  bl 0x82189140
	ctx.lr = 0x83296058;
	sub_82189140(ctx, base);
	// 83296058: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 8329605C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296060: 916AE784  stw r11, -0x187c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6268 as u32), ctx.r[11].u32 ) };
	// 83296064: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329606C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83296078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83296078 size=52
    let mut pc: u32 = 0x83296078;
    'dispatch: loop {
        match pc {
            0x83296078 => {
    //   block [0x83296078..0x832960AC)
	// 83296078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83296080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83296084: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83296088: 386BCF28  addi r3, r11, -0x30d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12504;
	// 8329608C: 4AEF30B5  bl 0x82189140
	ctx.lr = 0x83296090;
	sub_82189140(ctx, base);
	// 83296090: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296094: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296098: 916AE788  stw r11, -0x1878(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6264 as u32), ctx.r[11].u32 ) };
	// 8329609C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832960A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832960A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832960A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832960B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832960B0 size=52
    let mut pc: u32 = 0x832960B0;
    'dispatch: loop {
        match pc {
            0x832960B0 => {
    //   block [0x832960B0..0x832960E4)
	// 832960B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832960B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832960B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832960BC: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832960C0: 386BDFB8  addi r3, r11, -0x2048
	ctx.r[3].s64 = ctx.r[11].s64 + -8264;
	// 832960C4: 4AEF307D  bl 0x82189140
	ctx.lr = 0x832960C8;
	sub_82189140(ctx, base);
	// 832960C8: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 832960CC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832960D0: 916AE78C  stw r11, -0x1874(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6260 as u32), ctx.r[11].u32 ) };
	// 832960D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832960D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832960DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832960E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832960E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832960E8 size=52
    let mut pc: u32 = 0x832960E8;
    'dispatch: loop {
        match pc {
            0x832960E8 => {
    //   block [0x832960E8..0x8329611C)
	// 832960E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832960EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832960F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832960F4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832960F8: 386BDFAC  addi r3, r11, -0x2054
	ctx.r[3].s64 = ctx.r[11].s64 + -8276;
	// 832960FC: 4AEF3045  bl 0x82189140
	ctx.lr = 0x83296100;
	sub_82189140(ctx, base);
	// 83296100: 3D40834C  lis r10, -0x7cb4
	ctx.r[10].s64 = -2092171264;
	// 83296104: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83296108: 916AE790  stw r11, -0x1870(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6256 as u32), ctx.r[11].u32 ) };
	// 8329610C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83296110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83296114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83296118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


