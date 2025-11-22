pub fn sub_832400A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832400A0 size=60
    let mut pc: u32 = 0x832400A0;
    'dispatch: loop {
        match pc {
            0x832400A0 => {
    //   block [0x832400A0..0x832400DC)
	// 832400A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832400A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832400A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832400AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832400B0: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832400B4: 3BEB8468  addi r31, r11, -0x7b98
	ctx.r[31].s64 = ctx.r[11].s64 + -31640;
	// 832400B8: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 832400BC: 4BBB336D  bl 0x82df3428
	ctx.lr = 0x832400C0;
	sub_82DF3428(ctx, base);
	// 832400C0: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 832400C4: 4BBB3365  bl 0x82df3428
	ctx.lr = 0x832400C8;
	sub_82DF3428(ctx, base);
	// 832400C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832400CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832400D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832400D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832400D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832400E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832400E0 size=60
    let mut pc: u32 = 0x832400E0;
    'dispatch: loop {
        match pc {
            0x832400E0 => {
    //   block [0x832400E0..0x8324011C)
	// 832400E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832400E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832400E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832400EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832400F0: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832400F4: 3BEB9B00  addi r31, r11, -0x6500
	ctx.r[31].s64 = ctx.r[11].s64 + -25856;
	// 832400F8: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 832400FC: 4BBB332D  bl 0x82df3428
	ctx.lr = 0x83240100;
	sub_82DF3428(ctx, base);
	// 83240100: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83240104: 4BBB3325  bl 0x82df3428
	ctx.lr = 0x83240108;
	sub_82DF3428(ctx, base);
	// 83240108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324010C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240120 size=60
    let mut pc: u32 = 0x83240120;
    'dispatch: loop {
        match pc {
            0x83240120 => {
    //   block [0x83240120..0x8324015C)
	// 83240120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324012C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240130: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 83240134: 3BEBFCD8  addi r31, r11, -0x328
	ctx.r[31].s64 = ctx.r[11].s64 + -808;
	// 83240138: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 8324013C: 4BBB32ED  bl 0x82df3428
	ctx.lr = 0x83240140;
	sub_82DF3428(ctx, base);
	// 83240140: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83240144: 4BBB32E5  bl 0x82df3428
	ctx.lr = 0x83240148;
	sub_82DF3428(ctx, base);
	// 83240148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240160 size=80
    let mut pc: u32 = 0x83240160;
    'dispatch: loop {
        match pc {
            0x83240160 => {
    //   block [0x83240160..0x832401B0)
	// 83240160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324016C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240174: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 83240178: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8324017C: 396BB198  addi r11, r11, -0x4e68
	ctx.r[11].s64 = ctx.r[11].s64 + -20072;
	// 83240180: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 83240184: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 83240188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324018C: 4BBB329D  bl 0x82df3428
	ctx.lr = 0x83240190;
	sub_82DF3428(ctx, base);
	// 83240190: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240194: 4080FFF0  bge 0x83240184
	if !ctx.cr[0].lt {
	pc = 0x83240184; continue 'dispatch;
	}
	// 83240198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324019C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832401A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832401A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832401A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832401AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832401B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832401B0 size=12
    let mut pc: u32 = 0x832401B0;
    'dispatch: loop {
        match pc {
            0x832401B0 => {
    //   block [0x832401B0..0x832401BC)
	// 832401B0: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832401B4: 386BB1C8  addi r3, r11, -0x4e38
	ctx.r[3].s64 = ctx.r[11].s64 + -20024;
	// 832401B8: 4BBB3270  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832401C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832401C0 size=12
    let mut pc: u32 = 0x832401C0;
    'dispatch: loop {
        match pc {
            0x832401C0 => {
    //   block [0x832401C0..0x832401CC)
	// 832401C0: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832401C4: 386BB1D0  addi r3, r11, -0x4e30
	ctx.r[3].s64 = ctx.r[11].s64 + -20016;
	// 832401C8: 4BBB3260  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832401D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832401D0 size=80
    let mut pc: u32 = 0x832401D0;
    'dispatch: loop {
        match pc {
            0x832401D0 => {
    //   block [0x832401D0..0x83240220)
	// 832401D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832401D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832401D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832401DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832401E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832401E4: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832401E8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832401EC: 396BB1D4  addi r11, r11, -0x4e2c
	ctx.r[11].s64 = ctx.r[11].s64 + -20012;
	// 832401F0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 832401F4: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832401F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832401FC: 4BBB322D  bl 0x82df3428
	ctx.lr = 0x83240200;
	sub_82DF3428(ctx, base);
	// 83240200: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240204: 4080FFF0  bge 0x832401f4
	if !ctx.cr[0].lt {
	pc = 0x832401F4; continue 'dispatch;
	}
	// 83240208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240214: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240218: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324021C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240220 size=80
    let mut pc: u32 = 0x83240220;
    'dispatch: loop {
        match pc {
            0x83240220 => {
    //   block [0x83240220..0x83240270)
	// 83240220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324022C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240234: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 83240238: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 8324023C: 396BB1E8  addi r11, r11, -0x4e18
	ctx.r[11].s64 = ctx.r[11].s64 + -19992;
	// 83240240: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 83240244: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 83240248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324024C: 4BBB31DD  bl 0x82df3428
	ctx.lr = 0x83240250;
	sub_82DF3428(ctx, base);
	// 83240250: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240254: 4080FFF0  bge 0x83240244
	if !ctx.cr[0].lt {
	pc = 0x83240244; continue 'dispatch;
	}
	// 83240258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324025C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240264: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324026C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240270 size=80
    let mut pc: u32 = 0x83240270;
    'dispatch: loop {
        match pc {
            0x83240270 => {
    //   block [0x83240270..0x832402C0)
	// 83240270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324027C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240284: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 83240288: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 8324028C: 396BC870  addi r11, r11, -0x3790
	ctx.r[11].s64 = ctx.r[11].s64 + -14224;
	// 83240290: 3BEB2508  addi r31, r11, 0x2508
	ctx.r[31].s64 = ctx.r[11].s64 + 9480;
	// 83240294: 3BFFFC4C  addi r31, r31, -0x3b4
	ctx.r[31].s64 = ctx.r[31].s64 + -948;
	// 83240298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324029C: 4B66321D  bl 0x828a34b8
	ctx.lr = 0x832402A0;
	sub_828A34B8(ctx, base);
	// 832402A0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832402A4: 4080FFF0  bge 0x83240294
	if !ctx.cr[0].lt {
	pc = 0x83240294; continue 'dispatch;
	}
	// 832402A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832402AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832402B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832402B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832402B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832402BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832402C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832402C0 size=80
    let mut pc: u32 = 0x832402C0;
    'dispatch: loop {
        match pc {
            0x832402C0 => {
    //   block [0x832402C0..0x83240310)
	// 832402C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832402C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832402C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832402CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832402D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832402D4: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832402D8: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 832402DC: 396BEDA8  addi r11, r11, -0x1258
	ctx.r[11].s64 = ctx.r[11].s64 + -4696;
	// 832402E0: 3BEB004C  addi r31, r11, 0x4c
	ctx.r[31].s64 = ctx.r[11].s64 + 76;
	// 832402E4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832402E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832402EC: 4BBB313D  bl 0x82df3428
	ctx.lr = 0x832402F0;
	sub_82DF3428(ctx, base);
	// 832402F0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832402F4: 4080FFF0  bge 0x832402e4
	if !ctx.cr[0].lt {
	pc = 0x832402E4; continue 'dispatch;
	}
	// 832402F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832402FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240304: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240308: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324030C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240310 size=88
    let mut pc: u32 = 0x83240310;
    'dispatch: loop {
        match pc {
            0x83240310 => {
    //   block [0x83240310..0x83240368)
	// 83240310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324031C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240324: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 83240328: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 8324032C: 396BEDF4  addi r11, r11, -0x120c
	ctx.r[11].s64 = ctx.r[11].s64 + -4620;
	// 83240330: 3BEB1A3C  addi r31, r11, 0x1a3c
	ctx.r[31].s64 = ctx.r[11].s64 + 6716;
	// 83240334: 3BFFFD78  addi r31, r31, -0x288
	ctx.r[31].s64 = ctx.r[31].s64 + -648;
	// 83240338: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324033C: 4BBB30ED  bl 0x82df3428
	ctx.lr = 0x83240340;
	sub_82DF3428(ctx, base);
	// 83240340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240344: 4BBB30E5  bl 0x82df3428
	ctx.lr = 0x83240348;
	sub_82DF3428(ctx, base);
	// 83240348: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8324034C: 4080FFE8  bge 0x83240334
	if !ctx.cr[0].lt {
	pc = 0x83240334; continue 'dispatch;
	}
	// 83240350: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324035C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240360: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240368 size=80
    let mut pc: u32 = 0x83240368;
    'dispatch: loop {
        match pc {
            0x83240368 => {
    //   block [0x83240368..0x832403B8)
	// 83240368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324036C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240370: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240374: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240378: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324037C: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 83240380: 3BC0000E  li r30, 0xe
	ctx.r[30].s64 = 14;
	// 83240384: 396B0778  addi r11, r11, 0x778
	ctx.r[11].s64 = ctx.r[11].s64 + 1912;
	// 83240388: 3BEB007C  addi r31, r11, 0x7c
	ctx.r[31].s64 = ctx.r[11].s64 + 124;
	// 8324038C: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 83240390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240394: 4BBB3095  bl 0x82df3428
	ctx.lr = 0x83240398;
	sub_82DF3428(ctx, base);
	// 83240398: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8324039C: 4080FFF0  bge 0x8324038c
	if !ctx.cr[0].lt {
	pc = 0x8324038C; continue 'dispatch;
	}
	// 832403A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832403A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832403A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832403AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832403B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832403B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832403B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832403B8 size=60
    let mut pc: u32 = 0x832403B8;
    'dispatch: loop {
        match pc {
            0x832403B8 => {
    //   block [0x832403B8..0x832403F4)
	// 832403B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832403BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832403C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832403C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832403C8: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832403CC: 3BEB0ED8  addi r31, r11, 0xed8
	ctx.r[31].s64 = ctx.r[11].s64 + 3800;
	// 832403D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832403D4: 4BBB3055  bl 0x82df3428
	ctx.lr = 0x832403D8;
	sub_82DF3428(ctx, base);
	// 832403D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832403DC: 4BBB304D  bl 0x82df3428
	ctx.lr = 0x832403E0;
	sub_82DF3428(ctx, base);
	// 832403E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832403E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832403E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832403EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832403F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832403F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832403F8 size=80
    let mut pc: u32 = 0x832403F8;
    'dispatch: loop {
        match pc {
            0x832403F8 => {
    //   block [0x832403F8..0x83240448)
	// 832403F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832403FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324040C: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 83240410: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83240414: 396B131C  addi r11, r11, 0x131c
	ctx.r[11].s64 = ctx.r[11].s64 + 4892;
	// 83240418: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 8324041C: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 83240420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240424: 4BBB3005  bl 0x82df3428
	ctx.lr = 0x83240428;
	sub_82DF3428(ctx, base);
	// 83240428: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8324042C: 4080FFF0  bge 0x8324041c
	if !ctx.cr[0].lt {
	pc = 0x8324041C; continue 'dispatch;
	}
	// 83240430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324043C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240448 size=60
    let mut pc: u32 = 0x83240448;
    'dispatch: loop {
        match pc {
            0x83240448 => {
    //   block [0x83240448..0x83240484)
	// 83240448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324044C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240458: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 8324045C: 3BEB27E8  addi r31, r11, 0x27e8
	ctx.r[31].s64 = ctx.r[11].s64 + 10216;
	// 83240460: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83240464: 4BBB2FC5  bl 0x82df3428
	ctx.lr = 0x83240468;
	sub_82DF3428(ctx, base);
	// 83240468: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8324046C: 4BBB2FBD  bl 0x82df3428
	ctx.lr = 0x83240470;
	sub_82DF3428(ctx, base);
	// 83240470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324047C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240488 size=60
    let mut pc: u32 = 0x83240488;
    'dispatch: loop {
        match pc {
            0x83240488 => {
    //   block [0x83240488..0x832404C4)
	// 83240488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240498: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 8324049C: 3BEB1388  addi r31, r11, 0x1388
	ctx.r[31].s64 = ctx.r[11].s64 + 5000;
	// 832404A0: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 832404A4: 4BBB2F85  bl 0x82df3428
	ctx.lr = 0x832404A8;
	sub_82DF3428(ctx, base);
	// 832404A8: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 832404AC: 4BBB2F7D  bl 0x82df3428
	ctx.lr = 0x832404B0;
	sub_82DF3428(ctx, base);
	// 832404B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832404B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832404B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832404BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832404C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832404C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832404C8 size=80
    let mut pc: u32 = 0x832404C8;
    'dispatch: loop {
        match pc {
            0x832404C8 => {
    //   block [0x832404C8..0x83240518)
	// 832404C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832404CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832404D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832404D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832404D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832404DC: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832404E0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832404E4: 396B3C48  addi r11, r11, 0x3c48
	ctx.r[11].s64 = ctx.r[11].s64 + 15432;
	// 832404E8: 3BEB0014  addi r31, r11, 0x14
	ctx.r[31].s64 = ctx.r[11].s64 + 20;
	// 832404EC: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832404F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832404F4: 4BBB2F35  bl 0x82df3428
	ctx.lr = 0x832404F8;
	sub_82DF3428(ctx, base);
	// 832404F8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832404FC: 4080FFF0  bge 0x832404ec
	if !ctx.cr[0].lt {
	pc = 0x832404EC; continue 'dispatch;
	}
	// 83240500: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324050C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83240518 size=12
    let mut pc: u32 = 0x83240518;
    'dispatch: loop {
        match pc {
            0x83240518 => {
    //   block [0x83240518..0x83240524)
	// 83240518: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 8324051C: 386B3C58  addi r3, r11, 0x3c58
	ctx.r[3].s64 = ctx.r[11].s64 + 15448;
	// 83240520: 4BBB2F08  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240528 size=60
    let mut pc: u32 = 0x83240528;
    'dispatch: loop {
        match pc {
            0x83240528 => {
    //   block [0x83240528..0x83240564)
	// 83240528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324052C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240538: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 8324053C: 3BEB52B8  addi r31, r11, 0x52b8
	ctx.r[31].s64 = ctx.r[11].s64 + 21176;
	// 83240540: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83240544: 4BBB2EE5  bl 0x82df3428
	ctx.lr = 0x83240548;
	sub_82DF3428(ctx, base);
	// 83240548: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8324054C: 4BBB2EDD  bl 0x82df3428
	ctx.lr = 0x83240550;
	sub_82DF3428(ctx, base);
	// 83240550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324055C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240568 size=60
    let mut pc: u32 = 0x83240568;
    'dispatch: loop {
        match pc {
            0x83240568 => {
    //   block [0x83240568..0x832405A4)
	// 83240568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240570: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240578: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 8324057C: 3BEB68A8  addi r31, r11, 0x68a8
	ctx.r[31].s64 = ctx.r[11].s64 + 26792;
	// 83240580: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83240584: 4BBB2EA5  bl 0x82df3428
	ctx.lr = 0x83240588;
	sub_82DF3428(ctx, base);
	// 83240588: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8324058C: 4BBB2E9D  bl 0x82df3428
	ctx.lr = 0x83240590;
	sub_82DF3428(ctx, base);
	// 83240590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324059C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832405A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832405A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832405A8 size=60
    let mut pc: u32 = 0x832405A8;
    'dispatch: loop {
        match pc {
            0x832405A8 => {
    //   block [0x832405A8..0x832405E4)
	// 832405A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832405AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832405B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832405B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832405B8: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 832405BC: 3BEB7E98  addi r31, r11, 0x7e98
	ctx.r[31].s64 = ctx.r[11].s64 + 32408;
	// 832405C0: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 832405C4: 4BBB2E65  bl 0x82df3428
	ctx.lr = 0x832405C8;
	sub_82DF3428(ctx, base);
	// 832405C8: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 832405CC: 4BBB2E5D  bl 0x82df3428
	ctx.lr = 0x832405D0;
	sub_82DF3428(ctx, base);
	// 832405D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832405D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832405D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832405DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832405E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832405E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832405E8 size=60
    let mut pc: u32 = 0x832405E8;
    'dispatch: loop {
        match pc {
            0x832405E8 => {
    //   block [0x832405E8..0x83240624)
	// 832405E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832405EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832405F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832405F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832405F8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832405FC: 3BEB9488  addi r31, r11, -0x6b78
	ctx.r[31].s64 = ctx.r[11].s64 + -27512;
	// 83240600: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83240604: 4BBB2E25  bl 0x82df3428
	ctx.lr = 0x83240608;
	sub_82DF3428(ctx, base);
	// 83240608: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8324060C: 4BBB2E1D  bl 0x82df3428
	ctx.lr = 0x83240610;
	sub_82DF3428(ctx, base);
	// 83240610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324061C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240628 size=60
    let mut pc: u32 = 0x83240628;
    'dispatch: loop {
        match pc {
            0x83240628 => {
    //   block [0x83240628..0x83240664)
	// 83240628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324062C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240638: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 8324063C: 3BEB3CC8  addi r31, r11, 0x3cc8
	ctx.r[31].s64 = ctx.r[11].s64 + 15560;
	// 83240640: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83240644: 4BBB2DE5  bl 0x82df3428
	ctx.lr = 0x83240648;
	sub_82DF3428(ctx, base);
	// 83240648: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8324064C: 4BBB2DDD  bl 0x82df3428
	ctx.lr = 0x83240650;
	sub_82DF3428(ctx, base);
	// 83240650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324065C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83240668 size=16
    let mut pc: u32 = 0x83240668;
    'dispatch: loop {
        match pc {
            0x83240668 => {
    //   block [0x83240668..0x83240678)
	// 83240668: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8324066C: 396BAA78  addi r11, r11, -0x5588
	ctx.r[11].s64 = ctx.r[11].s64 + -21896;
	// 83240670: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83240674: 4BBB2DB4  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240678 size=80
    let mut pc: u32 = 0x83240678;
    'dispatch: loop {
        match pc {
            0x83240678 => {
    //   block [0x83240678..0x832406C8)
	// 83240678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324067C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240680: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240684: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240688: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324068C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240690: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83240694: 396BAC20  addi r11, r11, -0x53e0
	ctx.r[11].s64 = ctx.r[11].s64 + -21472;
	// 83240698: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 8324069C: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 832406A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832406A4: 4BBB2D85  bl 0x82df3428
	ctx.lr = 0x832406A8;
	sub_82DF3428(ctx, base);
	// 832406A8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832406AC: 4080FFF0  bge 0x8324069c
	if !ctx.cr[0].lt {
	pc = 0x8324069C; continue 'dispatch;
	}
	// 832406B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832406B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832406B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832406BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832406C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832406C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832406C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832406C8 size=60
    let mut pc: u32 = 0x832406C8;
    'dispatch: loop {
        match pc {
            0x832406C8 => {
    //   block [0x832406C8..0x83240704)
	// 832406C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832406CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832406D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832406D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832406D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832406DC: 3BEBACDC  addi r31, r11, -0x5324
	ctx.r[31].s64 = ctx.r[11].s64 + -21284;
	// 832406E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832406E4: 4BBB2D45  bl 0x82df3428
	ctx.lr = 0x832406E8;
	sub_82DF3428(ctx, base);
	// 832406E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832406EC: 4BBB2D3D  bl 0x82df3428
	ctx.lr = 0x832406F0;
	sub_82DF3428(ctx, base);
	// 832406F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832406F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832406F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832406FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83240708 size=12
    let mut pc: u32 = 0x83240708;
    'dispatch: loop {
        match pc {
            0x83240708 => {
    //   block [0x83240708..0x83240714)
	// 83240708: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8324070C: 386BB4A4  addi r3, r11, -0x4b5c
	ctx.r[3].s64 = ctx.r[11].s64 + -19292;
	// 83240710: 4B6D4DE8  b 0x829154f8
	sub_829154F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240718 size=80
    let mut pc: u32 = 0x83240718;
    'dispatch: loop {
        match pc {
            0x83240718 => {
    //   block [0x83240718..0x83240768)
	// 83240718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324071C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324072C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240730: 3BC0001F  li r30, 0x1f
	ctx.r[30].s64 = 31;
	// 83240734: 396BB4B0  addi r11, r11, -0x4b50
	ctx.r[11].s64 = ctx.r[11].s64 + -19280;
	// 83240738: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 8324073C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 83240740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240744: 4BBB2CE5  bl 0x82df3428
	ctx.lr = 0x83240748;
	sub_82DF3428(ctx, base);
	// 83240748: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8324074C: 4080FFF0  bge 0x8324073c
	if !ctx.cr[0].lt {
	pc = 0x8324073C; continue 'dispatch;
	}
	// 83240750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324075C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240768 size=88
    let mut pc: u32 = 0x83240768;
    'dispatch: loop {
        match pc {
            0x83240768 => {
    //   block [0x83240768..0x832407C0)
	// 83240768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324076C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324077C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240780: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 83240784: 396BB530  addi r11, r11, -0x4ad0
	ctx.r[11].s64 = ctx.r[11].s64 + -19152;
	// 83240788: 3BEB00C0  addi r31, r11, 0xc0
	ctx.r[31].s64 = ctx.r[11].s64 + 192;
	// 8324078C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240790: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240794: 4BBB2C95  bl 0x82df3428
	ctx.lr = 0x83240798;
	sub_82DF3428(ctx, base);
	// 83240798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324079C: 4BBB2C8D  bl 0x82df3428
	ctx.lr = 0x832407A0;
	sub_82DF3428(ctx, base);
	// 832407A0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832407A4: 4080FFE8  bge 0x8324078c
	if !ctx.cr[0].lt {
	pc = 0x8324078C; continue 'dispatch;
	}
	// 832407A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832407AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832407B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832407B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832407B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832407BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832407C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832407C0 size=88
    let mut pc: u32 = 0x832407C0;
    'dispatch: loop {
        match pc {
            0x832407C0 => {
    //   block [0x832407C0..0x83240818)
	// 832407C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832407C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832407C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832407CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832407D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832407D4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832407D8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832407DC: 396BB5F0  addi r11, r11, -0x4a10
	ctx.r[11].s64 = ctx.r[11].s64 + -18960;
	// 832407E0: 3BEB00C0  addi r31, r11, 0xc0
	ctx.r[31].s64 = ctx.r[11].s64 + 192;
	// 832407E4: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 832407E8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832407EC: 4BBB2C3D  bl 0x82df3428
	ctx.lr = 0x832407F0;
	sub_82DF3428(ctx, base);
	// 832407F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832407F4: 4BBB2C35  bl 0x82df3428
	ctx.lr = 0x832407F8;
	sub_82DF3428(ctx, base);
	// 832407F8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832407FC: 4080FFE8  bge 0x832407e4
	if !ctx.cr[0].lt {
	pc = 0x832407E4; continue 'dispatch;
	}
	// 83240800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324080C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240818 size=88
    let mut pc: u32 = 0x83240818;
    'dispatch: loop {
        match pc {
            0x83240818 => {
    //   block [0x83240818..0x83240870)
	// 83240818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324081C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324082C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240830: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 83240834: 396BB6B0  addi r11, r11, -0x4950
	ctx.r[11].s64 = ctx.r[11].s64 + -18768;
	// 83240838: 3BEB0180  addi r31, r11, 0x180
	ctx.r[31].s64 = ctx.r[11].s64 + 384;
	// 8324083C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240840: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240844: 4BBB2BE5  bl 0x82df3428
	ctx.lr = 0x83240848;
	sub_82DF3428(ctx, base);
	// 83240848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324084C: 4BBB2BDD  bl 0x82df3428
	ctx.lr = 0x83240850;
	sub_82DF3428(ctx, base);
	// 83240850: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240854: 4080FFE8  bge 0x8324083c
	if !ctx.cr[0].lt {
	pc = 0x8324083C; continue 'dispatch;
	}
	// 83240858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324085C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240864: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240870 size=88
    let mut pc: u32 = 0x83240870;
    'dispatch: loop {
        match pc {
            0x83240870 => {
    //   block [0x83240870..0x832408C8)
	// 83240870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240878: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324087C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240884: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240888: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 8324088C: 396BB830  addi r11, r11, -0x47d0
	ctx.r[11].s64 = ctx.r[11].s64 + -18384;
	// 83240890: 3BEB0180  addi r31, r11, 0x180
	ctx.r[31].s64 = ctx.r[11].s64 + 384;
	// 83240894: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240898: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324089C: 4BBB2B8D  bl 0x82df3428
	ctx.lr = 0x832408A0;
	sub_82DF3428(ctx, base);
	// 832408A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832408A4: 4BBB2B85  bl 0x82df3428
	ctx.lr = 0x832408A8;
	sub_82DF3428(ctx, base);
	// 832408A8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832408AC: 4080FFE8  bge 0x83240894
	if !ctx.cr[0].lt {
	pc = 0x83240894; continue 'dispatch;
	}
	// 832408B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832408B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832408B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832408BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832408C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832408C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832408C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832408C8 size=60
    let mut pc: u32 = 0x832408C8;
    'dispatch: loop {
        match pc {
            0x832408C8 => {
    //   block [0x832408C8..0x83240904)
	// 832408C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832408CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832408D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832408D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832408D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832408DC: 3BEBB9B0  addi r31, r11, -0x4650
	ctx.r[31].s64 = ctx.r[11].s64 + -18000;
	// 832408E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832408E4: 4BBB2B45  bl 0x82df3428
	ctx.lr = 0x832408E8;
	sub_82DF3428(ctx, base);
	// 832408E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832408EC: 4BBB2B3D  bl 0x82df3428
	ctx.lr = 0x832408F0;
	sub_82DF3428(ctx, base);
	// 832408F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832408F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832408F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832408FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240908 size=60
    let mut pc: u32 = 0x83240908;
    'dispatch: loop {
        match pc {
            0x83240908 => {
    //   block [0x83240908..0x83240944)
	// 83240908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240918: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8324091C: 3BEBB9E0  addi r31, r11, -0x4620
	ctx.r[31].s64 = ctx.r[11].s64 + -17952;
	// 83240920: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240924: 4BBB2B05  bl 0x82df3428
	ctx.lr = 0x83240928;
	sub_82DF3428(ctx, base);
	// 83240928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324092C: 4BBB2AFD  bl 0x82df3428
	ctx.lr = 0x83240930;
	sub_82DF3428(ctx, base);
	// 83240930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324093C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240948 size=88
    let mut pc: u32 = 0x83240948;
    'dispatch: loop {
        match pc {
            0x83240948 => {
    //   block [0x83240948..0x832409A0)
	// 83240948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324094C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324095C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240960: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83240964: 396BBA10  addi r11, r11, -0x45f0
	ctx.r[11].s64 = ctx.r[11].s64 + -17904;
	// 83240968: 3BEB0060  addi r31, r11, 0x60
	ctx.r[31].s64 = ctx.r[11].s64 + 96;
	// 8324096C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240970: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240974: 4BBB2AB5  bl 0x82df3428
	ctx.lr = 0x83240978;
	sub_82DF3428(ctx, base);
	// 83240978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324097C: 4BBB2AAD  bl 0x82df3428
	ctx.lr = 0x83240980;
	sub_82DF3428(ctx, base);
	// 83240980: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240984: 4080FFE8  bge 0x8324096c
	if !ctx.cr[0].lt {
	pc = 0x8324096C; continue 'dispatch;
	}
	// 83240988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324098C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240994: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324099C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832409A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832409A0 size=88
    let mut pc: u32 = 0x832409A0;
    'dispatch: loop {
        match pc {
            0x832409A0 => {
    //   block [0x832409A0..0x832409F8)
	// 832409A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832409A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832409A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832409AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832409B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832409B4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832409B8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832409BC: 396BBA70  addi r11, r11, -0x4590
	ctx.r[11].s64 = ctx.r[11].s64 + -17808;
	// 832409C0: 3BEB0060  addi r31, r11, 0x60
	ctx.r[31].s64 = ctx.r[11].s64 + 96;
	// 832409C4: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 832409C8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832409CC: 4BBB2A5D  bl 0x82df3428
	ctx.lr = 0x832409D0;
	sub_82DF3428(ctx, base);
	// 832409D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832409D4: 4BBB2A55  bl 0x82df3428
	ctx.lr = 0x832409D8;
	sub_82DF3428(ctx, base);
	// 832409D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832409DC: 4080FFE8  bge 0x832409c4
	if !ctx.cr[0].lt {
	pc = 0x832409C4; continue 'dispatch;
	}
	// 832409E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832409E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832409E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832409EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832409F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832409F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832409F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832409F8 size=60
    let mut pc: u32 = 0x832409F8;
    'dispatch: loop {
        match pc {
            0x832409F8 => {
    //   block [0x832409F8..0x83240A34)
	// 832409F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832409FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240A04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240A08: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240A0C: 3BEBBAD0  addi r31, r11, -0x4530
	ctx.r[31].s64 = ctx.r[11].s64 + -17712;
	// 83240A10: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240A14: 4BBB2A15  bl 0x82df3428
	ctx.lr = 0x83240A18;
	sub_82DF3428(ctx, base);
	// 83240A18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240A1C: 4BBB2A0D  bl 0x82df3428
	ctx.lr = 0x83240A20;
	sub_82DF3428(ctx, base);
	// 83240A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240A2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240A38 size=60
    let mut pc: u32 = 0x83240A38;
    'dispatch: loop {
        match pc {
            0x83240A38 => {
    //   block [0x83240A38..0x83240A74)
	// 83240A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240A48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240A4C: 3BEBBB00  addi r31, r11, -0x4500
	ctx.r[31].s64 = ctx.r[11].s64 + -17664;
	// 83240A50: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240A54: 4BBB29D5  bl 0x82df3428
	ctx.lr = 0x83240A58;
	sub_82DF3428(ctx, base);
	// 83240A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240A5C: 4BBB29CD  bl 0x82df3428
	ctx.lr = 0x83240A60;
	sub_82DF3428(ctx, base);
	// 83240A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240A78 size=60
    let mut pc: u32 = 0x83240A78;
    'dispatch: loop {
        match pc {
            0x83240A78 => {
    //   block [0x83240A78..0x83240AB4)
	// 83240A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240A84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240A88: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240A8C: 3BEBBB30  addi r31, r11, -0x44d0
	ctx.r[31].s64 = ctx.r[11].s64 + -17616;
	// 83240A90: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240A94: 4BBB2995  bl 0x82df3428
	ctx.lr = 0x83240A98;
	sub_82DF3428(ctx, base);
	// 83240A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240A9C: 4BBB298D  bl 0x82df3428
	ctx.lr = 0x83240AA0;
	sub_82DF3428(ctx, base);
	// 83240AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240AB8 size=60
    let mut pc: u32 = 0x83240AB8;
    'dispatch: loop {
        match pc {
            0x83240AB8 => {
    //   block [0x83240AB8..0x83240AF4)
	// 83240AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240AC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240AC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240AC8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240ACC: 3BEBBB60  addi r31, r11, -0x44a0
	ctx.r[31].s64 = ctx.r[11].s64 + -17568;
	// 83240AD0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240AD4: 4BBB2955  bl 0x82df3428
	ctx.lr = 0x83240AD8;
	sub_82DF3428(ctx, base);
	// 83240AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240ADC: 4BBB294D  bl 0x82df3428
	ctx.lr = 0x83240AE0;
	sub_82DF3428(ctx, base);
	// 83240AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240AEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240AF8 size=60
    let mut pc: u32 = 0x83240AF8;
    'dispatch: loop {
        match pc {
            0x83240AF8 => {
    //   block [0x83240AF8..0x83240B34)
	// 83240AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240B00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240B04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240B08: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240B0C: 3BEBBB90  addi r31, r11, -0x4470
	ctx.r[31].s64 = ctx.r[11].s64 + -17520;
	// 83240B10: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240B14: 4BBB2915  bl 0x82df3428
	ctx.lr = 0x83240B18;
	sub_82DF3428(ctx, base);
	// 83240B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240B1C: 4BBB290D  bl 0x82df3428
	ctx.lr = 0x83240B20;
	sub_82DF3428(ctx, base);
	// 83240B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240B2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240B38 size=60
    let mut pc: u32 = 0x83240B38;
    'dispatch: loop {
        match pc {
            0x83240B38 => {
    //   block [0x83240B38..0x83240B74)
	// 83240B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240B40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240B44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240B48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240B4C: 3BEBBBC0  addi r31, r11, -0x4440
	ctx.r[31].s64 = ctx.r[11].s64 + -17472;
	// 83240B50: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240B54: 4BBB28D5  bl 0x82df3428
	ctx.lr = 0x83240B58;
	sub_82DF3428(ctx, base);
	// 83240B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240B5C: 4BBB28CD  bl 0x82df3428
	ctx.lr = 0x83240B60;
	sub_82DF3428(ctx, base);
	// 83240B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240B6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240B78 size=88
    let mut pc: u32 = 0x83240B78;
    'dispatch: loop {
        match pc {
            0x83240B78 => {
    //   block [0x83240B78..0x83240BD0)
	// 83240B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240B80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240B84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240B88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240B8C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240B90: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 83240B94: 396BBBF0  addi r11, r11, -0x4410
	ctx.r[11].s64 = ctx.r[11].s64 + -17424;
	// 83240B98: 3BEB00C0  addi r31, r11, 0xc0
	ctx.r[31].s64 = ctx.r[11].s64 + 192;
	// 83240B9C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240BA0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240BA4: 4BBB2885  bl 0x82df3428
	ctx.lr = 0x83240BA8;
	sub_82DF3428(ctx, base);
	// 83240BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240BAC: 4BBB287D  bl 0x82df3428
	ctx.lr = 0x83240BB0;
	sub_82DF3428(ctx, base);
	// 83240BB0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240BB4: 4080FFE8  bge 0x83240b9c
	if !ctx.cr[0].lt {
	pc = 0x83240B9C; continue 'dispatch;
	}
	// 83240BB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240BC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240BC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240BD0 size=88
    let mut pc: u32 = 0x83240BD0;
    'dispatch: loop {
        match pc {
            0x83240BD0 => {
    //   block [0x83240BD0..0x83240C28)
	// 83240BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240BD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240BDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240BE4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240BE8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 83240BEC: 396BBCB0  addi r11, r11, -0x4350
	ctx.r[11].s64 = ctx.r[11].s64 + -17232;
	// 83240BF0: 3BEB00C0  addi r31, r11, 0xc0
	ctx.r[31].s64 = ctx.r[11].s64 + 192;
	// 83240BF4: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240BF8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240BFC: 4BBB282D  bl 0x82df3428
	ctx.lr = 0x83240C00;
	sub_82DF3428(ctx, base);
	// 83240C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240C04: 4BBB2825  bl 0x82df3428
	ctx.lr = 0x83240C08;
	sub_82DF3428(ctx, base);
	// 83240C08: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240C0C: 4080FFE8  bge 0x83240bf4
	if !ctx.cr[0].lt {
	pc = 0x83240BF4; continue 'dispatch;
	}
	// 83240C10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240C1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240C28 size=88
    let mut pc: u32 = 0x83240C28;
    'dispatch: loop {
        match pc {
            0x83240C28 => {
    //   block [0x83240C28..0x83240C80)
	// 83240C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240C3C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240C40: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 83240C44: 396BBD70  addi r11, r11, -0x4290
	ctx.r[11].s64 = ctx.r[11].s64 + -17040;
	// 83240C48: 3BEB0120  addi r31, r11, 0x120
	ctx.r[31].s64 = ctx.r[11].s64 + 288;
	// 83240C4C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240C50: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240C54: 4BBB27D5  bl 0x82df3428
	ctx.lr = 0x83240C58;
	sub_82DF3428(ctx, base);
	// 83240C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240C5C: 4BBB27CD  bl 0x82df3428
	ctx.lr = 0x83240C60;
	sub_82DF3428(ctx, base);
	// 83240C60: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240C64: 4080FFE8  bge 0x83240c4c
	if !ctx.cr[0].lt {
	pc = 0x83240C4C; continue 'dispatch;
	}
	// 83240C68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240C74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240C78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240C80 size=88
    let mut pc: u32 = 0x83240C80;
    'dispatch: loop {
        match pc {
            0x83240C80 => {
    //   block [0x83240C80..0x83240CD8)
	// 83240C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240C88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240C8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240C90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240C94: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240C98: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 83240C9C: 396BBE90  addi r11, r11, -0x4170
	ctx.r[11].s64 = ctx.r[11].s64 + -16752;
	// 83240CA0: 3BEB0120  addi r31, r11, 0x120
	ctx.r[31].s64 = ctx.r[11].s64 + 288;
	// 83240CA4: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240CA8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240CAC: 4BBB277D  bl 0x82df3428
	ctx.lr = 0x83240CB0;
	sub_82DF3428(ctx, base);
	// 83240CB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240CB4: 4BBB2775  bl 0x82df3428
	ctx.lr = 0x83240CB8;
	sub_82DF3428(ctx, base);
	// 83240CB8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240CBC: 4080FFE8  bge 0x83240ca4
	if !ctx.cr[0].lt {
	pc = 0x83240CA4; continue 'dispatch;
	}
	// 83240CC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240CCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240CD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240CD8 size=88
    let mut pc: u32 = 0x83240CD8;
    'dispatch: loop {
        match pc {
            0x83240CD8 => {
    //   block [0x83240CD8..0x83240D30)
	// 83240CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240CE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240CE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240CE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240CEC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240CF0: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 83240CF4: 396BBFB0  addi r11, r11, -0x4050
	ctx.r[11].s64 = ctx.r[11].s64 + -16464;
	// 83240CF8: 3BEB01E0  addi r31, r11, 0x1e0
	ctx.r[31].s64 = ctx.r[11].s64 + 480;
	// 83240CFC: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240D00: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240D04: 4BBB2725  bl 0x82df3428
	ctx.lr = 0x83240D08;
	sub_82DF3428(ctx, base);
	// 83240D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240D0C: 4BBB271D  bl 0x82df3428
	ctx.lr = 0x83240D10;
	sub_82DF3428(ctx, base);
	// 83240D10: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240D14: 4080FFE8  bge 0x83240cfc
	if !ctx.cr[0].lt {
	pc = 0x83240CFC; continue 'dispatch;
	}
	// 83240D18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240D24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240D30 size=88
    let mut pc: u32 = 0x83240D30;
    'dispatch: loop {
        match pc {
            0x83240D30 => {
    //   block [0x83240D30..0x83240D88)
	// 83240D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240D44: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240D48: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 83240D4C: 396BC190  addi r11, r11, -0x3e70
	ctx.r[11].s64 = ctx.r[11].s64 + -15984;
	// 83240D50: 3BEB01E0  addi r31, r11, 0x1e0
	ctx.r[31].s64 = ctx.r[11].s64 + 480;
	// 83240D54: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240D58: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240D5C: 4BBB26CD  bl 0x82df3428
	ctx.lr = 0x83240D60;
	sub_82DF3428(ctx, base);
	// 83240D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240D64: 4BBB26C5  bl 0x82df3428
	ctx.lr = 0x83240D68;
	sub_82DF3428(ctx, base);
	// 83240D68: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240D6C: 4080FFE8  bge 0x83240d54
	if !ctx.cr[0].lt {
	pc = 0x83240D54; continue 'dispatch;
	}
	// 83240D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240D88 size=88
    let mut pc: u32 = 0x83240D88;
    'dispatch: loop {
        match pc {
            0x83240D88 => {
    //   block [0x83240D88..0x83240DE0)
	// 83240D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240D9C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240DA0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83240DA4: 396BC370  addi r11, r11, -0x3c90
	ctx.r[11].s64 = ctx.r[11].s64 + -15504;
	// 83240DA8: 3BEB0060  addi r31, r11, 0x60
	ctx.r[31].s64 = ctx.r[11].s64 + 96;
	// 83240DAC: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240DB0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240DB4: 4BBB2675  bl 0x82df3428
	ctx.lr = 0x83240DB8;
	sub_82DF3428(ctx, base);
	// 83240DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240DBC: 4BBB266D  bl 0x82df3428
	ctx.lr = 0x83240DC0;
	sub_82DF3428(ctx, base);
	// 83240DC0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240DC4: 4080FFE8  bge 0x83240dac
	if !ctx.cr[0].lt {
	pc = 0x83240DAC; continue 'dispatch;
	}
	// 83240DC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240DD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240DD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240DE0 size=88
    let mut pc: u32 = 0x83240DE0;
    'dispatch: loop {
        match pc {
            0x83240DE0 => {
    //   block [0x83240DE0..0x83240E38)
	// 83240DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240DE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240DEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240DF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240DF4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240DF8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83240DFC: 396BC3D0  addi r11, r11, -0x3c30
	ctx.r[11].s64 = ctx.r[11].s64 + -15408;
	// 83240E00: 3BEB0060  addi r31, r11, 0x60
	ctx.r[31].s64 = ctx.r[11].s64 + 96;
	// 83240E04: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83240E08: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240E0C: 4BBB261D  bl 0x82df3428
	ctx.lr = 0x83240E10;
	sub_82DF3428(ctx, base);
	// 83240E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240E14: 4BBB2615  bl 0x82df3428
	ctx.lr = 0x83240E18;
	sub_82DF3428(ctx, base);
	// 83240E18: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240E1C: 4080FFE8  bge 0x83240e04
	if !ctx.cr[0].lt {
	pc = 0x83240E04; continue 'dispatch;
	}
	// 83240E20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240E2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83240E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240E38 size=60
    let mut pc: u32 = 0x83240E38;
    'dispatch: loop {
        match pc {
            0x83240E38 => {
    //   block [0x83240E38..0x83240E74)
	// 83240E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240E48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240E4C: 3BEBC430  addi r31, r11, -0x3bd0
	ctx.r[31].s64 = ctx.r[11].s64 + -15312;
	// 83240E50: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240E54: 4BBB25D5  bl 0x82df3428
	ctx.lr = 0x83240E58;
	sub_82DF3428(ctx, base);
	// 83240E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240E5C: 4BBB25CD  bl 0x82df3428
	ctx.lr = 0x83240E60;
	sub_82DF3428(ctx, base);
	// 83240E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240E78 size=60
    let mut pc: u32 = 0x83240E78;
    'dispatch: loop {
        match pc {
            0x83240E78 => {
    //   block [0x83240E78..0x83240EB4)
	// 83240E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240E88: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240E8C: 3BEBC460  addi r31, r11, -0x3ba0
	ctx.r[31].s64 = ctx.r[11].s64 + -15264;
	// 83240E90: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240E94: 4BBB2595  bl 0x82df3428
	ctx.lr = 0x83240E98;
	sub_82DF3428(ctx, base);
	// 83240E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240E9C: 4BBB258D  bl 0x82df3428
	ctx.lr = 0x83240EA0;
	sub_82DF3428(ctx, base);
	// 83240EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240EAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240EB8 size=60
    let mut pc: u32 = 0x83240EB8;
    'dispatch: loop {
        match pc {
            0x83240EB8 => {
    //   block [0x83240EB8..0x83240EF4)
	// 83240EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240EC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240EC8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240ECC: 3BEBC490  addi r31, r11, -0x3b70
	ctx.r[31].s64 = ctx.r[11].s64 + -15216;
	// 83240ED0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240ED4: 4BBB2555  bl 0x82df3428
	ctx.lr = 0x83240ED8;
	sub_82DF3428(ctx, base);
	// 83240ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240EDC: 4BBB254D  bl 0x82df3428
	ctx.lr = 0x83240EE0;
	sub_82DF3428(ctx, base);
	// 83240EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240EF8 size=60
    let mut pc: u32 = 0x83240EF8;
    'dispatch: loop {
        match pc {
            0x83240EF8 => {
    //   block [0x83240EF8..0x83240F34)
	// 83240EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240F00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240F04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240F08: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240F0C: 3BEBC4C0  addi r31, r11, -0x3b40
	ctx.r[31].s64 = ctx.r[11].s64 + -15168;
	// 83240F10: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240F14: 4BBB2515  bl 0x82df3428
	ctx.lr = 0x83240F18;
	sub_82DF3428(ctx, base);
	// 83240F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240F1C: 4BBB250D  bl 0x82df3428
	ctx.lr = 0x83240F20;
	sub_82DF3428(ctx, base);
	// 83240F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240F2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240F38 size=60
    let mut pc: u32 = 0x83240F38;
    'dispatch: loop {
        match pc {
            0x83240F38 => {
    //   block [0x83240F38..0x83240F74)
	// 83240F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240F48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240F4C: 3BEBC4F0  addi r31, r11, -0x3b10
	ctx.r[31].s64 = ctx.r[11].s64 + -15120;
	// 83240F50: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240F54: 4BBB24D5  bl 0x82df3428
	ctx.lr = 0x83240F58;
	sub_82DF3428(ctx, base);
	// 83240F58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240F5C: 4BBB24CD  bl 0x82df3428
	ctx.lr = 0x83240F60;
	sub_82DF3428(ctx, base);
	// 83240F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240F78 size=60
    let mut pc: u32 = 0x83240F78;
    'dispatch: loop {
        match pc {
            0x83240F78 => {
    //   block [0x83240F78..0x83240FB4)
	// 83240F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240F80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240F84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240F88: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240F8C: 3BEBC520  addi r31, r11, -0x3ae0
	ctx.r[31].s64 = ctx.r[11].s64 + -15072;
	// 83240F90: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83240F94: 4BBB2495  bl 0x82df3428
	ctx.lr = 0x83240F98;
	sub_82DF3428(ctx, base);
	// 83240F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240F9C: 4BBB248D  bl 0x82df3428
	ctx.lr = 0x83240FA0;
	sub_82DF3428(ctx, base);
	// 83240FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83240FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240FAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83240FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83240FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83240FB8 size=80
    let mut pc: u32 = 0x83240FB8;
    'dispatch: loop {
        match pc {
            0x83240FB8 => {
    //   block [0x83240FB8..0x83241008)
	// 83240FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83240FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83240FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83240FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83240FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83240FCC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83240FD0: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 83240FD4: 396BC860  addi r11, r11, -0x37a0
	ctx.r[11].s64 = ctx.r[11].s64 + -14240;
	// 83240FD8: 3BEB0198  addi r31, r11, 0x198
	ctx.r[31].s64 = ctx.r[11].s64 + 408;
	// 83240FDC: 3BFFFFD8  addi r31, r31, -0x28
	ctx.r[31].s64 = ctx.r[31].s64 + -40;
	// 83240FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83240FE4: 4B087CD5  bl 0x822c8cb8
	ctx.lr = 0x83240FE8;
	sub_822C8CB8(ctx, base);
	// 83240FE8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83240FEC: 4080FFF0  bge 0x83240fdc
	if !ctx.cr[0].lt {
	pc = 0x83240FDC; continue 'dispatch;
	}
	// 83240FF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83240FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83240FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83240FFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241008 size=20
    let mut pc: u32 = 0x83241008;
    'dispatch: loop {
        match pc {
            0x83241008 => {
    //   block [0x83241008..0x8324101C)
	// 83241008: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8324100C: 396BD194  addi r11, r11, -0x2e6c
	ctx.r[11].s64 = ctx.r[11].s64 + -11884;
	// 83241010: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83241014: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83241018: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324101C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324101C size=40
    let mut pc: u32 = 0x8324101C;
    'dispatch: loop {
        match pc {
            0x8324101C => {
    //   block [0x8324101C..0x83241044)
	// 8324101C: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 83241020: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 83241024: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83241028: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8324102C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83241030: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83241034: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83241038: 4082FFE8  bne 0x83241020
	if !ctx.cr[0].eq {
	pc = 0x83241020; continue 'dispatch;
	}
	// 8324103C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83241040: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241044(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241044 size=16
    let mut pc: u32 = 0x83241044;
    'dispatch: loop {
        match pc {
            0x83241044 => {
    //   block [0x83241044..0x83241054)
	// 83241044: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83241048: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8324104C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83241050: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241054(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241054 size=4
    let mut pc: u32 = 0x83241054;
    'dispatch: loop {
        match pc {
            0x83241054 => {
    //   block [0x83241054..0x83241058)
	// 83241054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241058 size=88
    let mut pc: u32 = 0x83241058;
    'dispatch: loop {
        match pc {
            0x83241058 => {
    //   block [0x83241058..0x832410B0)
	// 83241058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83241064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324106C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241070: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 83241074: 396BD938  addi r11, r11, -0x26c8
	ctx.r[11].s64 = ctx.r[11].s64 + -9928;
	// 83241078: 3BEB0120  addi r31, r11, 0x120
	ctx.r[31].s64 = ctx.r[11].s64 + 288;
	// 8324107C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83241080: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83241084: 4BBB23A5  bl 0x82df3428
	ctx.lr = 0x83241088;
	sub_82DF3428(ctx, base);
	// 83241088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324108C: 4BBB239D  bl 0x82df3428
	ctx.lr = 0x83241090;
	sub_82DF3428(ctx, base);
	// 83241090: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241094: 4080FFE8  bge 0x8324107c
	if !ctx.cr[0].lt {
	pc = 0x8324107C; continue 'dispatch;
	}
	// 83241098: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324109C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832410A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832410A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832410A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832410AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832410B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832410B0 size=88
    let mut pc: u32 = 0x832410B0;
    'dispatch: loop {
        match pc {
            0x832410B0 => {
    //   block [0x832410B0..0x83241108)
	// 832410B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832410B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832410B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832410BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832410C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832410C4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832410C8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832410CC: 396BDB18  addi r11, r11, -0x24e8
	ctx.r[11].s64 = ctx.r[11].s64 + -9448;
	// 832410D0: 3BEB00F0  addi r31, r11, 0xf0
	ctx.r[31].s64 = ctx.r[11].s64 + 240;
	// 832410D4: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 832410D8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832410DC: 4BBB234D  bl 0x82df3428
	ctx.lr = 0x832410E0;
	sub_82DF3428(ctx, base);
	// 832410E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832410E4: 4BBB2345  bl 0x82df3428
	ctx.lr = 0x832410E8;
	sub_82DF3428(ctx, base);
	// 832410E8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832410EC: 4080FFE8  bge 0x832410d4
	if !ctx.cr[0].lt {
	pc = 0x832410D4; continue 'dispatch;
	}
	// 832410F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832410F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832410F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832410FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241108 size=88
    let mut pc: u32 = 0x83241108;
    'dispatch: loop {
        match pc {
            0x83241108 => {
    //   block [0x83241108..0x83241160)
	// 83241108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83241114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324111C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241120: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 83241124: 396BDC08  addi r11, r11, -0x23f8
	ctx.r[11].s64 = ctx.r[11].s64 + -9208;
	// 83241128: 3BEB00F0  addi r31, r11, 0xf0
	ctx.r[31].s64 = ctx.r[11].s64 + 240;
	// 8324112C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83241130: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83241134: 4BBB22F5  bl 0x82df3428
	ctx.lr = 0x83241138;
	sub_82DF3428(ctx, base);
	// 83241138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324113C: 4BBB22ED  bl 0x82df3428
	ctx.lr = 0x83241140;
	sub_82DF3428(ctx, base);
	// 83241140: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241144: 4080FFE8  bge 0x8324112c
	if !ctx.cr[0].lt {
	pc = 0x8324112C; continue 'dispatch;
	}
	// 83241148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324114C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241154: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324115C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241160 size=80
    let mut pc: u32 = 0x83241160;
    'dispatch: loop {
        match pc {
            0x83241160 => {
    //   block [0x83241160..0x832411B0)
	// 83241160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324116C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241174: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241178: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 8324117C: 396BDD1C  addi r11, r11, -0x22e4
	ctx.r[11].s64 = ctx.r[11].s64 + -8932;
	// 83241180: 3BEB0024  addi r31, r11, 0x24
	ctx.r[31].s64 = ctx.r[11].s64 + 36;
	// 83241184: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 83241188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324118C: 4BBB229D  bl 0x82df3428
	ctx.lr = 0x83241190;
	sub_82DF3428(ctx, base);
	// 83241190: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241194: 4080FFF0  bge 0x83241184
	if !ctx.cr[0].lt {
	pc = 0x83241184; continue 'dispatch;
	}
	// 83241198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324119C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832411A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832411A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832411A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832411AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832411B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832411B0 size=80
    let mut pc: u32 = 0x832411B0;
    'dispatch: loop {
        match pc {
            0x832411B0 => {
    //   block [0x832411B0..0x83241200)
	// 832411B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832411B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832411B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832411BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832411C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832411C4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832411C8: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 832411CC: 396BDD3C  addi r11, r11, -0x22c4
	ctx.r[11].s64 = ctx.r[11].s64 + -8900;
	// 832411D0: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 832411D4: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832411D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832411DC: 4BBB224D  bl 0x82df3428
	ctx.lr = 0x832411E0;
	sub_82DF3428(ctx, base);
	// 832411E0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832411E4: 4080FFF0  bge 0x832411d4
	if !ctx.cr[0].lt {
	pc = 0x832411D4; continue 'dispatch;
	}
	// 832411E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832411EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832411F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832411F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832411F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832411FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241200 size=20
    let mut pc: u32 = 0x83241200;
    'dispatch: loop {
        match pc {
            0x83241200 => {
    //   block [0x83241200..0x83241214)
	// 83241200: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241204: 396BDDC4  addi r11, r11, -0x223c
	ctx.r[11].s64 = ctx.r[11].s64 + -8764;
	// 83241208: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324120C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83241210: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241214(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241214 size=40
    let mut pc: u32 = 0x83241214;
    'dispatch: loop {
        match pc {
            0x83241214 => {
    //   block [0x83241214..0x8324123C)
	// 83241214: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 83241218: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8324121C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83241220: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 83241224: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83241228: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324122C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83241230: 4082FFE8  bne 0x83241218
	if !ctx.cr[0].eq {
	pc = 0x83241218; continue 'dispatch;
	}
	// 83241234: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83241238: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324123C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324123C size=16
    let mut pc: u32 = 0x8324123C;
    'dispatch: loop {
        match pc {
            0x8324123C => {
    //   block [0x8324123C..0x8324124C)
	// 8324123C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83241240: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83241244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83241248: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324124C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324124C size=4
    let mut pc: u32 = 0x8324124C;
    'dispatch: loop {
        match pc {
            0x8324124C => {
    //   block [0x8324124C..0x83241250)
	// 8324124C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241250 size=20
    let mut pc: u32 = 0x83241250;
    'dispatch: loop {
        match pc {
            0x83241250 => {
    //   block [0x83241250..0x83241264)
	// 83241250: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241254: 396BDDCC  addi r11, r11, -0x2234
	ctx.r[11].s64 = ctx.r[11].s64 + -8756;
	// 83241258: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324125C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83241260: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241264 size=40
    let mut pc: u32 = 0x83241264;
    'dispatch: loop {
        match pc {
            0x83241264 => {
    //   block [0x83241264..0x8324128C)
	// 83241264: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 83241268: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8324126C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83241270: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 83241274: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83241278: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324127C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83241280: 4082FFE8  bne 0x83241268
	if !ctx.cr[0].eq {
	pc = 0x83241268; continue 'dispatch;
	}
	// 83241284: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83241288: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324128C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324128C size=16
    let mut pc: u32 = 0x8324128C;
    'dispatch: loop {
        match pc {
            0x8324128C => {
    //   block [0x8324128C..0x8324129C)
	// 8324128C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83241290: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83241294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83241298: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324129C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324129C size=4
    let mut pc: u32 = 0x8324129C;
    'dispatch: loop {
        match pc {
            0x8324129C => {
    //   block [0x8324129C..0x832412A0)
	// 8324129C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832412A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832412A0 size=20
    let mut pc: u32 = 0x832412A0;
    'dispatch: loop {
        match pc {
            0x832412A0 => {
    //   block [0x832412A0..0x832412B4)
	// 832412A0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832412A4: 396BDDE0  addi r11, r11, -0x2220
	ctx.r[11].s64 = ctx.r[11].s64 + -8736;
	// 832412A8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832412AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832412B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832412B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832412B4 size=40
    let mut pc: u32 = 0x832412B4;
    'dispatch: loop {
        match pc {
            0x832412B4 => {
    //   block [0x832412B4..0x832412DC)
	// 832412B4: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 832412B8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832412BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832412C0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832412C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832412C8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832412CC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832412D0: 4082FFE8  bne 0x832412b8
	if !ctx.cr[0].eq {
	pc = 0x832412B8; continue 'dispatch;
	}
	// 832412D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832412D8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832412DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832412DC size=16
    let mut pc: u32 = 0x832412DC;
    'dispatch: loop {
        match pc {
            0x832412DC => {
    //   block [0x832412DC..0x832412EC)
	// 832412DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832412E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832412E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832412E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832412EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832412EC size=4
    let mut pc: u32 = 0x832412EC;
    'dispatch: loop {
        match pc {
            0x832412EC => {
    //   block [0x832412EC..0x832412F0)
	// 832412EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832412F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832412F0 size=12
    let mut pc: u32 = 0x832412F0;
    'dispatch: loop {
        match pc {
            0x832412F0 => {
    //   block [0x832412F0..0x832412FC)
	// 832412F0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832412F4: 386BDFA4  addi r3, r11, -0x205c
	ctx.r[3].s64 = ctx.r[11].s64 + -8284;
	// 832412F8: 4BBB2130  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241300 size=80
    let mut pc: u32 = 0x83241300;
    'dispatch: loop {
        match pc {
            0x83241300 => {
    //   block [0x83241300..0x83241350)
	// 83241300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324130C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241314: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241318: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8324131C: 396BDF98  addi r11, r11, -0x2068
	ctx.r[11].s64 = ctx.r[11].s64 + -8296;
	// 83241320: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 83241324: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 83241328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324132C: 4BBB20FD  bl 0x82df3428
	ctx.lr = 0x83241330;
	sub_82DF3428(ctx, base);
	// 83241330: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241334: 4080FFF0  bge 0x83241324
	if !ctx.cr[0].lt {
	pc = 0x83241324; continue 'dispatch;
	}
	// 83241338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324133C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324134C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241350 size=80
    let mut pc: u32 = 0x83241350;
    'dispatch: loop {
        match pc {
            0x83241350 => {
    //   block [0x83241350..0x832413A0)
	// 83241350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324135C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241364: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241368: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8324136C: 396BE008  addi r11, r11, -0x1ff8
	ctx.r[11].s64 = ctx.r[11].s64 + -8184;
	// 83241370: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 83241374: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 83241378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324137C: 4BBB20AD  bl 0x82df3428
	ctx.lr = 0x83241380;
	sub_82DF3428(ctx, base);
	// 83241380: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241384: 4080FFF0  bge 0x83241374
	if !ctx.cr[0].lt {
	pc = 0x83241374; continue 'dispatch;
	}
	// 83241388: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324138C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241394: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324139C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832413A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832413A0 size=60
    let mut pc: u32 = 0x832413A0;
    'dispatch: loop {
        match pc {
            0x832413A0 => {
    //   block [0x832413A0..0x832413DC)
	// 832413A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832413A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832413A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832413AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832413B0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832413B4: 3BEBE068  addi r31, r11, -0x1f98
	ctx.r[31].s64 = ctx.r[11].s64 + -8088;
	// 832413B8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832413BC: 4BBB206D  bl 0x82df3428
	ctx.lr = 0x832413C0;
	sub_82DF3428(ctx, base);
	// 832413C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832413C4: 4BBB2065  bl 0x82df3428
	ctx.lr = 0x832413C8;
	sub_82DF3428(ctx, base);
	// 832413C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832413CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832413D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832413D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832413D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832413E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832413E0 size=60
    let mut pc: u32 = 0x832413E0;
    'dispatch: loop {
        match pc {
            0x832413E0 => {
    //   block [0x832413E0..0x8324141C)
	// 832413E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832413E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832413E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832413EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832413F0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832413F4: 3BEBE098  addi r31, r11, -0x1f68
	ctx.r[31].s64 = ctx.r[11].s64 + -8040;
	// 832413F8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832413FC: 4BBB202D  bl 0x82df3428
	ctx.lr = 0x83241400;
	sub_82DF3428(ctx, base);
	// 83241400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241404: 4BBB2025  bl 0x82df3428
	ctx.lr = 0x83241408;
	sub_82DF3428(ctx, base);
	// 83241408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324140C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241420 size=88
    let mut pc: u32 = 0x83241420;
    'dispatch: loop {
        match pc {
            0x83241420 => {
    //   block [0x83241420..0x83241478)
	// 83241420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324142C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241434: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241438: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8324143C: 396BE130  addi r11, r11, -0x1ed0
	ctx.r[11].s64 = ctx.r[11].s64 + -7888;
	// 83241440: 3BEB01B0  addi r31, r11, 0x1b0
	ctx.r[31].s64 = ctx.r[11].s64 + 432;
	// 83241444: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 83241448: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324144C: 4BBB1FDD  bl 0x82df3428
	ctx.lr = 0x83241450;
	sub_82DF3428(ctx, base);
	// 83241450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241454: 4BBB1FD5  bl 0x82df3428
	ctx.lr = 0x83241458;
	sub_82DF3428(ctx, base);
	// 83241458: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8324145C: 4080FFE8  bge 0x83241444
	if !ctx.cr[0].lt {
	pc = 0x83241444; continue 'dispatch;
	}
	// 83241460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83241464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324146C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241470: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241478 size=60
    let mut pc: u32 = 0x83241478;
    'dispatch: loop {
        match pc {
            0x83241478 => {
    //   block [0x83241478..0x832414B4)
	// 83241478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241488: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8324148C: 3BEBE2E4  addi r31, r11, -0x1d1c
	ctx.r[31].s64 = ctx.r[11].s64 + -7452;
	// 83241490: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83241494: 4BBB1F95  bl 0x82df3428
	ctx.lr = 0x83241498;
	sub_82DF3428(ctx, base);
	// 83241498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324149C: 4BBB1F8D  bl 0x82df3428
	ctx.lr = 0x832414A0;
	sub_82DF3428(ctx, base);
	// 832414A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832414A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832414A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832414AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832414B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832414B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832414B8 size=88
    let mut pc: u32 = 0x832414B8;
    'dispatch: loop {
        match pc {
            0x832414B8 => {
    //   block [0x832414B8..0x83241510)
	// 832414B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832414BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832414C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832414C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832414C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832414CC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832414D0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832414D4: 396BE368  addi r11, r11, -0x1c98
	ctx.r[11].s64 = ctx.r[11].s64 + -7320;
	// 832414D8: 3BEB0080  addi r31, r11, 0x80
	ctx.r[31].s64 = ctx.r[11].s64 + 128;
	// 832414DC: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 832414E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832414E4: 4BBB1F45  bl 0x82df3428
	ctx.lr = 0x832414E8;
	sub_82DF3428(ctx, base);
	// 832414E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832414EC: 4BBB1F3D  bl 0x82df3428
	ctx.lr = 0x832414F0;
	sub_82DF3428(ctx, base);
	// 832414F0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832414F4: 4080FFE8  bge 0x832414dc
	if !ctx.cr[0].lt {
	pc = 0x832414DC; continue 'dispatch;
	}
	// 832414F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832414FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241504: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324150C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241510 size=80
    let mut pc: u32 = 0x83241510;
    'dispatch: loop {
        match pc {
            0x83241510 => {
    //   block [0x83241510..0x83241560)
	// 83241510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324151C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241524: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241528: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8324152C: 396BE5A8  addi r11, r11, -0x1a58
	ctx.r[11].s64 = ctx.r[11].s64 + -6744;
	// 83241530: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 83241534: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 83241538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324153C: 4BBB1EED  bl 0x82df3428
	ctx.lr = 0x83241540;
	sub_82DF3428(ctx, base);
	// 83241540: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241544: 4080FFF0  bge 0x83241534
	if !ctx.cr[0].lt {
	pc = 0x83241534; continue 'dispatch;
	}
	// 83241548: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324154C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241554: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324155C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241560 size=80
    let mut pc: u32 = 0x83241560;
    'dispatch: loop {
        match pc {
            0x83241560 => {
    //   block [0x83241560..0x832415B0)
	// 83241560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324156C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241574: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241578: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8324157C: 396BE5CC  addi r11, r11, -0x1a34
	ctx.r[11].s64 = ctx.r[11].s64 + -6708;
	// 83241580: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 83241584: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 83241588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324158C: 4BBB1E9D  bl 0x82df3428
	ctx.lr = 0x83241590;
	sub_82DF3428(ctx, base);
	// 83241590: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241594: 4080FFF0  bge 0x83241584
	if !ctx.cr[0].lt {
	pc = 0x83241584; continue 'dispatch;
	}
	// 83241598: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324159C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832415A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832415A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832415A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832415AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832415B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832415B0 size=80
    let mut pc: u32 = 0x832415B0;
    'dispatch: loop {
        match pc {
            0x832415B0 => {
    //   block [0x832415B0..0x83241600)
	// 832415B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832415B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832415B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832415BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832415C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832415C4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 832415C8: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 832415CC: 396BE5E8  addi r11, r11, -0x1a18
	ctx.r[11].s64 = ctx.r[11].s64 + -6680;
	// 832415D0: 3BEB004C  addi r31, r11, 0x4c
	ctx.r[31].s64 = ctx.r[11].s64 + 76;
	// 832415D4: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 832415D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832415DC: 4BBB1E4D  bl 0x82df3428
	ctx.lr = 0x832415E0;
	sub_82DF3428(ctx, base);
	// 832415E0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832415E4: 4080FFF0  bge 0x832415d4
	if !ctx.cr[0].lt {
	pc = 0x832415D4; continue 'dispatch;
	}
	// 832415E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832415EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832415F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832415F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832415F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832415FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241600 size=80
    let mut pc: u32 = 0x83241600;
    'dispatch: loop {
        match pc {
            0x83241600 => {
    //   block [0x83241600..0x83241650)
	// 83241600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324160C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241614: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83241618: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8324161C: 396BEC6C  addi r11, r11, -0x1394
	ctx.r[11].s64 = ctx.r[11].s64 + -5012;
	// 83241620: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 83241624: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 83241628: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324162C: 4BBB1DFD  bl 0x82df3428
	ctx.lr = 0x83241630;
	sub_82DF3428(ctx, base);
	// 83241630: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241634: 4080FFF0  bge 0x83241624
	if !ctx.cr[0].lt {
	pc = 0x83241624; continue 'dispatch;
	}
	// 83241638: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324163C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241644: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324164C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241650 size=80
    let mut pc: u32 = 0x83241650;
    'dispatch: loop {
        match pc {
            0x83241650 => {
    //   block [0x83241650..0x832416A0)
	// 83241650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324165C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241664: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83241668: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8324166C: 396BEC90  addi r11, r11, -0x1370
	ctx.r[11].s64 = ctx.r[11].s64 + -4976;
	// 83241670: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 83241674: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 83241678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324167C: 4BBB1DAD  bl 0x82df3428
	ctx.lr = 0x83241680;
	sub_82DF3428(ctx, base);
	// 83241680: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241684: 4080FFF0  bge 0x83241674
	if !ctx.cr[0].lt {
	pc = 0x83241674; continue 'dispatch;
	}
	// 83241688: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324168C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241694: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324169C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832416A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832416A0 size=80
    let mut pc: u32 = 0x832416A0;
    'dispatch: loop {
        match pc {
            0x832416A0 => {
    //   block [0x832416A0..0x832416F0)
	// 832416A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832416A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832416A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832416AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832416B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832416B4: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832416B8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832416BC: 396BECA8  addi r11, r11, -0x1358
	ctx.r[11].s64 = ctx.r[11].s64 + -4952;
	// 832416C0: 3BEB001C  addi r31, r11, 0x1c
	ctx.r[31].s64 = ctx.r[11].s64 + 28;
	// 832416C4: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 832416C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832416CC: 4BBB1D5D  bl 0x82df3428
	ctx.lr = 0x832416D0;
	sub_82DF3428(ctx, base);
	// 832416D0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832416D4: 4080FFF0  bge 0x832416c4
	if !ctx.cr[0].lt {
	pc = 0x832416C4; continue 'dispatch;
	}
	// 832416D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832416DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832416E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832416E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832416E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832416EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832416F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832416F0 size=80
    let mut pc: u32 = 0x832416F0;
    'dispatch: loop {
        match pc {
            0x832416F0 => {
    //   block [0x832416F0..0x83241740)
	// 832416F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832416F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832416F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832416FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241704: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83241708: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8324170C: 396BECC0  addi r11, r11, -0x1340
	ctx.r[11].s64 = ctx.r[11].s64 + -4928;
	// 83241710: 3BEB0028  addi r31, r11, 0x28
	ctx.r[31].s64 = ctx.r[11].s64 + 40;
	// 83241714: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 83241718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324171C: 4BBB1D0D  bl 0x82df3428
	ctx.lr = 0x83241720;
	sub_82DF3428(ctx, base);
	// 83241720: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241724: 4080FFF0  bge 0x83241714
	if !ctx.cr[0].lt {
	pc = 0x83241714; continue 'dispatch;
	}
	// 83241728: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324172C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241734: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241738: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324173C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241740 size=60
    let mut pc: u32 = 0x83241740;
    'dispatch: loop {
        match pc {
            0x83241740 => {
    //   block [0x83241740..0x8324177C)
	// 83241740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324174C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241750: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241754: 3BEBE6B8  addi r31, r11, -0x1948
	ctx.r[31].s64 = ctx.r[11].s64 + -6472;
	// 83241758: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8324175C: 4B2288A5  bl 0x8246a000
	ctx.lr = 0x83241760;
	sub_8246A000(ctx, base);
	// 83241760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241764: 4B89BD8D  bl 0x82add4f0
	ctx.lr = 0x83241768;
	sub_82ADD4F0(ctx, base);
	// 83241768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324176C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241780 size=68
    let mut pc: u32 = 0x83241780;
    'dispatch: loop {
        match pc {
            0x83241780 => {
    //   block [0x83241780..0x832417C4)
	// 83241780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324178C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241790: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83241794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83241798: 3BEAE6E8  addi r31, r10, -0x1918
	ctx.r[31].s64 = ctx.r[10].s64 + -6424;
	// 8324179C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832417A0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832417A4: 4B22885D  bl 0x8246a000
	ctx.lr = 0x832417A8;
	sub_8246A000(ctx, base);
	// 832417A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832417AC: 4B89BD45  bl 0x82add4f0
	ctx.lr = 0x832417B0;
	sub_82ADD4F0(ctx, base);
	// 832417B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832417B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832417B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832417BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832417C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832417C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832417C8 size=68
    let mut pc: u32 = 0x832417C8;
    'dispatch: loop {
        match pc {
            0x832417C8 => {
    //   block [0x832417C8..0x8324180C)
	// 832417C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832417CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832417D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832417D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832417D8: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832417DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832417E0: 3BEAE710  addi r31, r10, -0x18f0
	ctx.r[31].s64 = ctx.r[10].s64 + -6384;
	// 832417E4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832417E8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832417EC: 4B228815  bl 0x8246a000
	ctx.lr = 0x832417F0;
	sub_8246A000(ctx, base);
	// 832417F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832417F4: 4B89BCFD  bl 0x82add4f0
	ctx.lr = 0x832417F8;
	sub_82ADD4F0(ctx, base);
	// 832417F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832417FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241810 size=60
    let mut pc: u32 = 0x83241810;
    'dispatch: loop {
        match pc {
            0x83241810 => {
    //   block [0x83241810..0x8324184C)
	// 83241810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324181C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241820: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241824: 3BEBE760  addi r31, r11, -0x18a0
	ctx.r[31].s64 = ctx.r[11].s64 + -6304;
	// 83241828: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8324182C: 4B2287D5  bl 0x8246a000
	ctx.lr = 0x83241830;
	sub_8246A000(ctx, base);
	// 83241830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241834: 4B89BCBD  bl 0x82add4f0
	ctx.lr = 0x83241838;
	sub_82ADD4F0(ctx, base);
	// 83241838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324183C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241850 size=60
    let mut pc: u32 = 0x83241850;
    'dispatch: loop {
        match pc {
            0x83241850 => {
    //   block [0x83241850..0x8324188C)
	// 83241850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241858: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324185C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241860: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241864: 3BEBE780  addi r31, r11, -0x1880
	ctx.r[31].s64 = ctx.r[11].s64 + -6272;
	// 83241868: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8324186C: 4B228795  bl 0x8246a000
	ctx.lr = 0x83241870;
	sub_8246A000(ctx, base);
	// 83241870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241874: 4B89BC7D  bl 0x82add4f0
	ctx.lr = 0x83241878;
	sub_82ADD4F0(ctx, base);
	// 83241878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324187C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241890 size=68
    let mut pc: u32 = 0x83241890;
    'dispatch: loop {
        match pc {
            0x83241890 => {
    //   block [0x83241890..0x832418D4)
	// 83241890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324189C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832418A0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832418A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832418A8: 3BEAE73C  addi r31, r10, -0x18c4
	ctx.r[31].s64 = ctx.r[10].s64 + -6340;
	// 832418AC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832418B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832418B4: 4B22874D  bl 0x8246a000
	ctx.lr = 0x832418B8;
	sub_8246A000(ctx, base);
	// 832418B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832418BC: 4B89BC35  bl 0x82add4f0
	ctx.lr = 0x832418C0;
	sub_82ADD4F0(ctx, base);
	// 832418C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832418C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832418C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832418CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832418D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832418D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832418D8 size=68
    let mut pc: u32 = 0x832418D8;
    'dispatch: loop {
        match pc {
            0x832418D8 => {
    //   block [0x832418D8..0x8324191C)
	// 832418D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832418DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832418E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832418E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832418E8: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832418EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832418F0: 3BEAE7CC  addi r31, r10, -0x1834
	ctx.r[31].s64 = ctx.r[10].s64 + -6196;
	// 832418F4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832418F8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832418FC: 4B228705  bl 0x8246a000
	ctx.lr = 0x83241900;
	sub_8246A000(ctx, base);
	// 83241900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241904: 4B89BBED  bl 0x82add4f0
	ctx.lr = 0x83241908;
	sub_82ADD4F0(ctx, base);
	// 83241908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324190C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241920 size=36
    let mut pc: u32 = 0x83241920;
    'dispatch: loop {
        match pc {
            0x83241920 => {
    //   block [0x83241920..0x83241944)
	// 83241920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324192C: 4B89FD5D  bl 0x82ae1688
	ctx.lr = 0x83241930;
	sub_82AE1688(ctx, base);
	// 83241930: 4B89FAE1  bl 0x82ae1410
	ctx.lr = 0x83241934;
	sub_82AE1410(ctx, base);
	// 83241934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324193C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241948 size=68
    let mut pc: u32 = 0x83241948;
    'dispatch: loop {
        match pc {
            0x83241948 => {
    //   block [0x83241948..0x8324198C)
	// 83241948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324194C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241958: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 8324195C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83241960: 3BEAE884  addi r31, r10, -0x177c
	ctx.r[31].s64 = ctx.r[10].s64 + -6012;
	// 83241964: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83241968: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8324196C: 4B228695  bl 0x8246a000
	ctx.lr = 0x83241970;
	sub_8246A000(ctx, base);
	// 83241970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241974: 4B89BB7D  bl 0x82add4f0
	ctx.lr = 0x83241978;
	sub_82ADD4F0(ctx, base);
	// 83241978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324197C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241990 size=68
    let mut pc: u32 = 0x83241990;
    'dispatch: loop {
        match pc {
            0x83241990 => {
    //   block [0x83241990..0x832419D4)
	// 83241990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324199C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832419A0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832419A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832419A8: 3BEAE8B0  addi r31, r10, -0x1750
	ctx.r[31].s64 = ctx.r[10].s64 + -5968;
	// 832419AC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832419B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832419B4: 4B22864D  bl 0x8246a000
	ctx.lr = 0x832419B8;
	sub_8246A000(ctx, base);
	// 832419B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832419BC: 4B89BB35  bl 0x82add4f0
	ctx.lr = 0x832419C0;
	sub_82ADD4F0(ctx, base);
	// 832419C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832419C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832419C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832419CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832419D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832419D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832419D8 size=68
    let mut pc: u32 = 0x832419D8;
    'dispatch: loop {
        match pc {
            0x832419D8 => {
    //   block [0x832419D8..0x83241A1C)
	// 832419D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832419DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832419E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832419E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832419E8: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 832419EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832419F0: 3BEAE8E8  addi r31, r10, -0x1718
	ctx.r[31].s64 = ctx.r[10].s64 + -5912;
	// 832419F4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832419F8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832419FC: 4B228605  bl 0x8246a000
	ctx.lr = 0x83241A00;
	sub_8246A000(ctx, base);
	// 83241A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241A04: 4B89BAED  bl 0x82add4f0
	ctx.lr = 0x83241A08;
	sub_82ADD4F0(ctx, base);
	// 83241A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241A14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241A20 size=68
    let mut pc: u32 = 0x83241A20;
    'dispatch: loop {
        match pc {
            0x83241A20 => {
    //   block [0x83241A20..0x83241A64)
	// 83241A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241A28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241A2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241A30: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83241A34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83241A38: 3BEAE920  addi r31, r10, -0x16e0
	ctx.r[31].s64 = ctx.r[10].s64 + -5856;
	// 83241A3C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83241A40: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83241A44: 4B2285BD  bl 0x8246a000
	ctx.lr = 0x83241A48;
	sub_8246A000(ctx, base);
	// 83241A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241A4C: 4B89BAA5  bl 0x82add4f0
	ctx.lr = 0x83241A50;
	sub_82ADD4F0(ctx, base);
	// 83241A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241A5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241A68 size=68
    let mut pc: u32 = 0x83241A68;
    'dispatch: loop {
        match pc {
            0x83241A68 => {
    //   block [0x83241A68..0x83241AAC)
	// 83241A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241A74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241A78: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83241A7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83241A80: 3BEAE948  addi r31, r10, -0x16b8
	ctx.r[31].s64 = ctx.r[10].s64 + -5816;
	// 83241A84: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83241A88: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83241A8C: 4B228575  bl 0x8246a000
	ctx.lr = 0x83241A90;
	sub_8246A000(ctx, base);
	// 83241A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241A94: 4B89BA5D  bl 0x82add4f0
	ctx.lr = 0x83241A98;
	sub_82ADD4F0(ctx, base);
	// 83241A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241AA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241AB0 size=68
    let mut pc: u32 = 0x83241AB0;
    'dispatch: loop {
        match pc {
            0x83241AB0 => {
    //   block [0x83241AB0..0x83241AF4)
	// 83241AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241AC0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83241AC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83241AC8: 3BEAE970  addi r31, r10, -0x1690
	ctx.r[31].s64 = ctx.r[10].s64 + -5776;
	// 83241ACC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83241AD0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83241AD4: 4B22852D  bl 0x8246a000
	ctx.lr = 0x83241AD8;
	sub_8246A000(ctx, base);
	// 83241AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241ADC: 4B89BA15  bl 0x82add4f0
	ctx.lr = 0x83241AE0;
	sub_82ADD4F0(ctx, base);
	// 83241AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241AEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241AF8 size=68
    let mut pc: u32 = 0x83241AF8;
    'dispatch: loop {
        match pc {
            0x83241AF8 => {
    //   block [0x83241AF8..0x83241B3C)
	// 83241AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241B00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241B04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241B08: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83241B0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83241B10: 3BEAE998  addi r31, r10, -0x1668
	ctx.r[31].s64 = ctx.r[10].s64 + -5736;
	// 83241B14: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83241B18: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83241B1C: 4B2284E5  bl 0x8246a000
	ctx.lr = 0x83241B20;
	sub_8246A000(ctx, base);
	// 83241B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241B24: 4B89B9CD  bl 0x82add4f0
	ctx.lr = 0x83241B28;
	sub_82ADD4F0(ctx, base);
	// 83241B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241B34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241B40 size=12
    let mut pc: u32 = 0x83241B40;
    'dispatch: loop {
        match pc {
            0x83241B40 => {
    //   block [0x83241B40..0x83241B4C)
	// 83241B40: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241B44: 386BE9F8  addi r3, r11, -0x1608
	ctx.r[3].s64 = ctx.r[11].s64 + -5640;
	// 83241B48: 4BBB18E0  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241B50 size=12
    let mut pc: u32 = 0x83241B50;
    'dispatch: loop {
        match pc {
            0x83241B50 => {
    //   block [0x83241B50..0x83241B5C)
	// 83241B50: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241B54: 386BEBA8  addi r3, r11, -0x1458
	ctx.r[3].s64 = ctx.r[11].s64 + -5208;
	// 83241B58: 4B951740  b 0x82b93298
	sub_82B93298(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241B60 size=20
    let mut pc: u32 = 0x83241B60;
    'dispatch: loop {
        match pc {
            0x83241B60 => {
    //   block [0x83241B60..0x83241B74)
	// 83241B60: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83241B64: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83241B68: 396BAE40  addi r11, r11, -0x51c0
	ctx.r[11].s64 = ctx.r[11].s64 + -20928;
	// 83241B6C: 916A9D74  stw r11, -0x628c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25228 as u32), ctx.r[11].u32 ) };
	// 83241B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241B78 size=12
    let mut pc: u32 = 0x83241B78;
    'dispatch: loop {
        match pc {
            0x83241B78 => {
    //   block [0x83241B78..0x83241B84)
	// 83241B78: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241B7C: 386B5618  addi r3, r11, 0x5618
	ctx.r[3].s64 = ctx.r[11].s64 + 22040;
	// 83241B80: 4BBAEF80  b 0x82df0b00
	sub_82DF0B00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241B88 size=76
    let mut pc: u32 = 0x83241B88;
    'dispatch: loop {
        match pc {
            0x83241B88 => {
    //   block [0x83241B88..0x83241BD4)
	// 83241B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241B90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241B94: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241B98: 806B563C  lwz r3, 0x563c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22076 as u32) ) } as u64;
	// 83241B9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83241BA0: 419A0024  beq cr6, 0x83241bc4
	if ctx.cr[6].eq {
	pc = 0x83241BC4; continue 'dispatch;
	}
	// 83241BA4: 4B15F50D  bl 0x823a10b0
	ctx.lr = 0x83241BA8;
	sub_823A10B0(ctx, base);
	// 83241BA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83241BAC: 41820018  beq 0x83241bc4
	if ctx.cr[0].eq {
	pc = 0x83241BC4; continue 'dispatch;
	}
	// 83241BB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83241BB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83241BB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83241BBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83241BC0: 4E800421  bctrl
	ctx.lr = 0x83241BC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83241BC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241BD8 size=12
    let mut pc: u32 = 0x83241BD8;
    'dispatch: loop {
        match pc {
            0x83241BD8 => {
    //   block [0x83241BD8..0x83241BE4)
	// 83241BD8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241BDC: 386B56D0  addi r3, r11, 0x56d0
	ctx.r[3].s64 = ctx.r[11].s64 + 22224;
	// 83241BE0: 4BBAEF20  b 0x82df0b00
	sub_82DF0B00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241BE8 size=12
    let mut pc: u32 = 0x83241BE8;
    'dispatch: loop {
        match pc {
            0x83241BE8 => {
    //   block [0x83241BE8..0x83241BF4)
	// 83241BE8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241BEC: 386B56D1  addi r3, r11, 0x56d1
	ctx.r[3].s64 = ctx.r[11].s64 + 22225;
	// 83241BF0: 4BBAFF58  b 0x82df1b48
	sub_82DF1B48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241BF8 size=12
    let mut pc: u32 = 0x83241BF8;
    'dispatch: loop {
        match pc {
            0x83241BF8 => {
    //   block [0x83241BF8..0x83241C04)
	// 83241BF8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241BFC: 386B5700  addi r3, r11, 0x5700
	ctx.r[3].s64 = ctx.r[11].s64 + 22272;
	// 83241C00: 4BBB1370  b 0x82df2f70
	sub_82DF2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241C08 size=12
    let mut pc: u32 = 0x83241C08;
    'dispatch: loop {
        match pc {
            0x83241C08 => {
    //   block [0x83241C08..0x83241C14)
	// 83241C08: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241C0C: 386B5F90  addi r3, r11, 0x5f90
	ctx.r[3].s64 = ctx.r[11].s64 + 24464;
	// 83241C10: 4BBB1818  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241C18 size=12
    let mut pc: u32 = 0x83241C18;
    'dispatch: loop {
        match pc {
            0x83241C18 => {
    //   block [0x83241C18..0x83241C24)
	// 83241C18: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241C1C: 386B5F94  addi r3, r11, 0x5f94
	ctx.r[3].s64 = ctx.r[11].s64 + 24468;
	// 83241C20: 4BBB1808  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241C28 size=16
    let mut pc: u32 = 0x83241C28;
    'dispatch: loop {
        match pc {
            0x83241C28 => {
    //   block [0x83241C28..0x83241C38)
	// 83241C28: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241C2C: 396B5FC0  addi r11, r11, 0x5fc0
	ctx.r[11].s64 = ctx.r[11].s64 + 24512;
	// 83241C30: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83241C34: 4BBB091C  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241C38 size=16
    let mut pc: u32 = 0x83241C38;
    'dispatch: loop {
        match pc {
            0x83241C38 => {
    //   block [0x83241C38..0x83241C48)
	// 83241C38: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241C3C: 396B5FC8  addi r11, r11, 0x5fc8
	ctx.r[11].s64 = ctx.r[11].s64 + 24520;
	// 83241C40: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83241C44: 4BBB090C  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241C48 size=16
    let mut pc: u32 = 0x83241C48;
    'dispatch: loop {
        match pc {
            0x83241C48 => {
    //   block [0x83241C48..0x83241C58)
	// 83241C48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241C4C: 396B5FD4  addi r11, r11, 0x5fd4
	ctx.r[11].s64 = ctx.r[11].s64 + 24532;
	// 83241C50: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83241C54: 4BBB08FC  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241C58 size=16
    let mut pc: u32 = 0x83241C58;
    'dispatch: loop {
        match pc {
            0x83241C58 => {
    //   block [0x83241C58..0x83241C68)
	// 83241C58: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241C5C: 396B5FDC  addi r11, r11, 0x5fdc
	ctx.r[11].s64 = ctx.r[11].s64 + 24540;
	// 83241C60: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83241C64: 4BBB08EC  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241C68 size=60
    let mut pc: u32 = 0x83241C68;
    'dispatch: loop {
        match pc {
            0x83241C68 => {
    //   block [0x83241C68..0x83241CA4)
	// 83241C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241C78: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241C7C: 3BEB5FEC  addi r31, r11, 0x5fec
	ctx.r[31].s64 = ctx.r[11].s64 + 24556;
	// 83241C80: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83241C84: 4BBB447D  bl 0x82df6100
	ctx.lr = 0x83241C88;
	sub_82DF6100(ctx, base);
	// 83241C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241C8C: 4BBB6815  bl 0x82df84a0
	ctx.lr = 0x83241C90;
	sub_82DF84A0(ctx, base);
	// 83241C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241CA8 size=60
    let mut pc: u32 = 0x83241CA8;
    'dispatch: loop {
        match pc {
            0x83241CA8 => {
    //   block [0x83241CA8..0x83241CE4)
	// 83241CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241CB8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241CBC: 3BEB6010  addi r31, r11, 0x6010
	ctx.r[31].s64 = ctx.r[11].s64 + 24592;
	// 83241CC0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83241CC4: 4BBDA0C5  bl 0x82e1bd88
	ctx.lr = 0x83241CC8;
	sub_82E1BD88(ctx, base);
	// 83241CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241CCC: 4BBB656D  bl 0x82df8238
	ctx.lr = 0x83241CD0;
	sub_82DF8238(ctx, base);
	// 83241CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241CE8 size=60
    let mut pc: u32 = 0x83241CE8;
    'dispatch: loop {
        match pc {
            0x83241CE8 => {
    //   block [0x83241CE8..0x83241D24)
	// 83241CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241CF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241CF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241CF8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241CFC: 3BEB6034  addi r31, r11, 0x6034
	ctx.r[31].s64 = ctx.r[11].s64 + 24628;
	// 83241D00: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83241D04: 4BBB6A05  bl 0x82df8708
	ctx.lr = 0x83241D08;
	sub_82DF8708(ctx, base);
	// 83241D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241D0C: 4BBB6DA5  bl 0x82df8ab0
	ctx.lr = 0x83241D10;
	sub_82DF8AB0(ctx, base);
	// 83241D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83241D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241D1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D28 size=12
    let mut pc: u32 = 0x83241D28;
    'dispatch: loop {
        match pc {
            0x83241D28 => {
    //   block [0x83241D28..0x83241D34)
	// 83241D28: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241D2C: 386B6270  addi r3, r11, 0x6270
	ctx.r[3].s64 = ctx.r[11].s64 + 25200;
	// 83241D30: 4BBBBE90  b 0x82dfdbc0
	sub_82DFDBC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D38 size=12
    let mut pc: u32 = 0x83241D38;
    'dispatch: loop {
        match pc {
            0x83241D38 => {
    //   block [0x83241D38..0x83241D44)
	// 83241D38: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241D3C: 386B62B0  addi r3, r11, 0x62b0
	ctx.r[3].s64 = ctx.r[11].s64 + 25264;
	// 83241D40: 4BBBBEE8  b 0x82dfdc28
	sub_82DFDC28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D48 size=20
    let mut pc: u32 = 0x83241D48;
    'dispatch: loop {
        match pc {
            0x83241D48 => {
    //   block [0x83241D48..0x83241D5C)
	// 83241D48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241D4C: 396B64E0  addi r11, r11, 0x64e0
	ctx.r[11].s64 = ctx.r[11].s64 + 25824;
	// 83241D50: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83241D54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83241D58: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D5C size=8
    let mut pc: u32 = 0x83241D5C;
    'dispatch: loop {
        match pc {
            0x83241D5C => {
    //   block [0x83241D5C..0x83241D64)
	// 83241D5C: 4B07EB34  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 83241D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D68 size=20
    let mut pc: u32 = 0x83241D68;
    'dispatch: loop {
        match pc {
            0x83241D68 => {
    //   block [0x83241D68..0x83241D7C)
	// 83241D68: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241D6C: 396B6524  addi r11, r11, 0x6524
	ctx.r[11].s64 = ctx.r[11].s64 + 25892;
	// 83241D70: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83241D74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83241D78: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D7C size=8
    let mut pc: u32 = 0x83241D7C;
    'dispatch: loop {
        match pc {
            0x83241D7C => {
    //   block [0x83241D7C..0x83241D84)
	// 83241D7C: 4B07EB14  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 83241D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D88 size=12
    let mut pc: u32 = 0x83241D88;
    'dispatch: loop {
        match pc {
            0x83241D88 => {
    //   block [0x83241D88..0x83241D94)
	// 83241D88: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241D8C: 386B6588  addi r3, r11, 0x6588
	ctx.r[3].s64 = ctx.r[11].s64 + 25992;
	// 83241D90: 4BBD1E98  b 0x82e13c28
	sub_82E13C28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241D98 size=12
    let mut pc: u32 = 0x83241D98;
    'dispatch: loop {
        match pc {
            0x83241D98 => {
    //   block [0x83241D98..0x83241DA4)
	// 83241D98: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241D9C: 386B65D0  addi r3, r11, 0x65d0
	ctx.r[3].s64 = ctx.r[11].s64 + 26064;
	// 83241DA0: 4B086F18  b 0x822c8cb8
	sub_822C8CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241DA8 size=80
    let mut pc: u32 = 0x83241DA8;
    'dispatch: loop {
        match pc {
            0x83241DA8 => {
    //   block [0x83241DA8..0x83241DF8)
	// 83241DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241DB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83241DB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241DB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241DBC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241DC0: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 83241DC4: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 83241DC8: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 83241DCC: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 83241DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241DD4: 4BBB1655  bl 0x82df3428
	ctx.lr = 0x83241DD8;
	sub_82DF3428(ctx, base);
	// 83241DD8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241DDC: 4080FFF0  bge 0x83241dcc
	if !ctx.cr[0].lt {
	pc = 0x83241DCC; continue 'dispatch;
	}
	// 83241DE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83241DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241DEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241DF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83241DF8 size=80
    let mut pc: u32 = 0x83241DF8;
    'dispatch: loop {
        match pc {
            0x83241DF8 => {
    //   block [0x83241DF8..0x83241E48)
	// 83241DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83241DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83241E00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83241E04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83241E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83241E0C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241E10: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 83241E14: 396B67DC  addi r11, r11, 0x67dc
	ctx.r[11].s64 = ctx.r[11].s64 + 26588;
	// 83241E18: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 83241E1C: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 83241E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83241E24: 4BBB1605  bl 0x82df3428
	ctx.lr = 0x83241E28;
	sub_82DF3428(ctx, base);
	// 83241E28: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83241E2C: 4080FFF0  bge 0x83241e1c
	if !ctx.cr[0].lt {
	pc = 0x83241E1C; continue 'dispatch;
	}
	// 83241E30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83241E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83241E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83241E3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83241E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83241E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241E48 size=12
    let mut pc: u32 = 0x83241E48;
    'dispatch: loop {
        match pc {
            0x83241E48 => {
    //   block [0x83241E48..0x83241E54)
	// 83241E48: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241E4C: 386B680C  addi r3, r11, 0x680c
	ctx.r[3].s64 = ctx.r[11].s64 + 26636;
	// 83241E50: 4BC0F558  b 0x82e513a8
	sub_82E513A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241E58 size=20
    let mut pc: u32 = 0x83241E58;
    'dispatch: loop {
        match pc {
            0x83241E58 => {
    //   block [0x83241E58..0x83241E6C)
	// 83241E58: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241E5C: 396B6818  addi r11, r11, 0x6818
	ctx.r[11].s64 = ctx.r[11].s64 + 26648;
	// 83241E60: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83241E64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83241E68: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241E6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241E6C size=8
    let mut pc: u32 = 0x83241E6C;
    'dispatch: loop {
        match pc {
            0x83241E6C => {
    //   block [0x83241E6C..0x83241E74)
	// 83241E6C: 4B07EA24  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 83241E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241E78 size=12
    let mut pc: u32 = 0x83241E78;
    'dispatch: loop {
        match pc {
            0x83241E78 => {
    //   block [0x83241E78..0x83241E84)
	// 83241E78: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241E7C: 386B6850  addi r3, r11, 0x6850
	ctx.r[3].s64 = ctx.r[11].s64 + 26704;
	// 83241E80: 4BBB15A8  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241E88 size=12
    let mut pc: u32 = 0x83241E88;
    'dispatch: loop {
        match pc {
            0x83241E88 => {
    //   block [0x83241E88..0x83241E94)
	// 83241E88: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241E8C: 386B6920  addi r3, r11, 0x6920
	ctx.r[3].s64 = ctx.r[11].s64 + 26912;
	// 83241E90: 4BBB06C0  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241E98 size=16
    let mut pc: u32 = 0x83241E98;
    'dispatch: loop {
        match pc {
            0x83241E98 => {
    //   block [0x83241E98..0x83241EA8)
	// 83241E98: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 83241E9C: 396B70B0  addi r11, r11, 0x70b0
	ctx.r[11].s64 = ctx.r[11].s64 + 28848;
	// 83241EA0: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 83241EA4: 4BC602D4  b 0x82ea2178
	sub_82EA2178(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241EA8 size=20
    let mut pc: u32 = 0x83241EA8;
    'dispatch: loop {
        match pc {
            0x83241EA8 => {
    //   block [0x83241EA8..0x83241EBC)
	// 83241EA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83241EAC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83241EB0: 396B9EAC  addi r11, r11, -0x6154
	ctx.r[11].s64 = ctx.r[11].s64 + -24916;
	// 83241EB4: 916A7104  stw r11, 0x7104(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28932 as u32), ctx.r[11].u32 ) };
	// 83241EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241EC0 size=20
    let mut pc: u32 = 0x83241EC0;
    'dispatch: loop {
        match pc {
            0x83241EC0 => {
    //   block [0x83241EC0..0x83241ED4)
	// 83241EC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83241EC4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 83241EC8: 396B9EAC  addi r11, r11, -0x6154
	ctx.r[11].s64 = ctx.r[11].s64 + -24916;
	// 83241ECC: 916A71F0  stw r11, 0x71f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29168 as u32), ctx.r[11].u32 ) };
	// 83241ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241ED8 size=20
    let mut pc: u32 = 0x83241ED8;
    'dispatch: loop {
        match pc {
            0x83241ED8 => {
    //   block [0x83241ED8..0x83241EEC)
	// 83241ED8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83241EDC: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83241EE0: 396B9EAC  addi r11, r11, -0x6154
	ctx.r[11].s64 = ctx.r[11].s64 + -24916;
	// 83241EE4: 916AF36C  stw r11, -0xc94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-3220 as u32), ctx.r[11].u32 ) };
	// 83241EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241EF0 size=20
    let mut pc: u32 = 0x83241EF0;
    'dispatch: loop {
        match pc {
            0x83241EF0 => {
    //   block [0x83241EF0..0x83241F04)
	// 83241EF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83241EF4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83241EF8: 396B9EAC  addi r11, r11, -0x6154
	ctx.r[11].s64 = ctx.r[11].s64 + -24916;
	// 83241EFC: 916A0444  stw r11, 0x444(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1092 as u32), ctx.r[11].u32 ) };
	// 83241F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241F08 size=20
    let mut pc: u32 = 0x83241F08;
    'dispatch: loop {
        match pc {
            0x83241F08 => {
    //   block [0x83241F08..0x83241F1C)
	// 83241F08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F0C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83241F10: 396BF820  addi r11, r11, -0x7e0
	ctx.r[11].s64 = ctx.r[11].s64 + -2016;
	// 83241F14: 916A2DD4  stw r11, 0x2dd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(11732 as u32), ctx.r[11].u32 ) };
	// 83241F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241F20 size=36
    let mut pc: u32 = 0x83241F20;
    'dispatch: loop {
        match pc {
            0x83241F20 => {
    //   block [0x83241F20..0x83241F44)
	// 83241F20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F24: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83241F28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F2C: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83241F30: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83241F34: 396B2F84  addi r11, r11, 0x2f84
	ctx.r[11].s64 = ctx.r[11].s64 + 12164;
	// 83241F38: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83241F3C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83241F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241F48 size=36
    let mut pc: u32 = 0x83241F48;
    'dispatch: loop {
        match pc {
            0x83241F48 => {
    //   block [0x83241F48..0x83241F6C)
	// 83241F48: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F4C: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83241F50: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F54: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83241F58: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83241F5C: 396B2FA8  addi r11, r11, 0x2fa8
	ctx.r[11].s64 = ctx.r[11].s64 + 12200;
	// 83241F60: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83241F64: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83241F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241F70 size=36
    let mut pc: u32 = 0x83241F70;
    'dispatch: loop {
        match pc {
            0x83241F70 => {
    //   block [0x83241F70..0x83241F94)
	// 83241F70: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F74: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83241F78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F7C: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83241F80: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83241F84: 396B2FCC  addi r11, r11, 0x2fcc
	ctx.r[11].s64 = ctx.r[11].s64 + 12236;
	// 83241F88: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83241F8C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83241F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241F98 size=36
    let mut pc: u32 = 0x83241F98;
    'dispatch: loop {
        match pc {
            0x83241F98 => {
    //   block [0x83241F98..0x83241FBC)
	// 83241F98: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241F9C: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83241FA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241FA4: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83241FA8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83241FAC: 396B2FF0  addi r11, r11, 0x2ff0
	ctx.r[11].s64 = ctx.r[11].s64 + 12272;
	// 83241FB0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83241FB4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83241FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241FC0 size=36
    let mut pc: u32 = 0x83241FC0;
    'dispatch: loop {
        match pc {
            0x83241FC0 => {
    //   block [0x83241FC0..0x83241FE4)
	// 83241FC0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241FC4: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83241FC8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241FCC: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83241FD0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83241FD4: 396B3014  addi r11, r11, 0x3014
	ctx.r[11].s64 = ctx.r[11].s64 + 12308;
	// 83241FD8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83241FDC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83241FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83241FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83241FE8 size=36
    let mut pc: u32 = 0x83241FE8;
    'dispatch: loop {
        match pc {
            0x83241FE8 => {
    //   block [0x83241FE8..0x8324200C)
	// 83241FE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241FEC: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83241FF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83241FF4: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83241FF8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83241FFC: 396B3038  addi r11, r11, 0x3038
	ctx.r[11].s64 = ctx.r[11].s64 + 12344;
	// 83242000: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83242004: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83242008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242010 size=36
    let mut pc: u32 = 0x83242010;
    'dispatch: loop {
        match pc {
            0x83242010 => {
    //   block [0x83242010..0x83242034)
	// 83242010: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83242014: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83242018: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8324201C: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83242020: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83242024: 396B305C  addi r11, r11, 0x305c
	ctx.r[11].s64 = ctx.r[11].s64 + 12380;
	// 83242028: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8324202C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83242030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242038 size=36
    let mut pc: u32 = 0x83242038;
    'dispatch: loop {
        match pc {
            0x83242038 => {
    //   block [0x83242038..0x8324205C)
	// 83242038: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8324203C: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83242040: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83242044: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83242048: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8324204C: 396B3080  addi r11, r11, 0x3080
	ctx.r[11].s64 = ctx.r[11].s64 + 12416;
	// 83242050: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83242054: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83242058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242060 size=36
    let mut pc: u32 = 0x83242060;
    'dispatch: loop {
        match pc {
            0x83242060 => {
    //   block [0x83242060..0x83242084)
	// 83242060: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83242064: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83242068: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8324206C: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83242070: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83242074: 396B30A4  addi r11, r11, 0x30a4
	ctx.r[11].s64 = ctx.r[11].s64 + 12452;
	// 83242078: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8324207C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83242080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242088 size=36
    let mut pc: u32 = 0x83242088;
    'dispatch: loop {
        match pc {
            0x83242088 => {
    //   block [0x83242088..0x832420AC)
	// 83242088: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8324208C: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 83242090: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83242094: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 83242098: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8324209C: 396B30C8  addi r11, r11, 0x30c8
	ctx.r[11].s64 = ctx.r[11].s64 + 12488;
	// 832420A0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832420A4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832420A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832420B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832420B0 size=36
    let mut pc: u32 = 0x832420B0;
    'dispatch: loop {
        match pc {
            0x832420B0 => {
    //   block [0x832420B0..0x832420D4)
	// 832420B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 832420B4: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 832420B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 832420BC: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 832420C0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832420C4: 396B30EC  addi r11, r11, 0x30ec
	ctx.r[11].s64 = ctx.r[11].s64 + 12524;
	// 832420C8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832420CC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832420D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832420D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832420D8 size=36
    let mut pc: u32 = 0x832420D8;
    'dispatch: loop {
        match pc {
            0x832420D8 => {
    //   block [0x832420D8..0x832420FC)
	// 832420D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 832420DC: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 832420E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 832420E4: 392BF820  addi r9, r11, -0x7e0
	ctx.r[9].s64 = ctx.r[11].s64 + -2016;
	// 832420E8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832420EC: 396B2F60  addi r11, r11, 0x2f60
	ctx.r[11].s64 = ctx.r[11].s64 + 12128;
	// 832420F0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832420F4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832420F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242100 size=16
    let mut pc: u32 = 0x83242100;
    'dispatch: loop {
        match pc {
            0x83242100 => {
    //   block [0x83242100..0x83242110)
	// 83242100: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242104: 396BBD78  addi r11, r11, -0x4288
	ctx.r[11].s64 = ctx.r[11].s64 + -17032;
	// 83242108: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8324210C: 4BE6FDCC  b 0x830b1ed8
	sub_830B1ED8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242110 size=12
    let mut pc: u32 = 0x83242110;
    'dispatch: loop {
        match pc {
            0x83242110 => {
    //   block [0x83242110..0x8324211C)
	// 83242110: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242114: 386BBE08  addi r3, r11, -0x41f8
	ctx.r[3].s64 = ctx.r[11].s64 + -16888;
	// 83242118: 4BE89738  b 0x830cb850
	sub_830CB850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242120 size=12
    let mut pc: u32 = 0x83242120;
    'dispatch: loop {
        match pc {
            0x83242120 => {
    //   block [0x83242120..0x8324212C)
	// 83242120: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242124: 386BC024  addi r3, r11, -0x3fdc
	ctx.r[3].s64 = ctx.r[11].s64 + -16348;
	// 83242128: 4BE89FD0  b 0x830cc0f8
	sub_830CC0F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242130 size=12
    let mut pc: u32 = 0x83242130;
    'dispatch: loop {
        match pc {
            0x83242130 => {
    //   block [0x83242130..0x8324213C)
	// 83242130: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242134: 386BC054  addi r3, r11, -0x3fac
	ctx.r[3].s64 = ctx.r[11].s64 + -16300;
	// 83242138: 4BE9BE60  b 0x830ddf98
	sub_830DDF98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242140 size=4
    let mut pc: u32 = 0x83242140;
    'dispatch: loop {
        match pc {
            0x83242140 => {
    //   block [0x83242140..0x83242144)
	// 83242140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242148 size=4
    let mut pc: u32 = 0x83242148;
    'dispatch: loop {
        match pc {
            0x83242148 => {
    //   block [0x83242148..0x8324214C)
	// 83242148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242150 size=4
    let mut pc: u32 = 0x83242150;
    'dispatch: loop {
        match pc {
            0x83242150 => {
    //   block [0x83242150..0x83242154)
	// 83242150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242158 size=12
    let mut pc: u32 = 0x83242158;
    'dispatch: loop {
        match pc {
            0x83242158 => {
    //   block [0x83242158..0x83242164)
	// 83242158: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324215C: 386BC74C  addi r3, r11, -0x38b4
	ctx.r[3].s64 = ctx.r[11].s64 + -14516;
	// 83242160: 4BEC0928  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242168 size=12
    let mut pc: u32 = 0x83242168;
    'dispatch: loop {
        match pc {
            0x83242168 => {
    //   block [0x83242168..0x83242174)
	// 83242168: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324216C: 386BC734  addi r3, r11, -0x38cc
	ctx.r[3].s64 = ctx.r[11].s64 + -14540;
	// 83242170: 4BEC0918  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242178 size=12
    let mut pc: u32 = 0x83242178;
    'dispatch: loop {
        match pc {
            0x83242178 => {
    //   block [0x83242178..0x83242184)
	// 83242178: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324217C: 386BC71C  addi r3, r11, -0x38e4
	ctx.r[3].s64 = ctx.r[11].s64 + -14564;
	// 83242180: 4BEC0908  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242188 size=12
    let mut pc: u32 = 0x83242188;
    'dispatch: loop {
        match pc {
            0x83242188 => {
    //   block [0x83242188..0x83242194)
	// 83242188: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324218C: 386BC704  addi r3, r11, -0x38fc
	ctx.r[3].s64 = ctx.r[11].s64 + -14588;
	// 83242190: 4BEC08F8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242198 size=12
    let mut pc: u32 = 0x83242198;
    'dispatch: loop {
        match pc {
            0x83242198 => {
    //   block [0x83242198..0x832421A4)
	// 83242198: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324219C: 386BC6EC  addi r3, r11, -0x3914
	ctx.r[3].s64 = ctx.r[11].s64 + -14612;
	// 832421A0: 4BEC08E8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832421A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832421A8 size=12
    let mut pc: u32 = 0x832421A8;
    'dispatch: loop {
        match pc {
            0x832421A8 => {
    //   block [0x832421A8..0x832421B4)
	// 832421A8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832421AC: 386BC77C  addi r3, r11, -0x3884
	ctx.r[3].s64 = ctx.r[11].s64 + -14468;
	// 832421B0: 4BEC08D8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832421B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832421B8 size=12
    let mut pc: u32 = 0x832421B8;
    'dispatch: loop {
        match pc {
            0x832421B8 => {
    //   block [0x832421B8..0x832421C4)
	// 832421B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832421BC: 386BC764  addi r3, r11, -0x389c
	ctx.r[3].s64 = ctx.r[11].s64 + -14492;
	// 832421C0: 4BEC08C8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832421C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832421C8 size=12
    let mut pc: u32 = 0x832421C8;
    'dispatch: loop {
        match pc {
            0x832421C8 => {
    //   block [0x832421C8..0x832421D4)
	// 832421C8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832421CC: 386BC814  addi r3, r11, -0x37ec
	ctx.r[3].s64 = ctx.r[11].s64 + -14316;
	// 832421D0: 4BEC08B8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832421D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832421D8 size=12
    let mut pc: u32 = 0x832421D8;
    'dispatch: loop {
        match pc {
            0x832421D8 => {
    //   block [0x832421D8..0x832421E4)
	// 832421D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832421DC: 386BC7FC  addi r3, r11, -0x3804
	ctx.r[3].s64 = ctx.r[11].s64 + -14340;
	// 832421E0: 4BEC08A8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832421E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832421E8 size=12
    let mut pc: u32 = 0x832421E8;
    'dispatch: loop {
        match pc {
            0x832421E8 => {
    //   block [0x832421E8..0x832421F4)
	// 832421E8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832421EC: 386BC93C  addi r3, r11, -0x36c4
	ctx.r[3].s64 = ctx.r[11].s64 + -14020;
	// 832421F0: 4BEC0898  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832421F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832421F8 size=12
    let mut pc: u32 = 0x832421F8;
    'dispatch: loop {
        match pc {
            0x832421F8 => {
    //   block [0x832421F8..0x83242204)
	// 832421F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832421FC: 386BC924  addi r3, r11, -0x36dc
	ctx.r[3].s64 = ctx.r[11].s64 + -14044;
	// 83242200: 4BEC0888  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242208 size=12
    let mut pc: u32 = 0x83242208;
    'dispatch: loop {
        match pc {
            0x83242208 => {
    //   block [0x83242208..0x83242214)
	// 83242208: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324220C: 386BC90C  addi r3, r11, -0x36f4
	ctx.r[3].s64 = ctx.r[11].s64 + -14068;
	// 83242210: 4BEC0878  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242218 size=12
    let mut pc: u32 = 0x83242218;
    'dispatch: loop {
        match pc {
            0x83242218 => {
    //   block [0x83242218..0x83242224)
	// 83242218: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324221C: 386BC8F4  addi r3, r11, -0x370c
	ctx.r[3].s64 = ctx.r[11].s64 + -14092;
	// 83242220: 4BEC0868  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242228 size=12
    let mut pc: u32 = 0x83242228;
    'dispatch: loop {
        match pc {
            0x83242228 => {
    //   block [0x83242228..0x83242234)
	// 83242228: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324222C: 386BC95C  addi r3, r11, -0x36a4
	ctx.r[3].s64 = ctx.r[11].s64 + -13988;
	// 83242230: 4BEC0858  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242238 size=12
    let mut pc: u32 = 0x83242238;
    'dispatch: loop {
        match pc {
            0x83242238 => {
    //   block [0x83242238..0x83242244)
	// 83242238: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324223C: 386BC98C  addi r3, r11, -0x3674
	ctx.r[3].s64 = ctx.r[11].s64 + -13940;
	// 83242240: 4BEC0848  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242248 size=12
    let mut pc: u32 = 0x83242248;
    'dispatch: loop {
        match pc {
            0x83242248 => {
    //   block [0x83242248..0x83242254)
	// 83242248: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324224C: 386BC974  addi r3, r11, -0x368c
	ctx.r[3].s64 = ctx.r[11].s64 + -13964;
	// 83242250: 4BEC0838  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242258 size=12
    let mut pc: u32 = 0x83242258;
    'dispatch: loop {
        match pc {
            0x83242258 => {
    //   block [0x83242258..0x83242264)
	// 83242258: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324225C: 386BC9EC  addi r3, r11, -0x3614
	ctx.r[3].s64 = ctx.r[11].s64 + -13844;
	// 83242260: 4BEC0828  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242268 size=12
    let mut pc: u32 = 0x83242268;
    'dispatch: loop {
        match pc {
            0x83242268 => {
    //   block [0x83242268..0x83242274)
	// 83242268: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324226C: 386BC9D4  addi r3, r11, -0x362c
	ctx.r[3].s64 = ctx.r[11].s64 + -13868;
	// 83242270: 4BEC0818  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242278 size=12
    let mut pc: u32 = 0x83242278;
    'dispatch: loop {
        match pc {
            0x83242278 => {
    //   block [0x83242278..0x83242284)
	// 83242278: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324227C: 386BC9BC  addi r3, r11, -0x3644
	ctx.r[3].s64 = ctx.r[11].s64 + -13892;
	// 83242280: 4BEC0808  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242288 size=12
    let mut pc: u32 = 0x83242288;
    'dispatch: loop {
        match pc {
            0x83242288 => {
    //   block [0x83242288..0x83242294)
	// 83242288: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324228C: 386BC9A4  addi r3, r11, -0x365c
	ctx.r[3].s64 = ctx.r[11].s64 + -13916;
	// 83242290: 4BEC07F8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242298 size=12
    let mut pc: u32 = 0x83242298;
    'dispatch: loop {
        match pc {
            0x83242298 => {
    //   block [0x83242298..0x832422A4)
	// 83242298: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324229C: 386BCA04  addi r3, r11, -0x35fc
	ctx.r[3].s64 = ctx.r[11].s64 + -13820;
	// 832422A0: 4BEC07E8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832422A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832422A8 size=12
    let mut pc: u32 = 0x832422A8;
    'dispatch: loop {
        match pc {
            0x832422A8 => {
    //   block [0x832422A8..0x832422B4)
	// 832422A8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832422AC: 386BCA68  addi r3, r11, -0x3598
	ctx.r[3].s64 = ctx.r[11].s64 + -13720;
	// 832422B0: 4BEC07D8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832422B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832422B8 size=12
    let mut pc: u32 = 0x832422B8;
    'dispatch: loop {
        match pc {
            0x832422B8 => {
    //   block [0x832422B8..0x832422C4)
	// 832422B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832422BC: 386BCA50  addi r3, r11, -0x35b0
	ctx.r[3].s64 = ctx.r[11].s64 + -13744;
	// 832422C0: 4BEC07C8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832422C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832422C8 size=12
    let mut pc: u32 = 0x832422C8;
    'dispatch: loop {
        match pc {
            0x832422C8 => {
    //   block [0x832422C8..0x832422D4)
	// 832422C8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832422CC: 386BCAE4  addi r3, r11, -0x351c
	ctx.r[3].s64 = ctx.r[11].s64 + -13596;
	// 832422D0: 4BEC07B8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832422D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832422D8 size=12
    let mut pc: u32 = 0x832422D8;
    'dispatch: loop {
        match pc {
            0x832422D8 => {
    //   block [0x832422D8..0x832422E4)
	// 832422D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832422DC: 386BCB14  addi r3, r11, -0x34ec
	ctx.r[3].s64 = ctx.r[11].s64 + -13548;
	// 832422E0: 4BEC07A8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832422E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832422E8 size=12
    let mut pc: u32 = 0x832422E8;
    'dispatch: loop {
        match pc {
            0x832422E8 => {
    //   block [0x832422E8..0x832422F4)
	// 832422E8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832422EC: 386BCAFC  addi r3, r11, -0x3504
	ctx.r[3].s64 = ctx.r[11].s64 + -13572;
	// 832422F0: 4BEC0798  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832422F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832422F8 size=12
    let mut pc: u32 = 0x832422F8;
    'dispatch: loop {
        match pc {
            0x832422F8 => {
    //   block [0x832422F8..0x83242304)
	// 832422F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832422FC: 386BCB2C  addi r3, r11, -0x34d4
	ctx.r[3].s64 = ctx.r[11].s64 + -13524;
	// 83242300: 4BEC0788  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242308 size=12
    let mut pc: u32 = 0x83242308;
    'dispatch: loop {
        match pc {
            0x83242308 => {
    //   block [0x83242308..0x83242314)
	// 83242308: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324230C: 386BCB5C  addi r3, r11, -0x34a4
	ctx.r[3].s64 = ctx.r[11].s64 + -13476;
	// 83242310: 4BEC0778  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242318 size=12
    let mut pc: u32 = 0x83242318;
    'dispatch: loop {
        match pc {
            0x83242318 => {
    //   block [0x83242318..0x83242324)
	// 83242318: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324231C: 386BCB44  addi r3, r11, -0x34bc
	ctx.r[3].s64 = ctx.r[11].s64 + -13500;
	// 83242320: 4BEC0768  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242328 size=12
    let mut pc: u32 = 0x83242328;
    'dispatch: loop {
        match pc {
            0x83242328 => {
    //   block [0x83242328..0x83242334)
	// 83242328: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324232C: 386BCBEC  addi r3, r11, -0x3414
	ctx.r[3].s64 = ctx.r[11].s64 + -13332;
	// 83242330: 4BEC0758  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242338 size=12
    let mut pc: u32 = 0x83242338;
    'dispatch: loop {
        match pc {
            0x83242338 => {
    //   block [0x83242338..0x83242344)
	// 83242338: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324233C: 386BCBD4  addi r3, r11, -0x342c
	ctx.r[3].s64 = ctx.r[11].s64 + -13356;
	// 83242340: 4BEC0748  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242348 size=12
    let mut pc: u32 = 0x83242348;
    'dispatch: loop {
        match pc {
            0x83242348 => {
    //   block [0x83242348..0x83242354)
	// 83242348: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324234C: 386BCBBC  addi r3, r11, -0x3444
	ctx.r[3].s64 = ctx.r[11].s64 + -13380;
	// 83242350: 4BEC0738  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242358 size=12
    let mut pc: u32 = 0x83242358;
    'dispatch: loop {
        match pc {
            0x83242358 => {
    //   block [0x83242358..0x83242364)
	// 83242358: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324235C: 386BCBA4  addi r3, r11, -0x345c
	ctx.r[3].s64 = ctx.r[11].s64 + -13404;
	// 83242360: 4BEC0728  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242368 size=12
    let mut pc: u32 = 0x83242368;
    'dispatch: loop {
        match pc {
            0x83242368 => {
    //   block [0x83242368..0x83242374)
	// 83242368: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324236C: 386BCB8C  addi r3, r11, -0x3474
	ctx.r[3].s64 = ctx.r[11].s64 + -13428;
	// 83242370: 4BEC0718  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242378 size=12
    let mut pc: u32 = 0x83242378;
    'dispatch: loop {
        match pc {
            0x83242378 => {
    //   block [0x83242378..0x83242384)
	// 83242378: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324237C: 386BCB74  addi r3, r11, -0x348c
	ctx.r[3].s64 = ctx.r[11].s64 + -13452;
	// 83242380: 4BEC0708  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242388 size=12
    let mut pc: u32 = 0x83242388;
    'dispatch: loop {
        match pc {
            0x83242388 => {
    //   block [0x83242388..0x83242394)
	// 83242388: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324238C: 386BCC08  addi r3, r11, -0x33f8
	ctx.r[3].s64 = ctx.r[11].s64 + -13304;
	// 83242390: 4BEC06F8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242398 size=12
    let mut pc: u32 = 0x83242398;
    'dispatch: loop {
        match pc {
            0x83242398 => {
    //   block [0x83242398..0x832423A4)
	// 83242398: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324239C: 386BCC54  addi r3, r11, -0x33ac
	ctx.r[3].s64 = ctx.r[11].s64 + -13228;
	// 832423A0: 4BEC06E8  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832423A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832423A8 size=4
    let mut pc: u32 = 0x832423A8;
    'dispatch: loop {
        match pc {
            0x832423A8 => {
    //   block [0x832423A8..0x832423AC)
	// 832423A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832423B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832423B0 size=12
    let mut pc: u32 = 0x832423B0;
    'dispatch: loop {
        match pc {
            0x832423B0 => {
    //   block [0x832423B0..0x832423BC)
	// 832423B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832423B4: 386BCCA0  addi r3, r11, -0x3360
	ctx.r[3].s64 = ctx.r[11].s64 + -13152;
	// 832423B8: 4BEC06D0  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832423C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832423C0 size=12
    let mut pc: u32 = 0x832423C0;
    'dispatch: loop {
        match pc {
            0x832423C0 => {
    //   block [0x832423C0..0x832423CC)
	// 832423C0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832423C4: 386BCCB8  addi r3, r11, -0x3348
	ctx.r[3].s64 = ctx.r[11].s64 + -13128;
	// 832423C8: 4BEC06C0  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832423D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832423D0 size=12
    let mut pc: u32 = 0x832423D0;
    'dispatch: loop {
        match pc {
            0x832423D0 => {
    //   block [0x832423D0..0x832423DC)
	// 832423D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832423D4: 386BCCD0  addi r3, r11, -0x3330
	ctx.r[3].s64 = ctx.r[11].s64 + -13104;
	// 832423D8: 4BEC06B0  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832423E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832423E0 size=12
    let mut pc: u32 = 0x832423E0;
    'dispatch: loop {
        match pc {
            0x832423E0 => {
    //   block [0x832423E0..0x832423EC)
	// 832423E0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832423E4: 386BCCE8  addi r3, r11, -0x3318
	ctx.r[3].s64 = ctx.r[11].s64 + -13080;
	// 832423E8: 4BEC06A0  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832423F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832423F0 size=12
    let mut pc: u32 = 0x832423F0;
    'dispatch: loop {
        match pc {
            0x832423F0 => {
    //   block [0x832423F0..0x832423FC)
	// 832423F0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832423F4: 386BC3E0  addi r3, r11, -0x3c20
	ctx.r[3].s64 = ctx.r[11].s64 + -15392;
	// 832423F8: 4BE93BB0  b 0x830d5fa8
	sub_830D5FA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242400 size=12
    let mut pc: u32 = 0x83242400;
    'dispatch: loop {
        match pc {
            0x83242400 => {
    //   block [0x83242400..0x8324240C)
	// 83242400: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242404: 386BD85C  addi r3, r11, -0x27a4
	ctx.r[3].s64 = ctx.r[11].s64 + -10148;
	// 83242408: 4BEC0680  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242410 size=12
    let mut pc: u32 = 0x83242410;
    'dispatch: loop {
        match pc {
            0x83242410 => {
    //   block [0x83242410..0x8324241C)
	// 83242410: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242414: 386BD874  addi r3, r11, -0x278c
	ctx.r[3].s64 = ctx.r[11].s64 + -10124;
	// 83242418: 4BEC0670  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242420 size=12
    let mut pc: u32 = 0x83242420;
    'dispatch: loop {
        match pc {
            0x83242420 => {
    //   block [0x83242420..0x8324242C)
	// 83242420: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242424: 386BD88C  addi r3, r11, -0x2774
	ctx.r[3].s64 = ctx.r[11].s64 + -10100;
	// 83242428: 4BEC0660  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242430 size=12
    let mut pc: u32 = 0x83242430;
    'dispatch: loop {
        match pc {
            0x83242430 => {
    //   block [0x83242430..0x8324243C)
	// 83242430: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242434: 386BD8A4  addi r3, r11, -0x275c
	ctx.r[3].s64 = ctx.r[11].s64 + -10076;
	// 83242438: 4BEC0650  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242440 size=12
    let mut pc: u32 = 0x83242440;
    'dispatch: loop {
        match pc {
            0x83242440 => {
    //   block [0x83242440..0x8324244C)
	// 83242440: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242444: 386BD8BC  addi r3, r11, -0x2744
	ctx.r[3].s64 = ctx.r[11].s64 + -10052;
	// 83242448: 4BEC0640  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242450 size=12
    let mut pc: u32 = 0x83242450;
    'dispatch: loop {
        match pc {
            0x83242450 => {
    //   block [0x83242450..0x8324245C)
	// 83242450: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242454: 386BD8D4  addi r3, r11, -0x272c
	ctx.r[3].s64 = ctx.r[11].s64 + -10028;
	// 83242458: 4BEC0630  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242460 size=12
    let mut pc: u32 = 0x83242460;
    'dispatch: loop {
        match pc {
            0x83242460 => {
    //   block [0x83242460..0x8324246C)
	// 83242460: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242464: 386BD838  addi r3, r11, -0x27c8
	ctx.r[3].s64 = ctx.r[11].s64 + -10184;
	// 83242468: 4BEA0EE0  b 0x830e3348
	sub_830E3348(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242470 size=12
    let mut pc: u32 = 0x83242470;
    'dispatch: loop {
        match pc {
            0x83242470 => {
    //   block [0x83242470..0x8324247C)
	// 83242470: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242474: 386BD818  addi r3, r11, -0x27e8
	ctx.r[3].s64 = ctx.r[11].s64 + -10216;
	// 83242478: 4BE9BB20  b 0x830ddf98
	sub_830DDF98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242480 size=12
    let mut pc: u32 = 0x83242480;
    'dispatch: loop {
        match pc {
            0x83242480 => {
    //   block [0x83242480..0x8324248C)
	// 83242480: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242484: 386BD828  addi r3, r11, -0x27d8
	ctx.r[3].s64 = ctx.r[11].s64 + -10200;
	// 83242488: 4BE9BB10  b 0x830ddf98
	sub_830DDF98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242490 size=12
    let mut pc: u32 = 0x83242490;
    'dispatch: loop {
        match pc {
            0x83242490 => {
    //   block [0x83242490..0x8324249C)
	// 83242490: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242494: 386BD900  addi r3, r11, -0x2700
	ctx.r[3].s64 = ctx.r[11].s64 + -9984;
	// 83242498: 4BEC05F0  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832424A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832424A0 size=12
    let mut pc: u32 = 0x832424A0;
    'dispatch: loop {
        match pc {
            0x832424A0 => {
    //   block [0x832424A0..0x832424AC)
	// 832424A0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832424A4: 386BD918  addi r3, r11, -0x26e8
	ctx.r[3].s64 = ctx.r[11].s64 + -9960;
	// 832424A8: 4BEC05E0  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832424B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832424B0 size=12
    let mut pc: u32 = 0x832424B0;
    'dispatch: loop {
        match pc {
            0x832424B0 => {
    //   block [0x832424B0..0x832424BC)
	// 832424B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832424B4: 386BD930  addi r3, r11, -0x26d0
	ctx.r[3].s64 = ctx.r[11].s64 + -9936;
	// 832424B8: 4BEC05D0  b 0x83102a88
	sub_83102A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832424C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832424C0 size=12
    let mut pc: u32 = 0x832424C0;
    'dispatch: loop {
        match pc {
            0x832424C0 => {
    //   block [0x832424C0..0x832424CC)
	// 832424C0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832424C4: 386BD8F4  addi r3, r11, -0x270c
	ctx.r[3].s64 = ctx.r[11].s64 + -9996;
	// 832424C8: 4BEA4448  b 0x830e6910
	sub_830E6910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832424D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832424D0 size=12
    let mut pc: u32 = 0x832424D0;
    'dispatch: loop {
        match pc {
            0x832424D0 => {
    //   block [0x832424D0..0x832424DC)
	// 832424D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832424D4: 386BD964  addi r3, r11, -0x269c
	ctx.r[3].s64 = ctx.r[11].s64 + -9884;
	// 832424D8: 4BEA4438  b 0x830e6910
	sub_830E6910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832424E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832424E0 size=12
    let mut pc: u32 = 0x832424E0;
    'dispatch: loop {
        match pc {
            0x832424E0 => {
    //   block [0x832424E0..0x832424EC)
	// 832424E0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832424E4: 386BD998  addi r3, r11, -0x2668
	ctx.r[3].s64 = ctx.r[11].s64 + -9832;
	// 832424E8: 4BE9BAB0  b 0x830ddf98
	sub_830DDF98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832424F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832424F0 size=12
    let mut pc: u32 = 0x832424F0;
    'dispatch: loop {
        match pc {
            0x832424F0 => {
    //   block [0x832424F0..0x832424FC)
	// 832424F0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 832424F4: 386BD9C4  addi r3, r11, -0x263c
	ctx.r[3].s64 = ctx.r[11].s64 + -9788;
	// 832424F8: 4BE9BAA0  b 0x830ddf98
	sub_830DDF98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242500 size=12
    let mut pc: u32 = 0x83242500;
    'dispatch: loop {
        match pc {
            0x83242500 => {
    //   block [0x83242500..0x8324250C)
	// 83242500: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242504: 386BD9F0  addi r3, r11, -0x2610
	ctx.r[3].s64 = ctx.r[11].s64 + -9744;
	// 83242508: 4BE9BA90  b 0x830ddf98
	sub_830DDF98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242510 size=12
    let mut pc: u32 = 0x83242510;
    'dispatch: loop {
        match pc {
            0x83242510 => {
    //   block [0x83242510..0x8324251C)
	// 83242510: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83242514: 386BDB6C  addi r3, r11, -0x2494
	ctx.r[3].s64 = ctx.r[11].s64 + -9364;
	// 83242518: 4BED5FF8  b 0x83118510
	sub_83118510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242520 size=4
    let mut pc: u32 = 0x83242520;
    'dispatch: loop {
        match pc {
            0x83242520 => {
    //   block [0x83242520..0x83242524)
	// 83242520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242528 size=12
    let mut pc: u32 = 0x83242528;
    'dispatch: loop {
        match pc {
            0x83242528 => {
    //   block [0x83242528..0x83242534)
	// 83242528: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8324252C: 386BDEEC  addi r3, r11, -0x2114
	ctx.r[3].s64 = ctx.r[11].s64 + -8468;
	// 83242530: 4BED3598  b 0x83115ac8
	sub_83115AC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242538 size=4
    let mut pc: u32 = 0x83242538;
    'dispatch: loop {
        match pc {
            0x83242538 => {
    //   block [0x83242538..0x8324253C)
	// 83242538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242540 size=4
    let mut pc: u32 = 0x83242540;
    'dispatch: loop {
        match pc {
            0x83242540 => {
    //   block [0x83242540..0x83242544)
	// 83242540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242548 size=20
    let mut pc: u32 = 0x83242548;
    'dispatch: loop {
        match pc {
            0x83242548 => {
    //   block [0x83242548..0x8324255C)
	// 83242548: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8324254C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 83242550: 396B8958  addi r11, r11, -0x76a8
	ctx.r[11].s64 = ctx.r[11].s64 + -30376;
	// 83242554: 916ABDB4  stw r11, -0x424c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16972 as u32), ctx.r[11].u32 ) };
	// 83242558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242560 size=20
    let mut pc: u32 = 0x83242560;
    'dispatch: loop {
        match pc {
            0x83242560 => {
    //   block [0x83242560..0x83242574)
	// 83242560: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83242564: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 83242568: 396BFF60  addi r11, r11, -0xa0
	ctx.r[11].s64 = ctx.r[11].s64 + -160;
	// 8324256C: 916A0C24  stw r11, 0xc24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(3108 as u32), ctx.r[11].u32 ) };
	// 83242570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83242578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83242578 size=20
    let mut pc: u32 = 0x83242578;
    'dispatch: loop {
        match pc {
            0x83242578 => {
    //   block [0x83242578..0x8324258C)
	// 83242578: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8324257C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 83242580: 396BFF60  addi r11, r11, -0xa0
	ctx.r[11].s64 = ctx.r[11].s64 + -160;
	// 83242584: 916A0C44  stw r11, 0xc44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(3140 as u32), ctx.r[11].u32 ) };
	// 83242588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324258C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324258C size=16
    let mut pc: u32 = 0x8324258C;
    'dispatch: loop {
        match pc {
            0x8324258C => {
    //   block [0x8324258C..0x8324259C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324259C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324259C size=16
    let mut pc: u32 = 0x8324259C;
    'dispatch: loop {
        match pc {
            0x8324259C => {
    //   block [0x8324259C..0x832425AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832425AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832425AC size=16
    let mut pc: u32 = 0x832425AC;
    'dispatch: loop {
        match pc {
            0x832425AC => {
    //   block [0x832425AC..0x832425BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832425BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832425BC size=16
    let mut pc: u32 = 0x832425BC;
    'dispatch: loop {
        match pc {
            0x832425BC => {
    //   block [0x832425BC..0x832425CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832425CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832425CC size=16
    let mut pc: u32 = 0x832425CC;
    'dispatch: loop {
        match pc {
            0x832425CC => {
    //   block [0x832425CC..0x832425DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832425DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832425DC size=16
    let mut pc: u32 = 0x832425DC;
    'dispatch: loop {
        match pc {
            0x832425DC => {
    //   block [0x832425DC..0x832425EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832425EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832425EC size=16
    let mut pc: u32 = 0x832425EC;
    'dispatch: loop {
        match pc {
            0x832425EC => {
    //   block [0x832425EC..0x832425FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832425FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832425FC size=16
    let mut pc: u32 = 0x832425FC;
    'dispatch: loop {
        match pc {
            0x832425FC => {
    //   block [0x832425FC..0x8324260C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324260C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324260C size=16
    let mut pc: u32 = 0x8324260C;
    'dispatch: loop {
        match pc {
            0x8324260C => {
    //   block [0x8324260C..0x8324261C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324261C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324261C size=16
    let mut pc: u32 = 0x8324261C;
    'dispatch: loop {
        match pc {
            0x8324261C => {
    //   block [0x8324261C..0x8324262C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324262C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324262C size=16
    let mut pc: u32 = 0x8324262C;
    'dispatch: loop {
        match pc {
            0x8324262C => {
    //   block [0x8324262C..0x8324263C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324263C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324263C size=16
    let mut pc: u32 = 0x8324263C;
    'dispatch: loop {
        match pc {
            0x8324263C => {
    //   block [0x8324263C..0x8324264C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324264C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324264C size=16
    let mut pc: u32 = 0x8324264C;
    'dispatch: loop {
        match pc {
            0x8324264C => {
    //   block [0x8324264C..0x8324265C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324265C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324265C size=16
    let mut pc: u32 = 0x8324265C;
    'dispatch: loop {
        match pc {
            0x8324265C => {
    //   block [0x8324265C..0x8324266C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324266C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324266C size=16
    let mut pc: u32 = 0x8324266C;
    'dispatch: loop {
        match pc {
            0x8324266C => {
    //   block [0x8324266C..0x8324267C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324267C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324267C size=16
    let mut pc: u32 = 0x8324267C;
    'dispatch: loop {
        match pc {
            0x8324267C => {
    //   block [0x8324267C..0x8324268C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324268C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324268C size=16
    let mut pc: u32 = 0x8324268C;
    'dispatch: loop {
        match pc {
            0x8324268C => {
    //   block [0x8324268C..0x8324269C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


