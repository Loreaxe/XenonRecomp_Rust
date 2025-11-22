pub fn sub_831333FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831333FC size=4
    let mut pc: u32 = 0x831333FC;
    'dispatch: loop {
        match pc {
            0x831333FC => {
    //   block [0x831333FC..0x83133400)
	// 831333FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133400 size=100
    let mut pc: u32 = 0x83133400;
    'dispatch: loop {
        match pc {
            0x83133400 => {
    //   block [0x83133400..0x83133464)
	// 83133400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8313340C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83133410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133414: 480054E5  bl 0x831388f8
	ctx.lr = 0x83133418;
	sub_831388F8(ctx, base);
	// 83133418: 480055D9  bl 0x831389f0
	ctx.lr = 0x8313341C;
	sub_831389F0(ctx, base);
	// 8313341C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83133420: 83FE7E28  lwz r31, 0x7e28(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32296 as u32) ) } as u64;
	// 83133424: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83133428: 409A0018  bne cr6, 0x83133440
	if !ctx.cr[6].eq {
	pc = 0x83133440; continue 'dispatch;
	}
	// 8313342C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83133430: 38A006C0  li r5, 0x6c0
	ctx.r[5].s64 = 1728;
	// 83133434: 386B94C0  addi r3, r11, -0x6b40
	ctx.r[3].s64 = ctx.r[11].s64 + -27456;
	// 83133438: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313343C: 48074DA5  bl 0x831a81e0
	ctx.lr = 0x83133440;
	sub_831A81E0(ctx, base);
	// 83133440: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 83133444: 917E7E28  stw r11, 0x7e28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32296 as u32), ctx.r[11].u32 ) };
	// 83133448: 480055E9  bl 0x83138a30
	ctx.lr = 0x8313344C;
	sub_83138A30(ctx, base);
	// 8313344C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83133450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133458: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8313345C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83133460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133468 size=80
    let mut pc: u32 = 0x83133468;
    'dispatch: loop {
        match pc {
            0x83133468 => {
    //   block [0x83133468..0x831334B8)
	// 83133468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313346C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133474: 4800557D  bl 0x831389f0
	ctx.lr = 0x83133478;
	sub_831389F0(ctx, base);
	// 83133478: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8313347C: 816A7E28  lwz r11, 0x7e28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32296 as u32) ) } as u64;
	// 83133480: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133484: 916A7E28  stw r11, 0x7e28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32296 as u32), ctx.r[11].u32 ) };
	// 83133488: 40820018  bne 0x831334a0
	if !ctx.cr[0].eq {
	pc = 0x831334A0; continue 'dispatch;
	}
	// 8313348C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83133490: 38A006C0  li r5, 0x6c0
	ctx.r[5].s64 = 1728;
	// 83133494: 386B94C0  addi r3, r11, -0x6b40
	ctx.r[3].s64 = ctx.r[11].s64 + -27456;
	// 83133498: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313349C: 48074D45  bl 0x831a81e0
	ctx.lr = 0x831334A0;
	sub_831A81E0(ctx, base);
	// 831334A0: 48005591  bl 0x83138a30
	ctx.lr = 0x831334A4;
	sub_83138A30(ctx, base);
	// 831334A4: 480054D5  bl 0x83138978
	ctx.lr = 0x831334A8;
	sub_83138978(ctx, base);
	// 831334A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831334AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831334B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831334B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831334B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831334B8 size=136
    let mut pc: u32 = 0x831334B8;
    'dispatch: loop {
        match pc {
            0x831334B8 => {
    //   block [0x831334B8..0x83133540)
	// 831334B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831334BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831334C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831334C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831334C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831334CC: 48005525  bl 0x831389f0
	ctx.lr = 0x831334D0;
	sub_831389F0(ctx, base);
	// 831334D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831334D4: 409A001C  bne cr6, 0x831334f0
	if !ctx.cr[6].eq {
	pc = 0x831334F0; continue 'dispatch;
	}
	// 831334D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831334DC: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831334E0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831334E4: 386B16DC  addi r3, r11, 0x16dc
	ctx.r[3].s64 = ctx.r[11].s64 + 5852;
	// 831334E8: 4BFFFDB9  bl 0x831332a0
	ctx.lr = 0x831334EC;
	sub_831332A0(ctx, base);
	// 831334EC: 4800003C  b 0x83133528
	pc = 0x83133528; continue 'dispatch;
	// 831334F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831334F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831334F8: 409A0018  bne cr6, 0x83133510
	if !ctx.cr[6].eq {
	pc = 0x83133510; continue 'dispatch;
	}
	// 831334FC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133500: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133504: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133508: 386B16B0  addi r3, r11, 0x16b0
	ctx.r[3].s64 = ctx.r[11].s64 + 5808;
	// 8313350C: 4BFFFFDC  b 0x831334e8
	pc = 0x831334E8; continue 'dispatch;
	// 83133510: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 83133514: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83133518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313351C: 48074CC5  bl 0x831a81e0
	ctx.lr = 0x83133520;
	sub_831A81E0(ctx, base);
	// 83133520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133524: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83133528: 48005509  bl 0x83138a30
	ctx.lr = 0x8313352C;
	sub_83138A30(ctx, base);
	// 8313352C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83133530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313353C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133540 size=124
    let mut pc: u32 = 0x83133540;
    'dispatch: loop {
        match pc {
            0x83133540 => {
    //   block [0x83133540..0x831335BC)
	// 83133540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133548: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313354C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133554: 4800549D  bl 0x831389f0
	ctx.lr = 0x83133558;
	sub_831389F0(ctx, base);
	// 83133558: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313355C: 409A0020  bne cr6, 0x8313357c
	if !ctx.cr[6].eq {
	pc = 0x8313357C; continue 'dispatch;
	}
	// 83133560: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133564: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133568: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313356C: 386B1710  addi r3, r11, 0x1710
	ctx.r[3].s64 = ctx.r[11].s64 + 5904;
	// 83133570: 4BFFFD31  bl 0x831332a0
	ctx.lr = 0x83133574;
	sub_831332A0(ctx, base);
	// 83133574: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83133578: 48000028  b 0x831335a0
	pc = 0x831335A0; continue 'dispatch;
	// 8313357C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133580: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133584: 409A0018  bne cr6, 0x8313359c
	if !ctx.cr[6].eq {
	pc = 0x8313359C; continue 'dispatch;
	}
	// 83133588: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313358C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133590: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133594: 386B1704  addi r3, r11, 0x1704
	ctx.r[3].s64 = ctx.r[11].s64 + 5892;
	// 83133598: 4BFFFFD8  b 0x83133570
	pc = 0x83133570; continue 'dispatch;
	// 8313359C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831335A0: 48005491  bl 0x83138a30
	ctx.lr = 0x831335A4;
	sub_83138A30(ctx, base);
	// 831335A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831335A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831335AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831335B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831335B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831335B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831335C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831335C0 size=112
    let mut pc: u32 = 0x831335C0;
    'dispatch: loop {
        match pc {
            0x831335C0 => {
    //   block [0x831335C0..0x83133630)
	// 831335C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831335C4: 48074BA9  bl 0x831a816c
	ctx.lr = 0x831335C8;
	sub_831A8130(ctx, base);
	// 831335C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831335CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831335D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831335D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831335D8: 48005419  bl 0x831389f0
	ctx.lr = 0x831335DC;
	sub_831389F0(ctx, base);
	// 831335DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831335E0: 409A001C  bne cr6, 0x831335fc
	if !ctx.cr[6].eq {
	pc = 0x831335FC; continue 'dispatch;
	}
	// 831335E4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831335E8: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831335EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831335F0: 386B1728  addi r3, r11, 0x1728
	ctx.r[3].s64 = ctx.r[11].s64 + 5928;
	// 831335F4: 4BFFFCAD  bl 0x831332a0
	ctx.lr = 0x831335F8;
	sub_831332A0(ctx, base);
	// 831335F8: 4800002C  b 0x83133624
	pc = 0x83133624; continue 'dispatch;
	// 831335FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133600: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133604: 409A0018  bne cr6, 0x8313361c
	if !ctx.cr[6].eq {
	pc = 0x8313361C; continue 'dispatch;
	}
	// 83133608: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313360C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133610: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133614: 386B171C  addi r3, r11, 0x171c
	ctx.r[3].s64 = ctx.r[11].s64 + 5916;
	// 83133618: 4BFFFFDC  b 0x831335f4
	pc = 0x831335F4; continue 'dispatch;
	// 8313361C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83133620: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83133624: 4800540D  bl 0x83138a30
	ctx.lr = 0x83133628;
	sub_83138A30(ctx, base);
	// 83133628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313362C: 48074B90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133630 size=128
    let mut pc: u32 = 0x83133630;
    'dispatch: loop {
        match pc {
            0x83133630 => {
    //   block [0x83133630..0x831336B0)
	// 83133630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313363C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133640: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133644: 480053AD  bl 0x831389f0
	ctx.lr = 0x83133648;
	sub_831389F0(ctx, base);
	// 83133648: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313364C: 409A001C  bne cr6, 0x83133668
	if !ctx.cr[6].eq {
	pc = 0x83133668; continue 'dispatch;
	}
	// 83133650: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133654: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133658: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313365C: 386B1740  addi r3, r11, 0x1740
	ctx.r[3].s64 = ctx.r[11].s64 + 5952;
	// 83133660: 4BFFFC41  bl 0x831332a0
	ctx.lr = 0x83133664;
	sub_831332A0(ctx, base);
	// 83133664: 48000034  b 0x83133698
	pc = 0x83133698; continue 'dispatch;
	// 83133668: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313366C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133670: 409A0018  bne cr6, 0x83133688
	if !ctx.cr[6].eq {
	pc = 0x83133688; continue 'dispatch;
	}
	// 83133674: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133678: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 8313367C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133680: 386B1734  addi r3, r11, 0x1734
	ctx.r[3].s64 = ctx.r[11].s64 + 5940;
	// 83133684: 4BFFFFDC  b 0x83133660
	pc = 0x83133660; continue 'dispatch;
	// 83133688: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8313368C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83133690: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83133694: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83133698: 48005399  bl 0x83138a30
	ctx.lr = 0x8313369C;
	sub_83138A30(ctx, base);
	// 8313369C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831336A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831336A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831336A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831336AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831336B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831336B0 size=184
    let mut pc: u32 = 0x831336B0;
    'dispatch: loop {
        match pc {
            0x831336B0 => {
    //   block [0x831336B0..0x83133768)
	// 831336B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831336B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831336B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831336BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831336C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831336C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831336C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831336CC: 48005325  bl 0x831389f0
	ctx.lr = 0x831336D0;
	sub_831389F0(ctx, base);
	// 831336D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831336D4: 409A001C  bne cr6, 0x831336f0
	if !ctx.cr[6].eq {
	pc = 0x831336F0; continue 'dispatch;
	}
	// 831336D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831336DC: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831336E0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831336E4: 386B1758  addi r3, r11, 0x1758
	ctx.r[3].s64 = ctx.r[11].s64 + 5976;
	// 831336E8: 4BFFFBB9  bl 0x831332a0
	ctx.lr = 0x831336EC;
	sub_831332A0(ctx, base);
	// 831336EC: 48000058  b 0x83133744
	pc = 0x83133744; continue 'dispatch;
	// 831336F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831336F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831336F8: 409A0018  bne cr6, 0x83133710
	if !ctx.cr[6].eq {
	pc = 0x83133710; continue 'dispatch;
	}
	// 831336FC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133700: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133704: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133708: 386B174C  addi r3, r11, 0x174c
	ctx.r[3].s64 = ctx.r[11].s64 + 5964;
	// 8313370C: 4BFFFFDC  b 0x831336e8
	pc = 0x831336E8; continue 'dispatch;
	// 83133710: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83133714: 409A000C  bne cr6, 0x83133720
	if !ctx.cr[6].eq {
	pc = 0x83133720; continue 'dispatch;
	}
	// 83133718: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8313371C: 4800002C  b 0x83133748
	pc = 0x83133748; continue 'dispatch;
	// 83133720: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83133724: 419A0020  beq cr6, 0x83133744
	if ctx.cr[6].eq {
	pc = 0x83133744; continue 'dispatch;
	}
	// 83133728: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313372C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83133730: 41820014  beq 0x83133744
	if ctx.cr[0].eq {
	pc = 0x83133744; continue 'dispatch;
	}
	// 83133734: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133738: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8313373C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83133740: 4E800421  bctrl
	ctx.lr = 0x83133744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83133744: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83133748: 480052E9  bl 0x83138a30
	ctx.lr = 0x8313374C;
	sub_83138A30(ctx, base);
	// 8313374C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83133750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83133754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313375C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83133760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83133764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133768 size=248
    let mut pc: u32 = 0x83133768;
    'dispatch: loop {
        match pc {
            0x83133768 => {
    //   block [0x83133768..0x83133860)
	// 83133768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313376C: 480749FD  bl 0x831a8168
	ctx.lr = 0x83133770;
	sub_831A8130(ctx, base);
	// 83133770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133778: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8313377C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83133780: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83133784: 4800526D  bl 0x831389f0
	ctx.lr = 0x83133788;
	sub_831389F0(ctx, base);
	// 83133788: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313378C: 409A001C  bne cr6, 0x831337a8
	if !ctx.cr[6].eq {
	pc = 0x831337A8; continue 'dispatch;
	}
	// 83133790: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133794: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133798: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313379C: 386B1770  addi r3, r11, 0x1770
	ctx.r[3].s64 = ctx.r[11].s64 + 6000;
	// 831337A0: 4BFFFB01  bl 0x831332a0
	ctx.lr = 0x831337A4;
	sub_831332A0(ctx, base);
	// 831337A4: 480000B0  b 0x83133854
	pc = 0x83133854; continue 'dispatch;
	// 831337A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831337AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831337B0: 409A0018  bne cr6, 0x831337c8
	if !ctx.cr[6].eq {
	pc = 0x831337C8; continue 'dispatch;
	}
	// 831337B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831337B8: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 831337BC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831337C0: 386B1764  addi r3, r11, 0x1764
	ctx.r[3].s64 = ctx.r[11].s64 + 5988;
	// 831337C4: 4BFFFFDC  b 0x831337a0
	pc = 0x831337A0; continue 'dispatch;
	// 831337C8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831337CC: 409A0014  bne cr6, 0x831337e0
	if !ctx.cr[6].eq {
	pc = 0x831337E0; continue 'dispatch;
	}
	// 831337D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831337D4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831337D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831337DC: 48000078  b 0x83133854
	pc = 0x83133854; continue 'dispatch;
	// 831337E0: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 831337E4: 409A0048  bne cr6, 0x8313382c
	if !ctx.cr[6].eq {
	pc = 0x8313382C; continue 'dispatch;
	}
	// 831337E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831337EC: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831337F0: 41980008  blt cr6, 0x831337f8
	if ctx.cr[6].lt {
	pc = 0x831337F8; continue 'dispatch;
	}
	// 831337F4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 831337F8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831337FC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83133800: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83133804: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83133808: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8313380C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83133810: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83133814: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83133818: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8313381C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133820: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83133824: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83133828: 4800002C  b 0x83133854
	pc = 0x83133854; continue 'dispatch;
	// 8313382C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133830: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83133834: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83133838: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313383C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83133840: 41820014  beq 0x83133854
	if ctx.cr[0].eq {
	pc = 0x83133854; continue 'dispatch;
	}
	// 83133844: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133848: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8313384C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83133850: 4E800421  bctrl
	ctx.lr = 0x83133854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83133854: 480051DD  bl 0x83138a30
	ctx.lr = 0x83133858;
	sub_83138A30(ctx, base);
	// 83133858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8313385C: 4807495C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133860 size=184
    let mut pc: u32 = 0x83133860;
    'dispatch: loop {
        match pc {
            0x83133860 => {
    //   block [0x83133860..0x83133918)
	// 83133860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133864: 48074909  bl 0x831a816c
	ctx.lr = 0x83133868;
	sub_831A8130(ctx, base);
	// 83133868: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313386C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83133870: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83133874: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83133878: 48005179  bl 0x831389f0
	ctx.lr = 0x8313387C;
	sub_831389F0(ctx, base);
	// 8313387C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83133880: 409A001C  bne cr6, 0x8313389c
	if !ctx.cr[6].eq {
	pc = 0x8313389C; continue 'dispatch;
	}
	// 83133884: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133888: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 8313388C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133890: 386B1788  addi r3, r11, 0x1788
	ctx.r[3].s64 = ctx.r[11].s64 + 6024;
	// 83133894: 4BFFFA0D  bl 0x831332a0
	ctx.lr = 0x83133898;
	sub_831332A0(ctx, base);
	// 83133898: 48000074  b 0x8313390c
	pc = 0x8313390C; continue 'dispatch;
	// 8313389C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831338A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831338A4: 409A0018  bne cr6, 0x831338bc
	if !ctx.cr[6].eq {
	pc = 0x831338BC; continue 'dispatch;
	}
	// 831338A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831338AC: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 831338B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831338B4: 386B177C  addi r3, r11, 0x177c
	ctx.r[3].s64 = ctx.r[11].s64 + 6012;
	// 831338B8: 4BFFFFDC  b 0x83133894
	pc = 0x83133894; continue 'dispatch;
	// 831338BC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831338C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831338C4: 40990048  ble cr6, 0x8313390c
	if !ctx.cr[6].gt {
	pc = 0x8313390C; continue 'dispatch;
	}
	// 831338C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831338CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831338D0: 419A003C  beq cr6, 0x8313390c
	if ctx.cr[6].eq {
	pc = 0x8313390C; continue 'dispatch;
	}
	// 831338D4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831338D8: 419A0034  beq cr6, 0x8313390c
	if ctx.cr[6].eq {
	pc = 0x8313390C; continue 'dispatch;
	}
	// 831338DC: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 831338E0: 419A002C  beq cr6, 0x8313390c
	if ctx.cr[6].eq {
	pc = 0x8313390C; continue 'dispatch;
	}
	// 831338E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831338E8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831338EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831338F0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 831338F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831338F8: 41820014  beq 0x8313390c
	if ctx.cr[0].eq {
	pc = 0x8313390C; continue 'dispatch;
	}
	// 831338FC: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133900: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83133904: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83133908: 4E800421  bctrl
	ctx.lr = 0x8313390C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313390C: 48005125  bl 0x83138a30
	ctx.lr = 0x83133910;
	sub_83138A30(ctx, base);
	// 83133910: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83133914: 480748A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133918 size=56
    let mut pc: u32 = 0x83133918;
    'dispatch: loop {
        match pc {
            0x83133918 => {
    //   block [0x83133918..0x83133950)
	// 83133918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313391C: 48074851  bl 0x831a816c
	ctx.lr = 0x83133920;
	sub_831A8130(ctx, base);
	// 83133920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8313392C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83133930: 480050C1  bl 0x831389f0
	ctx.lr = 0x83133934;
	sub_831389F0(ctx, base);
	// 83133934: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83133938: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8313393C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83133940: 4BFFF9B1  bl 0x831332f0
	ctx.lr = 0x83133944;
	sub_831332F0(ctx, base);
	// 83133944: 480050ED  bl 0x83138a30
	ctx.lr = 0x83133948;
	sub_83138A30(ctx, base);
	// 83133948: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313394C: 48074870  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133950 size=200
    let mut pc: u32 = 0x83133950;
    'dispatch: loop {
        match pc {
            0x83133950 => {
    //   block [0x83133950..0x83133A18)
	// 83133950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133954: 48074815  bl 0x831a8168
	ctx.lr = 0x83133958;
	sub_831A8130(ctx, base);
	// 83133958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313395C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133960: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83133964: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83133968: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8313396C: 48005085  bl 0x831389f0
	ctx.lr = 0x83133970;
	sub_831389F0(ctx, base);
	// 83133970: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83133974: 409A0020  bne cr6, 0x83133994
	if !ctx.cr[6].eq {
	pc = 0x83133994; continue 'dispatch;
	}
	// 83133978: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313397C: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133980: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133984: 386B17B8  addi r3, r11, 0x17b8
	ctx.r[3].s64 = ctx.r[11].s64 + 6072;
	// 83133988: 4BFFF919  bl 0x831332a0
	ctx.lr = 0x8313398C;
	sub_831332A0(ctx, base);
	// 8313398C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83133990: 48000078  b 0x83133a08
	pc = 0x83133A08; continue 'dispatch;
	// 83133994: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133998: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313399C: 409A0018  bne cr6, 0x831339b4
	if !ctx.cr[6].eq {
	pc = 0x831339B4; continue 'dispatch;
	}
	// 831339A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831339A4: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 831339A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831339AC: 386B17AC  addi r3, r11, 0x17ac
	ctx.r[3].s64 = ctx.r[11].s64 + 6060;
	// 831339B0: 4BFFFFD8  b 0x83133988
	pc = 0x83133988; continue 'dispatch;
	// 831339B4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831339B8: 419A0040  beq cr6, 0x831339f8
	if ctx.cr[6].eq {
	pc = 0x831339F8; continue 'dispatch;
	}
	// 831339BC: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 831339C0: 409A0018  bne cr6, 0x831339d8
	if !ctx.cr[6].eq {
	pc = 0x831339D8; continue 'dispatch;
	}
	// 831339C4: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831339C8: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831339CC: 4198002C  blt cr6, 0x831339f8
	if ctx.cr[6].lt {
	pc = 0x831339F8; continue 'dispatch;
	}
	// 831339D0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 831339D4: 48000024  b 0x831339f8
	pc = 0x831339F8; continue 'dispatch;
	// 831339D8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831339DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831339E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831339E4: 41820014  beq 0x831339f8
	if ctx.cr[0].eq {
	pc = 0x831339F8; continue 'dispatch;
	}
	// 831339E8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831339EC: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 831339F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831339F4: 4E800421  bctrl
	ctx.lr = 0x831339F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831339F8: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 831339FC: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83133A00: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83133A04: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83133A08: 48005029  bl 0x83138a30
	ctx.lr = 0x83133A0C;
	sub_83138A30(ctx, base);
	// 83133A0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83133A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83133A14: 480747A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133A18 size=124
    let mut pc: u32 = 0x83133A18;
    'dispatch: loop {
        match pc {
            0x83133A18 => {
    //   block [0x83133A18..0x83133A94)
	// 83133A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83133A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133A2C: 48004FC5  bl 0x831389f0
	ctx.lr = 0x83133A30;
	sub_831389F0(ctx, base);
	// 83133A30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83133A34: 409A0020  bne cr6, 0x83133a54
	if !ctx.cr[6].eq {
	pc = 0x83133A54; continue 'dispatch;
	}
	// 83133A38: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133A3C: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133A40: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133A44: 386B17D0  addi r3, r11, 0x17d0
	ctx.r[3].s64 = ctx.r[11].s64 + 6096;
	// 83133A48: 4BFFF859  bl 0x831332a0
	ctx.lr = 0x83133A4C;
	sub_831332A0(ctx, base);
	// 83133A4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83133A50: 48000028  b 0x83133a78
	pc = 0x83133A78; continue 'dispatch;
	// 83133A54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133A58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133A5C: 409A0018  bne cr6, 0x83133a74
	if !ctx.cr[6].eq {
	pc = 0x83133A74; continue 'dispatch;
	}
	// 83133A60: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133A64: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133A68: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133A6C: 386B17C4  addi r3, r11, 0x17c4
	ctx.r[3].s64 = ctx.r[11].s64 + 6084;
	// 83133A70: 4BFFFFD8  b 0x83133a48
	pc = 0x83133A48; continue 'dispatch;
	// 83133A74: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83133A78: 48004FB9  bl 0x83138a30
	ctx.lr = 0x83133A7C;
	sub_83138A30(ctx, base);
	// 83133A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83133A80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83133A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133A8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83133A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133A98 size=184
    let mut pc: u32 = 0x83133A98;
    'dispatch: loop {
        match pc {
            0x83133A98 => {
    //   block [0x83133A98..0x83133B50)
	// 83133A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133A9C: 480746D1  bl 0x831a816c
	ctx.lr = 0x83133AA0;
	sub_831A8130(ctx, base);
	// 83133AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133AA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83133AA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83133AAC: 48004F45  bl 0x831389f0
	ctx.lr = 0x83133AB0;
	sub_831389F0(ctx, base);
	// 83133AB0: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83133AB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83133AB8: 396B94C0  addi r11, r11, -0x6b40
	ctx.r[11].s64 = ctx.r[11].s64 + -27456;
	// 83133ABC: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 83133AC0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83133AC4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83133AC8: 419A0018  beq cr6, 0x83133ae0
	if ctx.cr[6].eq {
	pc = 0x83133AE0; continue 'dispatch;
	}
	// 83133ACC: 39290024  addi r9, r9, 0x24
	ctx.r[9].s64 = ctx.r[9].s64 + 36;
	// 83133AD0: 390B06C4  addi r8, r11, 0x6c4
	ctx.r[8].s64 = ctx.r[11].s64 + 1732;
	// 83133AD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83133AD8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83133ADC: 4198FFE4  blt cr6, 0x83133ac0
	if ctx.cr[6].lt {
	pc = 0x83133AC0; continue 'dispatch;
	}
	// 83133AE0: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 83133AE4: 409A000C  bne cr6, 0x83133af0
	if !ctx.cr[6].eq {
	pc = 0x83133AF0; continue 'dispatch;
	}
	// 83133AE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83133AEC: 48000054  b 0x83133b40
	pc = 0x83133B40; continue 'dispatch;
	// 83133AF0: 1D0A0024  mulli r8, r10, 0x24
	ctx.r[8].s64 = ctx.r[10].s64 * 36;
	// 83133AF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 83133AF8: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 83133AFC: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83133B00: 3D008313  lis r8, -0x7ced
	ctx.r[8].s64 = -2095906816;
	// 83133B04: 394A5A6C  addi r10, r10, 0x5a6c
	ctx.r[10].s64 = ctx.r[10].s64 + 23148;
	// 83133B08: 39291694  addi r9, r9, 0x1694
	ctx.r[9].s64 = ctx.r[9].s64 + 5780;
	// 83133B0C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83133B10: 39083290  addi r8, r8, 0x3290
	ctx.r[8].s64 = ctx.r[8].s64 + 12944;
	// 83133B14: 93AB0014  stw r29, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83133B18: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83133B1C: 93EB0018  stw r31, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 83133B20: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83133B24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83133B28: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83133B2C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 83133B30: 910B001C  stw r8, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 83133B34: 916B0020  stw r11, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83133B38: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83133B3C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83133B40: 48004EF1  bl 0x83138a30
	ctx.lr = 0x83133B44;
	sub_83138A30(ctx, base);
	// 83133B44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83133B48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83133B4C: 48074670  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133B50 size=12
    let mut pc: u32 = 0x83133B50;
    'dispatch: loop {
        match pc {
            0x83133B50 => {
    //   block [0x83133B50..0x83133B5C)
	// 83133B50: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133B54: 386B1820  addi r3, r11, 0x1820
	ctx.r[3].s64 = ctx.r[11].s64 + 6176;
	// 83133B58: 48004D98  b 0x831388f0
	sub_831388F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133B60 size=28
    let mut pc: u32 = 0x83133B60;
    'dispatch: loop {
        match pc {
            0x83133B60 => {
    //   block [0x83133B60..0x83133B7C)
	// 83133B60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83133B64: 409A0018  bne cr6, 0x83133b7c
	if !ctx.cr[6].eq {
		sub_83133B7C(ctx, base);
		return;
	}
	// 83133B68: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133B6C: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133B70: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133B74: 386B1880  addi r3, r11, 0x1880
	ctx.r[3].s64 = ctx.r[11].s64 + 6272;
	// 83133B78: 4BFFF728  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133B7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133B7C size=32
    let mut pc: u32 = 0x83133B7C;
    'dispatch: loop {
        match pc {
            0x83133B7C => {
    //   block [0x83133B7C..0x83133B9C)
	// 83133B7C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133B80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133B84: 409A0018  bne cr6, 0x83133b9c
	if !ctx.cr[6].eq {
		sub_83133B9C(ctx, base);
		return;
	}
	// 83133B88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133B8C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133B90: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133B94: 386B1874  addi r3, r11, 0x1874
	ctx.r[3].s64 = ctx.r[11].s64 + 6260;
	// 83133B98: 4BFFF708  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133B9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133B9C size=44
    let mut pc: u32 = 0x83133B9C;
    'dispatch: loop {
        match pc {
            0x83133B9C => {
    //   block [0x83133B9C..0x83133BC8)
	// 83133B9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133BA0: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133BA4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83133BA8: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83133BAC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83133BB0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83133BB4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83133BB8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83133BBC: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83133BC0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83133BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133BC8 size=32
    let mut pc: u32 = 0x83133BC8;
    'dispatch: loop {
        match pc {
            0x83133BC8 => {
    //   block [0x83133BC8..0x83133BE8)
	// 83133BC8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83133BCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83133BD0: 409A0018  bne cr6, 0x83133be8
	if !ctx.cr[6].eq {
		sub_83133BE8(ctx, base);
		return;
	}
	// 83133BD4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133BD8: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133BDC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133BE0: 386B18D4  addi r3, r11, 0x18d4
	ctx.r[3].s64 = ctx.r[11].s64 + 6356;
	// 83133BE4: 4BFFF6BC  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133BE8 size=32
    let mut pc: u32 = 0x83133BE8;
    'dispatch: loop {
        match pc {
            0x83133BE8 => {
    //   block [0x83133BE8..0x83133C08)
	// 83133BE8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133BEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83133BF0: 409A0018  bne cr6, 0x83133c08
	if !ctx.cr[6].eq {
		sub_83133C08(ctx, base);
		return;
	}
	// 83133BF4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133BF8: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133BFC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133C00: 386B18C8  addi r3, r11, 0x18c8
	ctx.r[3].s64 = ctx.r[11].s64 + 6344;
	// 83133C04: 4BFFF69C  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133C08 size=32
    let mut pc: u32 = 0x83133C08;
    'dispatch: loop {
        match pc {
            0x83133C08 => {
    //   block [0x83133C08..0x83133C28)
	// 83133C08: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133C0C: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83133C10: 40820018  bne 0x83133c28
	if !ctx.cr[0].eq {
		sub_83133C28(ctx, base);
		return;
	}
	// 83133C14: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133C18: 388B18B0  addi r4, r11, 0x18b0
	ctx.r[4].s64 = ctx.r[11].s64 + 6320;
	// 83133C1C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133C20: 386B18A4  addi r3, r11, 0x18a4
	ctx.r[3].s64 = ctx.r[11].s64 + 6308;
	// 83133C24: 4BFFF67C  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133C28 size=140
    let mut pc: u32 = 0x83133C28;
    'dispatch: loop {
        match pc {
            0x83133C28 => {
    //   block [0x83133C28..0x83133CB4)
	// 83133C28: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83133C2C: 409A0088  bne cr6, 0x83133cb4
	if !ctx.cr[6].eq {
		sub_83133CB4(ctx, base);
		return;
	}
	// 83133C30: 81030024  lwz r8, 0x24(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83133C34: 80E30014  lwz r7, 0x14(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83133C38: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83133C3C: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 83133C40: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 83133C44: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83133C48: 41980008  blt cr6, 0x83133c50
	if ctx.cr[6].lt {
	pc = 0x83133C50; continue 'dispatch;
	}
	// 83133C4C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83133C50: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83133C54: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83133C58: 40980008  bge cr6, 0x83133c60
	if !ctx.cr[6].lt {
	pc = 0x83133C60; continue 'dispatch;
	}
	// 83133C5C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83133C60: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83133C64: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83133C68: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83133C6C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83133C70: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83133C74: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83133C78: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133C7C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83133C80: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83133C84: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83133C88: 7CEA4BD6  divw r7, r10, r9
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 83133C8C: 7D2749D6  mullw r9, r7, r9
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83133C90: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83133C94: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83133C98: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133C9C: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 83133CA0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83133CA4: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133CA8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83133CAC: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83133CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133CB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133CB4 size=140
    let mut pc: u32 = 0x83133CB4;
    'dispatch: loop {
        match pc {
            0x83133CB4 => {
    //   block [0x83133CB4..0x83133D40)
	// 83133CB4: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 83133CB8: 409A0088  bne cr6, 0x83133d40
	if !ctx.cr[6].eq {
		sub_83133D40(ctx, base);
		return;
	}
	// 83133CBC: 81030024  lwz r8, 0x24(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83133CC0: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83133CC4: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83133CC8: 7D074050  subf r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 83133CCC: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 83133CD0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83133CD4: 41980008  blt cr6, 0x83133cdc
	if ctx.cr[6].lt {
	pc = 0x83133CDC; continue 'dispatch;
	}
	// 83133CD8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83133CDC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83133CE0: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83133CE4: 40980008  bge cr6, 0x83133cec
	if !ctx.cr[6].lt {
	pc = 0x83133CEC; continue 'dispatch;
	}
	// 83133CE8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83133CEC: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83133CF0: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83133CF4: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83133CF8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83133CFC: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83133D00: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83133D04: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133D08: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83133D0C: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83133D10: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83133D14: 7CEA4BD6  divw r7, r10, r9
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 83133D18: 7D2749D6  mullw r9, r7, r9
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83133D1C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83133D20: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83133D24: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133D28: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 83133D2C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83133D30: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133D34: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83133D38: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83133D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133D40 size=24
    let mut pc: u32 = 0x83133D40;
    'dispatch: loop {
        match pc {
            0x83133D40 => {
    //   block [0x83133D40..0x83133D58)
	// 83133D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133D44: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83133D48: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83133D4C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83133D50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83133D54: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133D58 size=16
    let mut pc: u32 = 0x83133D58;
    'dispatch: loop {
        match pc {
            0x83133D58 => {
    //   block [0x83133D58..0x83133D68)
	// 83133D58: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83133D5C: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83133D60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83133D64: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133D68 size=420
    let mut pc: u32 = 0x83133D68;
    'dispatch: loop {
        match pc {
            0x83133D68 => {
    //   block [0x83133D68..0x83133F0C)
	// 83133D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133D70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83133D74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83133D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133D7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133D80: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83133D84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83133D88: 409A001C  bne cr6, 0x83133da4
	if !ctx.cr[6].eq {
	pc = 0x83133DA4; continue 'dispatch;
	}
	// 83133D8C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133D90: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133D94: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133D98: 386B18EC  addi r3, r11, 0x18ec
	ctx.r[3].s64 = ctx.r[11].s64 + 6380;
	// 83133D9C: 4BFFF505  bl 0x831332a0
	ctx.lr = 0x83133DA0;
	sub_831332A0(ctx, base);
	// 83133DA0: 48000154  b 0x83133ef4
	pc = 0x83133EF4; continue 'dispatch;
	// 83133DA4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133DA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133DAC: 409A0018  bne cr6, 0x83133dc4
	if !ctx.cr[6].eq {
	pc = 0x83133DC4; continue 'dispatch;
	}
	// 83133DB0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133DB4: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133DB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133DBC: 386B18E0  addi r3, r11, 0x18e0
	ctx.r[3].s64 = ctx.r[11].s64 + 6368;
	// 83133DC0: 4BFFFFDC  b 0x83133d9c
	pc = 0x83133D9C; continue 'dispatch;
	// 83133DC4: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133DC8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83133DCC: 40810128  ble 0x83133ef4
	if !ctx.cr[0].gt {
	pc = 0x83133EF4; continue 'dispatch;
	}
	// 83133DD0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83133DD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83133DD8: 419A011C  beq cr6, 0x83133ef4
	if ctx.cr[6].eq {
	pc = 0x83133EF4; continue 'dispatch;
	}
	// 83133DDC: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 83133DE0: 409A00C4  bne cr6, 0x83133ea4
	if !ctx.cr[6].eq {
	pc = 0x83133EA4; continue 'dispatch;
	}
	// 83133DE4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83133DE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83133DEC: 41820014  beq 0x83133e00
	if ctx.cr[0].eq {
	pc = 0x83133E00; continue 'dispatch;
	}
	// 83133DF0: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83133DF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83133DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83133DFC: 4E800421  bctrl
	ctx.lr = 0x83133E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83133E00: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83133E04: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83133E08: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83133E0C: 7D682050  subf r11, r8, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[8].s64;
	// 83133E10: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83133E14: 4098002C  bge cr6, 0x83133e40
	if !ctx.cr[6].lt {
	pc = 0x83133E40; continue 'dispatch;
	}
	// 83133E18: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133E1C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83133E20: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83133E24: 41980008  blt cr6, 0x83133e2c
	if ctx.cr[6].lt {
	pc = 0x83133E2C; continue 'dispatch;
	}
	// 83133E28: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83133E2C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133E30: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 83133E34: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 83133E38: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83133E3C: 480746D5  bl 0x831a8510
	ctx.lr = 0x83133E40;
	sub_831A8510(ctx, base);
	// 83133E40: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133E44: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83133E48: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83133E4C: 7D235850  subf r9, r3, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83133E50: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133E54: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83133E58: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83133E5C: 40990024  ble cr6, 0x83133e80
	if !ctx.cr[6].gt {
	pc = 0x83133E80; continue 'dispatch;
	}
	// 83133E60: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 83133E64: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83133E68: 41980008  blt cr6, 0x83133e70
	if ctx.cr[6].lt {
	pc = 0x83133E70; continue 'dispatch;
	}
	// 83133E6C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83133E70: 7D4B1850  subf r10, r11, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83133E74: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 83133E78: 7C8A4A14  add r4, r10, r9
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83133E7C: 48074695  bl 0x831a8510
	ctx.lr = 0x83133E80;
	sub_831A8510(ctx, base);
	// 83133E80: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133E84: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83133E88: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83133E8C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83133E90: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83133E94: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133E98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83133E9C: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83133EA0: 48000054  b 0x83133ef4
	pc = 0x83133EF4; continue 'dispatch;
	// 83133EA4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83133EA8: 409A0024  bne cr6, 0x83133ecc
	if !ctx.cr[6].eq {
	pc = 0x83133ECC; continue 'dispatch;
	}
	// 83133EAC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83133EB0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83133EB4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83133EB8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83133EBC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133EC0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83133EC4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83133EC8: 4800002C  b 0x83133ef4
	pc = 0x83133EF4; continue 'dispatch;
	// 83133ECC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133ED0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83133ED4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83133ED8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83133EDC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83133EE0: 41820014  beq 0x83133ef4
	if ctx.cr[0].eq {
	pc = 0x83133EF4; continue 'dispatch;
	}
	// 83133EE4: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83133EE8: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83133EEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83133EF0: 4E800421  bctrl
	ctx.lr = 0x83133EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83133EF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83133EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133F00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83133F04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83133F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133F10 size=476
    let mut pc: u32 = 0x83133F10;
    'dispatch: loop {
        match pc {
            0x83133F10 => {
    //   block [0x83133F10..0x831340EC)
	// 83133F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133F18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83133F1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83133F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133F24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133F28: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83133F2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83133F30: 409A001C  bne cr6, 0x83133f4c
	if !ctx.cr[6].eq {
	pc = 0x83133F4C; continue 'dispatch;
	}
	// 83133F34: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133F38: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133F3C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133F40: 386B1910  addi r3, r11, 0x1910
	ctx.r[3].s64 = ctx.r[11].s64 + 6416;
	// 83133F44: 4BFFF35D  bl 0x831332a0
	ctx.lr = 0x83133F48;
	sub_831332A0(ctx, base);
	// 83133F48: 4800018C  b 0x831340d4
	pc = 0x831340D4; continue 'dispatch;
	// 83133F4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133F50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133F54: 409A0018  bne cr6, 0x83133f6c
	if !ctx.cr[6].eq {
	pc = 0x83133F6C; continue 'dispatch;
	}
	// 83133F58: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133F5C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133F60: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133F64: 386B1904  addi r3, r11, 0x1904
	ctx.r[3].s64 = ctx.r[11].s64 + 6404;
	// 83133F68: 4BFFFFDC  b 0x83133f44
	pc = 0x83133F44; continue 'dispatch;
	// 83133F6C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83133F70: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83133F74: 40820018  bne 0x83133f8c
	if !ctx.cr[0].eq {
	pc = 0x83133F8C; continue 'dispatch;
	}
	// 83133F78: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133F7C: 388B18B0  addi r4, r11, 0x18b0
	ctx.r[4].s64 = ctx.r[11].s64 + 6320;
	// 83133F80: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133F84: 386B18F8  addi r3, r11, 0x18f8
	ctx.r[3].s64 = ctx.r[11].s64 + 6392;
	// 83133F88: 4BFFFFBC  b 0x83133f44
	pc = 0x83133F44; continue 'dispatch;
	// 83133F8C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133F90: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83133F94: 40810140  ble 0x831340d4
	if !ctx.cr[0].gt {
	pc = 0x831340D4; continue 'dispatch;
	}
	// 83133F98: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83133F9C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83133FA0: 41820134  beq 0x831340d4
	if ctx.cr[0].eq {
	pc = 0x831340D4; continue 'dispatch;
	}
	// 83133FA4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83133FA8: 409A0080  bne cr6, 0x83134028
	if !ctx.cr[6].eq {
	pc = 0x83134028; continue 'dispatch;
	}
	// 83133FAC: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83133FB0: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83133FB4: 7D294050  subf r9, r9, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 83133FB8: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 83133FBC: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83133FC0: 7D0A5BD6  divw r8, r10, r11
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 83133FC4: 7CE95BD6  divw r7, r9, r11
	ctx.r[7].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 83133FC8: 7D0859D6  mullw r8, r8, r11
	ctx.r[8].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83133FCC: 7D6759D6  mullw r11, r7, r11
	ctx.r[11].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83133FD0: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83133FD4: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 83133FD8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83133FDC: 409A001C  bne cr6, 0x83133ff8
	if !ctx.cr[6].eq {
	pc = 0x83133FF8; continue 'dispatch;
	}
	// 83133FE0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83133FE4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83133FE8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133FEC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83133FF0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83133FF4: 48000020  b 0x83134014
	pc = 0x83134014; continue 'dispatch;
	// 83133FF8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83133FFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134000: 41820014  beq 0x83134014
	if ctx.cr[0].eq {
	pc = 0x83134014; continue 'dispatch;
	}
	// 83134004: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83134008: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8313400C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134010: 4E800421  bctrl
	ctx.lr = 0x83134014;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83134014: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83134018: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313401C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83134020: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83134024: 480000B0  b 0x831340d4
	pc = 0x831340D4; continue 'dispatch;
	// 83134028: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8313402C: 409A0080  bne cr6, 0x831340ac
	if !ctx.cr[6].eq {
	pc = 0x831340AC; continue 'dispatch;
	}
	// 83134030: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83134034: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83134038: 7D294050  subf r9, r9, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 8313403C: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 83134040: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83134044: 7D0A5BD6  divw r8, r10, r11
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 83134048: 7CE95BD6  divw r7, r9, r11
	ctx.r[7].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 8313404C: 7D0859D6  mullw r8, r8, r11
	ctx.r[8].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83134050: 7D6759D6  mullw r11, r7, r11
	ctx.r[11].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83134054: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83134058: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 8313405C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83134060: 409A001C  bne cr6, 0x8313407c
	if !ctx.cr[6].eq {
	pc = 0x8313407C; continue 'dispatch;
	}
	// 83134064: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83134068: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8313406C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134070: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83134074: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83134078: 48000020  b 0x83134098
	pc = 0x83134098; continue 'dispatch;
	// 8313407C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83134080: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134084: 41820014  beq 0x83134098
	if ctx.cr[0].eq {
	pc = 0x83134098; continue 'dispatch;
	}
	// 83134088: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8313408C: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83134090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134094: 4E800421  bctrl
	ctx.lr = 0x83134098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83134098: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8313409C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831340A0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831340A4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 831340A8: 4800002C  b 0x831340d4
	pc = 0x831340D4; continue 'dispatch;
	// 831340AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831340B0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831340B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831340B8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 831340BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831340C0: 41820014  beq 0x831340d4
	if ctx.cr[0].eq {
	pc = 0x831340D4; continue 'dispatch;
	}
	// 831340C4: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831340C8: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 831340CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831340D0: 4E800421  bctrl
	ctx.lr = 0x831340D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831340D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831340D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831340DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831340E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831340E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831340E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831340F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831340F0 size=100
    let mut pc: u32 = 0x831340F0;
    'dispatch: loop {
        match pc {
            0x831340F0 => {
    //   block [0x831340F0..0x83134154)
	// 831340F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831340F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831340F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831340FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134104: 480047F5  bl 0x831388f8
	ctx.lr = 0x83134108;
	sub_831388F8(ctx, base);
	// 83134108: 480048E9  bl 0x831389f0
	ctx.lr = 0x8313410C;
	sub_831389F0(ctx, base);
	// 8313410C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83134110: 83FE7E2C  lwz r31, 0x7e2c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32300 as u32) ) } as u64;
	// 83134114: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83134118: 409A0018  bne cr6, 0x83134130
	if !ctx.cr[6].eq {
	pc = 0x83134130; continue 'dispatch;
	}
	// 8313411C: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83134120: 38A06C00  li r5, 0x6c00
	ctx.r[5].s64 = 27648;
	// 83134124: 386B28C0  addi r3, r11, 0x28c0
	ctx.r[3].s64 = ctx.r[11].s64 + 10432;
	// 83134128: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313412C: 480740B5  bl 0x831a81e0
	ctx.lr = 0x83134130;
	sub_831A81E0(ctx, base);
	// 83134130: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 83134134: 917E7E2C  stw r11, 0x7e2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32300 as u32), ctx.r[11].u32 ) };
	// 83134138: 480048F9  bl 0x83138a30
	ctx.lr = 0x8313413C;
	sub_83138A30(ctx, base);
	// 8313413C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83134140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134148: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8313414C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134158 size=80
    let mut pc: u32 = 0x83134158;
    'dispatch: loop {
        match pc {
            0x83134158 => {
    //   block [0x83134158..0x831341A8)
	// 83134158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134164: 4800488D  bl 0x831389f0
	ctx.lr = 0x83134168;
	sub_831389F0(ctx, base);
	// 83134168: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8313416C: 816A7E2C  lwz r11, 0x7e2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32300 as u32) ) } as u64;
	// 83134170: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83134174: 916A7E2C  stw r11, 0x7e2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32300 as u32), ctx.r[11].u32 ) };
	// 83134178: 40820018  bne 0x83134190
	if !ctx.cr[0].eq {
	pc = 0x83134190; continue 'dispatch;
	}
	// 8313417C: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83134180: 38A06C00  li r5, 0x6c00
	ctx.r[5].s64 = 27648;
	// 83134184: 386B28C0  addi r3, r11, 0x28c0
	ctx.r[3].s64 = ctx.r[11].s64 + 10432;
	// 83134188: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313418C: 48074055  bl 0x831a81e0
	ctx.lr = 0x83134190;
	sub_831A81E0(ctx, base);
	// 83134190: 480048A1  bl 0x83138a30
	ctx.lr = 0x83134194;
	sub_83138A30(ctx, base);
	// 83134194: 480047E5  bl 0x83138978
	ctx.lr = 0x83134198;
	sub_83138978(ctx, base);
	// 83134198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8313419C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831341A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831341A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831341A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831341A8 size=136
    let mut pc: u32 = 0x831341A8;
    'dispatch: loop {
        match pc {
            0x831341A8 => {
    //   block [0x831341A8..0x83134230)
	// 831341A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831341AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831341B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831341B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831341B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831341BC: 48004835  bl 0x831389f0
	ctx.lr = 0x831341C0;
	sub_831389F0(ctx, base);
	// 831341C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831341C4: 409A001C  bne cr6, 0x831341e0
	if !ctx.cr[6].eq {
	pc = 0x831341E0; continue 'dispatch;
	}
	// 831341C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831341CC: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831341D0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831341D4: 386B1838  addi r3, r11, 0x1838
	ctx.r[3].s64 = ctx.r[11].s64 + 6200;
	// 831341D8: 4BFFF0C9  bl 0x831332a0
	ctx.lr = 0x831341DC;
	sub_831332A0(ctx, base);
	// 831341DC: 4800003C  b 0x83134218
	pc = 0x83134218; continue 'dispatch;
	// 831341E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831341E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831341E8: 409A0018  bne cr6, 0x83134200
	if !ctx.cr[6].eq {
	pc = 0x83134200; continue 'dispatch;
	}
	// 831341EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831341F0: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 831341F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831341F8: 386B182C  addi r3, r11, 0x182c
	ctx.r[3].s64 = ctx.r[11].s64 + 6188;
	// 831341FC: 4BFFFFDC  b 0x831341d8
	pc = 0x831341D8; continue 'dispatch;
	// 83134200: 38A00048  li r5, 0x48
	ctx.r[5].s64 = 72;
	// 83134204: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83134208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313420C: 48073FD5  bl 0x831a81e0
	ctx.lr = 0x83134210;
	sub_831A81E0(ctx, base);
	// 83134210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83134214: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83134218: 48004819  bl 0x83138a30
	ctx.lr = 0x8313421C;
	sub_83138A30(ctx, base);
	// 8313421C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313422C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134230 size=124
    let mut pc: u32 = 0x83134230;
    'dispatch: loop {
        match pc {
            0x83134230 => {
    //   block [0x83134230..0x831342AC)
	// 83134230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313423C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134244: 480047AD  bl 0x831389f0
	ctx.lr = 0x83134248;
	sub_831389F0(ctx, base);
	// 83134248: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313424C: 409A0020  bne cr6, 0x8313426c
	if !ctx.cr[6].eq {
	pc = 0x8313426C; continue 'dispatch;
	}
	// 83134250: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134254: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134258: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313425C: 386B1850  addi r3, r11, 0x1850
	ctx.r[3].s64 = ctx.r[11].s64 + 6224;
	// 83134260: 4BFFF041  bl 0x831332a0
	ctx.lr = 0x83134264;
	sub_831332A0(ctx, base);
	// 83134264: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134268: 48000028  b 0x83134290
	pc = 0x83134290; continue 'dispatch;
	// 8313426C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134270: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83134274: 409A0018  bne cr6, 0x8313428c
	if !ctx.cr[6].eq {
	pc = 0x8313428C; continue 'dispatch;
	}
	// 83134278: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313427C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134280: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134284: 386B1844  addi r3, r11, 0x1844
	ctx.r[3].s64 = ctx.r[11].s64 + 6212;
	// 83134288: 4BFFFFD8  b 0x83134260
	pc = 0x83134260; continue 'dispatch;
	// 8313428C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83134290: 480047A1  bl 0x83138a30
	ctx.lr = 0x83134294;
	sub_83138A30(ctx, base);
	// 83134294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8313429C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831342A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831342A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831342A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831342B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831342B0 size=112
    let mut pc: u32 = 0x831342B0;
    'dispatch: loop {
        match pc {
            0x831342B0 => {
    //   block [0x831342B0..0x83134320)
	// 831342B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831342B4: 48073EB9  bl 0x831a816c
	ctx.lr = 0x831342B8;
	sub_831A8130(ctx, base);
	// 831342B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831342BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831342C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831342C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831342C8: 48004729  bl 0x831389f0
	ctx.lr = 0x831342CC;
	sub_831389F0(ctx, base);
	// 831342CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831342D0: 409A001C  bne cr6, 0x831342ec
	if !ctx.cr[6].eq {
	pc = 0x831342EC; continue 'dispatch;
	}
	// 831342D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831342D8: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831342DC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831342E0: 386B1868  addi r3, r11, 0x1868
	ctx.r[3].s64 = ctx.r[11].s64 + 6248;
	// 831342E4: 4BFFEFBD  bl 0x831332a0
	ctx.lr = 0x831342E8;
	sub_831332A0(ctx, base);
	// 831342E8: 4800002C  b 0x83134314
	pc = 0x83134314; continue 'dispatch;
	// 831342EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831342F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831342F4: 409A0018  bne cr6, 0x8313430c
	if !ctx.cr[6].eq {
	pc = 0x8313430C; continue 'dispatch;
	}
	// 831342F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831342FC: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134300: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134304: 386B185C  addi r3, r11, 0x185c
	ctx.r[3].s64 = ctx.r[11].s64 + 6236;
	// 83134308: 4BFFFFDC  b 0x831342e4
	pc = 0x831342E4; continue 'dispatch;
	// 8313430C: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 83134310: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 83134314: 4800471D  bl 0x83138a30
	ctx.lr = 0x83134318;
	sub_83138A30(ctx, base);
	// 83134318: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313431C: 48073EA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134320 size=56
    let mut pc: u32 = 0x83134320;
    'dispatch: loop {
        match pc {
            0x83134320 => {
    //   block [0x83134320..0x83134358)
	// 83134320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313432C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134334: 480046BD  bl 0x831389f0
	ctx.lr = 0x83134338;
	sub_831389F0(ctx, base);
	// 83134338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313433C: 4BFFF825  bl 0x83133b60
	ctx.lr = 0x83134340;
	sub_83133B60(ctx, base);
	// 83134340: 480046F1  bl 0x83138a30
	ctx.lr = 0x83134344;
	sub_83138A30(ctx, base);
	// 83134344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313434C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134358 size=192
    let mut pc: u32 = 0x83134358;
    'dispatch: loop {
        match pc {
            0x83134358 => {
    //   block [0x83134358..0x83134418)
	// 83134358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313435C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134360: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83134364: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313436C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134370: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83134374: 4800467D  bl 0x831389f0
	ctx.lr = 0x83134378;
	sub_831389F0(ctx, base);
	// 83134378: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313437C: 409A001C  bne cr6, 0x83134398
	if !ctx.cr[6].eq {
	pc = 0x83134398; continue 'dispatch;
	}
	// 83134380: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134384: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134388: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313438C: 386B1898  addi r3, r11, 0x1898
	ctx.r[3].s64 = ctx.r[11].s64 + 6296;
	// 83134390: 4BFFEF11  bl 0x831332a0
	ctx.lr = 0x83134394;
	sub_831332A0(ctx, base);
	// 83134394: 48000060  b 0x831343f4
	pc = 0x831343F4; continue 'dispatch;
	// 83134398: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313439C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831343A0: 409A0018  bne cr6, 0x831343b8
	if !ctx.cr[6].eq {
	pc = 0x831343B8; continue 'dispatch;
	}
	// 831343A4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831343A8: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 831343AC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831343B0: 386B188C  addi r3, r11, 0x188c
	ctx.r[3].s64 = ctx.r[11].s64 + 6284;
	// 831343B4: 4BFFFFDC  b 0x83134390
	pc = 0x83134390; continue 'dispatch;
	// 831343B8: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 831343BC: 409A000C  bne cr6, 0x831343c8
	if !ctx.cr[6].eq {
	pc = 0x831343C8; continue 'dispatch;
	}
	// 831343C0: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831343C4: 48000034  b 0x831343f8
	pc = 0x831343F8; continue 'dispatch;
	// 831343C8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831343CC: 409A000C  bne cr6, 0x831343d8
	if !ctx.cr[6].eq {
	pc = 0x831343D8; continue 'dispatch;
	}
	// 831343D0: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831343D4: 48000024  b 0x831343f8
	pc = 0x831343F8; continue 'dispatch;
	// 831343D8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 831343DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831343E0: 41820014  beq 0x831343f4
	if ctx.cr[0].eq {
	pc = 0x831343F4; continue 'dispatch;
	}
	// 831343E4: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831343E8: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 831343EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831343F0: 4E800421  bctrl
	ctx.lr = 0x831343F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831343F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831343F8: 48004639  bl 0x83138a30
	ctx.lr = 0x831343FC;
	sub_83138A30(ctx, base);
	// 831343FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134400: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83134404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313440C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83134410: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134418 size=64
    let mut pc: u32 = 0x83134418;
    'dispatch: loop {
        match pc {
            0x83134418 => {
    //   block [0x83134418..0x83134458)
	// 83134418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313441C: 48073D4D  bl 0x831a8168
	ctx.lr = 0x83134420;
	sub_831A8130(ctx, base);
	// 83134420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134428: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8313442C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83134430: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83134434: 480045BD  bl 0x831389f0
	ctx.lr = 0x83134438;
	sub_831389F0(ctx, base);
	// 83134438: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8313443C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83134440: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83134444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134448: 4BFFF781  bl 0x83133bc8
	ctx.lr = 0x8313444C;
	sub_83133BC8(ctx, base);
	// 8313444C: 480045E5  bl 0x83138a30
	ctx.lr = 0x83134450;
	sub_83138A30(ctx, base);
	// 83134450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83134454: 48073D64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134458 size=56
    let mut pc: u32 = 0x83134458;
    'dispatch: loop {
        match pc {
            0x83134458 => {
    //   block [0x83134458..0x83134490)
	// 83134458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313445C: 48073D11  bl 0x831a816c
	ctx.lr = 0x83134460;
	sub_831A8130(ctx, base);
	// 83134460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134468: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8313446C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83134470: 48004581  bl 0x831389f0
	ctx.lr = 0x83134474;
	sub_831389F0(ctx, base);
	// 83134474: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83134478: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8313447C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134480: 4BFFF8E9  bl 0x83133d68
	ctx.lr = 0x83134484;
	sub_83133D68(ctx, base);
	// 83134484: 480045AD  bl 0x83138a30
	ctx.lr = 0x83134488;
	sub_83138A30(ctx, base);
	// 83134488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313448C: 48073D30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134490 size=56
    let mut pc: u32 = 0x83134490;
    'dispatch: loop {
        match pc {
            0x83134490 => {
    //   block [0x83134490..0x831344C8)
	// 83134490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134494: 48073CD9  bl 0x831a816c
	ctx.lr = 0x83134498;
	sub_831A8130(ctx, base);
	// 83134498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313449C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831344A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831344A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831344A8: 48004549  bl 0x831389f0
	ctx.lr = 0x831344AC;
	sub_831389F0(ctx, base);
	// 831344AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831344B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831344B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831344B8: 4BFFFA59  bl 0x83133f10
	ctx.lr = 0x831344BC;
	sub_83133F10(ctx, base);
	// 831344BC: 48004575  bl 0x83138a30
	ctx.lr = 0x831344C0;
	sub_83138A30(ctx, base);
	// 831344C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831344C4: 48073CF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831344C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831344C8 size=252
    let mut pc: u32 = 0x831344C8;
    'dispatch: loop {
        match pc {
            0x831344C8 => {
    //   block [0x831344C8..0x831345C4)
	// 831344C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831344CC: 48073C9D  bl 0x831a8168
	ctx.lr = 0x831344D0;
	sub_831A8130(ctx, base);
	// 831344D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831344D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831344D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831344DC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831344E0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831344E4: 4800450D  bl 0x831389f0
	ctx.lr = 0x831344E8;
	sub_831389F0(ctx, base);
	// 831344E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831344EC: 409A0020  bne cr6, 0x8313450c
	if !ctx.cr[6].eq {
	pc = 0x8313450C; continue 'dispatch;
	}
	// 831344F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831344F4: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831344F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831344FC: 386B1928  addi r3, r11, 0x1928
	ctx.r[3].s64 = ctx.r[11].s64 + 6440;
	// 83134500: 4BFFEDA1  bl 0x831332a0
	ctx.lr = 0x83134504;
	sub_831332A0(ctx, base);
	// 83134504: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134508: 480000AC  b 0x831345b4
	pc = 0x831345B4; continue 'dispatch;
	// 8313450C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134510: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83134514: 409A0018  bne cr6, 0x8313452c
	if !ctx.cr[6].eq {
	pc = 0x8313452C; continue 'dispatch;
	}
	// 83134518: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313451C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134520: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134524: 386B191C  addi r3, r11, 0x191c
	ctx.r[3].s64 = ctx.r[11].s64 + 6428;
	// 83134528: 4BFFFFD8  b 0x83134500
	pc = 0x83134500; continue 'dispatch;
	// 8313452C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83134530: 409A0018  bne cr6, 0x83134548
	if !ctx.cr[6].eq {
	pc = 0x83134548; continue 'dispatch;
	}
	// 83134534: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83134538: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8313453C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83134540: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83134544: 4800001C  b 0x83134560
	pc = 0x83134560; continue 'dispatch;
	// 83134548: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8313454C: 409A0038  bne cr6, 0x83134584
	if !ctx.cr[6].eq {
	pc = 0x83134584; continue 'dispatch;
	}
	// 83134550: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83134554: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83134558: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8313455C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83134560: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83134564: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83134568: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8313456C: 40980008  bge cr6, 0x83134574
	if !ctx.cr[6].lt {
	pc = 0x83134574; continue 'dispatch;
	}
	// 83134570: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83134574: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83134578: 4198002C  blt cr6, 0x831345a4
	if ctx.cr[6].lt {
	pc = 0x831345A4; continue 'dispatch;
	}
	// 8313457C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 83134580: 48000024  b 0x831345a4
	pc = 0x831345A4; continue 'dispatch;
	// 83134584: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83134588: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8313458C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134590: 41820014  beq 0x831345a4
	if ctx.cr[0].eq {
	pc = 0x831345A4; continue 'dispatch;
	}
	// 83134594: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83134598: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8313459C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831345A0: 4E800421  bctrl
	ctx.lr = 0x831345A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831345A4: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 831345A8: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 831345AC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831345B0: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831345B4: 4800447D  bl 0x83138a30
	ctx.lr = 0x831345B8;
	sub_83138A30(ctx, base);
	// 831345B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831345BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831345C0: 48073BF8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831345C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831345C8 size=124
    let mut pc: u32 = 0x831345C8;
    'dispatch: loop {
        match pc {
            0x831345C8 => {
    //   block [0x831345C8..0x83134644)
	// 831345C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831345CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831345D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831345D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831345D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831345DC: 48004415  bl 0x831389f0
	ctx.lr = 0x831345E0;
	sub_831389F0(ctx, base);
	// 831345E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831345E4: 409A0020  bne cr6, 0x83134604
	if !ctx.cr[6].eq {
	pc = 0x83134604; continue 'dispatch;
	}
	// 831345E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831345EC: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831345F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831345F4: 386B1940  addi r3, r11, 0x1940
	ctx.r[3].s64 = ctx.r[11].s64 + 6464;
	// 831345F8: 4BFFECA9  bl 0x831332a0
	ctx.lr = 0x831345FC;
	sub_831332A0(ctx, base);
	// 831345FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134600: 48000028  b 0x83134628
	pc = 0x83134628; continue 'dispatch;
	// 83134604: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134608: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313460C: 409A0018  bne cr6, 0x83134624
	if !ctx.cr[6].eq {
	pc = 0x83134624; continue 'dispatch;
	}
	// 83134610: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134614: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134618: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313461C: 386B1934  addi r3, r11, 0x1934
	ctx.r[3].s64 = ctx.r[11].s64 + 6452;
	// 83134620: 4BFFFFD8  b 0x831345f8
	pc = 0x831345F8; continue 'dispatch;
	// 83134624: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83134628: 48004409  bl 0x83138a30
	ctx.lr = 0x8313462C;
	sub_83138A30(ctx, base);
	// 8313462C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313463C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134648 size=124
    let mut pc: u32 = 0x83134648;
    'dispatch: loop {
        match pc {
            0x83134648 => {
    //   block [0x83134648..0x831346C4)
	// 83134648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134658: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313465C: 48004395  bl 0x831389f0
	ctx.lr = 0x83134660;
	sub_831389F0(ctx, base);
	// 83134660: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83134664: 409A0020  bne cr6, 0x83134684
	if !ctx.cr[6].eq {
	pc = 0x83134684; continue 'dispatch;
	}
	// 83134668: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313466C: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134670: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134674: 386B1958  addi r3, r11, 0x1958
	ctx.r[3].s64 = ctx.r[11].s64 + 6488;
	// 83134678: 4BFFEC29  bl 0x831332a0
	ctx.lr = 0x8313467C;
	sub_831332A0(ctx, base);
	// 8313467C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134680: 48000028  b 0x831346a8
	pc = 0x831346A8; continue 'dispatch;
	// 83134684: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134688: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313468C: 409A0018  bne cr6, 0x831346a4
	if !ctx.cr[6].eq {
	pc = 0x831346A4; continue 'dispatch;
	}
	// 83134690: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134694: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134698: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313469C: 386B194C  addi r3, r11, 0x194c
	ctx.r[3].s64 = ctx.r[11].s64 + 6476;
	// 831346A0: 4BFFFFD8  b 0x83134678
	pc = 0x83134678; continue 'dispatch;
	// 831346A4: 83FF0020  lwz r31, 0x20(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831346A8: 48004389  bl 0x83138a30
	ctx.lr = 0x831346AC;
	sub_83138A30(ctx, base);
	// 831346AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831346B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831346B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831346B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831346BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831346C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831346C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831346C8 size=124
    let mut pc: u32 = 0x831346C8;
    'dispatch: loop {
        match pc {
            0x831346C8 => {
    //   block [0x831346C8..0x83134744)
	// 831346C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831346CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831346D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831346D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831346D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831346DC: 48004315  bl 0x831389f0
	ctx.lr = 0x831346E0;
	sub_831389F0(ctx, base);
	// 831346E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831346E4: 409A0020  bne cr6, 0x83134704
	if !ctx.cr[6].eq {
	pc = 0x83134704; continue 'dispatch;
	}
	// 831346E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831346EC: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831346F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831346F4: 386B1970  addi r3, r11, 0x1970
	ctx.r[3].s64 = ctx.r[11].s64 + 6512;
	// 831346F8: 4BFFEBA9  bl 0x831332a0
	ctx.lr = 0x831346FC;
	sub_831332A0(ctx, base);
	// 831346FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134700: 48000028  b 0x83134728
	pc = 0x83134728; continue 'dispatch;
	// 83134704: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134708: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313470C: 409A0018  bne cr6, 0x83134724
	if !ctx.cr[6].eq {
	pc = 0x83134724; continue 'dispatch;
	}
	// 83134710: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134714: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134718: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313471C: 386B1964  addi r3, r11, 0x1964
	ctx.r[3].s64 = ctx.r[11].s64 + 6500;
	// 83134720: 4BFFFFD8  b 0x831346f8
	pc = 0x831346F8; continue 'dispatch;
	// 83134724: 83FF0024  lwz r31, 0x24(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83134728: 48004309  bl 0x83138a30
	ctx.lr = 0x8313472C;
	sub_83138A30(ctx, base);
	// 8313472C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313473C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134748 size=132
    let mut pc: u32 = 0x83134748;
    'dispatch: loop {
        match pc {
            0x83134748 => {
    //   block [0x83134748..0x831347CC)
	// 83134748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313474C: 48073A21  bl 0x831a816c
	ctx.lr = 0x83134750;
	sub_831A8130(ctx, base);
	// 83134750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134758: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8313475C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83134760: 48004291  bl 0x831389f0
	ctx.lr = 0x83134764;
	sub_831389F0(ctx, base);
	// 83134764: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83134768: 409A0020  bne cr6, 0x83134788
	if !ctx.cr[6].eq {
	pc = 0x83134788; continue 'dispatch;
	}
	// 8313476C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134770: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134774: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134778: 386B1988  addi r3, r11, 0x1988
	ctx.r[3].s64 = ctx.r[11].s64 + 6536;
	// 8313477C: 4BFFEB25  bl 0x831332a0
	ctx.lr = 0x83134780;
	sub_831332A0(ctx, base);
	// 83134780: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134784: 48000038  b 0x831347bc
	pc = 0x831347BC; continue 'dispatch;
	// 83134788: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313478C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83134790: 409A0018  bne cr6, 0x831347a8
	if !ctx.cr[6].eq {
	pc = 0x831347A8; continue 'dispatch;
	}
	// 83134794: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134798: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 8313479C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831347A0: 386B197C  addi r3, r11, 0x197c
	ctx.r[3].s64 = ctx.r[11].s64 + 6524;
	// 831347A4: 4BFFFFD8  b 0x8313477c
	pc = 0x8313477C; continue 'dispatch;
	// 831347A8: 397E0005  addi r11, r30, 5
	ctx.r[11].s64 = ctx.r[30].s64 + 5;
	// 831347AC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831347B0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 831347B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831347B8: 7FEBF82E  lwzx r31, r11, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831347BC: 48004275  bl 0x83138a30
	ctx.lr = 0x831347C0;
	sub_83138A30(ctx, base);
	// 831347C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831347C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831347C8: 480739F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831347D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831347D0 size=196
    let mut pc: u32 = 0x831347D0;
    'dispatch: loop {
        match pc {
            0x831347D0 => {
    //   block [0x831347D0..0x83134894)
	// 831347D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831347D4: 48073995  bl 0x831a8168
	ctx.lr = 0x831347D8;
	sub_831A8130(ctx, base);
	// 831347D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831347DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831347E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831347E4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831347E8: 48004209  bl 0x831389f0
	ctx.lr = 0x831347EC;
	sub_831389F0(ctx, base);
	// 831347EC: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831347F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831347F4: 396B28C0  addi r11, r11, 0x28c0
	ctx.r[11].s64 = ctx.r[11].s64 + 10432;
	// 831347F8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 831347FC: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 83134800: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134804: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83134808: 419A0018  beq cr6, 0x83134820
	if ctx.cr[6].eq {
	pc = 0x83134820; continue 'dispatch;
	}
	// 8313480C: 39290048  addi r9, r9, 0x48
	ctx.r[9].s64 = ctx.r[9].s64 + 72;
	// 83134810: 390B6C04  addi r8, r11, 0x6c04
	ctx.r[8].s64 = ctx.r[11].s64 + 27652;
	// 83134814: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83134818: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8313481C: 4198FFE4  blt cr6, 0x83134800
	if ctx.cr[6].lt {
	pc = 0x83134800; continue 'dispatch;
	}
	// 83134820: 2F0A0180  cmpwi cr6, r10, 0x180
	ctx.cr[6].compare_i32(ctx.r[10].s32, 384, &mut ctx.xer);
	// 83134824: 409A000C  bne cr6, 0x83134830
	if !ctx.cr[6].eq {
	pc = 0x83134830; continue 'dispatch;
	}
	// 83134828: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8313482C: 48000058  b 0x83134884
	pc = 0x83134884; continue 'dispatch;
	// 83134830: 1D0A0048  mulli r8, r10, 0x48
	ctx.r[8].s64 = ctx.r[10].s64 * 72;
	// 83134834: 7FE85A14  add r31, r8, r11
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 83134838: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8313483C: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83134840: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 83134844: 394A5A9C  addi r10, r10, 0x5a9c
	ctx.r[10].s64 = ctx.r[10].s64 + 23196;
	// 83134848: 39291810  addi r9, r9, 0x1810
	ctx.r[9].s64 = ctx.r[9].s64 + 6160;
	// 8313484C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83134850: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83134854: 396B3B50  addi r11, r11, 0x3b50
	ctx.r[11].s64 = ctx.r[11].s64 + 15184;
	// 83134858: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 8313485C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134860: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 83134864: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83134868: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8313486C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 83134870: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83134874: 93FF003C  stw r31, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 83134878: 90FF0040  stw r7, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[7].u32 ) };
	// 8313487C: 90FF0044  stw r7, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 83134880: 4BFFF2E1  bl 0x83133b60
	ctx.lr = 0x83134884;
	sub_83133B60(ctx, base);
	// 83134884: 480041AD  bl 0x83138a30
	ctx.lr = 0x83134888;
	sub_83138A30(ctx, base);
	// 83134888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313488C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83134890: 48073928  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134898 size=28
    let mut pc: u32 = 0x83134898;
    'dispatch: loop {
        match pc {
            0x83134898 => {
    //   block [0x83134898..0x831348B4)
	// 83134898: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8313489C: 409A0018  bne cr6, 0x831348b4
	if !ctx.cr[6].eq {
		sub_831348B4(ctx, base);
		return;
	}
	// 831348A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831348A4: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831348A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831348AC: 386B19F8  addi r3, r11, 0x19f8
	ctx.r[3].s64 = ctx.r[11].s64 + 6648;
	// 831348B0: 4BFFE9F0  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831348B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831348B4 size=32
    let mut pc: u32 = 0x831348B4;
    'dispatch: loop {
        match pc {
            0x831348B4 => {
    //   block [0x831348B4..0x831348D4)
	// 831348B4: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831348B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831348BC: 409A0018  bne cr6, 0x831348d4
	if !ctx.cr[6].eq {
		sub_831348D4(ctx, base);
		return;
	}
	// 831348C0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831348C4: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 831348C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831348CC: 386B19EC  addi r3, r11, 0x19ec
	ctx.r[3].s64 = ctx.r[11].s64 + 6636;
	// 831348D0: 4BFFE9D0  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831348D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831348D4 size=112
    let mut pc: u32 = 0x831348D4;
    'dispatch: loop {
        match pc {
            0x831348D4 => {
    //   block [0x831348D4..0x83134944)
	// 831348D4: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831348D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831348DC: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831348E0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831348E4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831348E8: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 831348EC: 40810030  ble 0x8313491c
	if !ctx.cr[0].gt {
	pc = 0x8313491C; continue 'dispatch;
	}
	// 831348F0: 39480008  addi r10, r8, 8
	ctx.r[10].s64 = ctx.r[8].s64 + 8;
	// 831348F4: 38EA0008  addi r7, r10, 8
	ctx.r[7].s64 = ctx.r[10].s64 + 8;
	// 831348F8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831348FC: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83134900: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83134904: 90EAFFF8  stw r7, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 83134908: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8313490C: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83134910: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 83134914: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83134918: 4198FFDC  blt cr6, 0x831348f4
	if ctx.cr[6].lt {
	pc = 0x831348F4; continue 'dispatch;
	}
	// 8313491C: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83134920: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 83134924: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83134928: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8313492C: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83134930: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83134934: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83134938: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8313493C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83134940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134948 size=284
    let mut pc: u32 = 0x83134948;
    'dispatch: loop {
        match pc {
            0x83134948 => {
    //   block [0x83134948..0x83134A64)
	// 83134948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313494C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83134954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313495C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83134960: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83134964: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83134968: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8313496C: 409A001C  bne cr6, 0x83134988
	if !ctx.cr[6].eq {
	pc = 0x83134988; continue 'dispatch;
	}
	// 83134970: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134974: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134978: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313497C: 386B1A28  addi r3, r11, 0x1a28
	ctx.r[3].s64 = ctx.r[11].s64 + 6696;
	// 83134980: 4BFFE921  bl 0x831332a0
	ctx.lr = 0x83134984;
	sub_831332A0(ctx, base);
	// 83134984: 480000C8  b 0x83134a4c
	pc = 0x83134A4C; continue 'dispatch;
	// 83134988: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313498C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83134990: 409A0018  bne cr6, 0x831349a8
	if !ctx.cr[6].eq {
	pc = 0x831349A8; continue 'dispatch;
	}
	// 83134994: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134998: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 8313499C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831349A0: 386B1A1C  addi r3, r11, 0x1a1c
	ctx.r[3].s64 = ctx.r[11].s64 + 6684;
	// 831349A4: 4BFFFFDC  b 0x83134980
	pc = 0x83134980; continue 'dispatch;
	// 831349A8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 831349AC: 41990078  bgt cr6, 0x83134a24
	if ctx.cr[6].gt {
	pc = 0x83134A24; continue 'dispatch;
	}
	// 831349B0: 396B0006  addi r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 + 6;
	// 831349B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831349B8: 7FEB182E  lwzx r31, r11, r3
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 831349BC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831349C0: 41820080  beq 0x83134a40
	if ctx.cr[0].eq {
	pc = 0x83134A40; continue 'dispatch;
	}
	// 831349C4: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 831349C8: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 831349CC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831349D0: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831349D4: 41990020  bgt cr6, 0x831349f4
	if ctx.cr[6].gt {
	pc = 0x831349F4; continue 'dispatch;
	}
	// 831349D8: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 831349DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831349E0: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 831349E4: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831349E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831349EC: 93E30014  stw r31, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 831349F0: 4800005C  b 0x83134a4c
	pc = 0x83134A4C; continue 'dispatch;
	// 831349F4: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 831349F8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 831349FC: 409A0044  bne cr6, 0x83134a40
	if !ctx.cr[6].eq {
	pc = 0x83134A40; continue 'dispatch;
	}
	// 83134A00: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83134A04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83134A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83134A0C: 48000785  bl 0x83135190
	ctx.lr = 0x83134A10;
	sub_83135190(ctx, base);
	// 83134A10: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83134A14: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83134A18: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83134A1C: F95F0008  std r10, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83134A20: 4800002C  b 0x83134a4c
	pc = 0x83134A4C; continue 'dispatch;
	// 83134A24: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83134A28: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134A2C: 41820014  beq 0x83134a40
	if ctx.cr[0].eq {
	pc = 0x83134A40; continue 'dispatch;
	}
	// 83134A30: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83134A34: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83134A38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134A3C: 4E800421  bctrl
	ctx.lr = 0x83134A40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83134A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83134A44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83134A48: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83134A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83134A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134A58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83134A5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134A68 size=28
    let mut pc: u32 = 0x83134A68;
    'dispatch: loop {
        match pc {
            0x83134A68 => {
    //   block [0x83134A68..0x83134A84)
	// 83134A68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83134A6C: 409A0018  bne cr6, 0x83134a84
	if !ctx.cr[6].eq {
		sub_83134A84(ctx, base);
		return;
	}
	// 83134A70: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134A74: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134A78: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134A7C: 386B1A40  addi r3, r11, 0x1a40
	ctx.r[3].s64 = ctx.r[11].s64 + 6720;
	// 83134A80: 4BFFE820  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134A84 size=32
    let mut pc: u32 = 0x83134A84;
    'dispatch: loop {
        match pc {
            0x83134A84 => {
    //   block [0x83134A84..0x83134AA4)
	// 83134A84: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134A88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83134A8C: 409A0018  bne cr6, 0x83134aa4
	if !ctx.cr[6].eq {
		sub_83134AA4(ctx, base);
		return;
	}
	// 83134A90: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134A94: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134A98: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134A9C: 386B1A34  addi r3, r11, 0x1a34
	ctx.r[3].s64 = ctx.r[11].s64 + 6708;
	// 83134AA0: 4BFFE800  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134AA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134AA4 size=20
    let mut pc: u32 = 0x83134AA4;
    'dispatch: loop {
        match pc {
            0x83134AA4 => {
    //   block [0x83134AA4..0x83134AB8)
	// 83134AA4: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 83134AA8: 419900CC  bgt cr6, 0x83134b74
	if ctx.cr[6].gt {
		sub_83134B74(ctx, base);
		return;
	}
	// 83134AAC: 80C50004  lwz r6, 4(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134AB0: 2C060000  cmpwi r6, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83134AB4: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134AB8 size=12
    let mut pc: u32 = 0x83134AB8;
    'dispatch: loop {
        match pc {
            0x83134AB8 => {
    //   block [0x83134AB8..0x83134AC4)
	// 83134AB8: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134ABC: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134AC0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134AC4 size=20
    let mut pc: u32 = 0x83134AC4;
    'dispatch: loop {
        match pc {
            0x83134AC4 => {
    //   block [0x83134AC4..0x83134AD8)
	// 83134AC4: 39640006  addi r11, r4, 6
	ctx.r[11].s64 = ctx.r[4].s64 + 6;
	// 83134AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83134ACC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83134AD0: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83134AD4: 4800000C  b 0x83134ae0
	sub_83134AD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134AD8 size=72
    let mut pc: u32 = 0x83134AD8;
    'dispatch: loop {
        match pc {
            0x83134AD8 => {
    //   block [0x83134AD8..0x83134B20)
	// 83134AD8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83134ADC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83134AE0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134AE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134AE8: 4082FFF0  bne 0x83134ad8
	if !ctx.cr[0].eq {
	pc = 0x83134AD8; continue 'dispatch;
	}
	// 83134AEC: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 83134AF0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83134AF4: 409A002C  bne cr6, 0x83134b20
	if !ctx.cr[6].eq {
		sub_83134B20(ctx, base);
		return;
	}
	// 83134AF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83134AFC: 419A0024  beq cr6, 0x83134b20
	if ctx.cr[6].eq {
		sub_83134B20(ctx, base);
		return;
	}
	// 83134B00: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83134B04: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83134B08: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 83134B0C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83134B10: 409A0010  bne cr6, 0x83134b20
	if !ctx.cr[6].eq {
		sub_83134B20(ctx, base);
		return;
	}
	// 83134B14: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 83134B18: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83134B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134B20 size=24
    let mut pc: u32 = 0x83134B20;
    'dispatch: loop {
        match pc {
            0x83134B20 => {
    //   block [0x83134B20..0x83134B38)
	// 83134B20: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83134B24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134B28: 40820020  bne 0x83134b48
	if !ctx.cr[0].eq {
		sub_83134B48(ctx, base);
		return;
	}
	// 83134B2C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83134B30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134B34: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134B38 size=16
    let mut pc: u32 = 0x83134B38;
    'dispatch: loop {
        match pc {
            0x83134B38 => {
    //   block [0x83134B38..0x83134B48)
	// 83134B38: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83134B3C: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83134B40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134B44: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134B48 size=44
    let mut pc: u32 = 0x83134B48;
    'dispatch: loop {
        match pc {
            0x83134B48 => {
    //   block [0x83134B48..0x83134B74)
	// 83134B48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134B4C: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 83134B50: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83134B54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83134B58: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83134B5C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134B60: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83134B64: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134B68: 91480004  stw r10, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83134B6C: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83134B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134B74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134B74 size=12
    let mut pc: u32 = 0x83134B74;
    'dispatch: loop {
        match pc {
            0x83134B74 => {
    //   block [0x83134B74..0x83134B80)
	// 83134B74: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83134B78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134B7C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134B80 size=16
    let mut pc: u32 = 0x83134B80;
    'dispatch: loop {
        match pc {
            0x83134B80 => {
    //   block [0x83134B80..0x83134B90)
	// 83134B80: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83134B84: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83134B88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134B8C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134B90 size=28
    let mut pc: u32 = 0x83134B90;
    'dispatch: loop {
        match pc {
            0x83134B90 => {
    //   block [0x83134B90..0x83134BAC)
	// 83134B90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83134B94: 409A0018  bne cr6, 0x83134bac
	if !ctx.cr[6].eq {
		sub_83134BAC(ctx, base);
		return;
	}
	// 83134B98: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134B9C: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134BA0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134BA4: 386B1A58  addi r3, r11, 0x1a58
	ctx.r[3].s64 = ctx.r[11].s64 + 6744;
	// 83134BA8: 4BFFE6F8  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134BAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134BAC size=32
    let mut pc: u32 = 0x83134BAC;
    'dispatch: loop {
        match pc {
            0x83134BAC => {
    //   block [0x83134BAC..0x83134BCC)
	// 83134BAC: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83134BB4: 409A0018  bne cr6, 0x83134bcc
	if !ctx.cr[6].eq {
		sub_83134BCC(ctx, base);
		return;
	}
	// 83134BB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134BBC: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134BC0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134BC4: 386B1A4C  addi r3, r11, 0x1a4c
	ctx.r[3].s64 = ctx.r[11].s64 + 6732;
	// 83134BC8: 4BFFE6D8  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134BCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134BCC size=20
    let mut pc: u32 = 0x83134BCC;
    'dispatch: loop {
        match pc {
            0x83134BCC => {
    //   block [0x83134BCC..0x83134BE0)
	// 83134BCC: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 83134BD0: 419900C0  bgt cr6, 0x83134c90
	if ctx.cr[6].gt {
		sub_83134C90(ctx, base);
		return;
	}
	// 83134BD4: 81050004  lwz r8, 4(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134BD8: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83134BDC: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134BE0 size=12
    let mut pc: u32 = 0x83134BE0;
    'dispatch: loop {
        match pc {
            0x83134BE0 => {
    //   block [0x83134BE0..0x83134BEC)
	// 83134BE0: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134BE4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134BE8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134BEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134BEC size=72
    let mut pc: u32 = 0x83134BEC;
    'dispatch: loop {
        match pc {
            0x83134BEC => {
    //   block [0x83134BEC..0x83134C34)
	// 83134BEC: 39640006  addi r11, r4, 6
	ctx.r[11].s64 = ctx.r[4].s64 + 6;
	// 83134BF0: 88E30005  lbz r7, 5(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 83134BF4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83134BF8: 2B070001  cmplwi cr6, r7, 1
	ctx.cr[6].compare_u32(ctx.r[7].u32, 1 as u32, &mut ctx.xer);
	// 83134BFC: 7D6A182E  lwzx r11, r10, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83134C00: 409A0034  bne cr6, 0x83134c34
	if !ctx.cr[6].eq {
		sub_83134C34(ctx, base);
		return;
	}
	// 83134C04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83134C08: 419A002C  beq cr6, 0x83134c34
	if ctx.cr[6].eq {
		sub_83134C34(ctx, base);
		return;
	}
	// 83134C0C: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83134C10: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 83134C14: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83134C18: 409A001C  bne cr6, 0x83134c34
	if !ctx.cr[6].eq {
		sub_83134C34(ctx, base);
		return;
	}
	// 83134C1C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83134C20: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83134C24: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134C28: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83134C2C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83134C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134C34 size=24
    let mut pc: u32 = 0x83134C34;
    'dispatch: loop {
        match pc {
            0x83134C34 => {
    //   block [0x83134C34..0x83134C4C)
	// 83134C34: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83134C38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134C3C: 40820020  bne 0x83134c5c
	if !ctx.cr[0].eq {
		sub_83134C5C(ctx, base);
		return;
	}
	// 83134C40: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83134C44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134C48: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134C4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134C4C size=16
    let mut pc: u32 = 0x83134C4C;
    'dispatch: loop {
        match pc {
            0x83134C4C => {
    //   block [0x83134C4C..0x83134C5C)
	// 83134C4C: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83134C50: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83134C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134C58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134C5C size=52
    let mut pc: u32 = 0x83134C5C;
    'dispatch: loop {
        match pc {
            0x83134C5C => {
    //   block [0x83134C5C..0x83134C90)
	// 83134C5C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134C60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83134C64: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 83134C68: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83134C6C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83134C70: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134C74: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83134C78: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134C7C: 91270004  stw r9, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83134C80: 7D2A182E  lwzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83134C84: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83134C88: 7D6A192E  stwx r11, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 83134C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134C90 size=12
    let mut pc: u32 = 0x83134C90;
    'dispatch: loop {
        match pc {
            0x83134C90 => {
    //   block [0x83134C90..0x83134C9C)
	// 83134C90: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83134C94: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134C98: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83134C9C size=16
    let mut pc: u32 = 0x83134C9C;
    'dispatch: loop {
        match pc {
            0x83134C9C => {
    //   block [0x83134C9C..0x83134CAC)
	// 83134C9C: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83134CA0: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83134CA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134CA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134CB0 size=100
    let mut pc: u32 = 0x83134CB0;
    'dispatch: loop {
        match pc {
            0x83134CB0 => {
    //   block [0x83134CB0..0x83134D14)
	// 83134CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134CB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83134CBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134CC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134CC4: 48003C35  bl 0x831388f8
	ctx.lr = 0x83134CC8;
	sub_831388F8(ctx, base);
	// 83134CC8: 48003D29  bl 0x831389f0
	ctx.lr = 0x83134CCC;
	sub_831389F0(ctx, base);
	// 83134CCC: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83134CD0: 83FE7E30  lwz r31, 0x7e30(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32304 as u32) ) } as u64;
	// 83134CD4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83134CD8: 409A0018  bne cr6, 0x83134cf0
	if !ctx.cr[6].eq {
	pc = 0x83134CF0; continue 'dispatch;
	}
	// 83134CDC: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83134CE0: 38A01200  li r5, 0x1200
	ctx.r[5].s64 = 4608;
	// 83134CE4: 386B16C0  addi r3, r11, 0x16c0
	ctx.r[3].s64 = ctx.r[11].s64 + 5824;
	// 83134CE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83134CEC: 480734F5  bl 0x831a81e0
	ctx.lr = 0x83134CF0;
	sub_831A81E0(ctx, base);
	// 83134CF0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 83134CF4: 917E7E30  stw r11, 0x7e30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32304 as u32), ctx.r[11].u32 ) };
	// 83134CF8: 48003D39  bl 0x83138a30
	ctx.lr = 0x83134CFC;
	sub_83138A30(ctx, base);
	// 83134CFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83134D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134D08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83134D0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134D18 size=80
    let mut pc: u32 = 0x83134D18;
    'dispatch: loop {
        match pc {
            0x83134D18 => {
    //   block [0x83134D18..0x83134D68)
	// 83134D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134D24: 48003CCD  bl 0x831389f0
	ctx.lr = 0x83134D28;
	sub_831389F0(ctx, base);
	// 83134D28: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83134D2C: 816A7E30  lwz r11, 0x7e30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32304 as u32) ) } as u64;
	// 83134D30: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83134D34: 916A7E30  stw r11, 0x7e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32304 as u32), ctx.r[11].u32 ) };
	// 83134D38: 40820018  bne 0x83134d50
	if !ctx.cr[0].eq {
	pc = 0x83134D50; continue 'dispatch;
	}
	// 83134D3C: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83134D40: 38A01200  li r5, 0x1200
	ctx.r[5].s64 = 4608;
	// 83134D44: 386B16C0  addi r3, r11, 0x16c0
	ctx.r[3].s64 = ctx.r[11].s64 + 5824;
	// 83134D48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83134D4C: 48073495  bl 0x831a81e0
	ctx.lr = 0x83134D50;
	sub_831A81E0(ctx, base);
	// 83134D50: 48003CE1  bl 0x83138a30
	ctx.lr = 0x83134D54;
	sub_83138A30(ctx, base);
	// 83134D54: 48003C25  bl 0x83138978
	ctx.lr = 0x83134D58;
	sub_83138978(ctx, base);
	// 83134D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134D68 size=136
    let mut pc: u32 = 0x83134D68;
    'dispatch: loop {
        match pc {
            0x83134D68 => {
    //   block [0x83134D68..0x83134DF0)
	// 83134D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134D70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134D74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134D78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134D7C: 48003C75  bl 0x831389f0
	ctx.lr = 0x83134D80;
	sub_831389F0(ctx, base);
	// 83134D80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83134D84: 409A001C  bne cr6, 0x83134da0
	if !ctx.cr[6].eq {
	pc = 0x83134DA0; continue 'dispatch;
	}
	// 83134D88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134D8C: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134D90: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134D94: 386B19B0  addi r3, r11, 0x19b0
	ctx.r[3].s64 = ctx.r[11].s64 + 6576;
	// 83134D98: 4BFFE509  bl 0x831332a0
	ctx.lr = 0x83134D9C;
	sub_831332A0(ctx, base);
	// 83134D9C: 4800003C  b 0x83134dd8
	pc = 0x83134DD8; continue 'dispatch;
	// 83134DA0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83134DA8: 409A0018  bne cr6, 0x83134dc0
	if !ctx.cr[6].eq {
	pc = 0x83134DC0; continue 'dispatch;
	}
	// 83134DAC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134DB0: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134DB4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134DB8: 386B19A4  addi r3, r11, 0x19a4
	ctx.r[3].s64 = ctx.r[11].s64 + 6564;
	// 83134DBC: 4BFFFFDC  b 0x83134d98
	pc = 0x83134D98; continue 'dispatch;
	// 83134DC0: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 83134DC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83134DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134DCC: 48073415  bl 0x831a81e0
	ctx.lr = 0x83134DD0;
	sub_831A81E0(ctx, base);
	// 83134DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83134DD4: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 83134DD8: 48003C59  bl 0x83138a30
	ctx.lr = 0x83134DDC;
	sub_83138A30(ctx, base);
	// 83134DDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134DE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134DF0 size=124
    let mut pc: u32 = 0x83134DF0;
    'dispatch: loop {
        match pc {
            0x83134DF0 => {
    //   block [0x83134DF0..0x83134E6C)
	// 83134DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134DF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134DFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134E00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134E04: 48003BED  bl 0x831389f0
	ctx.lr = 0x83134E08;
	sub_831389F0(ctx, base);
	// 83134E08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83134E0C: 409A0020  bne cr6, 0x83134e2c
	if !ctx.cr[6].eq {
	pc = 0x83134E2C; continue 'dispatch;
	}
	// 83134E10: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134E14: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134E18: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134E1C: 386B19C8  addi r3, r11, 0x19c8
	ctx.r[3].s64 = ctx.r[11].s64 + 6600;
	// 83134E20: 4BFFE481  bl 0x831332a0
	ctx.lr = 0x83134E24;
	sub_831332A0(ctx, base);
	// 83134E24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134E28: 48000028  b 0x83134e50
	pc = 0x83134E50; continue 'dispatch;
	// 83134E2C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134E30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83134E34: 409A0018  bne cr6, 0x83134e4c
	if !ctx.cr[6].eq {
	pc = 0x83134E4C; continue 'dispatch;
	}
	// 83134E38: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134E3C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134E40: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134E44: 386B19BC  addi r3, r11, 0x19bc
	ctx.r[3].s64 = ctx.r[11].s64 + 6588;
	// 83134E48: 4BFFFFD8  b 0x83134e20
	pc = 0x83134E20; continue 'dispatch;
	// 83134E4C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83134E50: 48003BE1  bl 0x83138a30
	ctx.lr = 0x83134E54;
	sub_83138A30(ctx, base);
	// 83134E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134E70 size=112
    let mut pc: u32 = 0x83134E70;
    'dispatch: loop {
        match pc {
            0x83134E70 => {
    //   block [0x83134E70..0x83134EE0)
	// 83134E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134E74: 480732F9  bl 0x831a816c
	ctx.lr = 0x83134E78;
	sub_831A8130(ctx, base);
	// 83134E78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134E7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134E80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83134E84: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83134E88: 48003B69  bl 0x831389f0
	ctx.lr = 0x83134E8C;
	sub_831389F0(ctx, base);
	// 83134E8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83134E90: 409A001C  bne cr6, 0x83134eac
	if !ctx.cr[6].eq {
	pc = 0x83134EAC; continue 'dispatch;
	}
	// 83134E94: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134E98: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134E9C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134EA0: 386B19E0  addi r3, r11, 0x19e0
	ctx.r[3].s64 = ctx.r[11].s64 + 6624;
	// 83134EA4: 4BFFE3FD  bl 0x831332a0
	ctx.lr = 0x83134EA8;
	sub_831332A0(ctx, base);
	// 83134EA8: 4800002C  b 0x83134ed4
	pc = 0x83134ED4; continue 'dispatch;
	// 83134EAC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134EB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83134EB4: 409A0018  bne cr6, 0x83134ecc
	if !ctx.cr[6].eq {
	pc = 0x83134ECC; continue 'dispatch;
	}
	// 83134EB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134EBC: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134EC0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134EC4: 386B19D4  addi r3, r11, 0x19d4
	ctx.r[3].s64 = ctx.r[11].s64 + 6612;
	// 83134EC8: 4BFFFFDC  b 0x83134ea4
	pc = 0x83134EA4; continue 'dispatch;
	// 83134ECC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 83134ED0: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83134ED4: 48003B5D  bl 0x83138a30
	ctx.lr = 0x83134ED8;
	sub_83138A30(ctx, base);
	// 83134ED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83134EDC: 480732E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134EE0 size=56
    let mut pc: u32 = 0x83134EE0;
    'dispatch: loop {
        match pc {
            0x83134EE0 => {
    //   block [0x83134EE0..0x83134F18)
	// 83134EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134EE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134EEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83134EF4: 48003AFD  bl 0x831389f0
	ctx.lr = 0x83134EF8;
	sub_831389F0(ctx, base);
	// 83134EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134EFC: 4BFFF99D  bl 0x83134898
	ctx.lr = 0x83134F00;
	sub_83134898(ctx, base);
	// 83134F00: 48003B31  bl 0x83138a30
	ctx.lr = 0x83134F04;
	sub_83138A30(ctx, base);
	// 83134F04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83134F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134F10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134F18 size=212
    let mut pc: u32 = 0x83134F18;
    'dispatch: loop {
        match pc {
            0x83134F18 => {
    //   block [0x83134F18..0x83134FEC)
	// 83134F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83134F20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83134F24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83134F28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134F2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83134F30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83134F34: 48003ABD  bl 0x831389f0
	ctx.lr = 0x83134F38;
	sub_831389F0(ctx, base);
	// 83134F38: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83134F3C: 409A001C  bne cr6, 0x83134f58
	if !ctx.cr[6].eq {
	pc = 0x83134F58; continue 'dispatch;
	}
	// 83134F40: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134F44: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83134F48: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134F4C: 386B1A10  addi r3, r11, 0x1a10
	ctx.r[3].s64 = ctx.r[11].s64 + 6672;
	// 83134F50: 4BFFE351  bl 0x831332a0
	ctx.lr = 0x83134F54;
	sub_831332A0(ctx, base);
	// 83134F54: 48000074  b 0x83134fc8
	pc = 0x83134FC8; continue 'dispatch;
	// 83134F58: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83134F5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83134F60: 409A0018  bne cr6, 0x83134f78
	if !ctx.cr[6].eq {
	pc = 0x83134F78; continue 'dispatch;
	}
	// 83134F64: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134F68: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83134F6C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83134F70: 386B1A04  addi r3, r11, 0x1a04
	ctx.r[3].s64 = ctx.r[11].s64 + 6660;
	// 83134F74: 4BFFFFDC  b 0x83134f50
	pc = 0x83134F50; continue 'dispatch;
	// 83134F78: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 83134F7C: 41990030  bgt cr6, 0x83134fac
	if ctx.cr[6].gt {
	pc = 0x83134FAC; continue 'dispatch;
	}
	// 83134F80: 397F0006  addi r11, r31, 6
	ctx.r[11].s64 = ctx.r[31].s64 + 6;
	// 83134F84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134F88: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83134F8C: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83134F90: 48000010  b 0x83134fa0
	pc = 0x83134FA0; continue 'dispatch;
	// 83134F94: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83134F98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83134F9C: 7FEAFA14  add r31, r10, r31
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 83134FA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134FA4: 4082FFF0  bne 0x83134f94
	if !ctx.cr[0].eq {
	pc = 0x83134F94; continue 'dispatch;
	}
	// 83134FA8: 48000024  b 0x83134fcc
	pc = 0x83134FCC; continue 'dispatch;
	// 83134FAC: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83134FB0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83134FB4: 41820014  beq 0x83134fc8
	if ctx.cr[0].eq {
	pc = 0x83134FC8; continue 'dispatch;
	}
	// 83134FB8: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83134FBC: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83134FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83134FC4: 4E800421  bctrl
	ctx.lr = 0x83134FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83134FC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83134FCC: 48003A65  bl 0x83138a30
	ctx.lr = 0x83134FD0;
	sub_83138A30(ctx, base);
	// 83134FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83134FD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83134FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83134FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83134FE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83134FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83134FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83134FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83134FF0 size=64
    let mut pc: u32 = 0x83134FF0;
    'dispatch: loop {
        match pc {
            0x83134FF0 => {
    //   block [0x83134FF0..0x83135030)
	// 83134FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83134FF4: 48073175  bl 0x831a8168
	ctx.lr = 0x83134FF8;
	sub_831A8130(ctx, base);
	// 83134FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83134FFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135000: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83135004: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83135008: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8313500C: 480039E5  bl 0x831389f0
	ctx.lr = 0x83135010;
	sub_831389F0(ctx, base);
	// 83135010: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83135014: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83135018: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8313501C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135020: 4BFFF929  bl 0x83134948
	ctx.lr = 0x83135024;
	sub_83134948(ctx, base);
	// 83135024: 48003A0D  bl 0x83138a30
	ctx.lr = 0x83135028;
	sub_83138A30(ctx, base);
	// 83135028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8313502C: 4807318C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135030 size=56
    let mut pc: u32 = 0x83135030;
    'dispatch: loop {
        match pc {
            0x83135030 => {
    //   block [0x83135030..0x83135068)
	// 83135030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135034: 48073139  bl 0x831a816c
	ctx.lr = 0x83135038;
	sub_831A8130(ctx, base);
	// 83135038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313503C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135040: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83135044: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83135048: 480039A9  bl 0x831389f0
	ctx.lr = 0x8313504C;
	sub_831389F0(ctx, base);
	// 8313504C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83135050: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83135054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135058: 4BFFFA11  bl 0x83134a68
	ctx.lr = 0x8313505C;
	sub_83134A68(ctx, base);
	// 8313505C: 480039D5  bl 0x83138a30
	ctx.lr = 0x83135060;
	sub_83138A30(ctx, base);
	// 83135060: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83135064: 48073158  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135068 size=56
    let mut pc: u32 = 0x83135068;
    'dispatch: loop {
        match pc {
            0x83135068 => {
    //   block [0x83135068..0x831350A0)
	// 83135068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313506C: 48073101  bl 0x831a816c
	ctx.lr = 0x83135070;
	sub_831A8130(ctx, base);
	// 83135070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135078: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8313507C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83135080: 48003971  bl 0x831389f0
	ctx.lr = 0x83135084;
	sub_831389F0(ctx, base);
	// 83135084: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83135088: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8313508C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135090: 4BFFFB01  bl 0x83134b90
	ctx.lr = 0x83135094;
	sub_83134B90(ctx, base);
	// 83135094: 4800399D  bl 0x83138a30
	ctx.lr = 0x83135098;
	sub_83138A30(ctx, base);
	// 83135098: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313509C: 48073120  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831350A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831350A0 size=240
    let mut pc: u32 = 0x831350A0;
    'dispatch: loop {
        match pc {
            0x831350A0 => {
    //   block [0x831350A0..0x83135190)
	// 831350A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831350A4: 480730C5  bl 0x831a8168
	ctx.lr = 0x831350A8;
	sub_831A8130(ctx, base);
	// 831350A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831350AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831350B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831350B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831350B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 831350BC: 48003935  bl 0x831389f0
	ctx.lr = 0x831350C0;
	sub_831389F0(ctx, base);
	// 831350C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831350C4: 409A001C  bne cr6, 0x831350e0
	if !ctx.cr[6].eq {
	pc = 0x831350E0; continue 'dispatch;
	}
	// 831350C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831350CC: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 831350D0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831350D4: 386B1A70  addi r3, r11, 0x1a70
	ctx.r[3].s64 = ctx.r[11].s64 + 6768;
	// 831350D8: 4BFFE1C9  bl 0x831332a0
	ctx.lr = 0x831350DC;
	sub_831332A0(ctx, base);
	// 831350DC: 480000A0  b 0x8313517c
	pc = 0x8313517C; continue 'dispatch;
	// 831350E0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831350E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831350E8: 409A0018  bne cr6, 0x83135100
	if !ctx.cr[6].eq {
	pc = 0x83135100; continue 'dispatch;
	}
	// 831350EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831350F0: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 831350F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831350F8: 386B1A64  addi r3, r11, 0x1a64
	ctx.r[3].s64 = ctx.r[11].s64 + 6756;
	// 831350FC: 4BFFFFDC  b 0x831350d8
	pc = 0x831350D8; continue 'dispatch;
	// 83135100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83135104: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 83135108: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8313510C: 41990054  bgt cr6, 0x83135160
	if ctx.cr[6].gt {
	pc = 0x83135160; continue 'dispatch;
	}
	// 83135110: 397E0006  addi r11, r30, 6
	ctx.r[11].s64 = ctx.r[30].s64 + 6;
	// 83135114: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83135118: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8313511C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83135120: 4182005C  beq 0x8313517c
	if ctx.cr[0].eq {
	pc = 0x8313517C; continue 'dispatch;
	}
	// 83135124: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 83135128: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8313512C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83135130: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83135134: 895F0005  lbz r10, 5(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 83135138: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8313513C: 409A0014  bne cr6, 0x83135150
	if !ctx.cr[6].eq {
	pc = 0x83135150; continue 'dispatch;
	}
	// 83135140: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83135144: 41980038  blt cr6, 0x8313517c
	if ctx.cr[6].lt {
	pc = 0x8313517C; continue 'dispatch;
	}
	// 83135148: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8313514C: 48000034  b 0x83135180
	pc = 0x83135180; continue 'dispatch;
	// 83135150: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 83135154: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83135158: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8313515C: 48000024  b 0x83135180
	pc = 0x83135180; continue 'dispatch;
	// 83135160: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83135164: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83135168: 41820014  beq 0x8313517c
	if ctx.cr[0].eq {
	pc = 0x8313517C; continue 'dispatch;
	}
	// 8313516C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83135170: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83135174: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83135178: 4E800421  bctrl
	ctx.lr = 0x8313517C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313517C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83135180: 480038B1  bl 0x83138a30
	ctx.lr = 0x83135184;
	sub_83138A30(ctx, base);
	// 83135184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8313518C: 4807302C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135190 size=68
    let mut pc: u32 = 0x83135190;
    'dispatch: loop {
        match pc {
            0x83135190 => {
    //   block [0x83135190..0x831351D4)
	// 83135190: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135194: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83135198: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313519C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831351A0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831351A4: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831351A8: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 831351AC: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831351B0: 40990008  ble cr6, 0x831351b8
	if !ctx.cr[6].gt {
	pc = 0x831351B8; continue 'dispatch;
	}
	// 831351B4: 90850004  stw r4, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 831351B8: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 831351BC: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 831351C0: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831351C4: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831351C8: 4082000C  bne 0x831351d4
	if !ctx.cr[0].eq {
		sub_831351D4(ctx, base);
		return;
	}
	// 831351CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831351D0: 48000010  b 0x831351e0
	sub_831351D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831351D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831351D4 size=20
    let mut pc: u32 = 0x831351D4;
    'dispatch: loop {
        match pc {
            0x831351D4 => {
    //   block [0x831351D4..0x831351E8)
	// 831351D4: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 831351D8: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 831351DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831351E0: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831351E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831351E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831351E8 size=172
    let mut pc: u32 = 0x831351E8;
    'dispatch: loop {
        match pc {
            0x831351E8 => {
    //   block [0x831351E8..0x83135294)
	// 831351E8: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831351EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 831351F0: 89230001  lbz r9, 1(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 831351F4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 831351F8: 89030002  lbz r8, 2(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 831351FC: 396B5B18  addi r11, r11, 0x5b18
	ctx.r[11].s64 = ctx.r[11].s64 + 23320;
	// 83135200: 88E30003  lbz r7, 3(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 83135204: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83135208: 88C30004  lbz r6, 4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313520C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83135210: 88A30005  lbz r5, 5(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 83135214: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 83135218: 88830006  lbz r4, 6(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8313521C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83135220: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83135224: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83135228: 7CE70774  extsb r7, r7
	ctx.r[7].s64 = ctx.r[7].s8 as i64;
	// 8313522C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83135230: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83135234: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83135238: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8313523C: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83135240: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83135244: 7CC70774  extsb r7, r6
	ctx.r[7].s64 = ctx.r[6].s8 as i64;
	// 83135248: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8313524C: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83135250: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83135254: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83135258: 7CA70774  extsb r7, r5
	ctx.r[7].s64 = ctx.r[5].s8 as i64;
	// 8313525C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83135260: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83135264: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83135268: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8313526C: 7C870774  extsb r7, r4
	ctx.r[7].s64 = ctx.r[4].s8 as i64;
	// 83135270: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83135274: 7D28582E  lwzx r9, r8, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83135278: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8313527C: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83135280: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83135284: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83135288: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8313528C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83135290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135298 size=184
    let mut pc: u32 = 0x83135298;
    'dispatch: loop {
        match pc {
            0x83135298 => {
    //   block [0x83135298..0x83135350)
	// 83135298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313529C: 48072EC9  bl 0x831a8164
	ctx.lr = 0x831352A0;
	sub_831A8130(ctx, base);
	// 831352A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831352A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831352A8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 831352AC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 831352B0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831352B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831352B8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831352BC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831352C0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831352C4: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831352C8: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831352CC: 40980078  bge cr6, 0x83135344
	if !ctx.cr[6].lt {
	pc = 0x83135344; continue 'dispatch;
	}
	// 831352D0: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 831352D4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831352D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831352DC: 48077AF5  bl 0x831acdd0
	ctx.lr = 0x831352E0;
	sub_831ACDD0(ctx, base);
	// 831352E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831352E4: 41820040  beq 0x83135324
	if ctx.cr[0].eq {
	pc = 0x83135324; continue 'dispatch;
	}
	// 831352E8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831352EC: 419A001C  beq cr6, 0x83135308
	if ctx.cr[6].eq {
	pc = 0x83135308; continue 'dispatch;
	}
	// 831352F0: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 831352F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831352F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831352FC: 48077AD5  bl 0x831acdd0
	ctx.lr = 0x83135300;
	sub_831ACDD0(ctx, base);
	// 83135300: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83135304: 41820040  beq 0x83135344
	if ctx.cr[0].eq {
	pc = 0x83135344; continue 'dispatch;
	}
	// 83135308: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8313530C: 4BFFFEDD  bl 0x831351e8
	ctx.lr = 0x83135310;
	sub_831351E8(ctx, base);
	// 83135310: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 83135314: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 83135318: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8313531C: 4198FFB4  blt cr6, 0x831352d0
	if ctx.cr[6].lt {
	pc = 0x831352D0; continue 'dispatch;
	}
	// 83135320: 48000018  b 0x83135338
	pc = 0x83135338; continue 'dispatch;
	// 83135324: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 83135328: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8313532C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83135330: 4BFFFEB9  bl 0x831351e8
	ctx.lr = 0x83135334;
	sub_831351E8(ctx, base);
	// 83135334: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83135338: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8313533C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135340: 41980008  blt cr6, 0x83135348
	if ctx.cr[6].lt {
	pc = 0x83135348; continue 'dispatch;
	}
	// 83135344: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83135348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8313534C: 48072E68  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135350 size=72
    let mut pc: u32 = 0x83135350;
    'dispatch: loop {
        match pc {
            0x83135350 => {
    //   block [0x83135350..0x83135398)
	// 83135350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313535C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135364: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83135368: 419A001C  beq cr6, 0x83135384
	if ctx.cr[6].eq {
	pc = 0x83135384; continue 'dispatch;
	}
	// 8313536C: 4BF92775  bl 0x830c7ae0
	ctx.lr = 0x83135370;
	sub_830C7AE0(ctx, base);
	// 83135370: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 83135374: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313537C: 48072E65  bl 0x831a81e0
	ctx.lr = 0x83135380;
	sub_831A81E0(ctx, base);
	// 83135380: 4BF92761  bl 0x830c7ae0
	ctx.lr = 0x83135384;
	sub_830C7AE0(ctx, base);
	// 83135384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83135388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313538C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135398 size=420
    let mut pc: u32 = 0x83135398;
    'dispatch: loop {
        match pc {
            0x83135398 => {
    //   block [0x83135398..0x8313553C)
	// 83135398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313539C: 48072DC1  bl 0x831a815c
	ctx.lr = 0x831353A0;
	sub_831A8130(ctx, base);
	// 831353A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831353A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831353A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831353AC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 831353B0: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 831353B4: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831353B8: 40810024  ble 0x831353dc
	if !ctx.cr[0].gt {
	pc = 0x831353DC; continue 'dispatch;
	}
	// 831353BC: 397E0014  addi r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 + 20;
	// 831353C0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831353C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831353C8: 893E0002  lbz r9, 2(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 831353CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831353D0: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 831353D4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831353D8: 4198FFE8  blt cr6, 0x831353c0
	if ctx.cr[6].lt {
	pc = 0x831353C0; continue 'dispatch;
	}
	// 831353DC: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 831353E0: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 831353E4: 939E002C  stw r28, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 831353E8: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831353EC: 4081009C  ble 0x83135488
	if !ctx.cr[0].gt {
	pc = 0x83135488; continue 'dispatch;
	}
	// 831353F0: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 831353F4: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831353F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831353FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135400: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83135404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83135408: 4E800421  bctrl
	ctx.lr = 0x8313540C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313540C: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135410: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 83135414: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313541C: 817A0024  lwz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 83135420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83135424: 4E800421  bctrl
	ctx.lr = 0x83135428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83135428: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8313542C: 817A0018  lwz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 83135430: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135438: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8313543C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83135440: 4E800421  bctrl
	ctx.lr = 0x83135444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83135444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135448: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8313544C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83135450: 48072D91  bl 0x831a81e0
	ctx.lr = 0x83135454;
	sub_831A81E0(ctx, base);
	// 83135454: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135458: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8313545C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135464: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83135468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313546C: 4E800421  bctrl
	ctx.lr = 0x83135470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83135470: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 83135474: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83135478: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8313547C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83135480: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83135484: 4198FF70  blt cr6, 0x831353f4
	if ctx.cr[6].lt {
	pc = 0x831353F4; continue 'dispatch;
	}
	// 83135488: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 8313548C: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83135490: 4081009C  ble 0x8313552c
	if !ctx.cr[0].gt {
	pc = 0x8313552C; continue 'dispatch;
	}
	// 83135494: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 83135498: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313549C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831354A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831354A4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831354A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831354AC: 4E800421  bctrl
	ctx.lr = 0x831354B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831354B0: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831354B4: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 831354B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831354BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831354C0: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 831354C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831354C8: 4E800421  bctrl
	ctx.lr = 0x831354CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831354CC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 831354D0: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 831354D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831354D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831354DC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 831354E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831354E4: 4E800421  bctrl
	ctx.lr = 0x831354E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831354E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831354EC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831354F0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831354F4: 48072CED  bl 0x831a81e0
	ctx.lr = 0x831354F8;
	sub_831A81E0(ctx, base);
	// 831354F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831354FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83135500: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135508: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313550C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83135510: 4E800421  bctrl
	ctx.lr = 0x83135514;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83135514: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 83135518: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8313551C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83135520: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83135524: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83135528: 4198FF70  blt cr6, 0x83135498
	if ctx.cr[6].lt {
	pc = 0x83135498; continue 'dispatch;
	}
	// 8313552C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83135530: 997E0001  stb r11, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83135534: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83135538: 48072C74  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135540 size=8
    let mut pc: u32 = 0x83135540;
    'dispatch: loop {
        match pc {
            0x83135540 => {
    //   block [0x83135540..0x83135548)
	// 83135540: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 83135544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135548 size=280
    let mut pc: u32 = 0x83135548;
    'dispatch: loop {
        match pc {
            0x83135548 => {
    //   block [0x83135548..0x83135660)
	// 83135548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313554C: 48072C1D  bl 0x831a8168
	ctx.lr = 0x83135550;
	sub_831A8130(ctx, base);
	// 83135550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135554: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83135558: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8313555C: 409A0018  bne cr6, 0x83135574
	if !ctx.cr[6].eq {
	pc = 0x83135574; continue 'dispatch;
	}
	// 83135560: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135564: 386B1AAC  addi r3, r11, 0x1aac
	ctx.r[3].s64 = ctx.r[11].s64 + 6828;
	// 83135568: 4BFFB8D1  bl 0x83130e38
	ctx.lr = 0x8313556C;
	sub_83130E38(ctx, base);
	// 8313556C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83135570: 480000E8  b 0x83135658
	pc = 0x83135658; continue 'dispatch;
	// 83135574: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83135578: 4BF92569  bl 0x830c7ae0
	ctx.lr = 0x8313557C;
	sub_830C7AE0(ctx, base);
	// 8313557C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83135580: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83135584: 394BDB40  addi r10, r11, -0x24c0
	ctx.r[10].s64 = ctx.r[11].s64 + -9408;
	// 83135588: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8313558C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 83135590: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83135594: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135598: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8313559C: 419A0020  beq cr6, 0x831355bc
	if ctx.cr[6].eq {
	pc = 0x831355BC; continue 'dispatch;
	}
	// 831355A0: 3D0A0001  addis r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 65536;
	// 831355A4: 396B0238  addi r11, r11, 0x238
	ctx.r[11].s64 = ctx.r[11].s64 + 568;
	// 831355A8: 39088E00  addi r8, r8, -0x7200
	ctx.r[8].s64 = ctx.r[8].s64 + -29184;
	// 831355AC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831355B0: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831355B4: 4198FFE0  blt cr6, 0x83135594
	if ctx.cr[6].lt {
	pc = 0x83135594; continue 'dispatch;
	}
	// 831355B8: 48000010  b 0x831355c8
	pc = 0x831355C8; continue 'dispatch;
	// 831355BC: 1D690238  mulli r11, r9, 0x238
	ctx.r[11].s64 = ctx.r[9].s64 * 568;
	// 831355C0: 7FEB5215  add. r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831355C4: 40820014  bne 0x831355d8
	if !ctx.cr[0].eq {
	pc = 0x831355D8; continue 'dispatch;
	}
	// 831355C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831355CC: 386B1A7C  addi r3, r11, 0x1a7c
	ctx.r[3].s64 = ctx.r[11].s64 + 6780;
	// 831355D0: 4BFFB869  bl 0x83130e38
	ctx.lr = 0x831355D4;
	sub_83130E38(ctx, base);
	// 831355D4: 48000078  b 0x8313564c
	pc = 0x8313564C; continue 'dispatch;
	// 831355D8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 831355DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831355E0: 9BBF0001  stb r29, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 831355E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831355E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831355EC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831355F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831355F4: 4E800421  bctrl
	ctx.lr = 0x831355F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831355F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831355FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83135600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135604: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83135608: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8313560C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83135610: 4E800421  bctrl
	ctx.lr = 0x83135614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83135614: 7D7C1A14  add r11, r28, r3
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[3].u64;
	// 83135618: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8313561C: 55671838  slwi r7, r11, 3
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83135620: 393F0050  addi r9, r31, 0x50
	ctx.r[9].s64 = ctx.r[31].s64 + 80;
	// 83135624: 7D0743D6  divw r8, r7, r8
	ctx.r[8].s32 = ctx.r[7].s32 / ctx.r[8].s32;
	// 83135628: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8313562C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83135630: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 83135634: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83135638: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8313563C: 39290020  addi r9, r9, 0x20
	ctx.r[9].s64 = ctx.r[9].s64 + 32;
	// 83135640: 4082FFF4  bne 0x83135634
	if !ctx.cr[0].eq {
	pc = 0x83135634; continue 'dispatch;
	}
	// 83135644: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83135648: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8313564C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83135650: 4BF92491  bl 0x830c7ae0
	ctx.lr = 0x83135654;
	sub_830C7AE0(ctx, base);
	// 83135654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8313565C: 48072B5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83135660 size=328
    let mut pc: u32 = 0x83135660;
    'dispatch: loop {
        match pc {
            0x83135660 => {
    //   block [0x83135660..0x831357A8)
	// 83135660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135664: 48072B09  bl 0x831a816c
	ctx.lr = 0x83135668;
	sub_831A8130(ctx, base);
	// 83135668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313566C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83135670: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83135674: 409A0018  bne cr6, 0x8313568c
	if !ctx.cr[6].eq {
	pc = 0x8313568C; continue 'dispatch;
	}
	// 83135678: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313567C: 386B1B08  addi r3, r11, 0x1b08
	ctx.r[3].s64 = ctx.r[11].s64 + 6920;
	// 83135680: 4BFFB7B9  bl 0x83130e38
	ctx.lr = 0x83135684;
	sub_83130E38(ctx, base);
	// 83135684: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83135688: 48000118  b 0x831357a0
	pc = 0x831357A0; continue 'dispatch;
	// 8313568C: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83135690: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 83135694: 4098FFF0  bge cr6, 0x83135684
	if !ctx.cr[6].lt {
	pc = 0x83135684; continue 'dispatch;
	}
	// 83135698: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8313569C: 409A0014  bne cr6, 0x831356b0
	if !ctx.cr[6].eq {
	pc = 0x831356B0; continue 'dispatch;
	}
	// 831356A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831356A4: 386B1ADC  addi r3, r11, 0x1adc
	ctx.r[3].s64 = ctx.r[11].s64 + 6876;
	// 831356A8: 4BFFB791  bl 0x83130e38
	ctx.lr = 0x831356AC;
	sub_83130E38(ctx, base);
	// 831356AC: 4BFFFFD8  b 0x83135684
	pc = 0x83135684; continue 'dispatch;
	// 831356B0: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 831356B4: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 831356B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831356BC: 6128FFFF  ori r8, r9, 0xffff
	ctx.r[8].u64 = ctx.r[9].u64 | 65535;
	// 831356C0: 392B000F  addi r9, r11, 0xf
	ctx.r[9].s64 = ctx.r[11].s64 + 15;
	// 831356C4: 7D232670  srawi r3, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 831356C8: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 831356CC: 54632036  slwi r3, r3, 4
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 831356D0: 7D234850  subf r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	// 831356D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831356D8: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831356DC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 831356E0: 81290038  lwz r9, 0x38(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 831356E4: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831356E8: 419A0008  beq cr6, 0x831356f0
	if ctx.cr[6].eq {
	pc = 0x831356F0; continue 'dispatch;
	}
	// 831356EC: 38690001  addi r3, r9, 1
	ctx.r[3].s64 = ctx.r[9].s64 + 1;
	// 831356F0: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831356F4: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 831356F8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831356FC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 83135700: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 83135704: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83135708: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8313570C: 8BE90000  lbz r31, 0(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135710: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83135714: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83135718: 409AFFF4  bne cr6, 0x8313570c
	if !ctx.cr[6].eq {
	pc = 0x8313570C; continue 'dispatch;
	}
	// 8313571C: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83135720: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83135724: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83135728: 5528003F  rotlwi. r8, r9, 0
	ctx.r[8].u64 = ((ctx.r[9].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8313572C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 83135730: 41820020  beq 0x83135750
	if ctx.cr[0].eq {
	pc = 0x83135750; continue 'dispatch;
	}
	// 83135734: 7FE920AE  lbzx r31, r9, r4
	ctx.r[31].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 83135738: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8313573C: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83135740: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83135744: 7FFFF214  add r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 83135748: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8313574C: 4198FFE8  blt cr6, 0x83135734
	if ctx.cr[6].lt {
	pc = 0x83135734; continue 'dispatch;
	}
	// 83135750: 90EB0014  stw r7, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83135754: 90CB0010  stw r6, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 83135758: 90AB000C  stw r5, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8313575C: 93AB0018  stw r29, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83135760: 93AB001C  stw r29, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 83135764: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 83135768: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8313576C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83135770: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 83135774: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83135778: 7D272670  srawi r7, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 8313577C: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 83135780: 7D070194  addze r8, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83135784: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83135788: 916A0024  stw r11, 0x24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8313578C: 7D684850  subf r11, r8, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 83135790: 916A001C  stw r11, 0x1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83135794: 409A000C  bne cr6, 0x831357a0
	if !ctx.cr[6].eq {
	pc = 0x831357A0; continue 'dispatch;
	}
	// 83135798: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8313579C: 996A0001  stb r11, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 831357A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831357A4: 48072A18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831357A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831357A8 size=16
    let mut pc: u32 = 0x831357A8;
    'dispatch: loop {
        match pc {
            0x831357A8 => {
    //   block [0x831357A8..0x831357B8)
	// 831357A8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831357AC: 816A7E38  lwz r11, 0x7e38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32312 as u32) ) } as u64;
	// 831357B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831357B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831357B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831357B8 size=28
    let mut pc: u32 = 0x831357B8;
    'dispatch: loop {
        match pc {
            0x831357B8 => {
    //   block [0x831357B8..0x831357D4)
	// 831357B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831357BC: 808B7E40  lwz r4, 0x7e40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32320 as u32) ) } as u64;
	// 831357C0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831357C4: 806B7E3C  lwz r3, 0x7e3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32316 as u32) ) } as u64;
	// 831357C8: 816A7E38  lwz r11, 0x7e38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32312 as u32) ) } as u64;
	// 831357CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831357D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831357D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831357D4 size=4
    let mut pc: u32 = 0x831357D4;
    'dispatch: loop {
        match pc {
            0x831357D4 => {
    //   block [0x831357D4..0x831357D8)
	// 831357D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831357D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831357D8 size=64
    let mut pc: u32 = 0x831357D8;
    'dispatch: loop {
        match pc {
            0x831357D8 => {
    //   block [0x831357D8..0x83135818)
	// 831357D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831357DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831357E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831357E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831357E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831357EC: 480CEC5D  bl 0x83204448
	ctx.lr = 0x831357F0;
	sub_83204448(ctx, base);
	// 831357F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831357F4: 4BFFFD55  bl 0x83135548
	ctx.lr = 0x831357F8;
	sub_83135548(ctx, base);
	// 831357F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831357FC: 480CEC55  bl 0x83204450
	ctx.lr = 0x83135800;
	sub_83204450(ctx, base);
	// 83135800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83135808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313580C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135818 size=64
    let mut pc: u32 = 0x83135818;
    'dispatch: loop {
        match pc {
            0x83135818 => {
    //   block [0x83135818..0x83135858)
	// 83135818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313581C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83135824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313582C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135830: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83135834: 480CEC15  bl 0x83204448
	ctx.lr = 0x83135838;
	sub_83204448(ctx, base);
	// 83135838: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8313583C: 480CEC15  bl 0x83204450
	ctx.lr = 0x83135840;
	sub_83204450(ctx, base);
	// 83135840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83135844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313584C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83135850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135858 size=80
    let mut pc: u32 = 0x83135858;
    'dispatch: loop {
        match pc {
            0x83135858 => {
    //   block [0x83135858..0x831358A8)
	// 83135858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313585C: 48072909  bl 0x831a8164
	ctx.lr = 0x83135860;
	sub_831A8130(ctx, base);
	// 83135860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135868: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8313586C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83135870: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83135874: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83135878: 480CEBD1  bl 0x83204448
	ctx.lr = 0x8313587C;
	sub_83204448(ctx, base);
	// 8313587C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83135880: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83135884: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83135888: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8313588C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135890: 4BFFFDD1  bl 0x83135660
	ctx.lr = 0x83135894;
	sub_83135660(ctx, base);
	// 83135894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135898: 480CEBB9  bl 0x83204450
	ctx.lr = 0x8313589C;
	sub_83204450(ctx, base);
	// 8313589C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831358A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831358A4: 48072910  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831358A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831358A8 size=96
    let mut pc: u32 = 0x831358A8;
    'dispatch: loop {
        match pc {
            0x831358A8 => {
    //   block [0x831358A8..0x83135908)
	// 831358A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831358AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831358B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831358B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831358B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831358BC: 480CEB8D  bl 0x83204448
	ctx.lr = 0x831358C0;
	sub_83204448(ctx, base);
	// 831358C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831358C4: 409A0014  bne cr6, 0x831358d8
	if !ctx.cr[6].eq {
	pc = 0x831358D8; continue 'dispatch;
	}
	// 831358C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831358CC: 386B1B34  addi r3, r11, 0x1b34
	ctx.r[3].s64 = ctx.r[11].s64 + 6964;
	// 831358D0: 4BFFB569  bl 0x83130e38
	ctx.lr = 0x831358D4;
	sub_83130E38(ctx, base);
	// 831358D4: 4800001C  b 0x831358f0
	pc = 0x831358F0; continue 'dispatch;
	// 831358D8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 831358DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831358E0: 409A0010  bne cr6, 0x831358f0
	if !ctx.cr[6].eq {
	pc = 0x831358F0; continue 'dispatch;
	}
	// 831358E4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 831358E8: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 831358EC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 831358F0: 480CEB61  bl 0x83204450
	ctx.lr = 0x831358F4;
	sub_83204450(ctx, base);
	// 831358F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831358F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831358FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135908 size=104
    let mut pc: u32 = 0x83135908;
    'dispatch: loop {
        match pc {
            0x83135908 => {
    //   block [0x83135908..0x83135970)
	// 83135908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313590C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83135914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313591C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83135924: 480CEB25  bl 0x83204448
	ctx.lr = 0x83135928;
	sub_83204448(ctx, base);
	// 83135928: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313592C: 409A0014  bne cr6, 0x83135940
	if !ctx.cr[6].eq {
	pc = 0x83135940; continue 'dispatch;
	}
	// 83135930: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135934: 386B1B60  addi r3, r11, 0x1b60
	ctx.r[3].s64 = ctx.r[11].s64 + 7008;
	// 83135938: 4BFFB501  bl 0x83130e38
	ctx.lr = 0x8313593C;
	sub_83130E38(ctx, base);
	// 8313593C: 48000018  b 0x83135954
	pc = 0x83135954; continue 'dispatch;
	// 83135940: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83135944: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83135948: 419A0008  beq cr6, 0x83135950
	if ctx.cr[6].eq {
	pc = 0x83135950; continue 'dispatch;
	}
	// 8313594C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83135950: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 83135954: 480CEAFD  bl 0x83204450
	ctx.lr = 0x83135958;
	sub_83204450(ctx, base);
	// 83135958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313595C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135964: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83135968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313596C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135970 size=104
    let mut pc: u32 = 0x83135970;
    'dispatch: loop {
        match pc {
            0x83135970 => {
    //   block [0x83135970..0x831359D8)
	// 83135970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8313597C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135984: 480CEAC5  bl 0x83204448
	ctx.lr = 0x83135988;
	sub_83204448(ctx, base);
	// 83135988: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8313598C: 3BCBDB40  addi r30, r11, -0x24c0
	ctx.r[30].s64 = ctx.r[11].s64 + -9408;
	// 83135990: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83135994: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135998: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8313599C: 409A000C  bne cr6, 0x831359a8
	if !ctx.cr[6].eq {
	pc = 0x831359A8; continue 'dispatch;
	}
	// 831359A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831359A4: 48003335  bl 0x83138cd8
	ctx.lr = 0x831359A8;
	sub_83138CD8(ctx, base);
	// 831359A8: 3D7E0001  addis r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 65536;
	// 831359AC: 3BFF0238  addi r31, r31, 0x238
	ctx.r[31].s64 = ctx.r[31].s64 + 568;
	// 831359B0: 396B8E00  addi r11, r11, -0x7200
	ctx.r[11].s64 = ctx.r[11].s64 + -29184;
	// 831359B4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831359B8: 4198FFDC  blt cr6, 0x83135994
	if ctx.cr[6].lt {
	pc = 0x83135994; continue 'dispatch;
	}
	// 831359BC: 480CEA95  bl 0x83204450
	ctx.lr = 0x831359C0;
	sub_83204450(ctx, base);
	// 831359C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831359C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831359C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831359CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831359D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831359D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831359D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831359D8 size=88
    let mut pc: u32 = 0x831359D8;
    'dispatch: loop {
        match pc {
            0x831359D8 => {
    //   block [0x831359D8..0x83135A30)
	// 831359D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831359DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831359E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831359E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831359E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831359EC: 480CEA5D  bl 0x83204448
	ctx.lr = 0x831359F0;
	sub_83204448(ctx, base);
	// 831359F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831359F4: 409A0018  bne cr6, 0x83135a0c
	if !ctx.cr[6].eq {
	pc = 0x83135A0C; continue 'dispatch;
	}
	// 831359F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831359FC: 386B1B8C  addi r3, r11, 0x1b8c
	ctx.r[3].s64 = ctx.r[11].s64 + 7052;
	// 83135A00: 4BFFB439  bl 0x83130e38
	ctx.lr = 0x83135A04;
	sub_83130E38(ctx, base);
	// 83135A04: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 83135A08: 4800000C  b 0x83135a14
	pc = 0x83135A14; continue 'dispatch;
	// 83135A0C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83135A10: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 83135A14: 480CEA3D  bl 0x83204450
	ctx.lr = 0x83135A18;
	sub_83204450(ctx, base);
	// 83135A18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83135A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135A28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135A30 size=84
    let mut pc: u32 = 0x83135A30;
    'dispatch: loop {
        match pc {
            0x83135A30 => {
    //   block [0x83135A30..0x83135A84)
	// 83135A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135A38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135A3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135A40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135A44: 480CEA05  bl 0x83204448
	ctx.lr = 0x83135A48;
	sub_83204448(ctx, base);
	// 83135A48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83135A4C: 409A0018  bne cr6, 0x83135a64
	if !ctx.cr[6].eq {
	pc = 0x83135A64; continue 'dispatch;
	}
	// 83135A50: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135A54: 386B1BB8  addi r3, r11, 0x1bb8
	ctx.r[3].s64 = ctx.r[11].s64 + 7096;
	// 83135A58: 4BFFB3E1  bl 0x83130e38
	ctx.lr = 0x83135A5C;
	sub_83130E38(ctx, base);
	// 83135A5C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 83135A60: 48000008  b 0x83135a68
	pc = 0x83135A68; continue 'dispatch;
	// 83135A64: 83FF0024  lwz r31, 0x24(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83135A68: 480CE9E9  bl 0x83204450
	ctx.lr = 0x83135A6C;
	sub_83204450(ctx, base);
	// 83135A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83135A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135A7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135A88 size=128
    let mut pc: u32 = 0x83135A88;
    'dispatch: loop {
        match pc {
            0x83135A88 => {
    //   block [0x83135A88..0x83135B08)
	// 83135A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83135A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135A9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83135AA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83135AA4: 480CE9A5  bl 0x83204448
	ctx.lr = 0x83135AA8;
	sub_83204448(ctx, base);
	// 83135AA8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83135AAC: 409A0014  bne cr6, 0x83135ac0
	if !ctx.cr[6].eq {
	pc = 0x83135AC0; continue 'dispatch;
	}
	// 83135AB0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135AB4: 386B1C10  addi r3, r11, 0x1c10
	ctx.r[3].s64 = ctx.r[11].s64 + 7184;
	// 83135AB8: 4BFFB381  bl 0x83130e38
	ctx.lr = 0x83135ABC;
	sub_83130E38(ctx, base);
	// 83135ABC: 48000030  b 0x83135aec
	pc = 0x83135AEC; continue 'dispatch;
	// 83135AC0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83135AC4: 41980018  blt cr6, 0x83135adc
	if ctx.cr[6].lt {
	pc = 0x83135ADC; continue 'dispatch;
	}
	// 83135AC8: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83135ACC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83135AD0: 4199000C  bgt cr6, 0x83135adc
	if ctx.cr[6].gt {
	pc = 0x83135ADC; continue 'dispatch;
	}
	// 83135AD4: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 83135AD8: 48000014  b 0x83135aec
	pc = 0x83135AEC; continue 'dispatch;
	// 83135ADC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135AE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83135AE4: 386B1BE4  addi r3, r11, 0x1be4
	ctx.r[3].s64 = ctx.r[11].s64 + 7140;
	// 83135AE8: 4BFFB351  bl 0x83130e38
	ctx.lr = 0x83135AEC;
	sub_83130E38(ctx, base);
	// 83135AEC: 480CE965  bl 0x83204450
	ctx.lr = 0x83135AF0;
	sub_83204450(ctx, base);
	// 83135AF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83135AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135AFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83135B00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135B08 size=88
    let mut pc: u32 = 0x83135B08;
    'dispatch: loop {
        match pc {
            0x83135B08 => {
    //   block [0x83135B08..0x83135B60)
	// 83135B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83135B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135B18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135B20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83135B24: 480CE925  bl 0x83204448
	ctx.lr = 0x83135B28;
	sub_83204448(ctx, base);
	// 83135B28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83135B2C: 409A0014  bne cr6, 0x83135b40
	if !ctx.cr[6].eq {
	pc = 0x83135B40; continue 'dispatch;
	}
	// 83135B30: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135B34: 386B1C3C  addi r3, r11, 0x1c3c
	ctx.r[3].s64 = ctx.r[11].s64 + 7228;
	// 83135B38: 4BFFB301  bl 0x83130e38
	ctx.lr = 0x83135B3C;
	sub_83130E38(ctx, base);
	// 83135B3C: 48000008  b 0x83135b44
	pc = 0x83135B44; continue 'dispatch;
	// 83135B40: 9BDF0003  stb r30, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[30].u8 ) };
	// 83135B44: 480CE90D  bl 0x83204450
	ctx.lr = 0x83135B48;
	sub_83204450(ctx, base);
	// 83135B48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83135B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135B54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83135B58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135B60 size=96
    let mut pc: u32 = 0x83135B60;
    'dispatch: loop {
        match pc {
            0x83135B60 => {
    //   block [0x83135B60..0x83135BC0)
	// 83135B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135B68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83135B6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135B78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83135B7C: 480CE8CD  bl 0x83204448
	ctx.lr = 0x83135B80;
	sub_83204448(ctx, base);
	// 83135B80: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 83135B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83135B88: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 83135B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83135B90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83135B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135B98: 4BFFFAC9  bl 0x83135660
	ctx.lr = 0x83135B9C;
	sub_83135660(ctx, base);
	// 83135B9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135BA0: 480CE8B1  bl 0x83204450
	ctx.lr = 0x83135BA4;
	sub_83204450(ctx, base);
	// 83135BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135BA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83135BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135BB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83135BB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135BC0 size=148
    let mut pc: u32 = 0x83135BC0;
    'dispatch: loop {
        match pc {
            0x83135BC0 => {
    //   block [0x83135BC0..0x83135C54)
	// 83135BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135BC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83135BCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135BD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135BD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135BD8: 480CE871  bl 0x83204448
	ctx.lr = 0x83135BDC;
	sub_83204448(ctx, base);
	// 83135BDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83135BE0: 409A0014  bne cr6, 0x83135bf4
	if !ctx.cr[6].eq {
	pc = 0x83135BF4; continue 'dispatch;
	}
	// 83135BE4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135BE8: 386B1C68  addi r3, r11, 0x1c68
	ctx.r[3].s64 = ctx.r[11].s64 + 7272;
	// 83135BEC: 4BFFB24D  bl 0x83130e38
	ctx.lr = 0x83135BF0;
	sub_83130E38(ctx, base);
	// 83135BF0: 48000048  b 0x83135c38
	pc = 0x83135C38; continue 'dispatch;
	// 83135BF4: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83135BF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83135BFC: 419A003C  beq cr6, 0x83135c38
	if ctx.cr[6].eq {
	pc = 0x83135C38; continue 'dispatch;
	}
	// 83135C00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83135C04: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83135C08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83135C0C: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 83135C10: 41820018  beq 0x83135c28
	if ctx.cr[0].eq {
	pc = 0x83135C28; continue 'dispatch;
	}
	// 83135C14: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83135C18: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83135C1C: 409A000C  bne cr6, 0x83135c28
	if !ctx.cr[6].eq {
	pc = 0x83135C28; continue 'dispatch;
	}
	// 83135C20: 4BFFD4E1  bl 0x83133100
	ctx.lr = 0x83135C24;
	sub_83133100(ctx, base);
	// 83135C24: 9BDF0002  stb r30, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 83135C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135C2C: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 83135C30: 4BFFFC79  bl 0x831358a8
	ctx.lr = 0x83135C34;
	sub_831358A8(ctx, base);
	// 83135C34: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 83135C38: 480CE819  bl 0x83204450
	ctx.lr = 0x83135C3C;
	sub_83204450(ctx, base);
	// 83135C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83135C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135C48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83135C4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135C58 size=88
    let mut pc: u32 = 0x83135C58;
    'dispatch: loop {
        match pc {
            0x83135C58 => {
    //   block [0x83135C58..0x83135CB0)
	// 83135C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135C60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135C64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135C6C: 480CE7DD  bl 0x83204448
	ctx.lr = 0x83135C70;
	sub_83204448(ctx, base);
	// 83135C70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83135C74: 419A0024  beq cr6, 0x83135c98
	if ctx.cr[6].eq {
	pc = 0x83135C98; continue 'dispatch;
	}
	// 83135C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135C7C: 4BFFFF45  bl 0x83135bc0
	ctx.lr = 0x83135C80;
	sub_83135BC0(ctx, base);
	// 83135C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83135C84: 38A00238  li r5, 0x238
	ctx.r[5].s64 = 568;
	// 83135C88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83135C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135C90: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83135C94: 4807254D  bl 0x831a81e0
	ctx.lr = 0x83135C98;
	sub_831A81E0(ctx, base);
	// 83135C98: 480CE7B9  bl 0x83204450
	ctx.lr = 0x83135C9C;
	sub_83204450(ctx, base);
	// 83135C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83135CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83135CB0 size=116
    let mut pc: u32 = 0x83135CB0;
    'dispatch: loop {
        match pc {
            0x83135CB0 => {
    //   block [0x83135CB0..0x83135D24)
	// 83135CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135CB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135CBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135CC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83135CC4: 480CE785  bl 0x83204448
	ctx.lr = 0x83135CC8;
	sub_83204448(ctx, base);
	// 83135CC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83135CCC: 409A0014  bne cr6, 0x83135ce0
	if !ctx.cr[6].eq {
	pc = 0x83135CE0; continue 'dispatch;
	}
	// 83135CD0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135CD4: 386B1C94  addi r3, r11, 0x1c94
	ctx.r[3].s64 = ctx.r[11].s64 + 7316;
	// 83135CD8: 4BFFB161  bl 0x83130e38
	ctx.lr = 0x83135CDC;
	sub_83130E38(ctx, base);
	// 83135CDC: 48000030  b 0x83135d0c
	pc = 0x83135D0C; continue 'dispatch;
	// 83135CE0: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83135CE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83135CE8: 419A000C  beq cr6, 0x83135cf4
	if ctx.cr[6].eq {
	pc = 0x83135CF4; continue 'dispatch;
	}
	// 83135CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83135CF0: 4BFFFED1  bl 0x83135bc0
	ctx.lr = 0x83135CF4;
	sub_83135BC0(ctx, base);
	// 83135CF4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83135CF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83135CFC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83135D00: 41990008  bgt cr6, 0x83135d08
	if ctx.cr[6].gt {
	pc = 0x83135D08; continue 'dispatch;
	}
	// 83135D04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83135D08: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83135D0C: 480CE745  bl 0x83204450
	ctx.lr = 0x83135D10;
	sub_83204450(ctx, base);
	// 83135D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83135D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135D1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83135D28 size=236
    let mut pc: u32 = 0x83135D28;
    'dispatch: loop {
        match pc {
            0x83135D28 => {
    //   block [0x83135D28..0x83135E14)
	// 83135D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83135D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83135D30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83135D34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83135D38: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 83135D3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83135D40: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 83135D44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 83135D48: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83135D4C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83135D50: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83135D54: C80BAA10  lfd f0, -0x55f0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22000 as u32) ) };
	// 83135D58: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 83135D5C: EFE0002C  fsqrts f31, f0
	ctx.f[31].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 83135D60: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 83135D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83135D68: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83135D6C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83135D70: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83135D74: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83135D78: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 83135D7C: C00B2490  lfs f0, 0x2490(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83135D80: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83135D84: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 83135D88: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 83135D8C: 4807311D  bl 0x831a8ea8
	ctx.lr = 0x83135D90;
	sub_831A8EA8(ctx, base);
	// 83135D90: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83135D94: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83135D98: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83135D9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83135DA0: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 83135DA4: EDBF6828  fsubs f13, f31, f13
	ctx.f[13].f64 = (((ctx.f[31].f64 - ctx.f[13].f64) as f32) as f64);
	// 83135DA8: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 83135DAC: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 83135DB0: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 83135DB4: ED80602C  fsqrts f12, f12
	ctx.f[12].f64 = ((ctx.f[12].f64).sqrt() as f32) as f64;
	// 83135DB8: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 83135DBC: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 83135DC0: C1AB8AD8  lfs f13, -0x7528(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29992 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83135DC4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83135DC8: ED800032  fmuls f12, f0, f0
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 83135DCC: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 83135DD0: C00B1CC0  lfs f0, 0x1cc0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83135DD4: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 83135DD8: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 83135DDC: D9A10058  stfd f13, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[13].u64 ) };
	// 83135DE0: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83135DE4: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 83135DE8: A161005E  lhz r11, 0x5e(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 83135DEC: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83135DF0: A1610056  lhz r11, 0x56(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 83135DF4: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83135DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83135DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83135E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83135E04: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 83135E08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83135E0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83135E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135E18 size=52
    let mut pc: u32 = 0x83135E18;
    'dispatch: loop {
        match pc {
            0x83135E18 => {
    //   block [0x83135E18..0x83135E4C)
	// 83135E18: 3544FFFF  addic. r10, r4, -1
	ctx.xer.ca = (ctx.r[4].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83135E1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83135E20: 4081001C  ble 0x83135e3c
	if !ctx.cr[0].gt {
	pc = 0x83135E3C; continue 'dispatch;
	}
	// 83135E24: 7D2B1A2E  lhzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83135E28: 2B098000  cmplwi cr6, r9, 0x8000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32768 as u32, &mut ctx.xer);
	// 83135E2C: 419A0020  beq cr6, 0x83135e4c
	if ctx.cr[6].eq {
		sub_83135E4C(ctx, base);
		return;
	}
	// 83135E30: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83135E34: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83135E38: 4198FFEC  blt cr6, 0x83135e24
	if ctx.cr[6].lt {
	pc = 0x83135E24; continue 'dispatch;
	}
	// 83135E3C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83135E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83135E44: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83135E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135E4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135E4C size=24
    let mut pc: u32 = 0x83135E4C;
    'dispatch: loop {
        match pc {
            0x83135E4C => {
    //   block [0x83135E4C..0x83135E64)
	// 83135E4C: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 83135E50: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 83135E54: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83135E58: 4098FFE4  bge cr6, 0x83135e3c
	if !ctx.cr[6].lt {
		sub_83135E18(ctx, base);
		return;
	}
	// 83135E5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83135E60: 4BFFFFE4  b 0x83135e44
	sub_83135E18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135E68 size=16
    let mut pc: u32 = 0x83135E68;
    'dispatch: loop {
        match pc {
            0x83135E68 => {
    //   block [0x83135E68..0x83135E78)
	// 83135E68: 2F040010  cmpwi cr6, r4, 0x10
	ctx.cr[6].compare_i32(ctx.r[4].s32, 16, &mut ctx.xer);
	// 83135E6C: 4098000C  bge cr6, 0x83135e78
	if !ctx.cr[6].lt {
		sub_83135E78(ctx, base);
		return;
	}
	// 83135E70: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83135E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135E78 size=36
    let mut pc: u32 = 0x83135E78;
    'dispatch: loop {
        match pc {
            0x83135E78 => {
    //   block [0x83135E78..0x83135E9C)
	// 83135E78: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135E7C: 88830001  lbz r4, 1(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 83135E80: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83135E84: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 83135E88: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83135E8C: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 83135E90: 419A000C  beq cr6, 0x83135e9c
	if ctx.cr[6].eq {
		sub_83135E9C(ctx, base);
		return;
	}
	// 83135E94: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83135E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135E9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135E9C size=168
    let mut pc: u32 = 0x83135E9C;
    'dispatch: loop {
        match pc {
            0x83135E9C => {
    //   block [0x83135E9C..0x83135F44)
	// 83135E9C: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 83135EA0: 88830003  lbz r4, 3(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 83135EA4: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83135EA8: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 83135EAC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83135EB0: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83135EB4: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83135EB8: 99660000  stb r11, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83135EBC: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 83135EC0: 99680000  stb r11, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83135EC4: 89630006  lbz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 83135EC8: 99670000  stb r11, 0(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83135ECC: 89630007  lbz r11, 7(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 83135ED0: 99690000  stb r11, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83135ED4: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83135ED8: 89230009  lbz r9, 9(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(9 as u32) ) } as u64;
	// 83135EDC: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83135EE0: 88C3000A  lbz r6, 0xa(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 83135EE4: 88A3000B  lbz r5, 0xb(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(11 as u32) ) } as u64;
	// 83135EE8: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 83135EEC: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83135EF0: 7D6B3378  or r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 83135EF4: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83135EF8: 7D6B2B78  or r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[5].u64;
	// 83135EFC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83135F00: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83135F04: 8943000D  lbz r10, 0xd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 83135F08: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83135F0C: 8923000E  lbz r9, 0xe(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 83135F10: 88C3000F  lbz r6, 0xf(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(15 as u32) ) } as u64;
	// 83135F14: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83135F18: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83135F1C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83135F20: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 83135F24: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83135F28: 7D6B3378  or r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[6].u64;
	// 83135F2C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83135F30: 89670000  lbz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135F34: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83135F38: 4082000C  bne 0x83135f44
	if !ctx.cr[0].eq {
		sub_83135F44(ctx, base);
		return;
	}
	// 83135F3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83135F40: 48000018  b 0x83135f58
	sub_83135F44(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135F44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135F44 size=36
    let mut pc: u32 = 0x83135F44;
    'dispatch: loop {
        match pc {
            0x83135F44 => {
    //   block [0x83135F44..0x83135F68)
	// 83135F44: 89480000  lbz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135F48: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83135F4C: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 83135F50: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83135F54: 7D6A5BD6  divw r11, r10, r11
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 83135F58: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83135F5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83135F60: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83135F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135F68 size=16
    let mut pc: u32 = 0x83135F68;
    'dispatch: loop {
        match pc {
            0x83135F68 => {
    //   block [0x83135F68..0x83135F78)
	// 83135F68: 2F040012  cmpwi cr6, r4, 0x12
	ctx.cr[6].compare_i32(ctx.r[4].s32, 18, &mut ctx.xer);
	// 83135F6C: 4098000C  bge cr6, 0x83135f78
	if !ctx.cr[6].lt {
		sub_83135F78(ctx, base);
		return;
	}
	// 83135F70: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83135F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135F78 size=20
    let mut pc: u32 = 0x83135F78;
    'dispatch: loop {
        match pc {
            0x83135F78 => {
    //   block [0x83135F78..0x83135F8C)
	// 83135F78: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135F7C: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 83135F80: 419A000C  beq cr6, 0x83135f8c
	if ctx.cr[6].eq {
		sub_83135F8C(ctx, base);
		return;
	}
	// 83135F84: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83135F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135F8C size=28
    let mut pc: u32 = 0x83135F8C;
    'dispatch: loop {
        match pc {
            0x83135F8C => {
    //   block [0x83135F8C..0x83135FA8)
	// 83135F8C: A9630002  lha r11, 2(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 83135F90: 2F0B000E  cmpwi cr6, r11, 0xe
	ctx.cr[6].compare_i32(ctx.r[11].s32, 14, &mut ctx.xer);
	// 83135F94: 4198FFDC  blt cr6, 0x83135f70
	if ctx.cr[6].lt {
		sub_83135F68(ctx, base);
		return;
	}
	// 83135F98: A1630010  lhz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83135F9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83135FA0: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83135FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135FA8 size=20
    let mut pc: u32 = 0x83135FA8;
    'dispatch: loop {
        match pc {
            0x83135FA8 => {
    //   block [0x83135FA8..0x83135FBC)
	// 83135FA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83135FAC: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 83135FB0: 4098000C  bge cr6, 0x83135fbc
	if !ctx.cr[6].lt {
		sub_83135FBC(ctx, base);
		return;
	}
	// 83135FB4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83135FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135FBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135FBC size=20
    let mut pc: u32 = 0x83135FBC;
    'dispatch: loop {
        match pc {
            0x83135FBC => {
    //   block [0x83135FBC..0x83135FD0)
	// 83135FBC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83135FC0: 2B0A8000  cmplwi cr6, r10, 0x8000
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32768 as u32, &mut ctx.xer);
	// 83135FC4: 419A000C  beq cr6, 0x83135fd0
	if ctx.cr[6].eq {
		sub_83135FD0(ctx, base);
		return;
	}
	// 83135FC8: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83135FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135FD0 size=36
    let mut pc: u32 = 0x83135FD0;
    'dispatch: loop {
        match pc {
            0x83135FD0 => {
    //   block [0x83135FD0..0x83135FF4)
	// 83135FD0: A94B0002  lha r10, 2(r11)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 83135FD4: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 83135FD8: 4198FFDC  blt cr6, 0x83135fb4
	if ctx.cr[6].lt {
		sub_83135FA8(ctx, base);
		return;
	}
	// 83135FDC: 894B0012  lbz r10, 0x12(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 83135FE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83135FE4: 99450000  stb r10, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83135FE8: 896B0013  lbz r11, 0x13(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19 as u32) ) } as u64;
	// 83135FEC: 99660000  stb r11, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83135FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83135FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83135FF8 size=100
    let mut pc: u32 = 0x83135FF8;
    'dispatch: loop {
        match pc {
            0x83135FF8 => {
    //   block [0x83135FF8..0x8313605C)
	// 83135FF8: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 83135FFC: 41980060  blt cr6, 0x8313605c
	if ctx.cr[6].lt {
		sub_8313605C(ctx, base);
		return;
	}
	// 83136000: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136004: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 83136008: 409A0054  bne cr6, 0x8313605c
	if !ctx.cr[6].eq {
		sub_8313605C(ctx, base);
		return;
	}
	// 8313600C: A9430002  lha r10, 2(r3)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 83136010: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 83136014: 41980048  blt cr6, 0x8313605c
	if ctx.cr[6].lt {
		sub_8313605C(ctx, base);
		return;
	}
	// 83136018: 89630012  lbz r11, 0x12(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 8313601C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83136020: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 83136024: 41980040  blt cr6, 0x83136064
	if ctx.cr[6].lt {
		sub_83136064(ctx, base);
		return;
	}
	// 83136028: 2F040020  cmpwi cr6, r4, 0x20
	ctx.cr[6].compare_i32(ctx.r[4].s32, 32, &mut ctx.xer);
	// 8313602C: 41980030  blt cr6, 0x8313605c
	if ctx.cr[6].lt {
		sub_8313605C(ctx, base);
		return;
	}
	// 83136030: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 83136034: 41980028  blt cr6, 0x8313605c
	if ctx.cr[6].lt {
		sub_8313605C(ctx, base);
		return;
	}
	// 83136038: A1630018  lhz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8313603C: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83136040: A163001A  lhz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 83136044: B1660000  sth r11, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83136048: A163001C  lhz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313604C: B1650002  sth r11, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83136050: A163001E  lhz r11, 0x1e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(30 as u32) ) } as u64;
	// 83136054: B1660002  sth r11, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83136058: 48000020  b 0x83136078
	sub_83136064(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8313605C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8313605C size=8
    let mut pc: u32 = 0x8313605C;
    'dispatch: loop {
        match pc {
            0x8313605C => {
    //   block [0x8313605C..0x83136064)
	// 8313605C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83136060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136064(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83136064 size=28
    let mut pc: u32 = 0x83136064;
    'dispatch: loop {
        match pc {
            0x83136064 => {
    //   block [0x83136064..0x83136080)
	// 83136064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83136068: B1660002  sth r11, 2(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 8313606C: B1650002  sth r11, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83136070: B1660000  sth r11, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83136074: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83136078: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8313607C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83136080 size=236
    let mut pc: u32 = 0x83136080;
    'dispatch: loop {
        match pc {
            0x83136080 => {
    //   block [0x83136080..0x8313616C)
	// 83136080: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 83136084: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83136088: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8313608C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83136090: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 83136094: B0660000  sth r3, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83136098: 4198003C  blt cr6, 0x831360d4
	if ctx.cr[6].lt {
	pc = 0x831360D4; continue 'dispatch;
	}
	// 8313609C: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831360A0: 2B038000  cmplwi cr6, r3, 0x8000
	ctx.cr[6].compare_u32(ctx.r[3].u32, 32768 as u32, &mut ctx.xer);
	// 831360A4: 409A0070  bne cr6, 0x83136114
	if !ctx.cr[6].eq {
	pc = 0x83136114; continue 'dispatch;
	}
	// 831360A8: ABCB0002  lha r30, 2(r11)
	ctx.r[30].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 831360AC: 2F1E0010  cmpwi cr6, r30, 0x10
	ctx.cr[6].compare_i32(ctx.r[30].s32, 16, &mut ctx.xer);
	// 831360B0: 41980024  blt cr6, 0x831360d4
	if ctx.cr[6].lt {
	pc = 0x831360D4; continue 'dispatch;
	}
	// 831360B4: 886B0012  lbz r3, 0x12(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 831360B8: 547F063E  clrlwi r31, r3, 0x18
	ctx.r[31].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 831360BC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 831360C0: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 831360C4: 409A0008  bne cr6, 0x831360cc
	if !ctx.cr[6].eq {
	pc = 0x831360CC; continue 'dispatch;
	}
	// 831360C8: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 831360CC: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831360D0: 4098000C  bge cr6, 0x831360dc
	if !ctx.cr[6].lt {
	pc = 0x831360DC; continue 'dispatch;
	}
	// 831360D4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831360D8: 48000088  b 0x83136160
	pc = 0x83136160; continue 'dispatch;
	// 831360DC: 3883FFFC  addi r4, r3, -4
	ctx.r[4].s64 = ctx.r[3].s64 + -4;
	// 831360E0: 7F1E2000  cmpw cr6, r30, r4
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831360E4: 4198FFF0  blt cr6, 0x831360d4
	if ctx.cr[6].lt {
	pc = 0x831360D4; continue 'dispatch;
	}
	// 831360E8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 831360EC: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 831360F0: 409A0008  bne cr6, 0x831360f8
	if !ctx.cr[6].eq {
	pc = 0x831360F8; continue 'dispatch;
	}
	// 831360F4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 831360F8: 7C645AAE  lhax r3, r4, r11
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32)) } as i16) as i64;
	// 831360FC: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 83136100: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83136104: 7CA45A2E  lhzx r5, r4, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83136108: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8313610C: B0A60000  sth r5, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83136110: 419A000C  beq cr6, 0x8313611c
	if ctx.cr[6].eq {
	pc = 0x8313611C; continue 'dispatch;
	}
	// 83136114: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83136118: 48000048  b 0x83136160
	pc = 0x83136160; continue 'dispatch;
	// 8313611C: 38C40004  addi r6, r4, 4
	ctx.r[6].s64 = ctx.r[4].s64 + 4;
	// 83136120: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83136124: 7CA65A2E  lhzx r5, r6, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83136128: 38C60002  addi r6, r6, 2
	ctx.r[6].s64 = ctx.r[6].s64 + 2;
	// 8313612C: B0A70000  sth r5, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83136130: 38E60004  addi r7, r6, 4
	ctx.r[7].s64 = ctx.r[6].s64 + 4;
	// 83136134: 7CA6582E  lwzx r5, r6, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83136138: 90A80000  stw r5, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8313613C: 39070004  addi r8, r7, 4
	ctx.r[8].s64 = ctx.r[7].s64 + 4;
	// 83136140: 7CC7582E  lwzx r6, r7, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83136144: 90C90000  stw r6, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 83136148: 7D285A14  add r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8313614C: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83136150: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83136154: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136158: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8313615C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83136160: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83136164: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83136168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136170 size=308
    let mut pc: u32 = 0x83136170;
    'dispatch: loop {
        match pc {
            0x83136170 => {
    //   block [0x83136170..0x831362A4)
	// 83136170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136174: 48071FF1  bl 0x831a8164
	ctx.lr = 0x83136178;
	sub_831A8130(ctx, base);
	// 83136178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313617C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83136180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83136184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136188: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8313618C: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83136190: 2F040014  cmpwi cr6, r4, 0x14
	ctx.cr[6].compare_i32(ctx.r[4].s32, 20, &mut ctx.xer);
	// 83136194: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83136198: 4198003C  blt cr6, 0x831361d4
	if ctx.cr[6].lt {
	pc = 0x831361D4; continue 'dispatch;
	}
	// 8313619C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831361A0: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 831361A4: 409A00AC  bne cr6, 0x83136250
	if !ctx.cr[6].eq {
	pc = 0x83136250; continue 'dispatch;
	}
	// 831361A8: A93F0002  lha r9, 2(r31)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as i16) as i64;
	// 831361AC: 2F090010  cmpwi cr6, r9, 0x10
	ctx.cr[6].compare_i32(ctx.r[9].s32, 16, &mut ctx.xer);
	// 831361B0: 41980024  blt cr6, 0x831361d4
	if ctx.cr[6].lt {
	pc = 0x831361D4; continue 'dispatch;
	}
	// 831361B4: 897F0012  lbz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 831361B8: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831361BC: 3960003C  li r11, 0x3c
	ctx.r[11].s64 = 60;
	// 831361C0: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 831361C4: 409A0008  bne cr6, 0x831361cc
	if !ctx.cr[6].eq {
	pc = 0x831361CC; continue 'dispatch;
	}
	// 831361C8: 39600048  li r11, 0x48
	ctx.r[11].s64 = 72;
	// 831361CC: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831361D0: 4098000C  bge cr6, 0x831361dc
	if !ctx.cr[6].lt {
	pc = 0x831361DC; continue 'dispatch;
	}
	// 831361D4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831361D8: 480000C4  b 0x8313629c
	pc = 0x8313629C; continue 'dispatch;
	// 831361DC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 831361E0: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831361E4: 4198FFF0  blt cr6, 0x831361d4
	if ctx.cr[6].lt {
	pc = 0x831361D4; continue 'dispatch;
	}
	// 831361E8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 831361EC: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 831361F0: 409A0008  bne cr6, 0x831361f8
	if !ctx.cr[6].eq {
	pc = 0x831361F8; continue 'dispatch;
	}
	// 831361F4: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 831361F8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831361FC: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83136200: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83136204: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83136208: 41820008  beq 0x83136210
	if ctx.cr[0].eq {
	pc = 0x83136210; continue 'dispatch;
	}
	// 8313620C: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 83136210: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83136214: 7D2BF8AE  lbzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83136218: 3D004149  lis r8, 0x4149
	ctx.r[8].s64 = 1095303168;
	// 8313621C: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 83136220: 61084E46  ori r8, r8, 0x4e46
	ctx.r[8].u64 = ctx.r[8].u64 | 20038;
	// 83136224: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83136228: 88EA0001  lbz r7, 1(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8313622C: 88AA0002  lbz r5, 2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 83136230: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 83136234: 894A0003  lbz r10, 3(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 83136238: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8313623C: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 83136240: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83136244: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 83136248: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8313624C: 419A000C  beq cr6, 0x83136258
	if ctx.cr[6].eq {
	pc = 0x83136258; continue 'dispatch;
	}
	// 83136250: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83136254: 48000048  b 0x8313629c
	pc = 0x8313629C; continue 'dispatch;
	// 83136258: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8313625C: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 83136260: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83136264: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83136268: 7C9EFA14  add r4, r30, r31
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8313626C: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83136270: 480722A1  bl 0x831a8510
	ctx.lr = 0x83136274;
	sub_831A8510(ctx, base);
	// 83136274: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 83136278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8313627C: 7D4BFA2E  lhzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83136280: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83136284: B15B0000  sth r10, 0(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83136288: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8313628C: 7D6BFA2E  lhzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83136290: B17D0000  sth r11, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83136294: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 83136298: B17D0002  sth r11, 2(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 8313629C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831362A0: 48071F14  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831362A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831362A8 size=16
    let mut pc: u32 = 0x831362A8;
    'dispatch: loop {
        match pc {
            0x831362A8 => {
    //   block [0x831362A8..0x831362B8)
	// 831362A8: 2F040010  cmpwi cr6, r4, 0x10
	ctx.cr[6].compare_i32(ctx.r[4].s32, 16, &mut ctx.xer);
	// 831362AC: 4098000C  bge cr6, 0x831362b8
	if !ctx.cr[6].lt {
		sub_831362B8(ctx, base);
		return;
	}
	// 831362B0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831362B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831362B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831362B8 size=20
    let mut pc: u32 = 0x831362B8;
    'dispatch: loop {
        match pc {
            0x831362B8 => {
    //   block [0x831362B8..0x831362CC)
	// 831362B8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831362BC: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 831362C0: 419A000C  beq cr6, 0x831362cc
	if ctx.cr[6].eq {
		sub_831362CC(ctx, base);
		return;
	}
	// 831362C4: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 831362C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831362CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831362CC size=20
    let mut pc: u32 = 0x831362CC;
    'dispatch: loop {
        match pc {
            0x831362CC => {
    //   block [0x831362CC..0x831362E0)
	// 831362CC: A1630002  lhz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 831362D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831362D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831362D8: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831362DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831362E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831362E0 size=372
    let mut pc: u32 = 0x831362E0;
    'dispatch: loop {
        match pc {
            0x831362E0 => {
    //   block [0x831362E0..0x83136454)
	// 831362E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831362E4: 48071E7D  bl 0x831a8160
	ctx.lr = 0x831362E8;
	sub_831A8130(ctx, base);
	// 831362E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831362EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831362F0: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831362F4: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831362F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831362FC: 4BFFB695  bl 0x83131990
	ctx.lr = 0x83136300;
	sub_83131990(ctx, base);
	// 83136300: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83136304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136308: 4BFFB691  bl 0x83131998
	ctx.lr = 0x8313630C;
	sub_83131998(ctx, base);
	// 8313630C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83136310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136314: 4BFFB69D  bl 0x831319b0
	ctx.lr = 0x83136318;
	sub_831319B0(ctx, base);
	// 83136318: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8313631C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83136320: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83136324: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83136328: 419A000C  beq cr6, 0x83136334
	if ctx.cr[6].eq {
	pc = 0x83136334; continue 'dispatch;
	}
	// 8313632C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83136330: 409A0020  bne cr6, 0x83136350
	if !ctx.cr[6].eq {
	pc = 0x83136350; continue 'dispatch;
	}
	// 83136334: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83136338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8313633C: 409A0014  bne cr6, 0x83136350
	if !ctx.cr[6].eq {
	pc = 0x83136350; continue 'dispatch;
	}
	// 83136340: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83136344: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136348: 4BFFB601  bl 0x83131948
	ctx.lr = 0x8313634C;
	sub_83131948(ctx, base);
	// 8313634C: 48000100  b 0x8313644c
	pc = 0x8313644C; continue 'dispatch;
	// 83136350: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136354: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83136358: 80BF0050  lwz r5, 0x50(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8313635C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136360: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83136364: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83136368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313636C: 4E800421  bctrl
	ctx.lr = 0x83136370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136370: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83136374: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83136378: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8313637C: 40980010  bge cr6, 0x8313638c
	if !ctx.cr[6].lt {
	pc = 0x8313638C; continue 'dispatch;
	}
	// 83136380: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83136384: 386B1CCC  addi r3, r11, 0x1ccc
	ctx.r[3].s64 = ctx.r[11].s64 + 7372;
	// 83136388: 4BFFA651  bl 0x831309d8
	ctx.lr = 0x8313638C;
	sub_831309D8(ctx, base);
	// 8313638C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136390: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83136394: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136398: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8313639C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831363A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831363A4: 4E800421  bctrl
	ctx.lr = 0x831363A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831363A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831363AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831363B0: 4BEAB921  bl 0x82fe1cd0
	ctx.lr = 0x831363B4;
	sub_82FE1CD0(ctx, base);
	// 831363B4: 7C9CD050  subf r4, r28, r26
	ctx.r[4].s64 = ctx.r[26].s64 - ctx.r[28].s64;
	// 831363B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831363BC: 909F0090  stw r4, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 831363C0: 4BFFB589  bl 0x83131948
	ctx.lr = 0x831363C4;
	sub_83131948(ctx, base);
	// 831363C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831363C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831363CC: 4BFFB585  bl 0x83131950
	ctx.lr = 0x831363D0;
	sub_83131950(ctx, base);
	// 831363D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831363D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831363D8: 4BD20299  bl 0x82e56670
	ctx.lr = 0x831363DC;
	sub_82E56670(ctx, base);
	// 831363DC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 831363E0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 831363E4: 409A0054  bne cr6, 0x83136438
	if !ctx.cr[6].eq {
	pc = 0x83136438; continue 'dispatch;
	}
	// 831363E8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831363EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831363F0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831363F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831363F8: 4E800421  bctrl
	ctx.lr = 0x831363FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831363FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136400: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83136404: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83136408: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8313640C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83136410: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83136414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136418: 4E800421  bctrl
	ctx.lr = 0x8313641C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313641C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136420: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83136424: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136428: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8313642C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83136430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136434: 4E800421  bctrl
	ctx.lr = 0x83136438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136438: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8313643C: 4BFFB65D  bl 0x83131a98
	ctx.lr = 0x83136440;
	sub_83131A98(ctx, base);
	// 83136440: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83136444: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83136448: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8313644C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83136450: 48071D60  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83136458 size=168
    let mut pc: u32 = 0x83136458;
    'dispatch: loop {
        match pc {
            0x83136458 => {
    //   block [0x83136458..0x83136500)
	// 83136458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313645C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83136460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83136464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83136468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313646C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136470: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136474: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136478: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8313647C: 4182006C  beq 0x831364e8
	if ctx.cr[0].eq {
	pc = 0x831364E8; continue 'dispatch;
	}
	// 83136480: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83136484: 419A0064  beq cr6, 0x831364e8
	if ctx.cr[6].eq {
	pc = 0x831364E8; continue 'dispatch;
	}
	// 83136488: 4BFFB511  bl 0x83131998
	ctx.lr = 0x8313648C;
	sub_83131998(ctx, base);
	// 8313648C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83136490: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 83136494: 419A0030  beq cr6, 0x831364c4
	if ctx.cr[6].eq {
	pc = 0x831364C4; continue 'dispatch;
	}
	// 83136498: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8313649C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831364A0: 409A0038  bne cr6, 0x831364d8
	if !ctx.cr[6].eq {
	pc = 0x831364D8; continue 'dispatch;
	}
	// 831364A4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831364A8: 4BE9DEF1  bl 0x82fd4398
	ctx.lr = 0x831364AC;
	sub_82FD4398(ctx, base);
	// 831364AC: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 831364B0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831364B4: 41980010  blt cr6, 0x831364c4
	if ctx.cr[6].lt {
	pc = 0x831364C4; continue 'dispatch;
	}
	// 831364B8: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 831364BC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831364C0: 4BFFB489  bl 0x83131948
	ctx.lr = 0x831364C4;
	sub_83131948(ctx, base);
	// 831364C4: 3C807FFF  lis r4, 0x7fff
	ctx.r[4].s64 = 2147418112;
	// 831364C8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831364CC: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 831364D0: 4BFFC2D1  bl 0x831327a0
	ctx.lr = 0x831364D4;
	sub_831327A0(ctx, base);
	// 831364D4: 48000014  b 0x831364e8
	pc = 0x831364E8; continue 'dispatch;
	// 831364D8: 7C6B5E70  srawi r11, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 831364DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831364E0: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 831364E4: 4BFFC1E5  bl 0x831326c8
	ctx.lr = 0x831364E8;
	sub_831326C8(ctx, base);
	// 831364E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831364EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831364F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831364F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831364F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831364FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136500 size=88
    let mut pc: u32 = 0x83136500;
    'dispatch: loop {
        match pc {
            0x83136500 => {
    //   block [0x83136500..0x83136558)
	// 83136500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83136508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313650C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136514: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136518: 4BFFB451  bl 0x83131968
	ctx.lr = 0x8313651C;
	sub_83131968(ctx, base);
	// 8313651C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83136520: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136528: A8BF0042  lha r5, 0x42(r31)
	ctx.r[5].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(66 as u32) ) } as i16) as i64;
	// 8313652C: 419A0014  beq cr6, 0x83136540
	if ctx.cr[6].eq {
	pc = 0x83136540; continue 'dispatch;
	}
	// 83136530: 4BFF7379  bl 0x8312d8a8
	ctx.lr = 0x83136534;
	sub_8312D8A8(ctx, base);
	// 83136534: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313653C: A8BF0044  lha r5, 0x44(r31)
	ctx.r[5].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as i16) as i64;
	// 83136540: 4BFF7369  bl 0x8312d8a8
	ctx.lr = 0x83136544;
	sub_8312D8A8(ctx, base);
	// 83136544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83136548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313654C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83136550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83136554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136558 size=772
    let mut pc: u32 = 0x83136558;
    'dispatch: loop {
        match pc {
            0x83136558 => {
    //   block [0x83136558..0x8313685C)
	// 83136558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313655C: 48071C05  bl 0x831a8160
	ctx.lr = 0x83136560;
	sub_831A8130(ctx, base);
	// 83136560: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136564: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83136568: 897A0098  lbz r11, 0x98(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(152 as u32) ) } as u64;
	// 8313656C: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136570: 83FA0014  lwz r31, 0x14(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 83136574: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83136578: 419A02DC  beq cr6, 0x83136854
	if ctx.cr[6].eq {
	pc = 0x83136854; continue 'dispatch;
	}
	// 8313657C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83136580: B1610052  sth r11, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 83136584: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 83136588: 4BF91559  bl 0x830c7ae0
	ctx.lr = 0x8313658C;
	sub_830C7AE0(ctx, base);
	// 8313658C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136590: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83136594: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83136598: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8313659C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831365A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831365A4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831365A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831365AC: 4E800421  bctrl
	ctx.lr = 0x831365B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831365B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831365B4: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 831365B8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 831365BC: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 831365C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831365C4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831365C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831365CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831365D0: 4E800421  bctrl
	ctx.lr = 0x831365D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831365D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831365D8: 4BFFB381  bl 0x83131958
	ctx.lr = 0x831365DC;
	sub_83131958(ctx, base);
	// 831365DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831365E0: 40820068  bne 0x83136648
	if !ctx.cr[0].eq {
	pc = 0x83136648; continue 'dispatch;
	}
	// 831365E4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831365E8: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 831365EC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831365F0: 4BFFFCB9  bl 0x831362a8
	ctx.lr = 0x831365F4;
	sub_831362A8(ctx, base);
	// 831365F4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831365F8: 41820050  beq 0x83136648
	if ctx.cr[0].eq {
	pc = 0x83136648; continue 'dispatch;
	}
	// 831365FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136600: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83136604: 4BFF77D5  bl 0x8312ddd8
	ctx.lr = 0x83136608;
	sub_8312DDD8(ctx, base);
	// 83136608: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313660C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83136610: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136618: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313661C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136620: 4E800421  bctrl
	ctx.lr = 0x83136624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136624: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136628: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8313662C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136634: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83136638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313663C: 4E800421  bctrl
	ctx.lr = 0x83136640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136640: 4BF914A1  bl 0x830c7ae0
	ctx.lr = 0x83136644;
	sub_830C7AE0(ctx, base);
	// 83136644: 48000210  b 0x83136854
	pc = 0x83136854; continue 'dispatch;
	// 83136648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8313664C: ABA10050  lha r29, 0x50(r1)
	ctx.r[29].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i16) as i64;
	// 83136650: 4BFFB309  bl 0x83131958
	ctx.lr = 0x83136654;
	sub_83131958(ctx, base);
	// 83136654: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83136658: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8313665C: 7C9D5850  subf r4, r29, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83136660: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83136664: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83136668: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8313666C: 409A0024  bne cr6, 0x83136690
	if !ctx.cr[6].eq {
	pc = 0x83136690; continue 'dispatch;
	}
	// 83136670: 48002F99  bl 0x83139608
	ctx.lr = 0x83136674;
	sub_83139608(ctx, base);
	// 83136674: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83136678: 41820024  beq 0x8313669c
	if ctx.cr[0].eq {
	pc = 0x8313669C; continue 'dispatch;
	}
	// 8313667C: 38A10052  addi r5, r1, 0x52
	ctx.r[5].s64 = ctx.r[1].s64 + 82;
	// 83136680: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83136684: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83136688: 48002F81  bl 0x83139608
	ctx.lr = 0x8313668C;
	sub_83139608(ctx, base);
	// 8313668C: 48000028  b 0x831366b4
	pc = 0x831366B4; continue 'dispatch;
	// 83136690: 4BFFF789  bl 0x83135e18
	ctx.lr = 0x83136694;
	sub_83135E18(ctx, base);
	// 83136694: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83136698: 4082000C  bne 0x831366a4
	if !ctx.cr[0].eq {
	pc = 0x831366A4; continue 'dispatch;
	}
	// 8313669C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831366A0: 48000014  b 0x831366b4
	pc = 0x831366B4; continue 'dispatch;
	// 831366A4: 38A10052  addi r5, r1, 0x52
	ctx.r[5].s64 = ctx.r[1].s64 + 82;
	// 831366A8: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 831366AC: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 831366B0: 4BFFF769  bl 0x83135e18
	ctx.lr = 0x831366B4;
	sub_83135E18(ctx, base);
	// 831366B4: A9610050  lha r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i16) as i64;
	// 831366B8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831366BC: AB610052  lha r27, 0x52(r1)
	ctx.r[27].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as i16) as i64;
	// 831366C0: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 831366C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831366C8: 419A0050  beq cr6, 0x83136718
	if ctx.cr[6].eq {
	pc = 0x83136718; continue 'dispatch;
	}
	// 831366CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831366D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831366D4: 419A0094  beq cr6, 0x83136768
	if ctx.cr[6].eq {
	pc = 0x83136768; continue 'dispatch;
	}
	// 831366D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831366DC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 831366E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831366E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831366E8: 4E800421  bctrl
	ctx.lr = 0x831366EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831366EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831366F0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831366F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831366F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831366FC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83136700: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136704: 4E800421  bctrl
	ctx.lr = 0x83136708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136708: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313670C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83136710: 4BFF76C9  bl 0x8312ddd8
	ctx.lr = 0x83136714;
	sub_8312DDD8(ctx, base);
	// 83136714: 4BFFFF2C  b 0x83136640
	pc = 0x83136640; continue 'dispatch;
	// 83136718: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313671C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83136720: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313672C: 4E800421  bctrl
	ctx.lr = 0x83136730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136730: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 83136734: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83136738: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8313673C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 83136740: 4BFFEA51  bl 0x83135190
	ctx.lr = 0x83136744;
	sub_83135190(ctx, base);
	// 83136744: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136748: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8313674C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136754: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83136758: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313675C: 4E800421  bctrl
	ctx.lr = 0x83136760;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136760: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 83136764: 4800004C  b 0x831367b0
	pc = 0x831367B0; continue 'dispatch;
	// 83136768: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8313676C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83136770: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136778: 4E800421  bctrl
	ctx.lr = 0x8313677C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313677C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 83136780: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83136784: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83136788: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8313678C: 4BFFEA05  bl 0x83135190
	ctx.lr = 0x83136790;
	sub_83135190(ctx, base);
	// 83136790: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136794: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83136798: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313679C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831367A0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831367A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831367A8: 4E800421  bctrl
	ctx.lr = 0x831367AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831367AC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 831367B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831367B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831367B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831367BC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831367C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831367C4: 4E800421  bctrl
	ctx.lr = 0x831367C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831367C8: 4BF91319  bl 0x830c7ae0
	ctx.lr = 0x831367CC;
	sub_830C7AE0(ctx, base);
	// 831367CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831367D0: 4BED27E9  bl 0x83008fb8
	ctx.lr = 0x831367D4;
	sub_83008FB8(ctx, base);
	// 831367D4: 815A00A4  lwz r10, 0xa4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(164 as u32) ) } as u64;
	// 831367D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831367DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831367E0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831367E4: 917A00A4  stw r11, 0xa4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 831367E8: 4BFFAAF9  bl 0x831312e0
	ctx.lr = 0x831367EC;
	sub_831312E0(ctx, base);
	// 831367EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831367F0: 4BFFAAA9  bl 0x83131298
	ctx.lr = 0x831367F4;
	sub_83131298(ctx, base);
	// 831367F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831367F8: 4BFFBA41  bl 0x83132238
	ctx.lr = 0x831367FC;
	sub_83132238(ctx, base);
	// 831367FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136800: 480CDD51  bl 0x83204550
	ctx.lr = 0x83136804;
	sub_83204550(ctx, base);
	// 83136804: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83136808: 419A0014  beq cr6, 0x8313681c
	if ctx.cr[6].eq {
	pc = 0x8313681C; continue 'dispatch;
	}
	// 8313680C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136810: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83136814: 4BFF75C5  bl 0x8312ddd8
	ctx.lr = 0x83136818;
	sub_8312DDD8(ctx, base);
	// 83136818: 4800003C  b 0x83136854
	pc = 0x83136854; continue 'dispatch;
	// 8313681C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136820: 809A0048  lwz r4, 0x48(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(72 as u32) ) } as u64;
	// 83136824: 4BFFAA5D  bl 0x83131280
	ctx.lr = 0x83136828;
	sub_83131280(ctx, base);
	// 83136828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8313682C: 4BFFB155  bl 0x83131980
	ctx.lr = 0x83136830;
	sub_83131980(ctx, base);
	// 83136830: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83136834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136838: 4BFFB111  bl 0x83131948
	ctx.lr = 0x8313683C;
	sub_83131948(ctx, base);
	// 8313683C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136840: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136844: 4BFFB10D  bl 0x83131950
	ctx.lr = 0x83136848;
	sub_83131950(ctx, base);
	// 83136848: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313684C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136850: 4BEAB481  bl 0x82fe1cd0
	ctx.lr = 0x83136854;
	sub_82FE1CD0(ctx, base);
	// 83136854: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83136858: 48071958  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136860 size=328
    let mut pc: u32 = 0x83136860;
    'dispatch: loop {
        match pc {
            0x83136860 => {
    //   block [0x83136860..0x831369A8)
	// 83136860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136864: 48071901  bl 0x831a8164
	ctx.lr = 0x83136868;
	sub_831A8130(ctx, base);
	// 83136868: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313686C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136870: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136874: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136878: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8313687C: 4BFF7F3D  bl 0x8312e7b8
	ctx.lr = 0x83136880;
	sub_8312E7B8(ctx, base);
	// 83136880: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83136884: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136888: 4BFF7F69  bl 0x8312e7f0
	ctx.lr = 0x8313688C;
	sub_8312E7F0(ctx, base);
	// 8313688C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83136890: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83136894: 2F0B2000  cmpwi cr6, r11, 0x2000
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8192, &mut ctx.xer);
	// 83136898: 41980008  blt cr6, 0x831368a0
	if ctx.cr[6].lt {
	pc = 0x831368A0; continue 'dispatch;
	}
	// 8313689C: 39602000  li r11, 0x2000
	ctx.r[11].s64 = 8192;
	// 831368A0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831368A4: 40980024  bge cr6, 0x831368c8
	if !ctx.cr[6].lt {
	pc = 0x831368C8; continue 'dispatch;
	}
	// 831368A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831368AC: 4BFFB0CD  bl 0x83131978
	ctx.lr = 0x831368B0;
	sub_83131978(ctx, base);
	// 831368B0: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 831368B4: 40990014  ble cr6, 0x831368c8
	if !ctx.cr[6].gt {
	pc = 0x831368C8; continue 'dispatch;
	}
	// 831368B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831368BC: 480CDC95  bl 0x83204550
	ctx.lr = 0x831368C0;
	sub_83204550(ctx, base);
	// 831368C0: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 831368C4: 409A004C  bne cr6, 0x83136910
	if !ctx.cr[6].eq {
	pc = 0x83136910; continue 'dispatch;
	}
	// 831368C8: 897F0070  lbz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 831368CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831368D0: 409A0038  bne cr6, 0x83136908
	if !ctx.cr[6].eq {
	pc = 0x83136908; continue 'dispatch;
	}
	// 831368D4: 897F0072  lbz r11, 0x72(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(114 as u32) ) } as u64;
	// 831368D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831368DC: 409A0024  bne cr6, 0x83136900
	if !ctx.cr[6].eq {
	pc = 0x83136900; continue 'dispatch;
	}
	// 831368E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831368E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831368E8: 4BFF7E61  bl 0x8312e748
	ctx.lr = 0x831368EC;
	sub_8312E748(ctx, base);
	// 831368EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831368F0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831368F4: 915F009C  stw r10, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 831368F8: 816B757C  lwz r11, 0x757c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30076 as u32) ) } as u64;
	// 831368FC: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 83136900: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83136904: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83136908: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8313690C: 997F0071  stb r11, 0x71(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(113 as u32), ctx.r[11].u8 ) };
	// 83136910: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136914: 480CDC3D  bl 0x83204550
	ctx.lr = 0x83136918;
	sub_83204550(ctx, base);
	// 83136918: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8313691C: 409A0084  bne cr6, 0x831369a0
	if !ctx.cr[6].eq {
	pc = 0x831369A0; continue 'dispatch;
	}
	// 83136920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136924: 4BFF6F0D  bl 0x8312d830
	ctx.lr = 0x83136928;
	sub_8312D830(ctx, base);
	// 83136928: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8313692C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83136930: 7D6B19D6  mullw r11, r11, r3
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[3].s32 as i64);
	// 83136934: 557C083C  slwi r28, r11, 1
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83136938: 40810068  ble 0x831369a0
	if !ctx.cr[0].gt {
	pc = 0x831369A0; continue 'dispatch;
	}
	// 8313693C: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 83136940: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83136944: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136948: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8313694C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83136950: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136958: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313695C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83136960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136964: 4E800421  bctrl
	ctx.lr = 0x83136968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136968: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313696C: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83136970: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83136974: 4807186D  bl 0x831a81e0
	ctx.lr = 0x83136978;
	sub_831A81E0(ctx, base);
	// 83136978: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313697C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83136980: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136988: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8313698C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136990: 4E800421  bctrl
	ctx.lr = 0x83136994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136994: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83136998: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8313699C: 4082FFA8  bne 0x83136944
	if !ctx.cr[0].eq {
	pc = 0x83136944; continue 'dispatch;
	}
	// 831369A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831369A4: 48071810  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831369A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831369A8 size=224
    let mut pc: u32 = 0x831369A8;
    'dispatch: loop {
        match pc {
            0x831369A8 => {
    //   block [0x831369A8..0x83136A88)
	// 831369A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831369AC: 480717B9  bl 0x831a8164
	ctx.lr = 0x831369B0;
	sub_831A8130(ctx, base);
	// 831369B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831369B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831369B8: 897D006C  lbz r11, 0x6c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 831369BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831369C0: 409A0030  bne cr6, 0x831369f0
	if !ctx.cr[6].eq {
	pc = 0x831369F0; continue 'dispatch;
	}
	// 831369C4: 817D00C0  lwz r11, 0xc0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 831369C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831369CC: 40990024  ble cr6, 0x831369f0
	if !ctx.cr[6].gt {
	pc = 0x831369F0; continue 'dispatch;
	}
	// 831369D0: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831369D4: 4BE9D9C5  bl 0x82fd4398
	ctx.lr = 0x831369D8;
	sub_82FD4398(ctx, base);
	// 831369D8: 817D00C0  lwz r11, 0xc0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 831369DC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831369E0: 41980010  blt cr6, 0x831369f0
	if ctx.cr[6].lt {
	pc = 0x831369F0; continue 'dispatch;
	}
	// 831369E4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 831369E8: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831369EC: 4BFFAF5D  bl 0x83131948
	ctx.lr = 0x831369F0;
	sub_83131948(ctx, base);
	// 831369F0: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831369F4: 480CDB5D  bl 0x83204550
	ctx.lr = 0x831369F8;
	sub_83204550(ctx, base);
	// 831369F8: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 831369FC: 409A0084  bne cr6, 0x83136a80
	if !ctx.cr[6].eq {
	pc = 0x83136A80; continue 'dispatch;
	}
	// 83136A00: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136A04: 4BFFAF65  bl 0x83131968
	ctx.lr = 0x83136A08;
	sub_83131968(ctx, base);
	// 83136A08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83136A0C: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83136A10: 93CB16A4  stw r30, 0x16a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5796 as u32), ctx.r[30].u32 ) };
	// 83136A14: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136A18: 4BFF8199  bl 0x8312ebb0
	ctx.lr = 0x83136A1C;
	sub_8312EBB0(ctx, base);
	// 83136A1C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83136A20: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83136A24: 40990040  ble cr6, 0x83136a64
	if !ctx.cr[6].gt {
	pc = 0x83136A64; continue 'dispatch;
	}
	// 83136A28: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 83136A2C: 3F608343  lis r27, -0x7cbd
	ctx.r[27].s64 = -2092761088;
	// 83136A30: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136A34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136A38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136A3C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83136A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136A44: 4E800421  bctrl
	ctx.lr = 0x83136A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136A48: 907B16A0  stw r3, 0x16a0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(5792 as u32), ctx.r[3].u32 ) };
	// 83136A4C: 2F030040  cmpwi cr6, r3, 0x40
	ctx.cr[6].compare_i32(ctx.r[3].s32, 64, &mut ctx.xer);
	// 83136A50: 40980014  bge cr6, 0x83136a64
	if !ctx.cr[6].lt {
	pc = 0x83136A64; continue 'dispatch;
	}
	// 83136A54: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83136A58: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83136A5C: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83136A60: 4198FFD0  blt cr6, 0x83136a30
	if ctx.cr[6].lt {
	pc = 0x83136A30; continue 'dispatch;
	}
	// 83136A64: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83136A68: 409A0018  bne cr6, 0x83136a80
	if !ctx.cr[6].eq {
	pc = 0x83136A80; continue 'dispatch;
	}
	// 83136A6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136A70: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136A74: 4BFF7CB5  bl 0x8312e728
	ctx.lr = 0x83136A78;
	sub_8312E728(ctx, base);
	// 83136A78: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83136A7C: 997D0001  stb r11, 1(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83136A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83136A84: 48071730  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136A88 size=80
    let mut pc: u32 = 0x83136A88;
    'dispatch: loop {
        match pc {
            0x83136A88 => {
    //   block [0x83136A88..0x83136AD8)
	// 83136A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83136A90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83136A94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136A98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136A9C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136AA0: 4BFF7D19  bl 0x8312e7b8
	ctx.lr = 0x83136AA4;
	sub_8312E7B8(ctx, base);
	// 83136AA4: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83136AA8: 906B16A8  stw r3, 0x16a8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5800 as u32), ctx.r[3].u32 ) };
	// 83136AAC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136AB0: 4BFF7D51  bl 0x8312e800
	ctx.lr = 0x83136AB4;
	sub_8312E800(ctx, base);
	// 83136AB4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83136AB8: 419A000C  beq cr6, 0x83136ac4
	if ctx.cr[6].eq {
	pc = 0x83136AC4; continue 'dispatch;
	}
	// 83136ABC: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83136AC0: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83136AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83136AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83136ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83136AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83136AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136AD8 size=96
    let mut pc: u32 = 0x83136AD8;
    'dispatch: loop {
        match pc {
            0x83136AD8 => {
    //   block [0x83136AD8..0x83136B38)
	// 83136AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136ADC: 48071691  bl 0x831a816c
	ctx.lr = 0x83136AE0;
	sub_831A8130(ctx, base);
	// 83136AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136AE8: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 83136AEC: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 83136AF0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136AF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83136AF8: 41820018  beq 0x83136b10
	if ctx.cr[0].eq {
	pc = 0x83136B10; continue 'dispatch;
	}
	// 83136AFC: 4BFFBB8D  bl 0x83132688
	ctx.lr = 0x83136B00;
	sub_83132688(ctx, base);
	// 83136B00: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83136B04: 409A000C  bne cr6, 0x83136b10
	if !ctx.cr[6].eq {
	pc = 0x83136B10; continue 'dispatch;
	}
	// 83136B08: B3BF0060  sth r29, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u16 ) };
	// 83136B0C: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 83136B10: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83136B14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83136B18: 41820018  beq 0x83136b30
	if ctx.cr[0].eq {
	pc = 0x83136B30; continue 'dispatch;
	}
	// 83136B1C: 4BFFEEBD  bl 0x831359d8
	ctx.lr = 0x83136B20;
	sub_831359D8(ctx, base);
	// 83136B20: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83136B24: 409A000C  bne cr6, 0x83136b30
	if !ctx.cr[6].eq {
	pc = 0x83136B30; continue 'dispatch;
	}
	// 83136B28: B3BF0060  sth r29, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u16 ) };
	// 83136B2C: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 83136B30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83136B34: 48071688  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136B38 size=104
    let mut pc: u32 = 0x83136B38;
    'dispatch: loop {
        match pc {
            0x83136B38 => {
    //   block [0x83136B38..0x83136BA0)
	// 83136B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83136B40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83136B44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136B48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136B4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136B50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83136B54: 419A0038  beq cr6, 0x83136b8c
	if ctx.cr[6].eq {
	pc = 0x83136B8C; continue 'dispatch;
	}
	// 83136B58: 4BFF6B59  bl 0x8312d6b0
	ctx.lr = 0x83136B5C;
	sub_8312D6B0(ctx, base);
	// 83136B5C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83136B60: 4182002C  beq 0x83136b8c
	if ctx.cr[0].eq {
	pc = 0x83136B8C; continue 'dispatch;
	}
	// 83136B64: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83136B68: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83136B6C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83136B70: 40980010  bge cr6, 0x83136b80
	if !ctx.cr[6].lt {
	pc = 0x83136B80; continue 'dispatch;
	}
	// 83136B74: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136B78: 4BFFBB11  bl 0x83132688
	ctx.lr = 0x83136B7C;
	sub_83132688(ctx, base);
	// 83136B7C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83136B80: 409A000C  bne cr6, 0x83136b8c
	if !ctx.cr[6].eq {
	pc = 0x83136B8C; continue 'dispatch;
	}
	// 83136B84: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136B88: 4BFFA709  bl 0x83131290
	ctx.lr = 0x83136B8C;
	sub_83131290(ctx, base);
	// 83136B8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83136B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83136B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83136B98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83136B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136BA0 size=144
    let mut pc: u32 = 0x83136BA0;
    'dispatch: loop {
        match pc {
            0x83136BA0 => {
    //   block [0x83136BA0..0x83136C30)
	// 83136BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136BA4: 480715C1  bl 0x831a8164
	ctx.lr = 0x83136BA8;
	sub_831A8130(ctx, base);
	// 83136BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136BAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83136BB0: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136BB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136BB8: 4BFFADD9  bl 0x83131990
	ctx.lr = 0x83136BBC;
	sub_83131990(ctx, base);
	// 83136BBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83136BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136BC4: 4BFFADD5  bl 0x83131998
	ctx.lr = 0x83136BC8;
	sub_83131998(ctx, base);
	// 83136BC8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83136BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136BD0: 4BFFADE1  bl 0x831319b0
	ctx.lr = 0x83136BD4;
	sub_831319B0(ctx, base);
	// 83136BD4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83136BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136BDC: 4BFFAEB5  bl 0x83131a90
	ctx.lr = 0x83136BE0;
	sub_83131A90(ctx, base);
	// 83136BE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136BE8: 4BEAB0E9  bl 0x82fe1cd0
	ctx.lr = 0x83136BEC;
	sub_82FE1CD0(ctx, base);
	// 83136BEC: 7C9DD850  subf r4, r29, r27
	ctx.r[4].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 83136BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136BF4: 909E0090  stw r4, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 83136BF8: 4BFFAD51  bl 0x83131948
	ctx.lr = 0x83136BFC;
	sub_83131948(ctx, base);
	// 83136BFC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83136C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136C04: 4BFFAD4D  bl 0x83131950
	ctx.lr = 0x83136C08;
	sub_83131950(ctx, base);
	// 83136C08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83136C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136C10: 4BD1FA61  bl 0x82e56670
	ctx.lr = 0x83136C14;
	sub_82E56670(ctx, base);
	// 83136C14: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 83136C18: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83136C1C: 388B62E0  addi r4, r11, 0x62e0
	ctx.r[4].s64 = ctx.r[11].s64 + 25312;
	// 83136C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136C24: 4BFFAD15  bl 0x83131938
	ctx.lr = 0x83136C28;
	sub_83131938(ctx, base);
	// 83136C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83136C2C: 48071588  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83136C30 size=932
    let mut pc: u32 = 0x83136C30;
    'dispatch: loop {
        match pc {
            0x83136C30 => {
    //   block [0x83136C30..0x83136FD4)
	// 83136C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136C34: 48071531  bl 0x831a8164
	ctx.lr = 0x83136C38;
	sub_831A8130(ctx, base);
	// 83136C38: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136C3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136C40: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83136C44: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83136C48: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83136C4C: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 83136C50: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 83136C54: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83136C58: 4182000C  beq 0x83136c64
	if ctx.cr[0].eq {
	pc = 0x83136C64; continue 'dispatch;
	}
	// 83136C5C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83136C60: 409A0058  bne cr6, 0x83136cb8
	if !ctx.cr[6].eq {
	pc = 0x83136CB8; continue 'dispatch;
	}
	// 83136C64: 897F00A8  lbz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 83136C68: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83136C6C: 409A004C  bne cr6, 0x83136cb8
	if !ctx.cr[6].eq {
	pc = 0x83136CB8; continue 'dispatch;
	}
	// 83136C70: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136C74: 4BFFBA15  bl 0x83132688
	ctx.lr = 0x83136C78;
	sub_83132688(ctx, base);
	// 83136C78: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83136C7C: 419A0350  beq cr6, 0x83136fcc
	if ctx.cr[6].eq {
	pc = 0x83136FCC; continue 'dispatch;
	}
	// 83136C80: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83136C84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83136C88: 41820014  beq 0x83136c9c
	if ctx.cr[0].eq {
	pc = 0x83136C9C; continue 'dispatch;
	}
	// 83136C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83136C90: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83136C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136C98: 4E800421  bctrl
	ctx.lr = 0x83136C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136C9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136CA0: 80FF00BC  lwz r7, 0xbc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 83136CA4: 80DF00B8  lwz r6, 0xb8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 83136CA8: 80BF00B4  lwz r5, 0xb4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83136CAC: 809F00B0  lwz r4, 0xb0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 83136CB0: 4BFF7649  bl 0x8312e2f8
	ctx.lr = 0x83136CB4;
	sub_8312E2F8(ctx, base);
	// 83136CB4: 9B7F00A8  stb r27, 0xa8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[27].u8 ) };
	// 83136CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136CBC: 480CD895  bl 0x83204550
	ctx.lr = 0x83136CC0;
	sub_83204550(ctx, base);
	// 83136CC0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83136CC4: 409A02F8  bne cr6, 0x83136fbc
	if !ctx.cr[6].eq {
	pc = 0x83136FBC; continue 'dispatch;
	}
	// 83136CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136CCC: 4BFFAC9D  bl 0x83131968
	ctx.lr = 0x83136CD0;
	sub_83131968(ctx, base);
	// 83136CD0: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83136CD4: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 83136CD8: 7F032000  cmpw cr6, r3, r4
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83136CDC: 4099002C  ble cr6, 0x83136d08
	if !ctx.cr[6].gt {
	pc = 0x83136D08; continue 'dispatch;
	}
	// 83136CE0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83136CE4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83136CE8: 4BFF9ED1  bl 0x83130bb8
	ctx.lr = 0x83136CEC;
	sub_83130BB8(ctx, base);
	// 83136CEC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83136CF0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83136CF4: 386B1CF8  addi r3, r11, 0x1cf8
	ctx.r[3].s64 = ctx.r[11].s64 + 7416;
	// 83136CF8: 4BFF9D69  bl 0x83130a60
	ctx.lr = 0x83136CFC;
	sub_83130A60(ctx, base);
	// 83136CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136D00: 4BFF6979  bl 0x8312d678
	ctx.lr = 0x83136D04;
	sub_8312D678(ctx, base);
	// 83136D04: 480002C8  b 0x83136fcc
	pc = 0x83136FCC; continue 'dispatch;
	// 83136D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136D0C: 4BFFAC55  bl 0x83131960
	ctx.lr = 0x83136D10;
	sub_83131960(ctx, base);
	// 83136D10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83136D14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136D18: 4BFFAC71  bl 0x83131988
	ctx.lr = 0x83136D1C;
	sub_83131988(ctx, base);
	// 83136D1C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83136D20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136D24: 4BFFAC35  bl 0x83131958
	ctx.lr = 0x83136D28;
	sub_83131958(ctx, base);
	// 83136D28: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83136D2C: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 83136D30: 7D7D5BD6  divw r11, r29, r11
	ctx.r[11].s32 = ctx.r[29].s32 / ctx.r[11].s32;
	// 83136D34: 409A0018  bne cr6, 0x83136d4c
	if !ctx.cr[6].eq {
	pc = 0x83136D4C; continue 'dispatch;
	}
	// 83136D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136D3C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83136D40: 4BFFAC39  bl 0x83131978
	ctx.lr = 0x83136D44;
	sub_83131978(ctx, base);
	// 83136D44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83136D48: 48000028  b 0x83136d70
	pc = 0x83136D70; continue 'dispatch;
	// 83136D4C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83136D50: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 83136D54: 4199000C  bgt cr6, 0x83136d60
	if ctx.cr[6].gt {
	pc = 0x83136D60; continue 'dispatch;
	}
	// 83136D58: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83136D5C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83136D60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136D64: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83136D68: 4BFFAC11  bl 0x83131978
	ctx.lr = 0x83136D6C;
	sub_83131978(ctx, base);
	// 83136D6C: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83136D70: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83136D74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136D78: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83136D7C: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 83136D80: 7C8A59D6  mullw r4, r10, r11
	ctx.r[4].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83136D84: 909F0048  stw r4, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 83136D88: 4BFFA4F9  bl 0x83131280
	ctx.lr = 0x83136D8C;
	sub_83131280(ctx, base);
	// 83136D8C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83136D90: 409900BC  ble cr6, 0x83136e4c
	if !ctx.cr[6].gt {
	pc = 0x83136E4C; continue 'dispatch;
	}
	// 83136D94: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83136D98: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83136D9C: 409A000C  bne cr6, 0x83136da8
	if !ctx.cr[6].eq {
	pc = 0x83136DA8; continue 'dispatch;
	}
	// 83136DA0: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 83136DA4: 48000064  b 0x83136e08
	pc = 0x83136E08; continue 'dispatch;
	// 83136DA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136DAC: 4BFFAC0D  bl 0x831319b8
	ctx.lr = 0x83136DB0;
	sub_831319B8(ctx, base);
	// 83136DB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83136DB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136DB8: 7D6A5E70  srawi r10, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 83136DBC: 392B07FF  addi r9, r11, 0x7ff
	ctx.r[9].s64 = ctx.r[11].s64 + 2047;
	// 83136DC0: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83136DC4: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83136DC8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83136DCC: 216B0800  subfic r11, r11, 0x800
	ctx.xer.ca = ctx.r[11].u32 <= 2048 as u32;
	ctx.r[11].s64 = (2048 as i64) - ctx.r[11].s64;
	// 83136DD0: 7D6A5E70  srawi r10, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 83136DD4: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83136DD8: 7D295E70  srawi r9, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 83136DDC: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83136DE0: 7C890194  addze r4, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[4].s64 = tmp.s64;
	// 83136DE4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83136DE8: 909F008C  stw r4, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83136DEC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83136DF0: 4BFFB9B1  bl 0x831327a0
	ctx.lr = 0x83136DF4;
	sub_831327A0(ctx, base);
	// 83136DF4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 83136DF8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136DFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83136E00: 388B6458  addi r4, r11, 0x6458
	ctx.r[4].s64 = ctx.r[11].s64 + 25688;
	// 83136E04: 4BFFB96D  bl 0x83132770
	ctx.lr = 0x83136E08;
	sub_83132770(ctx, base);
	// 83136E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E0C: 4BFFABA5  bl 0x831319b0
	ctx.lr = 0x83136E10;
	sub_831319B0(ctx, base);
	// 83136E10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E14: 4BFFAB7D  bl 0x83131990
	ctx.lr = 0x83136E18;
	sub_83131990(ctx, base);
	// 83136E18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83136E1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E20: 909F0090  stw r4, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 83136E24: 4BFFAB25  bl 0x83131948
	ctx.lr = 0x83136E28;
	sub_83131948(ctx, base);
	// 83136E28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136E2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E30: 4BFFAB21  bl 0x83131950
	ctx.lr = 0x83136E34;
	sub_83131950(ctx, base);
	// 83136E34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136E38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E3C: 4BEAAE95  bl 0x82fe1cd0
	ctx.lr = 0x83136E40;
	sub_82FE1CD0(ctx, base);
	// 83136E40: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 83136E44: 388B6BA0  addi r4, r11, 0x6ba0
	ctx.r[4].s64 = ctx.r[11].s64 + 27552;
	// 83136E48: 48000050  b 0x83136e98
	pc = 0x83136E98; continue 'dispatch;
	// 83136E4C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83136E50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83136E54: 41820010  beq 0x83136e64
	if ctx.cr[0].eq {
	pc = 0x83136E64; continue 'dispatch;
	}
	// 83136E58: 3C807FFF  lis r4, 0x7fff
	ctx.r[4].s64 = 2147418112;
	// 83136E5C: 6084FFFF  ori r4, r4, 0xffff
	ctx.r[4].u64 = ctx.r[4].u64 | 65535;
	// 83136E60: 4BFFB941  bl 0x831327a0
	ctx.lr = 0x83136E64;
	sub_831327A0(ctx, base);
	// 83136E64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E68: 4BFFAB19  bl 0x83131980
	ctx.lr = 0x83136E6C;
	sub_83131980(ctx, base);
	// 83136E6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83136E70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E74: 4BFFAAD5  bl 0x83131948
	ctx.lr = 0x83136E78;
	sub_83131948(ctx, base);
	// 83136E78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E80: 4BFFAAD1  bl 0x83131950
	ctx.lr = 0x83136E84;
	sub_83131950(ctx, base);
	// 83136E84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83136E88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136E8C: 4BEAAE45  bl 0x82fe1cd0
	ctx.lr = 0x83136E90;
	sub_82FE1CD0(ctx, base);
	// 83136E90: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 83136E94: 388B6558  addi r4, r11, 0x6558
	ctx.r[4].s64 = ctx.r[11].s64 + 25944;
	// 83136E98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83136E9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136EA0: 4BFFAA99  bl 0x83131938
	ctx.lr = 0x83136EA4;
	sub_83131938(ctx, base);
	// 83136EA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136EA8: 4BFFAAB9  bl 0x83131960
	ctx.lr = 0x83136EAC;
	sub_83131960(ctx, base);
	// 83136EAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83136EB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136EB4: 4BFFAAB5  bl 0x83131968
	ctx.lr = 0x83136EB8;
	sub_83131968(ctx, base);
	// 83136EB8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83136EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136EC0: 4BFFAAC1  bl 0x83131980
	ctx.lr = 0x83136EC4;
	sub_83131980(ctx, base);
	// 83136EC4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83136EC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136ECC: 4BFFAAA5  bl 0x83131970
	ctx.lr = 0x83136ED0;
	sub_83131970(ctx, base);
	// 83136ED0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83136ED4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136ED8: 4BF90C09  bl 0x830c7ae0
	ctx.lr = 0x83136EDC;
	sub_830C7AE0(ctx, base);
	// 83136EDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83136EE0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136EE4: 4BFF79E5  bl 0x8312e8c8
	ctx.lr = 0x83136EE8;
	sub_8312E8C8(ctx, base);
	// 83136EE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83136EEC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136EF0: 4BFF79C1  bl 0x8312e8b0
	ctx.lr = 0x83136EF4;
	sub_8312E8B0(ctx, base);
	// 83136EF4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83136EF8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136EFC: 4BF90BE5  bl 0x830c7ae0
	ctx.lr = 0x83136F00;
	sub_830C7AE0(ctx, base);
	// 83136F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136F04: A89F0040  lha r4, 0x40(r31)
	ctx.r[4].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as i16) as i64;
	// 83136F08: 4BFF6A41  bl 0x8312d948
	ctx.lr = 0x83136F0C;
	sub_8312D948(ctx, base);
	// 83136F0C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83136F10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83136F14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136F18: 4BFF6DF9  bl 0x8312dd10
	ctx.lr = 0x83136F1C;
	sub_8312DD10(ctx, base);
	// 83136F1C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83136F20: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83136F24: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83136F28: 409A000C  bne cr6, 0x83136f34
	if !ctx.cr[6].eq {
	pc = 0x83136F34; continue 'dispatch;
	}
	// 83136F2C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83136F30: 419A000C  beq cr6, 0x83136f3c
	if ctx.cr[6].eq {
	pc = 0x83136F3C; continue 'dispatch;
	}
	// 83136F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136F38: 4BFF6D89  bl 0x8312dcc0
	ctx.lr = 0x83136F3C;
	sub_8312DCC0(ctx, base);
	// 83136F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136F40: 4BFFF5C1  bl 0x83136500
	ctx.lr = 0x83136F44;
	sub_83136500(ctx, base);
	// 83136F44: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83136F48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83136F4C: 4182000C  beq 0x83136f58
	if ctx.cr[0].eq {
	pc = 0x83136F58; continue 'dispatch;
	}
	// 83136F50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83136F54: 4BFFE5ED  bl 0x83135540
	ctx.lr = 0x83136F58;
	sub_83135540(ctx, base);
	// 83136F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136F5C: 4BFFA9FD  bl 0x83131958
	ctx.lr = 0x83136F60;
	sub_83131958(ctx, base);
	// 83136F60: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83136F64: 409A0018  bne cr6, 0x83136f7c
	if !ctx.cr[6].eq {
	pc = 0x83136F7C; continue 'dispatch;
	}
	// 83136F68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83136F6C: 4BCDC08D  bl 0x82e12ff8
	ctx.lr = 0x83136F70;
	sub_82E12FF8(ctx, base);
	// 83136F70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83136F74: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136F78: 48047511  bl 0x8317e488
	ctx.lr = 0x83136F7C;
	sub_8317E488(ctx, base);
	// 83136F7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83136F80: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83136F84: 4BFF77A5  bl 0x8312e728
	ctx.lr = 0x83136F88;
	sub_8312E728(ctx, base);
	// 83136F88: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83136F8C: 814B7E44  lwz r10, 0x7e44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32324 as u32) ) } as u64;
	// 83136F90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83136F94: 419A0020  beq cr6, 0x83136fb4
	if ctx.cr[6].eq {
	pc = 0x83136FB4; continue 'dispatch;
	}
	// 83136F98: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83136F9C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83136FA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83136FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83136FA8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83136FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83136FB0: 4E800421  bctrl
	ctx.lr = 0x83136FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83136FB4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83136FB8: 48000010  b 0x83136fc8
	pc = 0x83136FC8; continue 'dispatch;
	// 83136FBC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83136FC0: 409A000C  bne cr6, 0x83136fcc
	if !ctx.cr[6].eq {
	pc = 0x83136FCC; continue 'dispatch;
	}
	// 83136FC4: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 83136FC8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83136FCC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83136FD0: 480711E4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83136FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83136FD8 size=172
    let mut pc: u32 = 0x83136FD8;
    'dispatch: loop {
        match pc {
            0x83136FD8 => {
    //   block [0x83136FD8..0x83137084)
	// 83136FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83136FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83136FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83136FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83136FE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83136FEC: 4BFEFDBD  bl 0x83126da8
	ctx.lr = 0x83136FF0;
	sub_83126DA8(ctx, base);
	// 83136FF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83136FF4: 409A0014  bne cr6, 0x83137008
	if !ctx.cr[6].eq {
	pc = 0x83137008; continue 'dispatch;
	}
	// 83136FF8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83136FFC: 386B1D38  addi r3, r11, 0x1d38
	ctx.r[3].s64 = ctx.r[11].s64 + 7480;
	// 83137000: 4BFF99D9  bl 0x831309d8
	ctx.lr = 0x83137004;
	sub_831309D8(ctx, base);
	// 83137004: 48000068  b 0x8313706c
	pc = 0x8313706C; continue 'dispatch;
	// 83137008: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8313700C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83137010: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83137014: 409A0010  bne cr6, 0x83137024
	if !ctx.cr[6].eq {
	pc = 0x83137024; continue 'dispatch;
	}
	// 83137018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313701C: 4BFFF98D  bl 0x831369a8
	ctx.lr = 0x83137020;
	sub_831369A8(ctx, base);
	// 83137020: 4800003C  b 0x8313705c
	pc = 0x8313705C; continue 'dispatch;
	// 83137024: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83137028: 409A0010  bne cr6, 0x83137038
	if !ctx.cr[6].eq {
	pc = 0x83137038; continue 'dispatch;
	}
	// 8313702C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137030: 4BFFFC01  bl 0x83136c30
	ctx.lr = 0x83137034;
	sub_83136C30(ctx, base);
	// 83137034: 48000028  b 0x8313705c
	pc = 0x8313705C; continue 'dispatch;
	// 83137038: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8313703C: 409A0010  bne cr6, 0x8313704c
	if !ctx.cr[6].eq {
	pc = 0x8313704C; continue 'dispatch;
	}
	// 83137040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137044: 4BFFF81D  bl 0x83136860
	ctx.lr = 0x83137048;
	sub_83136860(ctx, base);
	// 83137048: 48000014  b 0x8313705c
	pc = 0x8313705C; continue 'dispatch;
	// 8313704C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83137050: 409A000C  bne cr6, 0x8313705c
	if !ctx.cr[6].eq {
	pc = 0x8313705C; continue 'dispatch;
	}
	// 83137054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137058: 4BFFFA31  bl 0x83136a88
	ctx.lr = 0x8313705C;
	sub_83136A88(ctx, base);
	// 8313705C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137060: 4BFFFAD9  bl 0x83136b38
	ctx.lr = 0x83137064;
	sub_83136B38(ctx, base);
	// 83137064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137068: 4BFFFA71  bl 0x83136ad8
	ctx.lr = 0x8313706C;
	sub_83136AD8(ctx, base);
	// 8313706C: 4BFEFD7D  bl 0x83126de8
	ctx.lr = 0x83137070;
	sub_83126DE8(ctx, base);
	// 83137070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313707C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83137088 size=132
    let mut pc: u32 = 0x83137088;
    'dispatch: loop {
        match pc {
            0x83137088 => {
    //   block [0x83137088..0x8313710C)
	// 83137088: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8313708C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83137090: 3D208344  lis r9, -0x7cbc
	ctx.r[9].s64 = -2092695552;
	// 83137094: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 83137098: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8313709C: 390AD2A0  addi r8, r10, -0x2d60
	ctx.r[8].s64 = ctx.r[10].s64 + -11616;
	// 831370A0: 8169D3A0  lwz r11, -0x2c60(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-11360 as u32) ) } as u64;
	// 831370A4: 7D7F2670  srawi r31, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 831370A8: 7D5F0194  addze r10, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[10].s64 = tmp.s64;
	// 831370AC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831370B0: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831370B4: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831370B8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 831370BC: 3D008344  lis r8, -0x7cbc
	ctx.r[8].s64 = -2092695552;
	// 831370C0: 3908D3C0  addi r8, r8, -0x2c40
	ctx.r[8].s64 = ctx.r[8].s64 + -11328;
	// 831370C4: 409A0014  bne cr6, 0x831370d8
	if !ctx.cr[6].eq {
	pc = 0x831370D8; continue 'dispatch;
	}
	// 831370C8: 547F083C  slwi r31, r3, 1
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 831370CC: 7FDF422E  lhzx r30, r31, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831370D0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831370D4: 7FDF432E  sthx r30, r31, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[30].u16) };
	// 831370D8: 547F083C  slwi r31, r3, 1
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 831370DC: 986B0000  stb r3, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 831370E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831370E4: 988B0001  stb r4, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[4].u8 ) };
	// 831370E8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 831370EC: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 831370F0: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 831370F4: 7D1F422E  lhzx r8, r31, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831370F8: 9149D3A0  stw r10, -0x2c60(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-11360 as u32), ctx.r[10].u32 ) };
	// 831370FC: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 83137100: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137104: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83137108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137110 size=120
    let mut pc: u32 = 0x83137110;
    'dispatch: loop {
        match pc {
            0x83137110 => {
    //   block [0x83137110..0x83137188)
	// 83137110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313711C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137124: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137128: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8313712C: 419A0048  beq cr6, 0x83137174
	if ctx.cr[6].eq {
	pc = 0x83137174; continue 'dispatch;
	}
	// 83137130: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83137134: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83137138: 409A003C  bne cr6, 0x83137174
	if !ctx.cr[6].eq {
	pc = 0x83137174; continue 'dispatch;
	}
	// 8313713C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83137140: 816BD3A4  lwz r11, -0x2c5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11356 as u32) ) } as u64;
	// 83137144: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83137148: 409A0010  bne cr6, 0x83137158
	if !ctx.cr[6].eq {
	pc = 0x83137158; continue 'dispatch;
	}
	// 8313714C: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83137150: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83137154: 4BF9098D  bl 0x830c7ae0
	ctx.lr = 0x83137158;
	sub_830C7AE0(ctx, base);
	// 83137158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8313715C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137160: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83137164: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83137168: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8313716C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83137170: 4E800421  bctrl
	ctx.lr = 0x83137174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83137174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313717C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137188 size=240
    let mut pc: u32 = 0x83137188;
    'dispatch: loop {
        match pc {
            0x83137188 => {
    //   block [0x83137188..0x83137278)
	// 83137188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313718C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313719C: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 831371A0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 831371A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831371A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831371AC: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 831371B0: 4BFFFED9  bl 0x83137088
	ctx.lr = 0x831371B4;
	sub_83137088(ctx, base);
	// 831371B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831371B8: 409A0018  bne cr6, 0x831371d0
	if !ctx.cr[6].eq {
	pc = 0x831371D0; continue 'dispatch;
	}
	// 831371BC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831371C0: 386B1DB8  addi r3, r11, 0x1db8
	ctx.r[3].s64 = ctx.r[11].s64 + 7608;
	// 831371C4: 4BFF9815  bl 0x831309d8
	ctx.lr = 0x831371C8;
	sub_831309D8(ctx, base);
	// 831371C8: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 831371CC: 48000098  b 0x83137264
	pc = 0x83137264; continue 'dispatch;
	// 831371D0: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 831371D4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831371D8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831371DC: 419A0084  beq cr6, 0x83137260
	if ctx.cr[6].eq {
	pc = 0x83137260; continue 'dispatch;
	}
	// 831371E0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 831371E4: 409A0010  bne cr6, 0x831371f4
	if !ctx.cr[6].eq {
	pc = 0x831371F4; continue 'dispatch;
	}
	// 831371E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831371EC: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 831371F0: 48000070  b 0x83137260
	pc = 0x83137260; continue 'dispatch;
	// 831371F4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831371F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831371FC: 40820018  bne 0x83137214
	if !ctx.cr[0].eq {
	pc = 0x83137214; continue 'dispatch;
	}
	// 83137200: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83137204: 386B1D8C  addi r3, r11, 0x1d8c
	ctx.r[3].s64 = ctx.r[11].s64 + 7564;
	// 83137208: 4BFF97D1  bl 0x831309d8
	ctx.lr = 0x8313720C;
	sub_831309D8(ctx, base);
	// 8313720C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83137210: 48000054  b 0x83137264
	pc = 0x83137264; continue 'dispatch;
	// 83137214: 4BFFBEED  bl 0x83133100
	ctx.lr = 0x83137218;
	sub_83133100(ctx, base);
	// 83137218: 4BF908C9  bl 0x830c7ae0
	ctx.lr = 0x8313721C;
	sub_830C7AE0(ctx, base);
	// 8313721C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137220: 4BFFB501  bl 0x83132720
	ctx.lr = 0x83137224;
	sub_83132720(ctx, base);
	// 83137224: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83137228: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8313722C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137230: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83137234: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83137238: 4BFFFED9  bl 0x83137110
	ctx.lr = 0x8313723C;
	sub_83137110(ctx, base);
	// 8313723C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83137240: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83137244: 4BF9089D  bl 0x830c7ae0
	ctx.lr = 0x83137248;
	sub_830C7AE0(ctx, base);
	// 83137248: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8313724C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83137250: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83137254: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83137258: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 8313725C: 4BFFFE2D  bl 0x83137088
	ctx.lr = 0x83137260;
	sub_83137088(ctx, base);
	// 83137260: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83137264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313726C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137278 size=320
    let mut pc: u32 = 0x83137278;
    'dispatch: loop {
        match pc {
            0x83137278 => {
    //   block [0x83137278..0x831373B8)
	// 83137278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83137284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313728C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137290: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137294: 4BFFB3F5  bl 0x83132688
	ctx.lr = 0x83137298;
	sub_83132688(ctx, base);
	// 83137298: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8313729C: 409A0014  bne cr6, 0x831372b0
	if !ctx.cr[6].eq {
	pc = 0x831372B0; continue 'dispatch;
	}
	// 831372A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831372A4: 4BFFFE6D  bl 0x83137110
	ctx.lr = 0x831372A8;
	sub_83137110(ctx, base);
	// 831372A8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 831372AC: 480000F0  b 0x8313739c
	pc = 0x8313739C; continue 'dispatch;
	// 831372B0: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 831372B4: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 831372B8: 409A009C  bne cr6, 0x83137354
	if !ctx.cr[6].eq {
	pc = 0x83137354; continue 'dispatch;
	}
	// 831372BC: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831372C0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831372C4: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831372C8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831372CC: 409A003C  bne cr6, 0x83137308
	if !ctx.cr[6].eq {
	pc = 0x83137308; continue 'dispatch;
	}
	// 831372D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831372D4: 4BFFB105  bl 0x831323d8
	ctx.lr = 0x831372D8;
	sub_831323D8(ctx, base);
	// 831372D8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831372DC: 4182002C  beq 0x83137308
	if ctx.cr[0].eq {
	pc = 0x83137308; continue 'dispatch;
	}
	// 831372E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831372E4: 4BFFBBBD  bl 0x83132ea0
	ctx.lr = 0x831372E8;
	sub_83132EA0(ctx, base);
	// 831372E8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831372EC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831372F0: 41990018  bgt cr6, 0x83137308
	if ctx.cr[6].gt {
	pc = 0x83137308; continue 'dispatch;
	}
	// 831372F4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 831372F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831372FC: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83137300: 4BFFFE11  bl 0x83137110
	ctx.lr = 0x83137304;
	sub_83137110(ctx, base);
	// 83137304: 4800009C  b 0x831373a0
	pc = 0x831373A0; continue 'dispatch;
	// 83137308: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313730C: 4BFFB37D  bl 0x83132688
	ctx.lr = 0x83137310;
	sub_83132688(ctx, base);
	// 83137310: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83137314: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83137318: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313731C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83137320: 4BFFB401  bl 0x83132720
	ctx.lr = 0x83137324;
	sub_83132720(ctx, base);
	// 83137324: 895F0001  lbz r10, 1(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83137328: 7D7E1850  subf r11, r30, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 8313732C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83137330: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 83137334: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83137338: 419A000C  beq cr6, 0x83137344
	if ctx.cr[6].eq {
	pc = 0x83137344; continue 'dispatch;
	}
	// 8313733C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83137340: 409A0014  bne cr6, 0x83137354
	if !ctx.cr[6].eq {
	pc = 0x83137354; continue 'dispatch;
	}
	// 83137344: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83137348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313734C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83137350: 4BFFFDC1  bl 0x83137110
	ctx.lr = 0x83137354;
	sub_83137110(ctx, base);
	// 83137354: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83137358: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8313735C: 409A0044  bne cr6, 0x831373a0
	if !ctx.cr[6].eq {
	pc = 0x831373A0; continue 'dispatch;
	}
	// 83137360: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137364: 4BFFB325  bl 0x83132688
	ctx.lr = 0x83137368;
	sub_83132688(ctx, base);
	// 83137368: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8313736C: 409A0034  bne cr6, 0x831373a0
	if !ctx.cr[6].eq {
	pc = 0x831373A0; continue 'dispatch;
	}
	// 83137370: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137374: 4BFFB3AD  bl 0x83132720
	ctx.lr = 0x83137378;
	sub_83132720(ctx, base);
	// 83137378: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8313737C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83137380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137384: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83137388: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8313738C: 4BFFFD85  bl 0x83137110
	ctx.lr = 0x83137390;
	sub_83137110(ctx, base);
	// 83137390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83137394: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83137398: 995F0003  stb r10, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 8313739C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 831373A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831373A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831373A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831373AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831373B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831373B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831373B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831373B8 size=120
    let mut pc: u32 = 0x831373B8;
    'dispatch: loop {
        match pc {
            0x831373B8 => {
    //   block [0x831373B8..0x83137430)
	// 831373B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831373BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831373C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831373C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831373C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831373CC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831373D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831373D4: 3BCB1D64  addi r30, r11, 0x1d64
	ctx.r[30].s64 = ctx.r[11].s64 + 7524;
	// 831373D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831373DC: 409A0010  bne cr6, 0x831373ec
	if !ctx.cr[6].eq {
	pc = 0x831373EC; continue 'dispatch;
	}
	// 831373E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831373E4: 4BFF95F5  bl 0x831309d8
	ctx.lr = 0x831373E8;
	sub_831309D8(ctx, base);
	// 831373E8: 48000014  b 0x831373fc
	pc = 0x831373FC; continue 'dispatch;
	// 831373EC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831373F0: 4BFFAFE9  bl 0x831323d8
	ctx.lr = 0x831373F4;
	sub_831323D8(ctx, base);
	// 831373F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831373F8: 419A0020  beq cr6, 0x83137418
	if ctx.cr[6].eq {
	pc = 0x83137418; continue 'dispatch;
	}
	// 831373FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137400: 4BFFB019  bl 0x83132418
	ctx.lr = 0x83137404;
	sub_83132418(ctx, base);
	// 83137404: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83137408: 41820010  beq 0x83137418
	if ctx.cr[0].eq {
	pc = 0x83137418; continue 'dispatch;
	}
	// 8313740C: 4BFF98FD  bl 0x83130d08
	ctx.lr = 0x83137410;
	sub_83130D08(ctx, base);
	// 83137410: 4BFF66F1  bl 0x8312db00
	ctx.lr = 0x83137414;
	sub_8312DB00(ctx, base);
	// 83137414: 4BFFFFC4  b 0x831373d8
	pc = 0x831373D8; continue 'dispatch;
	// 83137418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313741C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83137428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313742C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137430 size=116
    let mut pc: u32 = 0x83137430;
    'dispatch: loop {
        match pc {
            0x83137430 => {
    //   block [0x83137430..0x831374A4)
	// 83137430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313743C: 2B0300FF  cmplwi cr6, r3, 0xff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 255 as u32, &mut ctx.xer);
	// 83137440: 41990044  bgt cr6, 0x83137484
	if ctx.cr[6].gt {
	pc = 0x83137484; continue 'dispatch;
	}
	// 83137444: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83137448: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8313744C: 396BD400  addi r11, r11, -0x2c00
	ctx.r[11].s64 = ctx.r[11].s64 + -11264;
	// 83137450: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83137454: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83137458: 4182002C  beq 0x83137484
	if ctx.cr[0].eq {
	pc = 0x83137484; continue 'dispatch;
	}
	// 8313745C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83137460: 41980018  blt cr6, 0x83137478
	if ctx.cr[6].lt {
	pc = 0x83137478; continue 'dispatch;
	}
	// 83137464: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137468: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8313746C: 4098000C  bge cr6, 0x83137478
	if !ctx.cr[6].lt {
	pc = 0x83137478; continue 'dispatch;
	}
	// 83137470: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83137474: 48000020  b 0x83137494
	pc = 0x83137494; continue 'dispatch;
	// 83137478: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313747C: 386B1E00  addi r3, r11, 0x1e00
	ctx.r[3].s64 = ctx.r[11].s64 + 7680;
	// 83137480: 4800000C  b 0x8313748c
	pc = 0x8313748C; continue 'dispatch;
	// 83137484: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83137488: 386B1DDC  addi r3, r11, 0x1ddc
	ctx.r[3].s64 = ctx.r[11].s64 + 7644;
	// 8313748C: 4BFF954D  bl 0x831309d8
	ctx.lr = 0x83137490;
	sub_831309D8(ctx, base);
	// 83137490: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 83137494: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313749C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831374A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831374A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831374A8 size=188
    let mut pc: u32 = 0x831374A8;
    'dispatch: loop {
        match pc {
            0x831374A8 => {
    //   block [0x831374A8..0x83137564)
	// 831374A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831374AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831374B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831374B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831374B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831374BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831374C0: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 831374C4: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 831374C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831374CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831374D0: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 831374D4: 4BFFFBB5  bl 0x83137088
	ctx.lr = 0x831374D8;
	sub_83137088(ctx, base);
	// 831374D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831374DC: 419A0070  beq cr6, 0x8313754c
	if ctx.cr[6].eq {
	pc = 0x8313754C; continue 'dispatch;
	}
	// 831374E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831374E4: 4BFFFED5  bl 0x831373b8
	ctx.lr = 0x831374E8;
	sub_831373B8(ctx, base);
	// 831374E8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 831374EC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 831374F0: 409A000C  bne cr6, 0x831374fc
	if !ctx.cr[6].eq {
	pc = 0x831374FC; continue 'dispatch;
	}
	// 831374F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831374F8: 4BFFFC91  bl 0x83137188
	ctx.lr = 0x831374FC;
	sub_83137188(ctx, base);
	// 831374FC: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137500: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83137504: 41820020  beq 0x83137524
	if ctx.cr[0].eq {
	pc = 0x83137524; continue 'dispatch;
	}
	// 83137508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8313750C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83137510: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83137514: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83137518: 4BFFBCA9  bl 0x831331c0
	ctx.lr = 0x8313751C;
	sub_831331C0(ctx, base);
	// 8313751C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83137520: 4BFFBD11  bl 0x83133230
	ctx.lr = 0x83137524;
	sub_83133230(ctx, base);
	// 83137524: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 83137528: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313752C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137530: 48070CB1  bl 0x831a81e0
	ctx.lr = 0x83137534;
	sub_831A81E0(ctx, base);
	// 83137534: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 83137538: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8313753C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83137540: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83137544: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83137548: 4BFFFB41  bl 0x83137088
	ctx.lr = 0x8313754C;
	sub_83137088(ctx, base);
	// 8313754C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83137550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137558: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8313755C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137568 size=108
    let mut pc: u32 = 0x83137568;
    'dispatch: loop {
        match pc {
            0x83137568 => {
    //   block [0x83137568..0x831375D4)
	// 83137568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313756C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137570: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83137574: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313757C: 4BFEF82D  bl 0x83126da8
	ctx.lr = 0x83137580;
	sub_83126DA8(ctx, base);
	// 83137580: 4BF90561  bl 0x830c7ae0
	ctx.lr = 0x83137584;
	sub_830C7AE0(ctx, base);
	// 83137584: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83137588: 3BCBD800  addi r30, r11, -0x2800
	ctx.r[30].s64 = ctx.r[11].s64 + -10240;
	// 8313758C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83137590: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83137594: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83137598: 409A000C  bne cr6, 0x831375a4
	if !ctx.cr[6].eq {
	pc = 0x831375A4; continue 'dispatch;
	}
	// 8313759C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831375A0: 4BFFFCD9  bl 0x83137278
	ctx.lr = 0x831375A4;
	sub_83137278(ctx, base);
	// 831375A4: 3BFF0034  addi r31, r31, 0x34
	ctx.r[31].s64 = ctx.r[31].s64 + 52;
	// 831375A8: 397E0340  addi r11, r30, 0x340
	ctx.r[11].s64 = ctx.r[30].s64 + 832;
	// 831375AC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831375B0: 4198FFE0  blt cr6, 0x83137590
	if ctx.cr[6].lt {
	pc = 0x83137590; continue 'dispatch;
	}
	// 831375B4: 4BF9052D  bl 0x830c7ae0
	ctx.lr = 0x831375B8;
	sub_830C7AE0(ctx, base);
	// 831375B8: 4BFEF831  bl 0x83126de8
	ctx.lr = 0x831375BC;
	sub_83126DE8(ctx, base);
	// 831375BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831375C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831375C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831375C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831375CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831375D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831375D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831375D8 size=420
    let mut pc: u32 = 0x831375D8;
    'dispatch: loop {
        match pc {
            0x831375D8 => {
    //   block [0x831375D8..0x8313777C)
	// 831375D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831375DC: 48070B79  bl 0x831a8154
	ctx.lr = 0x831375E0;
	sub_831A8130(ctx, base);
	// 831375E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831375E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831375E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831375EC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 831375F0: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 831375F4: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 831375F8: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 831375FC: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 83137600: 4BFFFE31  bl 0x83137430
	ctx.lr = 0x83137604;
	sub_83137430(ctx, base);
	// 83137604: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83137608: 4080002C  bge 0x83137634
	if !ctx.cr[0].lt {
	pc = 0x83137634; continue 'dispatch;
	}
	// 8313760C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83137610: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83137614: 419A0008  beq cr6, 0x8313761c
	if ctx.cr[6].eq {
	pc = 0x8313761C; continue 'dispatch;
	}
	// 83137618: 99590000  stb r10, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8313761C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83137620: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83137624: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137628: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8313762C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137630: 48000140  b 0x83137770
	pc = 0x83137770; continue 'dispatch;
	// 83137634: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83137638: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8313763C: 396BD400  addi r11, r11, -0x2c00
	ctx.r[11].s64 = ctx.r[11].s64 + -11264;
	// 83137640: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83137644: 897D000F  lbz r11, 0xf(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(15 as u32) ) } as u64;
	// 83137648: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8313764C: 409A00AC  bne cr6, 0x831376f8
	if !ctx.cr[6].eq {
	pc = 0x831376F8; continue 'dispatch;
	}
	// 83137650: 395D0118  addi r10, r29, 0x118
	ctx.r[10].s64 = ctx.r[29].s64 + 280;
	// 83137654: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83137658: 7D695E70  srawi r9, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8313765C: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83137660: 7D685E70  srawi r8, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 83137664: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83137668: 55085828  slwi r8, r8, 0xb
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(11);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8313766C: 7D685851  subf. r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83137670: 40810008  ble 0x83137678
	if !ctx.cr[0].gt {
	pc = 0x83137678; continue 'dispatch;
	}
	// 83137674: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83137678: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 8313767C: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 83137680: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83137684: 40990040  ble cr6, 0x831376c4
	if !ctx.cr[6].gt {
	pc = 0x831376C4; continue 'dispatch;
	}
	// 83137688: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8313768C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83137690: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83137694: 7D275E70  srawi r7, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 83137698: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 8313769C: 7D265E70  srawi r6, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 831376A0: 7CC60194  addze r6, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[6].s64 = tmp.s64;
	// 831376A4: 54C65828  slwi r6, r6, 0xb
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(11);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 831376A8: 7D264851  subf. r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831376AC: 40810008  ble 0x831376b4
	if !ctx.cr[0].gt {
	pc = 0x831376B4; continue 'dispatch;
	}
	// 831376B0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 831376B4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831376B8: 7FC7F214  add r30, r7, r30
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 831376BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831376C0: 4082FFD0  bne 0x83137690
	if !ctx.cr[0].eq {
	pc = 0x83137690; continue 'dispatch;
	}
	// 831376C4: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831376C8: 7D6A402E  lwzx r11, r10, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831376CC: 7D695E70  srawi r9, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 831376D0: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831376D4: 7D675E70  srawi r7, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 831376D8: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 831376DC: 54E75828  slwi r7, r7, 0xb
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(11);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831376E0: 7D675851  subf. r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831376E4: 40810008  ble 0x831376ec
	if !ctx.cr[0].gt {
	pc = 0x831376EC; continue 'dispatch;
	}
	// 831376E8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831376EC: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831376F0: 7D6A402E  lwzx r11, r10, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 831376F4: 48000048  b 0x8313773c
	pc = 0x8313773C; continue 'dispatch;
	// 831376F8: 397D0118  addi r11, r29, 0x118
	ctx.r[11].s64 = ctx.r[29].s64 + 280;
	// 831376FC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83137700: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 83137704: A3CB0000  lhz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83137708: 40990020  ble cr6, 0x83137728
	if !ctx.cr[6].gt {
	pc = 0x83137728; continue 'dispatch;
	}
	// 8313770C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83137710: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83137714: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83137718: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313771C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83137720: 7FC8F214  add r30, r8, r30
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[30].u64;
	// 83137724: 4082FFF0  bne 0x83137714
	if !ctx.cr[0].eq {
	pc = 0x83137714; continue 'dispatch;
	}
	// 83137728: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8313772C: 7D4B4A2E  lhzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83137730: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83137734: 7D6B4A2E  lhzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83137738: 556B583E  rotlwi r11, r11, 0xb
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(11)) as u64;
	// 8313773C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137740: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83137744: 419A0018  beq cr6, 0x8313775c
	if ctx.cr[6].eq {
	pc = 0x8313775C; continue 'dispatch;
	}
	// 83137748: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 8313774C: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 83137750: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 83137754: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83137758: 4BFF81E9  bl 0x8312f940
	ctx.lr = 0x8313775C;
	sub_8312F940(ctx, base);
	// 8313775C: 817D0110  lwz r11, 0x110(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(272 as u32) ) } as u64;
	// 83137760: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137764: 817D0114  lwz r11, 0x114(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(276 as u32) ) } as u64;
	// 83137768: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8313776C: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137770: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83137774: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83137778: 48070A2C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137780 size=68
    let mut pc: u32 = 0x83137780;
    'dispatch: loop {
        match pc {
            0x83137780 => {
    //   block [0x83137780..0x831377C4)
	// 83137780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313778C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137794: 4BFEF615  bl 0x83126da8
	ctx.lr = 0x83137798;
	sub_83126DA8(ctx, base);
	// 83137798: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8313779C: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831377A0: 396BD400  addi r11, r11, -0x2c00
	ctx.r[11].s64 = ctx.r[11].s64 + -11264;
	// 831377A4: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831377A8: 4BFEF641  bl 0x83126de8
	ctx.lr = 0x831377AC;
	sub_83126DE8(ctx, base);
	// 831377AC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 831377B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831377B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831377B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831377BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831377C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831377C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831377C8 size=100
    let mut pc: u32 = 0x831377C8;
    'dispatch: loop {
        match pc {
            0x831377C8 => {
    //   block [0x831377C8..0x8313782C)
	// 831377C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831377CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831377D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831377D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831377D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831377DC: 4BFEF5CD  bl 0x83126da8
	ctx.lr = 0x831377E0;
	sub_83126DA8(ctx, base);
	// 831377E0: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831377E4: 3BCBD800  addi r30, r11, -0x2800
	ctx.r[30].s64 = ctx.r[11].s64 + -10240;
	// 831377E8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 831377EC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831377F0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 831377F4: 409A000C  bne cr6, 0x83137800
	if !ctx.cr[6].eq {
	pc = 0x83137800; continue 'dispatch;
	}
	// 831377F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831377FC: 4BFFFCAD  bl 0x831374a8
	ctx.lr = 0x83137800;
	sub_831374A8(ctx, base);
	// 83137800: 3BFF0034  addi r31, r31, 0x34
	ctx.r[31].s64 = ctx.r[31].s64 + 52;
	// 83137804: 397E0340  addi r11, r30, 0x340
	ctx.r[11].s64 = ctx.r[30].s64 + 832;
	// 83137808: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8313780C: 4198FFE0  blt cr6, 0x831377ec
	if ctx.cr[6].lt {
	pc = 0x831377EC; continue 'dispatch;
	}
	// 83137810: 4BFEF5D9  bl 0x83126de8
	ctx.lr = 0x83137814;
	sub_83126DE8(ctx, base);
	// 83137814: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83137818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313781C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137820: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83137824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137830 size=92
    let mut pc: u32 = 0x83137830;
    'dispatch: loop {
        match pc {
            0x83137830 => {
    //   block [0x83137830..0x8313788C)
	// 83137830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137834: 4807092D  bl 0x831a8160
	ctx.lr = 0x83137838;
	sub_831A8130(ctx, base);
	// 83137838: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313783C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137840: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83137844: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83137848: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8313784C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83137850: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 83137854: 4BFEF555  bl 0x83126da8
	ctx.lr = 0x83137858;
	sub_83126DA8(ctx, base);
	// 83137858: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8313785C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 83137860: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83137864: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83137868: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8313786C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83137870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137874: 4BFFFD65  bl 0x831375d8
	ctx.lr = 0x83137878;
	sub_831375D8(ctx, base);
	// 83137878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313787C: 4BFEF56D  bl 0x83126de8
	ctx.lr = 0x83137880;
	sub_83126DE8(ctx, base);
	// 83137880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137884: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83137888: 48070928  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83137890 size=332
    let mut pc: u32 = 0x83137890;
    'dispatch: loop {
        match pc {
            0x83137890 => {
    //   block [0x83137890..0x831379DC)
	// 83137890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137894: 480708D9  bl 0x831a816c
	ctx.lr = 0x83137898;
	sub_831A8130(ctx, base);
	// 83137898: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8313789C: 814B7EA0  lwz r10, 0x7ea0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32416 as u32) ) } as u64;
	// 831378A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831378A4: 409A000C  bne cr6, 0x831378b0
	if !ctx.cr[6].eq {
	pc = 0x831378B0; continue 'dispatch;
	}
	// 831378A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831378AC: 914B7EA0  stw r10, 0x7ea0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32416 as u32), ctx.r[10].u32 ) };
	// 831378B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831378B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831378B8: B3C50000  sth r30, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 831378BC: B3C60000  sth r30, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 831378C0: B3C70000  sth r30, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 831378C4: 409A000C  bne cr6, 0x831378d0
	if !ctx.cr[6].eq {
	pc = 0x831378D0; continue 'dispatch;
	}
	// 831378C8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831378CC: 40990108  ble cr6, 0x831379d4
	if !ctx.cr[6].gt {
	pc = 0x831379D4; continue 'dispatch;
	}
	// 831378D0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831378D4: 3BE04A1D  li r31, 0x4a1d
	ctx.r[31].s64 = 18973;
	// 831378D8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 831378DC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831378E0: 396B1E58  addi r11, r11, 0x1e58
	ctx.r[11].s64 = ctx.r[11].s64 + 7768;
	// 831378E4: 40990044  ble cr6, 0x83137928
	if !ctx.cr[6].gt {
	pc = 0x83137928; continue 'dispatch;
	}
	// 831378E8: 7D2A18AE  lbzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 831378EC: 390B0100  addi r8, r11, 0x100
	ctx.r[8].s64 = ctx.r[11].s64 + 256;
	// 831378F0: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 831378F4: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 831378F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831378FC: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83137900: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83137904: 7D2942AE  lhax r9, r9, r8
	ctx.r[9].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as i16) as i64;
	// 83137908: 7D29F9D6  mullw r9, r9, r31
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[31].s32 as i64);
	// 8313790C: 7D285670  srawi r8, r9, 0xa
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 10) as i64;
	// 83137910: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83137914: 5508502A  slwi r8, r8, 0xa
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83137918: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8313791C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83137920: 7FE95A2E  lhzx r31, r9, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83137924: 4198FFC4  blt cr6, 0x831378e8
	if ctx.cr[6].lt {
	pc = 0x831378E8; continue 'dispatch;
	}
	// 83137928: 390053FF  li r8, 0x53ff
	ctx.r[8].s64 = 21503;
	// 8313792C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83137930: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83137934: 40990044  ble cr6, 0x83137978
	if !ctx.cr[6].gt {
	pc = 0x83137978; continue 'dispatch;
	}
	// 83137938: 7D2A18AE  lbzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8313793C: 3BAB0100  addi r29, r11, 0x100
	ctx.r[29].s64 = ctx.r[11].s64 + 256;
	// 83137940: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 83137944: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83137948: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8313794C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83137950: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83137954: 7D29EAAE  lhax r9, r9, r29
	ctx.r[9].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32)) } as i16) as i64;
	// 83137958: 7D2941D6  mullw r9, r9, r8
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8313795C: 7D285670  srawi r8, r9, 0xa
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 10) as i64;
	// 83137960: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83137964: 5508502A  slwi r8, r8, 0xa
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83137968: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8313796C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83137970: 7D095A2E  lhzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83137974: 4198FFC4  blt cr6, 0x83137938
	if ctx.cr[6].lt {
	pc = 0x83137938; continue 'dispatch;
	}
	// 83137978: 39205DC1  li r9, 0x5dc1
	ctx.r[9].s64 = 24001;
	// 8313797C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83137980: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83137984: 40990044  ble cr6, 0x831379c8
	if !ctx.cr[6].gt {
	pc = 0x831379C8; continue 'dispatch;
	}
	// 83137988: 7FCA18AE  lbzx r30, r10, r3
	ctx.r[30].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8313798C: 3BAB0100  addi r29, r11, 0x100
	ctx.r[29].s64 = ctx.r[11].s64 + 256;
	// 83137990: 7D290734  extsh r9, r9
	ctx.r[9].s64 = ctx.r[9].s16 as i64;
	// 83137994: 7FDE0774  extsb r30, r30
	ctx.r[30].s64 = ctx.r[30].s8 as i64;
	// 83137998: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8313799C: 57DE083C  slwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 831379A0: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831379A4: 7FDEEAAE  lhax r30, r30, r29
	ctx.r[30].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as i16) as i64;
	// 831379A8: 7D3E49D6  mullw r9, r30, r9
	ctx.r[9].s64 = (ctx.r[30].s32 as i64) * (ctx.r[9].s32 as i64);
	// 831379AC: 7D3E5670  srawi r30, r9, 0xa
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[9].s32 >> 10) as i64;
	// 831379B0: 7FDE0194  addze r30, r30
	tmp.s64 = ctx.r[30].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[30].u32);
	ctx.r[30].s64 = tmp.s64;
	// 831379B4: 57DE502A  slwi r30, r30, 0xa
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(10);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 831379B8: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 831379BC: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831379C0: 7D295A2E  lhzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831379C4: 4198FFC4  blt cr6, 0x83137988
	if ctx.cr[6].lt {
	pc = 0x83137988; continue 'dispatch;
	}
	// 831379C8: B3E50000  sth r31, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 831379CC: B1060000  sth r8, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 831379D0: B1270000  sth r9, 0(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831379D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831379D8: 480707E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831379E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831379E0 size=12
    let mut pc: u32 = 0x831379E0;
    'dispatch: loop {
        match pc {
            0x831379E0 => {
    //   block [0x831379E0..0x831379EC)
	// 831379E0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831379E4: 806B7EBC  lwz r3, 0x7ebc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32444 as u32) ) } as u64;
	// 831379E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831379F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831379F0 size=80
    let mut pc: u32 = 0x831379F0;
    'dispatch: loop {
        match pc {
            0x831379F0 => {
    //   block [0x831379F0..0x83137A40)
	// 831379F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831379F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831379F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831379FC: 48001C95  bl 0x83139690
	ctx.lr = 0x83137A00;
	sub_83139690(ctx, base);
	// 83137A00: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83137A04: 38A01D80  li r5, 0x1d80
	ctx.r[5].s64 = 7552;
	// 83137A08: 386BF920  addi r3, r11, -0x6e0
	ctx.r[3].s64 = ctx.r[11].s64 + -1760;
	// 83137A0C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83137A10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83137A14: 814B7EA0  lwz r10, 0x7ea0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32416 as u32) ) } as u64;
	// 83137A18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83137A1C: 914B7EA0  stw r10, 0x7ea0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32416 as u32), ctx.r[10].u32 ) };
	// 83137A20: 480707C1  bl 0x831a81e0
	ctx.lr = 0x83137A24;
	sub_831A81E0(ctx, base);
	// 83137A24: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83137A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83137A2C: 916A7EBC  stw r11, 0x7ebc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32444 as u32), ctx.r[11].u32 ) };
	// 83137A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137A40 size=48
    let mut pc: u32 = 0x83137A40;
    'dispatch: loop {
        match pc {
            0x83137A40 => {
    //   block [0x83137A40..0x83137A70)
	// 83137A40: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 83137A44: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137A48: 8143008C  lwz r10, 0x8c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 83137A4C: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 83137A50: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83137A54: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137A58: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83137A5C: 81430088  lwz r10, 0x88(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 83137A60: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83137A64: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83137A68: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83137A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137A70 size=28
    let mut pc: u32 = 0x83137A70;
    'dispatch: loop {
        match pc {
            0x83137A70 => {
    //   block [0x83137A70..0x83137A8C)
	// 83137A70: 8143008C  lwz r10, 0x8c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 83137A74: 81630088  lwz r11, 0x88(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 83137A78: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 83137A7C: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83137A80: 9143008C  stw r10, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 83137A84: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83137A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137A90 size=92
    let mut pc: u32 = 0x83137A90;
    'dispatch: loop {
        match pc {
            0x83137A90 => {
    //   block [0x83137A90..0x83137AEC)
	// 83137A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83137A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137AA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83137AAC: 419A0028  beq cr6, 0x83137ad4
	if ctx.cr[6].eq {
	pc = 0x83137AD4; continue 'dispatch;
	}
	// 83137AB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83137AB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137AB8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83137ABC: 48001D2D  bl 0x831397e8
	ctx.lr = 0x83137AC0;
	sub_831397E8(ctx, base);
	// 83137AC0: 38A000EC  li r5, 0xec
	ctx.r[5].s64 = 236;
	// 83137AC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83137AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137ACC: 48070715  bl 0x831a81e0
	ctx.lr = 0x83137AD0;
	sub_831A81E0(ctx, base);
	// 83137AD0: B3DF0000  sth r30, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 83137AD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83137AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137AE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83137AE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137AF0 size=164
    let mut pc: u32 = 0x83137AF0;
    'dispatch: loop {
        match pc {
            0x83137AF0 => {
    //   block [0x83137AF0..0x83137B94)
	// 83137AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137AF4: 48070679  bl 0x831a816c
	ctx.lr = 0x83137AF8;
	sub_831A8130(ctx, base);
	// 83137AF8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 83137AFC: A0E3009A  lhz r7, 0x9a(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(154 as u32) ) } as u64;
	// 83137B00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83137B04: 80C3003C  lwz r6, 0x3c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83137B08: 6145BB80  ori r5, r10, 0xbb80
	ctx.r[5].u64 = ctx.r[10].u64 | 48000;
	// 83137B0C: 80830040  lwz r4, 0x40(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 83137B10: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 83137B14: 83E30044  lwz r31, 0x44(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 83137B18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 83137B1C: 615DFFFF  ori r29, r10, 0xffff
	ctx.r[29].u64 = ctx.r[10].u64 | 65535;
	// 83137B20: B0E30098  sth r7, 0x98(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[7].u16 ) };
	// 83137B24: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83137B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83137B2C: 3920007F  li r9, 0x7f
	ctx.r[9].s64 = 127;
	// 83137B30: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 83137B34: 39400400  li r10, 0x400
	ctx.r[10].s64 = 1024;
	// 83137B38: 90C3005C  stw r6, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 83137B3C: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 83137B40: 9903000E  stb r8, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[8].u8 ) };
	// 83137B44: 93A30018  stw r29, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83137B48: 91030050  stw r8, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 83137B4C: 9923000F  stb r9, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[9].u8 ) };
	// 83137B50: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83137B54: 9BC3000D  stb r30, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[30].u8 ) };
	// 83137B58: 91230054  stw r9, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 83137B5C: 91430058  stw r10, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 83137B60: 90830060  stw r4, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 83137B64: 93E30064  stw r31, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 83137B68: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83137B6C: B163001C  sth r11, 0x1c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 83137B70: B1630024  sth r11, 0x24(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 83137B74: B1630026  sth r11, 0x26(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 83137B78: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83137B7C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83137B80: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83137B84: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83137B88: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83137B8C: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83137B90: 4807062C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137B98 size=12
    let mut pc: u32 = 0x83137B98;
    'dispatch: loop {
        match pc {
            0x83137B98 => {
    //   block [0x83137B98..0x83137BA4)
	// 83137B98: 90830078  stw r4, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 83137B9C: 90A3007C  stw r5, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 83137BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137BA8 size=8
    let mut pc: u32 = 0x83137BA8;
    'dispatch: loop {
        match pc {
            0x83137BA8 => {
    //   block [0x83137BA8..0x83137BB0)
	// 83137BA8: A8630098  lha r3, 0x98(r3)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 83137BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137BB0 size=64
    let mut pc: u32 = 0x83137BB0;
    'dispatch: loop {
        match pc {
            0x83137BB0 => {
    //   block [0x83137BB0..0x83137BF0)
	// 83137BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137BBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83137BC0: 409A0018  bne cr6, 0x83137bd8
	if !ctx.cr[6].eq {
	pc = 0x83137BD8; continue 'dispatch;
	}
	// 83137BC4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83137BC8: 386B2658  addi r3, r11, 0x2658
	ctx.r[3].s64 = ctx.r[11].s64 + 9816;
	// 83137BCC: 4BFF8E0D  bl 0x831309d8
	ctx.lr = 0x83137BD0;
	sub_831309D8(ctx, base);
	// 83137BD0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83137BD4: 4800000C  b 0x83137be0
	pc = 0x83137BE0; continue 'dispatch;
	// 83137BD8: 8963000E  lbz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 83137BDC: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 83137BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137BF0 size=12
    let mut pc: u32 = 0x83137BF0;
    'dispatch: loop {
        match pc {
            0x83137BF0 => {
    //   block [0x83137BF0..0x83137BFC)
	// 83137BF0: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 83137BF4: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 83137BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C00 size=40
    let mut pc: u32 = 0x83137C00;
    'dispatch: loop {
        match pc {
            0x83137C00 => {
    //   block [0x83137C00..0x83137C28)
	// 83137C00: A9630098  lha r11, 0x98(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 83137C04: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83137C08: 41820048  beq 0x83137c50
	if ctx.cr[0].eq {
		sub_83137C50(ctx, base);
		return;
	}
	// 83137C0C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83137C10: 409A0028  bne cr6, 0x83137c38
	if !ctx.cr[6].eq {
		sub_83137C38(ctx, base);
		return;
	}
	// 83137C14: A963009C  lha r11, 0x9c(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as i16) as i64;
	// 83137C18: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83137C1C: 409A000C  bne cr6, 0x83137c28
	if !ctx.cr[6].eq {
		sub_83137C28(ctx, base);
		return;
	}
	// 83137C20: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83137C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C28 size=16
    let mut pc: u32 = 0x83137C28;
    'dispatch: loop {
        match pc {
            0x83137C28 => {
    //   block [0x83137C28..0x83137C38)
	// 83137C28: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83137C2C: 409A0024  bne cr6, 0x83137c50
	if !ctx.cr[6].eq {
		sub_83137C50(ctx, base);
		return;
	}
	// 83137C30: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83137C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C38 size=24
    let mut pc: u32 = 0x83137C38;
    'dispatch: loop {
        match pc {
            0x83137C38 => {
    //   block [0x83137C38..0x83137C50)
	// 83137C38: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83137C3C: 409A0014  bne cr6, 0x83137c50
	if !ctx.cr[6].eq {
		sub_83137C50(ctx, base);
		return;
	}
	// 83137C40: A163009C  lhz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 83137C44: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83137C48: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83137C4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C50 size=8
    let mut pc: u32 = 0x83137C50;
    'dispatch: loop {
        match pc {
            0x83137C50 => {
    //   block [0x83137C50..0x83137C58)
	// 83137C50: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83137C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C58 size=8
    let mut pc: u32 = 0x83137C58;
    'dispatch: loop {
        match pc {
            0x83137C58 => {
    //   block [0x83137C58..0x83137C60)
	// 83137C58: A8630024  lha r3, 0x24(r3)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as i16) as i64;
	// 83137C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C60 size=8
    let mut pc: u32 = 0x83137C60;
    'dispatch: loop {
        match pc {
            0x83137C60 => {
    //   block [0x83137C60..0x83137C68)
	// 83137C60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83137C64: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C68 size=8
    let mut pc: u32 = 0x83137C68;
    'dispatch: loop {
        match pc {
            0x83137C68 => {
    //   block [0x83137C68..0x83137C70)
	// 83137C68: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83137C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C70 size=8
    let mut pc: u32 = 0x83137C70;
    'dispatch: loop {
        match pc {
            0x83137C70 => {
    //   block [0x83137C70..0x83137C78)
	// 83137C70: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 83137C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C78 size=8
    let mut pc: u32 = 0x83137C78;
    'dispatch: loop {
        match pc {
            0x83137C78 => {
    //   block [0x83137C78..0x83137C80)
	// 83137C78: A06300D4  lhz r3, 0xd4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 83137C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137C80 size=16
    let mut pc: u32 = 0x83137C80;
    'dispatch: loop {
        match pc {
            0x83137C80 => {
    //   block [0x83137C80..0x83137C90)
	// 83137C80: 3964006B  addi r11, r4, 0x6b
	ctx.r[11].s64 = ctx.r[4].s64 + 107;
	// 83137C84: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83137C88: 7C6B1A2E  lhzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83137C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137C90 size=76
    let mut pc: u32 = 0x83137C90;
    'dispatch: loop {
        match pc {
            0x83137C90 => {
    //   block [0x83137C90..0x83137CDC)
	// 83137C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137CA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137CA4: 38BF00B0  addi r5, r31, 0xb0
	ctx.r[5].s64 = ctx.r[31].s64 + 176;
	// 83137CA8: 389F00AC  addi r4, r31, 0xac
	ctx.r[4].s64 = ctx.r[31].s64 + 172;
	// 83137CAC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137CB0: 48001AE1  bl 0x83139790
	ctx.lr = 0x83137CB4;
	sub_83139790(ctx, base);
	// 83137CB4: 38DF00AA  addi r6, r31, 0xaa
	ctx.r[6].s64 = ctx.r[31].s64 + 170;
	// 83137CB8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137CBC: 38BF00A8  addi r5, r31, 0xa8
	ctx.r[5].s64 = ctx.r[31].s64 + 168;
	// 83137CC0: 389F00A6  addi r4, r31, 0xa6
	ctx.r[4].s64 = ctx.r[31].s64 + 166;
	// 83137CC4: 48001B05  bl 0x831397c8
	ctx.lr = 0x83137CC8;
	sub_831397C8(ctx, base);
	// 83137CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137CD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137CE0 size=76
    let mut pc: u32 = 0x83137CE0;
    'dispatch: loop {
        match pc {
            0x83137CE0 => {
    //   block [0x83137CE0..0x83137D2C)
	// 83137CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137CEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137CF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137CF4: 38BF00B0  addi r5, r31, 0xb0
	ctx.r[5].s64 = ctx.r[31].s64 + 176;
	// 83137CF8: 389F00AC  addi r4, r31, 0xac
	ctx.r[4].s64 = ctx.r[31].s64 + 172;
	// 83137CFC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137D00: 48001A69  bl 0x83139768
	ctx.lr = 0x83137D04;
	sub_83139768(ctx, base);
	// 83137D04: A0DF00AA  lhz r6, 0xaa(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(170 as u32) ) } as u64;
	// 83137D08: A0BF00A8  lhz r5, 0xa8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 83137D0C: A09F00A6  lhz r4, 0xa6(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(166 as u32) ) } as u64;
	// 83137D10: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137D14: 48001AA5  bl 0x831397b8
	ctx.lr = 0x83137D18;
	sub_831397B8(ctx, base);
	// 83137D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137D30 size=224
    let mut pc: u32 = 0x83137D30;
    'dispatch: loop {
        match pc {
            0x83137D30 => {
    //   block [0x83137D30..0x83137E10)
	// 83137D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137D34: 48070439  bl 0x831a816c
	ctx.lr = 0x83137D38;
	sub_831A8130(ctx, base);
	// 83137D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137D3C: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83137D40: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 83137D44: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 83137D48: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 83137D4C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 83137D50: 419800A4  blt cr6, 0x83137df4
	if ctx.cr[6].lt {
	pc = 0x83137DF4; continue 'dispatch;
	}
	// 83137D54: 54AB063E  clrlwi r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 83137D58: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 83137D5C: 41980034  blt cr6, 0x83137d90
	if ctx.cr[6].lt {
	pc = 0x83137D90; continue 'dispatch;
	}
	// 83137D60: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83137D64: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83137D68: 38AB2680  addi r5, r11, 0x2680
	ctx.r[5].s64 = ctx.r[11].s64 + 9856;
	// 83137D6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83137D70: 4BFF7C61  bl 0x8312f9d0
	ctx.lr = 0x83137D74;
	sub_8312F9D0(ctx, base);
	// 83137D74: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83137D78: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83137D7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83137D80: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83137D84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83137D88: 4BFFFB09  bl 0x83137890
	ctx.lr = 0x83137D8C;
	sub_83137890(ctx, base);
	// 83137D8C: 48000078  b 0x83137e04
	pc = 0x83137E04; continue 'dispatch;
	// 83137D90: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 83137D94: 41980060  blt cr6, 0x83137df4
	if ctx.cr[6].lt {
	pc = 0x83137DF4; continue 'dispatch;
	}
	// 83137D98: A16300A0  lhz r11, 0xa0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 83137D9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83137DA0: 409A003C  bne cr6, 0x83137ddc
	if !ctx.cr[6].eq {
	pc = 0x83137DDC; continue 'dispatch;
	}
	// 83137DA4: A16300A2  lhz r11, 0xa2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(162 as u32) ) } as u64;
	// 83137DA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83137DAC: 409A0030  bne cr6, 0x83137ddc
	if !ctx.cr[6].eq {
	pc = 0x83137DDC; continue 'dispatch;
	}
	// 83137DB0: A16300A4  lhz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 83137DB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83137DB8: 409A0024  bne cr6, 0x83137ddc
	if !ctx.cr[6].eq {
	pc = 0x83137DDC; continue 'dispatch;
	}
	// 83137DBC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83137DC0: 396B7EB8  addi r11, r11, 0x7eb8
	ctx.r[11].s64 = ctx.r[11].s64 + 32440;
	// 83137DC4: A14BFFF8  lhz r10, -8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137DC8: B14300A0  sth r10, 0xa0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 83137DCC: A14BFFFC  lhz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83137DD0: B14300A2  sth r10, 0xa2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(162 as u32), ctx.r[10].u16 ) };
	// 83137DD4: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83137DD8: B16300A4  sth r11, 0xa4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u16 ) };
	// 83137DDC: A16300A0  lhz r11, 0xa0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 83137DE0: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83137DE4: A16300A2  lhz r11, 0xa2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(162 as u32) ) } as u64;
	// 83137DE8: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83137DEC: A16300A4  lhz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 83137DF0: 48000010  b 0x83137e00
	pc = 0x83137E00; continue 'dispatch;
	// 83137DF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83137DF8: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83137DFC: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83137E00: B17D0000  sth r11, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83137E04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83137E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83137E0C: 480703B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137E10 size=28
    let mut pc: u32 = 0x83137E10;
    'dispatch: loop {
        match pc {
            0x83137E10 => {
    //   block [0x83137E10..0x83137E2C)
	// 83137E10: A1630098  lhz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 83137E14: 90830048  stw r4, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 83137E18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83137E1C: 409A0010  bne cr6, 0x83137e2c
	if !ctx.cr[6].eq {
		sub_83137E2C(ctx, base);
		return;
	}
	// 83137E20: 8963000F  lbz r11, 0xf(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(15 as u32) ) } as u64;
	// 83137E24: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 83137E28: 48000020  b 0x83137e48
	sub_83137E2C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137E2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83137E2C size=64
    let mut pc: u32 = 0x83137E2C;
    'dispatch: loop {
        match pc {
            0x83137E2C => {
    //   block [0x83137E2C..0x83137E6C)
	// 83137E2C: 8943000D  lbz r10, 0xd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 83137E30: 8923000E  lbz r9, 0xe(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 83137E34: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83137E38: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83137E3C: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 83137E40: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83137E44: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83137E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83137E4C: 7D4553D6  divw r10, r5, r10
	ctx.r[10].s32 = ctx.r[5].s32 / ctx.r[10].s32;
	// 83137E50: 91630074  stw r11, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83137E54: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83137E58: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 83137E5C: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83137E60: 916300E0  stw r11, 0xe0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 83137E64: 916300DC  stw r11, 0xdc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 83137E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137E70 size=12
    let mut pc: u32 = 0x83137E70;
    'dispatch: loop {
        match pc {
            0x83137E70 => {
    //   block [0x83137E70..0x83137E7C)
	// 83137E70: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137E74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83137E78: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137E7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137E7C size=12
    let mut pc: u32 = 0x83137E7C;
    'dispatch: loop {
        match pc {
            0x83137E7C => {
    //   block [0x83137E7C..0x83137E88)
	// 83137E7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83137E80: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83137E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137E88 size=56
    let mut pc: u32 = 0x83137E88;
    'dispatch: loop {
        match pc {
            0x83137E88 => {
    //   block [0x83137E88..0x83137EC0)
	// 83137E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137E94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137E98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137E9C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137EA0: 480019F9  bl 0x83139898
	ctx.lr = 0x83137EA4;
	sub_83139898(ctx, base);
	// 83137EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83137EA8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83137EAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137EB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137EC0 size=72
    let mut pc: u32 = 0x83137EC0;
    'dispatch: loop {
        match pc {
            0x83137EC0 => {
    //   block [0x83137EC0..0x83137F08)
	// 83137EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137EC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137ECC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137ED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83137ED4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83137ED8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83137EDC: 409A0018  bne cr6, 0x83137ef4
	if !ctx.cr[6].eq {
	pc = 0x83137EF4; continue 'dispatch;
	}
	// 83137EE0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137EE4: 480019CD  bl 0x831398b0
	ctx.lr = 0x83137EE8;
	sub_831398B0(ctx, base);
	// 83137EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83137EEC: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83137EF0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83137EF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137F00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137F08 size=8
    let mut pc: u32 = 0x83137F08;
    'dispatch: loop {
        match pc {
            0x83137F08 => {
    //   block [0x83137F08..0x83137F10)
	// 83137F08: 80630094  lwz r3, 0x94(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 83137F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83137F10 size=8
    let mut pc: u32 = 0x83137F10;
    'dispatch: loop {
        match pc {
            0x83137F10 => {
    //   block [0x83137F10..0x83137F18)
	// 83137F10: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83137F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137F18 size=88
    let mut pc: u32 = 0x83137F18;
    'dispatch: loop {
        match pc {
            0x83137F18 => {
    //   block [0x83137F18..0x83137F70)
	// 83137F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137F28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83137F2C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83137F30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83137F34: 812B0068  lwz r9, 0x68(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 83137F38: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137F3C: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83137F40: 808B0048  lwz r4, 0x48(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83137F44: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83137F48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137F4C: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83137F50: 480018B9  bl 0x83139808
	ctx.lr = 0x83137F54;
	sub_83139808(ctx, base);
	// 83137F54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137F58: 48001921  bl 0x83139878
	ctx.lr = 0x83137F5C;
	sub_83139878(ctx, base);
	// 83137F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137F70 size=96
    let mut pc: u32 = 0x83137F70;
    'dispatch: loop {
        match pc {
            0x83137F70 => {
    //   block [0x83137F70..0x83137FD0)
	// 83137F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83137F78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83137F7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137F80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83137F84: 5485083C  slwi r5, r4, 1
	ctx.r[5].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83137F88: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 83137F8C: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83137F90: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83137F94: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83137F98: 808B0048  lwz r4, 0x48(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83137F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137FA0: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 83137FA4: 7CCA4A14  add r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83137FA8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83137FAC: 7CEB3214  add r7, r11, r6
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 83137FB0: 48001891  bl 0x83139840
	ctx.lr = 0x83137FB4;
	sub_83139840(ctx, base);
	// 83137FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83137FB8: 480018C1  bl 0x83139878
	ctx.lr = 0x83137FBC;
	sub_83139878(ctx, base);
	// 83137FBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83137FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83137FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83137FC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83137FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83137FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83137FD0 size=204
    let mut pc: u32 = 0x83137FD0;
    'dispatch: loop {
        match pc {
            0x83137FD0 => {
    //   block [0x83137FD0..0x8313809C)
	// 83137FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83137FD4: 48070199  bl 0x831a816c
	ctx.lr = 0x83137FD8;
	sub_831A8130(ctx, base);
	// 83137FD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83137FDC: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 83137FE0: 80C30070  lwz r6, 0x70(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 83137FE4: 81230068  lwz r9, 0x68(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 83137FE8: 7D465A14  add r10, r6, r11
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 83137FEC: 81030060  lwz r8, 0x60(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 83137FF0: 83A30050  lwz r29, 0x50(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 83137FF4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83137FF8: 8083004C  lwz r4, 0x4c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 83137FFC: 7CE94050  subf r7, r9, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 83138000: 80A3006C  lwz r5, 0x6c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 83138004: 7FCA5BD6  divw r30, r10, r11
	ctx.r[30].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 83138008: 7CE75A14  add r7, r7, r11
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8313800C: 7FDE59D6  mullw r30, r30, r11
	ctx.r[30].s64 = (ctx.r[30].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83138010: 7FEA5BD6  divw r31, r10, r11
	ctx.r[31].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 83138014: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 83138018: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8313801C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83138020: 7CE75BD6  divw r7, r7, r11
	ctx.r[7].s32 = ctx.r[7].s32 / ctx.r[11].s32;
	// 83138024: 7C84EBD6  divw r4, r4, r29
	ctx.r[4].s32 = ctx.r[4].s32 / ctx.r[29].s32;
	// 83138028: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8313802C: 7F1F3800  cmpw cr6, r31, r7
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83138030: 4098001C  bge cr6, 0x8313804c
	if !ctx.cr[6].lt {
	pc = 0x8313804C; continue 'dispatch;
	}
	// 83138034: 7FC759D6  mullw r30, r7, r11
	ctx.r[30].s64 = (ctx.r[7].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83138038: 7FCAF050  subf r30, r10, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 8313803C: 7D3E4A14  add r9, r30, r9
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[9].u64;
	// 83138040: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83138044: 40980008  bge cr6, 0x8313804c
	if !ctx.cr[6].lt {
	pc = 0x8313804C; continue 'dispatch;
	}
	// 83138048: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8313804C: 7F062800  cmpw cr6, r6, r5
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83138050: 40980008  bge cr6, 0x83138058
	if !ctx.cr[6].lt {
	pc = 0x83138058; continue 'dispatch;
	}
	// 83138054: 7CAA2A14  add r5, r10, r5
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 83138058: 7D655BD6  divw r11, r5, r11
	ctx.r[11].s32 = ctx.r[5].s32 / ctx.r[11].s32;
	// 8313805C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83138060: 40990008  ble cr6, 0x83138068
	if !ctx.cr[6].gt {
	pc = 0x83138068; continue 'dispatch;
	}
	// 83138064: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83138068: 7F04F800  cmpw cr6, r4, r31
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8313806C: 40990008  ble cr6, 0x83138074
	if !ctx.cr[6].gt {
	pc = 0x83138074; continue 'dispatch;
	}
	// 83138070: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83138074: 7F043800  cmpw cr6, r4, r7
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[7].s32, &mut ctx.xer);
	// 83138078: 40990008  ble cr6, 0x83138080
	if !ctx.cr[6].gt {
	pc = 0x83138080; continue 'dispatch;
	}
	// 8313807C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 83138080: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 83138084: 409A000C  bne cr6, 0x83138090
	if !ctx.cr[6].eq {
	pc = 0x83138090; continue 'dispatch;
	}
	// 83138088: 4BFFFEE9  bl 0x83137f70
	ctx.lr = 0x8313808C;
	sub_83137F70(ctx, base);
	// 8313808C: 48000008  b 0x83138094
	pc = 0x83138094; continue 'dispatch;
	// 83138090: 4BFFFE89  bl 0x83137f18
	ctx.lr = 0x83138094;
	sub_83137F18(ctx, base);
	// 83138094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83138098: 48070124  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831380A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831380A0 size=72
    let mut pc: u32 = 0x831380A0;
    'dispatch: loop {
        match pc {
            0x831380A0 => {
    //   block [0x831380A0..0x831380E8)
	// 831380A0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831380A4: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831380A8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831380AC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831380B0: 40990018  ble cr6, 0x831380c8
	if !ctx.cr[6].gt {
	pc = 0x831380C8; continue 'dispatch;
	}
	// 831380B4: 7D4B1850  subf r10, r11, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 831380B8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831380BC: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 831380C0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831380C4: 4200FFF4  bdnz 0x831380b8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831380B8; continue 'dispatch;
	}
	// 831380C8: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831380CC: 7D642A14  add r11, r4, r5
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 831380D0: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831380D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831380D8: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 831380DC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831380E0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831380E4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831380E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831380E8 size=24
    let mut pc: u32 = 0x831380E8;
    'dispatch: loop {
        match pc {
            0x831380E8 => {
    //   block [0x831380E8..0x83138100)
	// 831380E8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831380EC: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831380F0: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 831380F4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831380F8: 4200FFF4  bdnz 0x831380ec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831380EC; continue 'dispatch;
	}
	// 831380FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138100 size=224
    let mut pc: u32 = 0x83138100;
    'dispatch: loop {
        match pc {
            0x83138100 => {
    //   block [0x83138100..0x831381E0)
	// 83138100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138104: 48070051  bl 0x831a8154
	ctx.lr = 0x83138108;
	sub_831A8130(ctx, base);
	// 83138108: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313810C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83138110: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 83138114: 839E0040  lwz r28, 0x40(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83138118: 82FE0044  lwz r23, 0x44(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8313811C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83138120: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83138124: 835F000C  lwz r26, 0xc(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83138128: 837F0028  lwz r27, 0x28(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8313812C: 833F0020  lwz r25, 0x20(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138130: 831F0014  lwz r24, 0x14(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83138134: 4801C4C5  bl 0x831545f8
	ctx.lr = 0x83138138;
	sub_831545F8(ctx, base);
	// 83138138: 7D5BEA14  add r10, r27, r29
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[29].u64;
	// 8313813C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83138140: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 83138144: 7D43E9D6  mullw r10, r3, r29
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[29].s32 as i64);
	// 83138148: 7D09EBD6  divw r8, r9, r29
	ctx.r[8].s32 = ctx.r[9].s32 / ctx.r[29].s32;
	// 8313814C: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 83138150: 7D6859D6  mullw r11, r8, r11
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83138154: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83138158: 4199001C  bgt cr6, 0x83138174
	if ctx.cr[6].gt {
	pc = 0x83138174; continue 'dispatch;
	}
	// 8313815C: 7D69EBD6  divw r11, r9, r29
	ctx.r[11].s32 = ctx.r[9].s32 / ctx.r[29].s32;
	// 83138160: 7D6BE9D6  mullw r11, r11, r29
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 83138164: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83138168: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8313816C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83138170: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 83138174: 7D23D1D6  mullw r9, r3, r26
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[26].s32 as i64);
	// 83138178: 915E0090  stw r10, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8313817C: 7D6ACA14  add r11, r10, r25
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[25].u64;
	// 83138180: 913E0094  stw r9, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 83138184: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83138188: 41980050  blt cr6, 0x831381d8
	if ctx.cr[6].lt {
	pc = 0x831381D8; continue 'dispatch;
	}
	// 8313818C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83138190: 7CDC5850  subf r6, r28, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 83138194: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 83138198: 409A0018  bne cr6, 0x831381b0
	if !ctx.cr[6].eq {
	pc = 0x831381B0; continue 'dispatch;
	}
	// 8313819C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 831381A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831381A4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 831381A8: 4BFFFEF9  bl 0x831380a0
	ctx.lr = 0x831381AC;
	sub_831380A0(ctx, base);
	// 831381AC: 4800002C  b 0x831381d8
	pc = 0x831381D8; continue 'dispatch;
	// 831381B0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 831381B4: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831381B8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831381BC: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 831381C0: 40990018  ble cr6, 0x831381d8
	if !ctx.cr[6].gt {
	pc = 0x831381D8; continue 'dispatch;
	}
	// 831381C4: 7D4BC050  subf r10, r11, r24
	ctx.r[10].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 831381C8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831381CC: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 831381D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 831381D4: 4200FFF4  bdnz 0x831381c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831381C8; continue 'dispatch;
	}
	// 831381D8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831381DC: 4806FFC8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831381E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831381E0 size=208
    let mut pc: u32 = 0x831381E0;
    'dispatch: loop {
        match pc {
            0x831381E0 => {
    //   block [0x831381E0..0x831382B0)
	// 831381E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831381E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831381E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831381EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831381F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831381F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831381F8: 3BDF0048  addi r30, r31, 0x48
	ctx.r[30].s64 = ctx.r[31].s64 + 72;
	// 831381FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83138200: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83138204: 409A0040  bne cr6, 0x83138244
	if !ctx.cr[6].eq {
	pc = 0x83138244; continue 'dispatch;
	}
	// 83138208: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8313820C: 4BECFF1D  bl 0x83008128
	ctx.lr = 0x83138210;
	sub_83008128(ctx, base);
	// 83138210: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138214: 40820030  bne 0x83138244
	if !ctx.cr[0].eq {
	pc = 0x83138244; continue 'dispatch;
	}
	// 83138218: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8313821C: 38DE0028  addi r6, r30, 0x28
	ctx.r[6].s64 = ctx.r[30].s64 + 40;
	// 83138220: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83138224: 38BE0024  addi r5, r30, 0x24
	ctx.r[5].s64 = ctx.r[30].s64 + 36;
	// 83138228: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 8313822C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83138230: 4E800421  bctrl
	ctx.lr = 0x83138234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83138234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138238: 4BFFFD99  bl 0x83137fd0
	ctx.lr = 0x8313823C;
	sub_83137FD0(ctx, base);
	// 8313823C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83138240: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83138244: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83138248: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8313824C: 409A004C  bne cr6, 0x83138298
	if !ctx.cr[6].eq {
	pc = 0x83138298; continue 'dispatch;
	}
	// 83138250: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83138254: 48001675  bl 0x831398c8
	ctx.lr = 0x83138258;
	sub_831398C8(ctx, base);
	// 83138258: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8313825C: 4BECFECD  bl 0x83008128
	ctx.lr = 0x83138260;
	sub_83008128(ctx, base);
	// 83138260: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83138264: 409A0034  bne cr6, 0x83138298
	if !ctx.cr[6].eq {
	pc = 0x83138298; continue 'dispatch;
	}
	// 83138268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313826C: 4BFFFE95  bl 0x83138100
	ctx.lr = 0x83138270;
	sub_83138100(ctx, base);
	// 83138270: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83138274: 4800163D  bl 0x831398b0
	ctx.lr = 0x83138278;
	sub_831398B0(ctx, base);
	// 83138278: 80BF0090  lwz r5, 0x90(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8313827C: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83138280: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83138284: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83138288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313828C: 4E800421  bctrl
	ctx.lr = 0x83138290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83138290: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83138294: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83138298: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313829C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831382A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831382A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831382A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831382AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831382B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831382B0 size=248
    let mut pc: u32 = 0x831382B0;
    'dispatch: loop {
        match pc {
            0x831382B0 => {
    //   block [0x831382B0..0x831383A8)
	// 831382B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831382B4: 4806FEAD  bl 0x831a8160
	ctx.lr = 0x831382B8;
	sub_831A8130(ctx, base);
	// 831382B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831382BC: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831382C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831382C4: 396BF920  addi r11, r11, -0x6e0
	ctx.r[11].s64 = ctx.r[11].s64 + -1760;
	// 831382C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831382CC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831382D0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831382D4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 831382D8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 831382DC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831382E0: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831382E4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 831382E8: 419A0018  beq cr6, 0x83138300
	if ctx.cr[6].eq {
	pc = 0x83138300; continue 'dispatch;
	}
	// 831382EC: 392900EC  addi r9, r9, 0xec
	ctx.r[9].s64 = ctx.r[9].s64 + 236;
	// 831382F0: 390B1D80  addi r8, r11, 0x1d80
	ctx.r[8].s64 = ctx.r[11].s64 + 7552;
	// 831382F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831382F8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831382FC: 4198FFE4  blt cr6, 0x831382e0
	if ctx.cr[6].lt {
	pc = 0x831382E0; continue 'dispatch;
	}
	// 83138300: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 83138304: 409A000C  bne cr6, 0x83138310
	if !ctx.cr[6].eq {
	pc = 0x83138310; continue 'dispatch;
	}
	// 83138308: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8313830C: 48000094  b 0x831383a0
	pc = 0x831383A0; continue 'dispatch;
	// 83138310: 1D4A00EC  mulli r10, r10, 0xec
	ctx.r[10].s64 = ctx.r[10].s64 * 236;
	// 83138314: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83138318: 38A000EC  li r5, 0xec
	ctx.r[5].s64 = 236;
	// 8313831C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83138320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138324: 4806FEBD  bl 0x831a81e0
	ctx.lr = 0x83138328;
	sub_831A81E0(ctx, base);
	// 83138328: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8313832C: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83138330: 48001379  bl 0x831396a8
	ctx.lr = 0x83138334;
	sub_831396A8(ctx, base);
	// 83138334: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83138338: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8313833C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138340: 4082000C  bne 0x8313834c
	if !ctx.cr[0].eq {
	pc = 0x8313834C; continue 'dispatch;
	}
	// 83138344: 4BFFF74D  bl 0x83137a90
	ctx.lr = 0x83138348;
	sub_83137A90(ctx, base);
	// 83138348: 4BFFFFC0  b 0x83138308
	pc = 0x83138308; continue 'dispatch;
	// 8313834C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 83138350: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 83138354: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 83138358: 939F003C  stw r28, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[28].u32 ) };
	// 8313835C: 392B7A70  addi r9, r11, 0x7a70
	ctx.r[9].s64 = ctx.r[11].s64 + 31344;
	// 83138360: 937F0040  stw r27, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[27].u32 ) };
	// 83138364: 3960FF80  li r11, -0x80
	ctx.r[11].s64 = -128;
	// 83138368: 935F0044  stw r26, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[26].u32 ) };
	// 8313836C: 394A7A40  addi r10, r10, 0x7a40
	ctx.r[10].s64 = ctx.r[10].s64 + 31296;
	// 83138370: 93FF007C  stw r31, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 83138374: 93FF0084  stw r31, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 83138378: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 8313837C: 913F0080  stw r9, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 83138380: B3DF00D4  sth r30, 0xd4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u16 ) };
	// 83138384: 915F0078  stw r10, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 83138388: B17F00D6  sth r11, 0xd6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[11].u16 ) };
	// 8313838C: B17F00D8  sth r11, 0xd8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u16 ) };
	// 83138390: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83138394: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 83138398: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 8313839C: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 831383A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831383A4: 4806FE0C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831383A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831383A8 size=700
    let mut pc: u32 = 0x831383A8;
    'dispatch: loop {
        match pc {
            0x831383A8 => {
    //   block [0x831383A8..0x83138664)
	// 831383A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831383AC: 4806FDA5  bl 0x831a8150
	ctx.lr = 0x831383B0;
	sub_831A8130(ctx, base);
	// 831383B0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831383B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831383B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831383BC: 3ADF0010  addi r22, r31, 0x10
	ctx.r[22].s64 = ctx.r[31].s64 + 16;
	// 831383C0: 3B3F0018  addi r25, r31, 0x18
	ctx.r[25].s64 = ctx.r[31].s64 + 24;
	// 831383C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831383C8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831383CC: 3B5F0014  addi r26, r31, 0x14
	ctx.r[26].s64 = ctx.r[31].s64 + 20;
	// 831383D0: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 831383D4: 3B1F000E  addi r24, r31, 0xe
	ctx.r[24].s64 = ctx.r[31].s64 + 14;
	// 831383D8: 92C1005C  stw r22, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[22].u32 ) };
	// 831383DC: 3AFF000F  addi r23, r31, 0xf
	ctx.r[23].s64 = ctx.r[31].s64 + 15;
	// 831383E0: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 831383E4: 3B7F000D  addi r27, r31, 0xd
	ctx.r[27].s64 = ctx.r[31].s64 + 13;
	// 831383E8: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 831383EC: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 831383F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831383F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831383F8: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 831383FC: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 83138400: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 83138404: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83138408: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8313840C: 4BFFDA5D  bl 0x83135e68
	ctx.lr = 0x83138410;
	sub_83135E68(ctx, base);
	// 83138410: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138414: 4080000C  bge 0x83138420
	if !ctx.cr[0].lt {
	pc = 0x83138420; continue 'dispatch;
	}
	// 83138418: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8313841C: 48000240  b 0x8313865c
	pc = 0x8313865C; continue 'dispatch;
	// 83138420: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138424: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83138428: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8313842C: 409900F0  ble cr6, 0x8313851c
	if !ctx.cr[6].gt {
	pc = 0x8313851C; continue 'dispatch;
	}
	// 83138430: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83138434: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83138438: 409A0020  bne cr6, 0x83138458
	if !ctx.cr[6].eq {
	pc = 0x83138458; continue 'dispatch;
	}
	// 8313843C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83138440: 388B26A8  addi r4, r11, 0x26a8
	ctx.r[4].s64 = ctx.r[11].s64 + 9896;
	// 83138444: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83138448: 386B2688  addi r3, r11, 0x2688
	ctx.r[3].s64 = ctx.r[11].s64 + 9864;
	// 8313844C: 4BFF8615  bl 0x83130a60
	ctx.lr = 0x83138450;
	sub_83130A60(ctx, base);
	// 83138450: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83138454: 48000208  b 0x8313865c
	pc = 0x8313865C; continue 'dispatch;
	// 83138458: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313845C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83138460: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83138464: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83138468: 39200060  li r9, 0x60
	ctx.r[9].s64 = 96;
	// 8313846C: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 83138470: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 83138474: B3DF001C  sth r30, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u16 ) };
	// 83138478: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8313847C: 995B0000  stb r10, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83138480: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83138484: B3DF0024  sth r30, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u16 ) };
	// 83138488: 38A10061  addi r5, r1, 0x61
	ctx.r[5].s64 = ctx.r[1].s64 + 97;
	// 8313848C: 91360000  stw r9, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83138490: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83138494: B3DF0026  sth r30, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[30].u16 ) };
	// 83138498: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8313849C: B11F0098  sth r8, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[8].u16 ) };
	// 831384A0: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 831384A4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 831384A8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 831384AC: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 831384B0: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 831384B4: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 831384B8: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 831384BC: 4BFFDAED  bl 0x83135fa8
	ctx.lr = 0x831384C0;
	sub_83135FA8(ctx, base);
	// 831384C0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831384C4: 4180FF54  blt 0x83138418
	if ctx.cr[0].lt {
	pc = 0x83138418; continue 'dispatch;
	}
	// 831384C8: 3921007E  addi r9, r1, 0x7e
	ctx.r[9].s64 = ctx.r[1].s64 + 126;
	// 831384CC: 80D90000  lwz r6, 0(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 831384D0: 3901007C  addi r8, r1, 0x7c
	ctx.r[8].s64 = ctx.r[1].s64 + 124;
	// 831384D4: 88A10060  lbz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 831384D8: 38E1007A  addi r7, r1, 0x7a
	ctx.r[7].s64 = ctx.r[1].s64 + 122;
	// 831384DC: 88810061  lbz r4, 0x61(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(97 as u32) ) } as u64;
	// 831384E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831384E4: B3C10078  sth r30, 0x78(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u16 ) };
	// 831384E8: 4BFFF849  bl 0x83137d30
	ctx.lr = 0x831384EC;
	sub_83137D30(ctx, base);
	// 831384EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831384F0: 4180FF60  blt 0x83138450
	if ctx.cr[0].lt {
	pc = 0x83138450; continue 'dispatch;
	}
	// 831384F4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831384F8: 814B7EAC  lwz r10, 0x7eac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32428 as u32) ) } as u64;
	// 831384FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83138500: 419A011C  beq cr6, 0x8313861c
	if ctx.cr[6].eq {
	pc = 0x8313861C; continue 'dispatch;
	}
	// 83138504: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83138508: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8313850C: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83138514: 4E800421  bctrl
	ctx.lr = 0x83138518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83138518: 48000104  b 0x8313861c
	pc = 0x8313861C; continue 'dispatch;
	// 8313851C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83138520: 38A10061  addi r5, r1, 0x61
	ctx.r[5].s64 = ctx.r[1].s64 + 97;
	// 83138524: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83138528: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8313852C: 4BFFDA7D  bl 0x83135fa8
	ctx.lr = 0x83138530;
	sub_83135FA8(ctx, base);
	// 83138530: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138534: 4180FEE4  blt 0x83138418
	if ctx.cr[0].lt {
	pc = 0x83138418; continue 'dispatch;
	}
	// 83138538: 39210062  addi r9, r1, 0x62
	ctx.r[9].s64 = ctx.r[1].s64 + 98;
	// 8313853C: 80D90000  lwz r6, 0(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138540: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 83138544: 88A10060  lbz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83138548: 38E10066  addi r7, r1, 0x66
	ctx.r[7].s64 = ctx.r[1].s64 + 102;
	// 8313854C: 88810061  lbz r4, 0x61(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(97 as u32) ) } as u64;
	// 83138550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138554: 4BFFF7DD  bl 0x83137d30
	ctx.lr = 0x83138558;
	sub_83137D30(ctx, base);
	// 83138558: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8313855C: 4180FEF4  blt 0x83138450
	if ctx.cr[0].lt {
	pc = 0x83138450; continue 'dispatch;
	}
	// 83138560: A0C10062  lhz r6, 0x62(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(98 as u32) ) } as u64;
	// 83138564: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83138568: A0A10064  lhz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8313856C: A0810066  lhz r4, 0x66(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 83138570: 48001249  bl 0x831397b8
	ctx.lr = 0x83138574;
	sub_831397B8(ctx, base);
	// 83138574: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 83138578: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8313857C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83138580: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83138584: 4BFFD9E5  bl 0x83135f68
	ctx.lr = 0x83138588;
	sub_83135F68(ctx, base);
	// 83138588: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8313858C: 4180FE8C  blt 0x83138418
	if ctx.cr[0].lt {
	pc = 0x83138418; continue 'dispatch;
	}
	// 83138590: 38C1006C  addi r6, r1, 0x6c
	ctx.r[6].s64 = ctx.r[1].s64 + 108;
	// 83138594: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 83138598: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8313859C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831385A0: 4BFFDA59  bl 0x83135ff8
	ctx.lr = 0x831385A4;
	sub_83135FF8(ctx, base);
	// 831385A4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831385A8: 4180FE70  blt 0x83138418
	if ctx.cr[0].lt {
	pc = 0x83138418; continue 'dispatch;
	}
	// 831385AC: A8BE0000  lha r5, 0(r30)
	ctx.r[5].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 831385B0: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831385B4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831385B8: 48001199  bl 0x83139750
	ctx.lr = 0x831385BC;
	sub_83139750(ctx, base);
	// 831385BC: 38A1006C  addi r5, r1, 0x6c
	ctx.r[5].s64 = ctx.r[1].s64 + 108;
	// 831385C0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831385C4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 831385C8: 480011A1  bl 0x83139768
	ctx.lr = 0x831385CC;
	sub_83139768(ctx, base);
	// 831385CC: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 831385D0: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 831385D4: 393F002C  addi r9, r31, 0x2c
	ctx.r[9].s64 = ctx.r[31].s64 + 44;
	// 831385D8: 391F0028  addi r8, r31, 0x28
	ctx.r[8].s64 = ctx.r[31].s64 + 40;
	// 831385DC: 38FF0026  addi r7, r31, 0x26
	ctx.r[7].s64 = ctx.r[31].s64 + 38;
	// 831385E0: 38DF0024  addi r6, r31, 0x24
	ctx.r[6].s64 = ctx.r[31].s64 + 36;
	// 831385E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831385E8: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 831385EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831385F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831385F4: 4BFFDA8D  bl 0x83136080
	ctx.lr = 0x831385F8;
	sub_83136080(ctx, base);
	// 831385F8: 391F00D6  addi r8, r31, 0xd6
	ctx.r[8].s64 = ctx.r[31].s64 + 214;
	// 831385FC: 38FF00D4  addi r7, r31, 0xd4
	ctx.r[7].s64 = ctx.r[31].s64 + 212;
	// 83138600: 38DF00C4  addi r6, r31, 0xc4
	ctx.r[6].s64 = ctx.r[31].s64 + 196;
	// 83138604: 38BF00C0  addi r5, r31, 0xc0
	ctx.r[5].s64 = ctx.r[31].s64 + 192;
	// 83138608: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8313860C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83138610: 4BFFDB61  bl 0x83136170
	ctx.lr = 0x83138614;
	sub_83136170(ctx, base);
	// 83138614: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83138618: B3DF0098  sth r30, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u16 ) };
	// 8313861C: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138620: 89570000  lbz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138624: 81360000  lwz r9, 0(r22)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138628: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8313862C: 811F003C  lwz r8, 0x3c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83138630: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83138634: 80FF0040  lwz r7, 0x40(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83138638: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8313863C: A8610068  lha r3, 0x68(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as i16) as i64;
	// 83138640: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 83138644: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83138648: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8313864C: 913F0058  stw r9, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 83138650: 911F005C  stw r8, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83138654: 90FF0060  stw r7, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 83138658: 90DF0064  stw r6, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 8313865C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83138660: 4806FB40  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138668 size=240
    let mut pc: u32 = 0x83138668;
    'dispatch: loop {
        match pc {
            0x83138668 => {
    //   block [0x83138668..0x83138758)
	// 83138668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313866C: 4806FB01  bl 0x831a816c
	ctx.lr = 0x83138670;
	sub_831A8130(ctx, base);
	// 83138670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83138678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8313867C: 3940FF80  li r10, -0x80
	ctx.r[10].s64 = -128;
	// 83138680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83138684: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83138688: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8313868C: B17F00D4  sth r11, 0xd4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u16 ) };
	// 83138690: B15F00D6  sth r10, 0xd6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[10].u16 ) };
	// 83138694: B15F00D8  sth r10, 0xd8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[10].u16 ) };
	// 83138698: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 8313869C: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 831386A0: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 831386A4: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 831386A8: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831386AC: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 831386B0: 409A000C  bne cr6, 0x831386bc
	if !ctx.cr[6].eq {
	pc = 0x831386BC; continue 'dispatch;
	}
	// 831386B4: 4BFFFCF5  bl 0x831383a8
	ctx.lr = 0x831386B8;
	sub_831383A8(ctx, base);
	// 831386B8: 48000098  b 0x83138750
	pc = 0x83138750; continue 'dispatch;
	// 831386BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831386C0: 480024F1  bl 0x8313abb0
	ctx.lr = 0x831386C4;
	sub_8313ABB0(ctx, base);
	// 831386C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831386C8: 41820018  beq 0x831386e0
	if ctx.cr[0].eq {
	pc = 0x831386E0; continue 'dispatch;
	}
	// 831386CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831386D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831386D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831386D8: 480022A1  bl 0x8313a978
	ctx.lr = 0x831386DC;
	sub_8313A978(ctx, base);
	// 831386DC: 48000074  b 0x83138750
	pc = 0x83138750; continue 'dispatch;
	// 831386E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831386E4: 48000E75  bl 0x83139558
	ctx.lr = 0x831386E8;
	sub_83139558(ctx, base);
	// 831386E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831386EC: 41820018  beq 0x83138704
	if ctx.cr[0].eq {
	pc = 0x83138704; continue 'dispatch;
	}
	// 831386F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831386F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831386F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831386FC: 4800090D  bl 0x83139008
	ctx.lr = 0x83138700;
	sub_83139008(ctx, base);
	// 83138700: 48000050  b 0x83138750
	pc = 0x83138750; continue 'dispatch;
	// 83138704: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83138708: 48001EE1  bl 0x8313a5e8
	ctx.lr = 0x8313870C;
	sub_8313A5E8(ctx, base);
	// 8313870C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138710: 41820018  beq 0x83138728
	if ctx.cr[0].eq {
	pc = 0x83138728; continue 'dispatch;
	}
	// 83138714: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83138718: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8313871C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138720: 48002039  bl 0x8313a758
	ctx.lr = 0x83138724;
	sub_8313A758(ctx, base);
	// 83138724: 4800002C  b 0x83138750
	pc = 0x83138750; continue 'dispatch;
	// 83138728: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8313872C: 480014C5  bl 0x83139bf0
	ctx.lr = 0x83138730;
	sub_83139BF0(ctx, base);
	// 83138730: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138734: 41820018  beq 0x8313874c
	if ctx.cr[0].eq {
	pc = 0x8313874C; continue 'dispatch;
	}
	// 83138738: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8313873C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83138740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138744: 4800161D  bl 0x83139d60
	ctx.lr = 0x83138748;
	sub_83139D60(ctx, base);
	// 83138748: 48000008  b 0x83138750
	pc = 0x83138750; continue 'dispatch;
	// 8313874C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83138750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83138754: 4806FA68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138758 size=232
    let mut pc: u32 = 0x83138758;
    'dispatch: loop {
        match pc {
            0x83138758 => {
    //   block [0x83138758..0x83138840)
	// 83138758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83138760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83138764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313876C: A97F0098  lha r11, 0x98(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 83138770: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138774: 4082000C  bne 0x83138780
	if !ctx.cr[0].eq {
	pc = 0x83138780; continue 'dispatch;
	}
	// 83138778: 4BFFFA69  bl 0x831381e0
	ctx.lr = 0x8313877C;
	sub_831381E0(ctx, base);
	// 8313877C: 48000064  b 0x831387e0
	pc = 0x831387E0; continue 'dispatch;
	// 83138780: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 83138784: 409A0010  bne cr6, 0x83138794
	if !ctx.cr[6].eq {
	pc = 0x83138794; continue 'dispatch;
	}
	// 83138788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313878C: 48000135  bl 0x831388c0
	ctx.lr = 0x83138790;
	sub_831388C0(ctx, base);
	// 83138790: 48000050  b 0x831387e0
	pc = 0x831387E0; continue 'dispatch;
	// 83138794: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83138798: 409A0010  bne cr6, 0x831387a8
	if !ctx.cr[6].eq {
	pc = 0x831387A8; continue 'dispatch;
	}
	// 8313879C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831387A0: 480022B9  bl 0x8313aa58
	ctx.lr = 0x831387A4;
	sub_8313AA58(ctx, base);
	// 831387A4: 4800003C  b 0x831387e0
	pc = 0x831387E0; continue 'dispatch;
	// 831387A8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 831387AC: 409A0010  bne cr6, 0x831387bc
	if !ctx.cr[6].eq {
	pc = 0x831387BC; continue 'dispatch;
	}
	// 831387B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831387B4: 4800209D  bl 0x8313a850
	ctx.lr = 0x831387B8;
	sub_8313A850(ctx, base);
	// 831387B8: 48000028  b 0x831387e0
	pc = 0x831387E0; continue 'dispatch;
	// 831387BC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 831387C0: 409A0010  bne cr6, 0x831387d0
	if !ctx.cr[6].eq {
	pc = 0x831387D0; continue 'dispatch;
	}
	// 831387C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831387C8: 48001AF1  bl 0x8313a2b8
	ctx.lr = 0x831387CC;
	sub_8313A2B8(ctx, base);
	// 831387CC: 48000014  b 0x831387e0
	pc = 0x831387E0; continue 'dispatch;
	// 831387D0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831387D4: 409A000C  bne cr6, 0x831387e0
	if !ctx.cr[6].eq {
	pc = 0x831387E0; continue 'dispatch;
	}
	// 831387D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831387DC: 48000DFD  bl 0x831395d8
	ctx.lr = 0x831387E0;
	sub_831395D8(ctx, base);
	// 831387E0: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 831387E4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831387E8: 41820044  beq 0x8313882c
	if ctx.cr[0].eq {
	pc = 0x8313882C; continue 'dispatch;
	}
	// 831387EC: 813F0094  lwz r9, 0x94(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 831387F0: 811F00DC  lwz r8, 0xdc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 831387F4: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 831387F8: 7C884851  subf. r4, r8, r9
	ctx.r[4].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831387FC: 4080000C  bge 0x83138808
	if !ctx.cr[0].lt {
	pc = 0x83138808; continue 'dispatch;
	}
	// 83138800: 3C848000  addis r4, r4, -0x8000
	ctx.r[4].s64 = ctx.r[4].s64 + -2147483648;
	// 83138804: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83138808: 893F000E  lbz r9, 0xe(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 8313880C: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 83138810: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83138814: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83138818: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8313881C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83138820: 4E800421  bctrl
	ctx.lr = 0x83138824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83138824: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83138828: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 8313882C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83138830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83138834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83138838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313883C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83138840 size=12
    let mut pc: u32 = 0x83138840;
    'dispatch: loop {
        match pc {
            0x83138840 => {
    //   block [0x83138840..0x8313884C)
	// 83138840: 806300B4  lwz r3, 0xb4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 83138844: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83138848: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8313884C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8313884C size=16
    let mut pc: u32 = 0x8313884C;
    'dispatch: loop {
        match pc {
            0x8313884C => {
    //   block [0x8313884C..0x8313885C)
	// 8313884C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83138850: 816B7EC0  lwz r11, 0x7ec0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32448 as u32) ) } as u64;
	// 83138854: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83138858: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8313885C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8313885C size=4
    let mut pc: u32 = 0x8313885C;
    'dispatch: loop {
        match pc {
            0x8313885C => {
    //   block [0x8313885C..0x83138860)
	// 8313885C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138860 size=96
    let mut pc: u32 = 0x83138860;
    'dispatch: loop {
        match pc {
            0x83138860 => {
    //   block [0x83138860..0x831388C0)
	// 83138860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83138868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8313886C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83138870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138874: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83138878: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8313887C: 807E00B4  lwz r3, 0xb4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 83138880: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83138884: 41820014  beq 0x83138898
	if ctx.cr[0].eq {
	pc = 0x83138898; continue 'dispatch;
	}
	// 83138888: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8313888C: 816B7EC4  lwz r11, 0x7ec4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32452 as u32) ) } as u64;
	// 83138890: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83138894: 4E800421  bctrl
	ctx.lr = 0x83138898;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83138898: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 8313889C: 93FE00B8  stw r31, 0xb8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(184 as u32), ctx.r[31].u32 ) };
	// 831388A0: 7D7F5BD6  divw r11, r31, r11
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 831388A4: 917E00BC  stw r11, 0xbc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 831388A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831388AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831388B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831388B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831388B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831388BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831388C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831388C0 size=16
    let mut pc: u32 = 0x831388C0;
    'dispatch: loop {
        match pc {
            0x831388C0 => {
    //   block [0x831388C0..0x831388D0)
	// 831388C0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831388C4: 816B7ECC  lwz r11, 0x7ecc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32460 as u32) ) } as u64;
	// 831388C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831388CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831388D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831388D0 size=12
    let mut pc: u32 = 0x831388D0;
    'dispatch: loop {
        match pc {
            0x831388D0 => {
    //   block [0x831388D0..0x831388DC)
	// 831388D0: 806300B4  lwz r3, 0xb4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 831388D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831388D8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831388DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831388DC size=16
    let mut pc: u32 = 0x831388DC;
    'dispatch: loop {
        match pc {
            0x831388DC => {
    //   block [0x831388DC..0x831388EC)
	// 831388DC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831388E0: 816B7EC8  lwz r11, 0x7ec8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32456 as u32) ) } as u64;
	// 831388E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831388E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831388EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831388EC size=4
    let mut pc: u32 = 0x831388EC;
    'dispatch: loop {
        match pc {
            0x831388EC => {
    //   block [0x831388EC..0x831388F0)
	// 831388EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831388F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831388F0 size=4
    let mut pc: u32 = 0x831388F0;
    'dispatch: loop {
        match pc {
            0x831388F0 => {
    //   block [0x831388F0..0x831388F4)
	// 831388F0: 4BFEDAF8  b 0x831263e8
	sub_831263E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831388F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831388F8 size=128
    let mut pc: u32 = 0x831388F8;
    'dispatch: loop {
        match pc {
            0x831388F8 => {
    //   block [0x831388F8..0x83138978)
	// 831388F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831388FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83138900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83138904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138908: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8313890C: 3BEB7ED0  addi r31, r11, 0x7ed0
	ctx.r[31].s64 = ctx.r[11].s64 + 32464;
	// 83138910: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 83138914: 395F002C  addi r10, r31, 0x2c
	ctx.r[10].s64 = ctx.r[31].s64 + 44;
	// 83138918: 393F002C  addi r9, r31, 0x2c
	ctx.r[9].s64 = ctx.r[31].s64 + 44;
	// 8313891C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138920: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83138924: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83138928: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313892C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83138930: 409A0034  bne cr6, 0x83138964
	if !ctx.cr[6].eq {
	pc = 0x83138964; continue 'dispatch;
	}
	// 83138934: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83138938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8313893C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83138940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138944: 914B7EF4  stw r10, 0x7ef4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32500 as u32), ctx.r[10].u32 ) };
	// 83138948: 4BFF6549  bl 0x8312ee90
	ctx.lr = 0x8313894C;
	sub_8312EE90(ctx, base);
	// 8313894C: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83138950: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83138954: 40820010  bne 0x83138964
	if !ctx.cr[0].eq {
	pc = 0x83138964; continue 'dispatch;
	}
	// 83138958: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313895C: 386B26CC  addi r3, r11, 0x26cc
	ctx.r[3].s64 = ctx.r[11].s64 + 9932;
	// 83138960: 4BFED8E1  bl 0x83126240
	ctx.lr = 0x83138964;
	sub_83126240(ctx, base);
	// 83138964: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83138968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313896C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83138970: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83138974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138978 size=116
    let mut pc: u32 = 0x83138978;
    'dispatch: loop {
        match pc {
            0x83138978 => {
    //   block [0x83138978..0x831389EC)
	// 83138978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313897C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83138980: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83138984: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138988: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8313898C: 3BEB7EF0  addi r31, r11, 0x7ef0
	ctx.r[31].s64 = ctx.r[11].s64 + 32496;
	// 83138990: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 83138994: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 83138998: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 8313899C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831389A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831389A4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831389A8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831389AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831389B0: 409A0028  bne cr6, 0x831389d8
	if !ctx.cr[6].eq {
	pc = 0x831389D8; continue 'dispatch;
	}
	// 831389B4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831389B8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831389BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831389C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831389C4: 914B7EF4  stw r10, 0x7ef4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32500 as u32), ctx.r[10].u32 ) };
	// 831389C8: 419A0010  beq cr6, 0x831389d8
	if ctx.cr[6].eq {
	pc = 0x831389D8; continue 'dispatch;
	}
	// 831389CC: 4BFF65A5  bl 0x8312ef70
	ctx.lr = 0x831389D0;
	sub_8312EF70(ctx, base);
	// 831389D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831389D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831389D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831389DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831389E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831389E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831389E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831389F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831389F0 size=60
    let mut pc: u32 = 0x831389F0;
    'dispatch: loop {
        match pc {
            0x831389F0 => {
    //   block [0x831389F0..0x83138A2C)
	// 831389F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831389F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831389F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831389FC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83138A00: 806B7EF0  lwz r3, 0x7ef0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32496 as u32) ) } as u64;
	// 83138A04: 4BFF65FD  bl 0x8312f000
	ctx.lr = 0x83138A08;
	sub_8312F000(ctx, base);
	// 83138A08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138A0C: 40800010  bge 0x83138a1c
	if !ctx.cr[0].lt {
	pc = 0x83138A1C; continue 'dispatch;
	}
	// 83138A10: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83138A14: 386B2700  addi r3, r11, 0x2700
	ctx.r[3].s64 = ctx.r[11].s64 + 9984;
	// 83138A18: 4BFED829  bl 0x83126240
	ctx.lr = 0x83138A1C;
	sub_83126240(ctx, base);
	// 83138A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83138A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83138A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83138A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138A30 size=60
    let mut pc: u32 = 0x83138A30;
    'dispatch: loop {
        match pc {
            0x83138A30 => {
    //   block [0x83138A30..0x83138A6C)
	// 83138A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83138A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138A3C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83138A40: 806B7EF0  lwz r3, 0x7ef0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32496 as u32) ) } as u64;
	// 83138A44: 4BFF6655  bl 0x8312f098
	ctx.lr = 0x83138A48;
	sub_8312F098(ctx, base);
	// 83138A48: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138A4C: 40800010  bge 0x83138a5c
	if !ctx.cr[0].lt {
	pc = 0x83138A5C; continue 'dispatch;
	}
	// 83138A50: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83138A54: 386B2760  addi r3, r11, 0x2760
	ctx.r[3].s64 = ctx.r[11].s64 + 10080;
	// 83138A58: 4BFED7E9  bl 0x83126240
	ctx.lr = 0x83138A5C;
	sub_83126240(ctx, base);
	// 83138A5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83138A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83138A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83138A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138A70 size=268
    let mut pc: u32 = 0x83138A70;
    'dispatch: loop {
        match pc {
            0x83138A70 => {
    //   block [0x83138A70..0x83138B7C)
	// 83138A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138A74: 4806F6F9  bl 0x831a816c
	ctx.lr = 0x83138A78;
	sub_831A8130(ctx, base);
	// 83138A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138A7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83138A80: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138A84: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83138A88: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138A8C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83138A90: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83138A94: 409900E0  ble cr6, 0x83138b74
	if !ctx.cr[6].gt {
	pc = 0x83138B74; continue 'dispatch;
	}
	// 83138A98: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138A9C: 4BFFA575  bl 0x83133010
	ctx.lr = 0x83138AA0;
	sub_83133010(ctx, base);
	// 83138AA0: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138AA4: 4BFFA6BD  bl 0x83133160
	ctx.lr = 0x83138AA8;
	sub_83133160(ctx, base);
	// 83138AA8: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83138AAC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83138AB0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83138AB4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138AB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83138ABC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83138AC0: 409AFFF4  bne cr6, 0x83138ab4
	if !ctx.cr[6].eq {
	pc = 0x83138AB4; continue 'dispatch;
	}
	// 83138AC4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83138AC8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83138ACC: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 83138AD0: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 83138AD4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83138AD8: 5529003F  rotlwi. r9, r9, 0
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83138ADC: 41820018  beq 0x83138af4
	if ctx.cr[0].eq {
	pc = 0x83138AF4; continue 'dispatch;
	}
	// 83138AE0: 7D0B20AE  lbzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 83138AE4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83138AE8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 83138AEC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83138AF0: 4198FFF0  blt cr6, 0x83138ae0
	if ctx.cr[6].lt {
	pc = 0x83138AE0; continue 'dispatch;
	}
	// 83138AF4: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83138AF8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83138AFC: 419A0014  beq cr6, 0x83138b10
	if ctx.cr[6].eq {
	pc = 0x83138B10; continue 'dispatch;
	}
	// 83138B00: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83138B04: 386B27C0  addi r3, r11, 0x27c0
	ctx.r[3].s64 = ctx.r[11].s64 + 10176;
	// 83138B08: 4BFF8331  bl 0x83130e38
	ctx.lr = 0x83138B0C;
	sub_83130E38(ctx, base);
	// 83138B0C: 48000068  b 0x83138b74
	pc = 0x83138B74; continue 'dispatch;
	// 83138B10: E97E004E  lwa r11, 0x4c(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as i32) as i64;
	// 83138B14: 80DE0048  lwz r6, 0x48(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83138B18: 79675D24  sldi r7, r11, 0xb
	ctx.r[7].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 83138B1C: 80BE0044  lwz r5, 0x44(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83138B20: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138B24: 4BFFA405  bl 0x83132f28
	ctx.lr = 0x83138B28;
	sub_83132F28(ctx, base);
	// 83138B28: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83138B2C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138B30: 4BFF9C71  bl 0x831327a0
	ctx.lr = 0x83138B34;
	sub_831327A0(ctx, base);
	// 83138B34: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83138B38: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83138B3C: 93BE0054  stw r29, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83138B40: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83138B44: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83138B48: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138B4C: 9BBF0002  stb r29, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 83138B50: 4BFFA319  bl 0x83132e68
	ctx.lr = 0x83138B54;
	sub_83132E68(ctx, base);
	// 83138B54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83138B58: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138B5C: 4BFF9B6D  bl 0x831326c8
	ctx.lr = 0x83138B60;
	sub_831326C8(ctx, base);
	// 83138B60: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138B64: 4BFFA42D  bl 0x83132f90
	ctx.lr = 0x83138B68;
	sub_83132F90(ctx, base);
	// 83138B68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83138B6C: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 83138B70: 917E0050  stw r11, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83138B74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83138B78: 4806F644  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138B80 size=160
    let mut pc: u32 = 0x83138B80;
    'dispatch: loop {
        match pc {
            0x83138B80 => {
    //   block [0x83138B80..0x83138C20)
	// 83138B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83138B88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83138B8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83138B90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138B94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83138B98: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138B9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83138BA0: 40820014  bne 0x83138bb4
	if !ctx.cr[0].eq {
	pc = 0x83138BB4; continue 'dispatch;
	}
	// 83138BA4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83138BA8: 386B2800  addi r3, r11, 0x2800
	ctx.r[3].s64 = ctx.r[11].s64 + 10240;
	// 83138BAC: 4BFF828D  bl 0x83130e38
	ctx.lr = 0x83138BB0;
	sub_83130E38(ctx, base);
	// 83138BB0: 48000058  b 0x83138c08
	pc = 0x83138C08; continue 'dispatch;
	// 83138BB4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138BB8: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138BBC: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83138BC0: 4BFF9AC9  bl 0x83132688
	ctx.lr = 0x83138BC4;
	sub_83132688(ctx, base);
	// 83138BC4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83138BC8: 419A0034  beq cr6, 0x83138bfc
	if ctx.cr[6].eq {
	pc = 0x83138BFC; continue 'dispatch;
	}
	// 83138BCC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83138BD0: 419A0018  beq cr6, 0x83138be8
	if ctx.cr[6].eq {
	pc = 0x83138BE8; continue 'dispatch;
	}
	// 83138BD4: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83138BD8: 409A0030  bne cr6, 0x83138c08
	if !ctx.cr[6].eq {
	pc = 0x83138C08; continue 'dispatch;
	}
	// 83138BDC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83138BE0: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83138BE4: 48000024  b 0x83138c08
	pc = 0x83138C08; continue 'dispatch;
	// 83138BE8: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83138BEC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83138BF0: 917E0054  stw r11, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83138BF4: 915E0050  stw r10, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 83138BF8: 48000010  b 0x83138c08
	pc = 0x83138C08; continue 'dispatch;
	// 83138BFC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138C00: 4BFF9B21  bl 0x83132720
	ctx.lr = 0x83138C04;
	sub_83132720(ctx, base);
	// 83138C04: 907E0054  stw r3, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83138C08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83138C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83138C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83138C14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83138C18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83138C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83138C20 size=184
    let mut pc: u32 = 0x83138C20;
    'dispatch: loop {
        match pc {
            0x83138C20 => {
    //   block [0x83138C20..0x83138CD8)
	// 83138C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138C24: 4806F541  bl 0x831a8164
	ctx.lr = 0x83138C28;
	sub_831A8130(ctx, base);
	// 83138C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83138C30: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83138C34: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83138C38: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83138C3C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83138C40: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83138C44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83138C48: 419A0088  beq cr6, 0x83138cd0
	if ctx.cr[6].eq {
	pc = 0x83138CD0; continue 'dispatch;
	}
	// 83138C4C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83138C50: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83138C54: 409A0020  bne cr6, 0x83138c74
	if !ctx.cr[6].eq {
	pc = 0x83138C74; continue 'dispatch;
	}
	// 83138C58: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138C5C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138C60: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83138C64: 83CB003C  lwz r30, 0x3c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83138C68: 83AB0044  lwz r29, 0x44(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83138C6C: 838B0048  lwz r28, 0x48(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83138C70: 836B004C  lwz r27, 0x4c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 83138C74: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138C78: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83138C7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83138C80: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83138C84: 7D492670  srawi r9, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 83138C88: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83138C8C: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83138C90: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83138C94: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 83138C98: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83138C9C: 41810010  bgt 0x83138cac
	if ctx.cr[0].gt {
	pc = 0x83138CAC; continue 'dispatch;
	}
	// 83138CA0: 4BFFCB09  bl 0x831357a8
	ctx.lr = 0x83138CA4;
	sub_831357A8(ctx, base);
	// 83138CA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83138CA8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83138CAC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83138CB0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83138CB4: 409A001C  bne cr6, 0x83138cd0
	if !ctx.cr[6].eq {
	pc = 0x83138CD0; continue 'dispatch;
	}
	// 83138CB8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83138CBC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83138CC0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83138CC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83138CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138CCC: 4BFFCB8D  bl 0x83135858
	ctx.lr = 0x83138CD0;
	sub_83135858(ctx, base);
	// 83138CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83138CD4: 4806F4E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83138CD8 size=168
    let mut pc: u32 = 0x83138CD8;
    'dispatch: loop {
        match pc {
            0x83138CD8 => {
    //   block [0x83138CD8..0x83138D80)
	// 83138CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83138CE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83138CE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83138CEC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83138CF0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83138CF4: 419A0078  beq cr6, 0x83138d6c
	if ctx.cr[6].eq {
	pc = 0x83138D6C; continue 'dispatch;
	}
	// 83138CF8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83138CFC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83138D00: 409A006C  bne cr6, 0x83138d6c
	if !ctx.cr[6].eq {
	pc = 0x83138D6C; continue 'dispatch;
	}
	// 83138D04: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83138D08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83138D0C: 40990060  ble cr6, 0x83138d6c
	if !ctx.cr[6].gt {
	pc = 0x83138D6C; continue 'dispatch;
	}
	// 83138D10: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138D14: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138D18: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83138D1C: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 83138D20: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83138D24: 409A0008  bne cr6, 0x83138d2c
	if !ctx.cr[6].eq {
	pc = 0x83138D2C; continue 'dispatch;
	}
	// 83138D28: 4BFFFE59  bl 0x83138b80
	ctx.lr = 0x83138D2C;
	sub_83138B80(ctx, base);
	// 83138D2C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138D30: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138D34: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83138D38: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 83138D3C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83138D40: 409A000C  bne cr6, 0x83138d4c
	if !ctx.cr[6].eq {
	pc = 0x83138D4C; continue 'dispatch;
	}
	// 83138D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138D48: 4BFFFED9  bl 0x83138c20
	ctx.lr = 0x83138D4C;
	sub_83138C20(ctx, base);
	// 83138D4C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83138D50: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138D54: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83138D58: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 83138D5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83138D60: 409A000C  bne cr6, 0x83138d6c
	if !ctx.cr[6].eq {
	pc = 0x83138D6C; continue 'dispatch;
	}
	// 83138D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83138D68: 4BFFFD09  bl 0x83138a70
	ctx.lr = 0x83138D6C;
	sub_83138A70(ctx, base);
	// 83138D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83138D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83138D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83138D78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83138D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83138D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83138D80 size=648
    let mut pc: u32 = 0x83138D80;
    'dispatch: loop {
        match pc {
            0x83138D80 => {
    //   block [0x83138D80..0x83139008)
	// 83138D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83138D84: 4806F3D1  bl 0x831a8154
	ctx.lr = 0x83138D88;
	sub_831A8130(ctx, base);
	// 83138D88: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83138D8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83138D90: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83138D94: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 83138D98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83138D9C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83138DA0: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 83138DA4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83138DA8: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 83138DAC: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 83138DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83138DB4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83138DB8: 3B8B5CDC  addi r28, r11, 0x5cdc
	ctx.r[28].s64 = ctx.r[11].s64 + 23772;
	// 83138DBC: 40990048  ble cr6, 0x83138e04
	if !ctx.cr[6].gt {
	pc = 0x83138E04; continue 'dispatch;
	}
	// 83138DC0: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138DC4: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 83138DC8: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 83138DCC: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 83138DD0: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138DD4: 88A90000  lbz r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138DD8: 7CE53851  subf. r7, r5, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83138DDC: 40820014  bne 0x83138df0
	if !ctx.cr[0].eq {
	pc = 0x83138DF0; continue 'dispatch;
	}
	// 83138DE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83138DE4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83138DE8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83138DEC: 409AFFE4  bne cr6, 0x83138dd0
	if !ctx.cr[6].eq {
	pc = 0x83138DD0; continue 'dispatch;
	}
	// 83138DF0: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138DF4: 41820010  beq 0x83138e04
	if ctx.cr[0].eq {
	pc = 0x83138E04; continue 'dispatch;
	}
	// 83138DF8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83138DFC: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83138E00: 4198FFC4  blt cr6, 0x83138dc4
	if ctx.cr[6].lt {
	pc = 0x83138DC4; continue 'dispatch;
	}
	// 83138E04: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83138E08: 419A01F4  beq cr6, 0x83138ffc
	if ctx.cr[6].eq {
	pc = 0x83138FFC; continue 'dispatch;
	}
	// 83138E0C: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 83138E10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83138E14: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 83138E18: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 83138E1C: 4806F6F5  bl 0x831a8510
	ctx.lr = 0x83138E20;
	sub_831A8510(ctx, base);
	// 83138E20: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83138E24: 7D6A0734  extsh r10, r11
	ctx.r[10].s64 = ctx.r[11].s16 as i64;
	// 83138E28: 556BC23E  srwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83138E2C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83138E30: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 83138E34: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83138E38: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83138E3C: 419901C0  bgt cr6, 0x83138ffc
	if ctx.cr[6].gt {
	pc = 0x83138FFC; continue 'dispatch;
	}
	// 83138E40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83138E44: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83138E48: 40990048  ble cr6, 0x83138e90
	if !ctx.cr[6].gt {
	pc = 0x83138E90; continue 'dispatch;
	}
	// 83138E4C: 80DC0004  lwz r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83138E50: 7D68FA14  add r11, r8, r31
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 83138E54: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83138E58: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 83138E5C: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138E60: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138E64: 7CE53851  subf. r7, r5, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83138E68: 40820014  bne 0x83138e7c
	if !ctx.cr[0].eq {
	pc = 0x83138E7C; continue 'dispatch;
	}
	// 83138E6C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83138E70: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83138E74: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83138E78: 409AFFE4  bne cr6, 0x83138e5c
	if !ctx.cr[6].eq {
	pc = 0x83138E5C; continue 'dispatch;
	}
	// 83138E7C: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83138E80: 41820010  beq 0x83138e90
	if ctx.cr[0].eq {
	pc = 0x83138E90; continue 'dispatch;
	}
	// 83138E84: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83138E88: 7F08E800  cmpw cr6, r8, r29
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83138E8C: 4198FFC4  blt cr6, 0x83138e50
	if ctx.cr[6].lt {
	pc = 0x83138E50; continue 'dispatch;
	}
	// 83138E90: 7F08E800  cmpw cr6, r8, r29
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83138E94: 419A0168  beq cr6, 0x83138ffc
	if ctx.cr[6].eq {
	pc = 0x83138FFC; continue 'dispatch;
	}
	// 83138E98: A141005C  lhz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83138E9C: 7D68FA14  add r11, r8, r31
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 83138EA0: A081005E  lhz r4, 0x5e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 83138EA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83138EA8: 554AC23E  srwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83138EAC: 5484C23E  srwi r4, r4, 8
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shr(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83138EB0: 5549063E  clrlwi r9, r10, 0x18
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 83138EB4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83138EB8: 886B0007  lbz r3, 7(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 83138EBC: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 83138EC0: 8BEB0006  lbz r31, 6(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 83138EC4: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 83138EC8: 8B8B0005  lbz r28, 5(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 83138ECC: 5147843E  rlwimi r7, r10, 0x10, 0x10, 0x1f
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF0000);
	// 83138ED0: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83138ED4: 5146801E  rlwimi r6, r10, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 83138ED8: 39480008  addi r10, r8, 8
	ctx.r[10].s64 = ctx.r[8].s64 + 8;
	// 83138EDC: 5463403E  rotlwi r3, r3, 8
	ctx.r[3].u64 = ((ctx.r[3].u32).rotate_left(8)) as u64;
	// 83138EE0: 54E8C43E  rlwinm r8, r7, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 83138EE4: 7C63FB78  or r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 | ctx.r[31].u64;
	// 83138EE8: 54C7401E  rlwinm r7, r6, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 83138EEC: A0C10052  lhz r6, 0x52(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 83138EF0: B1590000  sth r10, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83138EF4: 546A402E  slwi r10, r3, 8
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83138EF8: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 83138EFC: 98B80000  stb r5, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 83138F00: 7D4AE378  or r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 83138F04: 54C6C23E  srwi r6, r6, 8
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shr(8);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 83138F08: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83138F0C: 7D3D0774  extsb r29, r9
	ctx.r[29].s64 = ctx.r[9].s8 as i64;
	// 83138F10: 91170000  stw r8, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83138F14: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83138F18: 8101011C  lwz r8, 0x11c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 83138F1C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83138F20: 98DB0000  stb r6, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 83138F24: 7D4BEBD6  divw r10, r11, r29
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[29].s32;
	// 83138F28: 989E0000  stb r4, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 83138F2C: 993A0000  stb r9, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83138F30: 81210114  lwz r9, 0x114(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 83138F34: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83138F38: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83138F3C: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138F40: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83138F44: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 83138F48: 409A0014  bne cr6, 0x83138f5c
	if !ctx.cr[6].eq {
	pc = 0x83138F5C; continue 'dispatch;
	}
	// 83138F4C: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 83138F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83138F54: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83138F58: 48000064  b 0x83138fbc
	pc = 0x83138FBC; continue 'dispatch;
	// 83138F5C: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 83138F60: 409A0010  bne cr6, 0x83138f70
	if !ctx.cr[6].eq {
	pc = 0x83138F70; continue 'dispatch;
	}
	// 83138F64: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 83138F68: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83138F6C: 48000050  b 0x83138fbc
	pc = 0x83138FBC; continue 'dispatch;
	// 83138F70: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83138F74: 409A0048  bne cr6, 0x83138fbc
	if !ctx.cr[6].eq {
	pc = 0x83138FBC; continue 'dispatch;
	}
	// 83138F78: 895B0000  lbz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138F7C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83138F80: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83138F84: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83138F88: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83138F8C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83138F90: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83138F94: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83138F98: 995A0000  stb r10, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83138F9C: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83138FA0: 895B0000  lbz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138FA4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83138FA8: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 83138FAC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83138FB0: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 83138FB4: 98DE0000  stb r6, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 83138FB8: B0AB0000  sth r5, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83138FBC: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138FC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83138FC4: 419A0038  beq cr6, 0x83138ffc
	if ctx.cr[6].eq {
	pc = 0x83138FFC; continue 'dispatch;
	}
	// 83138FC8: 897A0000  lbz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138FCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83138FD0: 419A002C  beq cr6, 0x83138ffc
	if ctx.cr[6].eq {
	pc = 0x83138FFC; continue 'dispatch;
	}
	// 83138FD4: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138FD8: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83138FDC: 40810020  ble 0x83138ffc
	if !ctx.cr[0].gt {
	pc = 0x83138FFC; continue 'dispatch;
	}
	// 83138FE0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83138FE4: 41990018  bgt cr6, 0x83138ffc
	if ctx.cr[6].gt {
	pc = 0x83138FFC; continue 'dispatch;
	}
	// 83138FE8: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83138FEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83138FF0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83138FF4: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 83138FF8: 48000008  b 0x83139000
	pc = 0x83139000; continue 'dispatch;
	// 83138FFC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83139000: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83139004: 4806F1A0  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83139008 size=216
    let mut pc: u32 = 0x83139008;
    'dispatch: loop {
        match pc {
            0x83139008 => {
    //   block [0x83139008..0x831390E0)
	// 83139008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313900C: 4806F159  bl 0x831a8164
	ctx.lr = 0x83139010;
	sub_831A8130(ctx, base);
	// 83139010: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83139014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83139018: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8313901C: 393F0018  addi r9, r31, 0x18
	ctx.r[9].s64 = ctx.r[31].s64 + 24;
	// 83139020: 397F009C  addi r11, r31, 0x9c
	ctx.r[11].s64 = ctx.r[31].s64 + 156;
	// 83139024: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 83139028: 3BDF000E  addi r30, r31, 0xe
	ctx.r[30].s64 = ctx.r[31].s64 + 14;
	// 8313902C: 3BBF000F  addi r29, r31, 0xf
	ctx.r[29].s64 = ctx.r[31].s64 + 15;
	// 83139030: B37F0002  sth r27, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[27].u16 ) };
	// 83139034: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83139038: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8313903C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83139040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83139044: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 83139048: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8313904C: 38FF000D  addi r7, r31, 0xd
	ctx.r[7].s64 = ctx.r[31].s64 + 13;
	// 83139050: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 83139054: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 83139058: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8313905C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 83139060: 4BFFFD21  bl 0x83138d80
	ctx.lr = 0x83139064;
	sub_83138D80(ctx, base);
	// 83139064: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139068: 41800070  blt 0x831390d8
	if ctx.cr[0].lt {
	pc = 0x831390D8; continue 'dispatch;
	}
	// 8313906C: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83139074: 893D0000  lbz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139078: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313907C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83139080: 80FF003C  lwz r7, 0x3c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83139084: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83139088: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8313908C: 80BF0044  lwz r5, 0x44(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83139090: A8610070  lha r3, 0x70(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as i16) as i64;
	// 83139094: B17F001C  sth r11, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 83139098: B17F0026  sth r11, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 8313909C: B17F0024  sth r11, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 831390A0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 831390A4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 831390A8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 831390AC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 831390B0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 831390B4: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 831390B8: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 831390BC: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 831390C0: 90FF005C  stw r7, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 831390C4: 90DF0060  stw r6, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 831390C8: 90BF0064  stw r5, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 831390CC: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 831390D0: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 831390D4: B37F0098  sth r27, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[27].u16 ) };
	// 831390D8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831390DC: 4806F0D8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831390E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831390E0 size=380
    let mut pc: u32 = 0x831390E0;
    'dispatch: loop {
        match pc {
            0x831390E0 => {
    //   block [0x831390E0..0x8313925C)
	// 831390E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831390E4: 4806F089  bl 0x831a816c
	ctx.lr = 0x831390E8;
	sub_831A8130(ctx, base);
	// 831390E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831390EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831390F0: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 831390F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831390F8: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831390FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83139100: 409A0128  bne cr6, 0x83139228
	if !ctx.cr[6].eq {
	pc = 0x83139228; continue 'dispatch;
	}
	// 83139104: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83139108: 4BECF021  bl 0x83008128
	ctx.lr = 0x8313910C;
	sub_83008128(ctx, base);
	// 8313910C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139110: 40820118  bne 0x83139228
	if !ctx.cr[0].eq {
	pc = 0x83139228; continue 'dispatch;
	}
	// 83139114: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 83139118: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8313911C: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 83139120: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 83139124: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 83139128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313912C: 4E800421  bctrl
	ctx.lr = 0x83139130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83139130: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83139134: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83139138: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8313913C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83139140: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83139144: 40990008  ble cr6, 0x8313914c
	if !ctx.cr[6].gt {
	pc = 0x8313914C; continue 'dispatch;
	}
	// 83139148: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8313914C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139150: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83139154: 40990008  ble cr6, 0x8313915c
	if !ctx.cr[6].gt {
	pc = 0x8313915C; continue 'dispatch;
	}
	// 83139158: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8313915C: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 83139160: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83139164: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83139168: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8313916C: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 83139170: 409A0064  bne cr6, 0x831391d4
	if !ctx.cr[6].eq {
	pc = 0x831391D4; continue 'dispatch;
	}
	// 83139174: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83139178: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313917C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 83139180: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139184: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83139188: 40990080  ble cr6, 0x83139208
	if !ctx.cr[6].gt {
	pc = 0x83139208; continue 'dispatch;
	}
	// 8313918C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 83139190: 7CE93850  subf r7, r9, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 83139194: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 83139198: A0CA0000  lhz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313919C: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831391A0: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831391A4: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 831391A8: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831391AC: 7CC74B2E  sthx r6, r7, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u16) };
	// 831391B0: A0CA0002  lhz r6, 2(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 831391B4: 88AA0002  lbz r5, 2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 831391B8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831391BC: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 831391C0: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831391C4: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831391C8: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 831391CC: 4082FFCC  bne 0x83139198
	if !ctx.cr[0].eq {
	pc = 0x83139198; continue 'dispatch;
	}
	// 831391D0: 48000038  b 0x83139208
	pc = 0x83139208; continue 'dispatch;
	// 831391D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831391D8: 40990030  ble cr6, 0x83139208
	if !ctx.cr[6].gt {
	pc = 0x83139208; continue 'dispatch;
	}
	// 831391DC: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 831391E0: 7D1D3850  subf r8, r29, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 831391E4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831391E8: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831391EC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831391F0: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831391F4: 54E7403E  rotlwi r7, r7, 8
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 831391F8: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 831391FC: 7CE8532E  sthx r7, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u16) };
	// 83139200: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83139204: 4082FFE4  bne 0x831391e8
	if !ctx.cr[0].eq {
	pc = 0x831391E8; continue 'dispatch;
	}
	// 83139208: 895E000E  lbz r10, 0xe(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8313920C: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 83139210: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 83139214: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83139218: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8313921C: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83139220: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83139224: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83139228: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313922C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83139230: 409A0024  bne cr6, 0x83139254
	if !ctx.cr[6].eq {
	pc = 0x83139254; continue 'dispatch;
	}
	// 83139234: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 83139238: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 8313923C: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 83139240: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 83139244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83139248: 4E800421  bctrl
	ctx.lr = 0x8313924C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313924C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83139250: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83139254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83139258: 4806EF64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83139260 size=376
    let mut pc: u32 = 0x83139260;
    'dispatch: loop {
        match pc {
            0x83139260 => {
    //   block [0x83139260..0x831393D8)
	// 83139260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83139264: 4806EF09  bl 0x831a816c
	ctx.lr = 0x83139268;
	sub_831A8130(ctx, base);
	// 83139268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313926C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83139270: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 83139274: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139278: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313927C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83139280: 409A0124  bne cr6, 0x831393a4
	if !ctx.cr[6].eq {
	pc = 0x831393A4; continue 'dispatch;
	}
	// 83139284: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83139288: 4BECEEA1  bl 0x83008128
	ctx.lr = 0x8313928C;
	sub_83008128(ctx, base);
	// 8313928C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139290: 40820114  bne 0x831393a4
	if !ctx.cr[0].eq {
	pc = 0x831393A4; continue 'dispatch;
	}
	// 83139294: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 83139298: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8313929C: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 831392A0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 831392A4: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 831392A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831392AC: 4E800421  bctrl
	ctx.lr = 0x831392B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831392B0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831392B4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831392B8: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 831392BC: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 831392C0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831392C4: 40990008  ble cr6, 0x831392cc
	if !ctx.cr[6].gt {
	pc = 0x831392CC; continue 'dispatch;
	}
	// 831392C8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 831392CC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831392D0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831392D4: 40990008  ble cr6, 0x831392dc
	if !ctx.cr[6].gt {
	pc = 0x831392DC; continue 'dispatch;
	}
	// 831392D8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 831392DC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831392E0: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831392E4: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 831392E8: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 831392EC: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 831392F0: 409A0064  bne cr6, 0x83139354
	if !ctx.cr[6].eq {
	pc = 0x83139354; continue 'dispatch;
	}
	// 831392F4: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831392F8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831392FC: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 83139300: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83139304: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83139308: 40990080  ble cr6, 0x83139388
	if !ctx.cr[6].gt {
	pc = 0x83139388; continue 'dispatch;
	}
	// 8313930C: 7CDD4850  subf r6, r29, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 83139310: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 83139314: 7CFD4050  subf r7, r29, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[29].s64;
	// 83139318: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8313931C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 83139320: 6129FF80  ori r9, r9, 0xff80
	ctx.r[9].u64 = ctx.r[9].u64 | 65408;
	// 83139324: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139328: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8313932C: 7CA54A14  add r5, r5, r9
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 83139330: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83139334: 7CA75B2E  sthx r5, r7, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 83139338: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8313933C: 7CA54A14  add r5, r5, r9
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 83139340: 54A5402E  slwi r5, r5, 8
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83139344: 7CA65B2E  sthx r5, r6, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 83139348: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8313934C: 4082FFD8  bne 0x83139324
	if !ctx.cr[0].eq {
	pc = 0x83139324; continue 'dispatch;
	}
	// 83139350: 48000038  b 0x83139388
	pc = 0x83139388; continue 'dispatch;
	// 83139354: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83139358: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8313935C: 4099002C  ble cr6, 0x83139388
	if !ctx.cr[6].gt {
	pc = 0x83139388; continue 'dispatch;
	}
	// 83139360: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 83139364: 6129FF80  ori r9, r9, 0xff80
	ctx.r[9].u64 = ctx.r[9].u64 | 65408;
	// 83139368: 7CEBE8AE  lbzx r7, r11, r29
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8313936C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83139370: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 83139374: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83139378: 54E7402E  slwi r7, r7, 8
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8313937C: B0E80000  sth r7, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83139380: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 83139384: 4198FFE4  blt cr6, 0x83139368
	if ctx.cr[6].lt {
	pc = 0x83139368; continue 'dispatch;
	}
	// 83139388: 897E000E  lbz r11, 0xe(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8313938C: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 83139390: 915E0090  stw r10, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 83139394: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83139398: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8313939C: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831393A0: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 831393A4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831393A8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831393AC: 409A0024  bne cr6, 0x831393d0
	if !ctx.cr[6].eq {
	pc = 0x831393D0; continue 'dispatch;
	}
	// 831393B0: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 831393B4: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 831393B8: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 831393BC: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 831393C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831393C4: 4E800421  bctrl
	ctx.lr = 0x831393C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831393C8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 831393CC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831393D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831393D4: 4806EDE8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831393D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831393D8 size=380
    let mut pc: u32 = 0x831393D8;
    'dispatch: loop {
        match pc {
            0x831393D8 => {
    //   block [0x831393D8..0x83139554)
	// 831393D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831393DC: 4806ED91  bl 0x831a816c
	ctx.lr = 0x831393E0;
	sub_831A8130(ctx, base);
	// 831393E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831393E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831393E8: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 831393EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831393F0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831393F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831393F8: 409A0128  bne cr6, 0x83139520
	if !ctx.cr[6].eq {
	pc = 0x83139520; continue 'dispatch;
	}
	// 831393FC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83139400: 4BECED29  bl 0x83008128
	ctx.lr = 0x83139404;
	sub_83008128(ctx, base);
	// 83139404: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139408: 40820118  bne 0x83139520
	if !ctx.cr[0].eq {
	pc = 0x83139520; continue 'dispatch;
	}
	// 8313940C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 83139410: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 83139414: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 83139418: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8313941C: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 83139420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83139424: 4E800421  bctrl
	ctx.lr = 0x83139428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83139428: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8313942C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83139430: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83139434: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83139438: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8313943C: 40990008  ble cr6, 0x83139444
	if !ctx.cr[6].gt {
	pc = 0x83139444; continue 'dispatch;
	}
	// 83139440: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 83139444: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139448: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8313944C: 40990008  ble cr6, 0x83139454
	if !ctx.cr[6].gt {
	pc = 0x83139454; continue 'dispatch;
	}
	// 83139450: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 83139454: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 83139458: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8313945C: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83139460: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 83139464: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 83139468: 409A0064  bne cr6, 0x831394cc
	if !ctx.cr[6].eq {
	pc = 0x831394CC; continue 'dispatch;
	}
	// 8313946C: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83139470: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83139474: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 83139478: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8313947C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83139480: 40990080  ble cr6, 0x83139500
	if !ctx.cr[6].gt {
	pc = 0x83139500; continue 'dispatch;
	}
	// 83139484: 395D0003  addi r10, r29, 3
	ctx.r[10].s64 = ctx.r[29].s64 + 3;
	// 83139488: 7CE93850  subf r7, r9, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 8313948C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 83139490: 88CAFFFF  lbz r6, -1(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-1 as u32) ) } as u64;
	// 83139494: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83139498: 88AAFFFD  lbz r5, -3(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-3 as u32) ) } as u64;
	// 8313949C: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 831394A0: 7CC62B78  or r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 831394A4: 7CC74B2E  sthx r6, r7, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[6].u16) };
	// 831394A8: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831394AC: 88AAFFFE  lbz r5, -2(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-2 as u32) ) } as u64;
	// 831394B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831394B4: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 831394B8: 7CA63378  or r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 831394BC: B0C90000  sth r6, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 831394C0: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 831394C4: 4082FFCC  bne 0x83139490
	if !ctx.cr[0].eq {
	pc = 0x83139490; continue 'dispatch;
	}
	// 831394C8: 48000038  b 0x83139500
	pc = 0x83139500; continue 'dispatch;
	// 831394CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831394D0: 40990030  ble cr6, 0x83139500
	if !ctx.cr[6].gt {
	pc = 0x83139500; continue 'dispatch;
	}
	// 831394D4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 831394D8: 7D1D3850  subf r8, r29, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 831394DC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 831394E0: 88EA0001  lbz r7, 1(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 831394E4: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831394E8: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831394EC: 54E7403E  rotlwi r7, r7, 8
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 831394F0: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 831394F4: 7CE8532E  sthx r7, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u16) };
	// 831394F8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 831394FC: 4082FFE4  bne 0x831394e0
	if !ctx.cr[0].eq {
	pc = 0x831394E0; continue 'dispatch;
	}
	// 83139500: 895E000E  lbz r10, 0xe(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 83139504: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 83139508: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8313950C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83139510: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83139514: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83139518: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8313951C: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83139520: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139524: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83139528: 409A0024  bne cr6, 0x8313954c
	if !ctx.cr[6].eq {
	pc = 0x8313954C; continue 'dispatch;
	}
	// 8313952C: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 83139530: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 83139534: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 83139538: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8313953C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83139540: 4E800421  bctrl
	ctx.lr = 0x83139544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83139544: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83139548: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8313954C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83139550: 4806EC6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139558 size=116
    let mut pc: u32 = 0x83139558;
    'dispatch: loop {
        match pc {
            0x83139558 => {
    //   block [0x83139558..0x831395CC)
	// 83139558: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8313955C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83139560: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 83139564: 394A2828  addi r10, r10, 0x2828
	ctx.r[10].s64 = ctx.r[10].s64 + 10280;
	// 83139568: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313956C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139570: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83139574: 40820014  bne 0x83139588
	if !ctx.cr[0].eq {
	pc = 0x83139588; continue 'dispatch;
	}
	// 83139578: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8313957C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83139580: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83139584: 409AFFE4  bne cr6, 0x83139568
	if !ctx.cr[6].eq {
	pc = 0x83139568; continue 'dispatch;
	}
	// 83139588: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8313958C: 40820040  bne 0x831395cc
	if !ctx.cr[0].eq {
		sub_831395CC(ctx, base);
		return;
	}
	// 83139590: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 83139594: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83139598: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8313959C: 394A2820  addi r10, r10, 0x2820
	ctx.r[10].s64 = ctx.r[10].s64 + 10272;
	// 831395A0: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831395A4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831395A8: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831395AC: 40820014  bne 0x831395c0
	if !ctx.cr[0].eq {
	pc = 0x831395C0; continue 'dispatch;
	}
	// 831395B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831395B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831395B8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831395BC: 409AFFE4  bne cr6, 0x831395a0
	if !ctx.cr[6].eq {
	pc = 0x831395A0; continue 'dispatch;
	}
	// 831395C0: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831395C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831395C8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831395CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831395CC size=8
    let mut pc: u32 = 0x831395CC;
    'dispatch: loop {
        match pc {
            0x831395CC => {
    //   block [0x831395CC..0x831395D4)
	// 831395CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831395D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831395D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831395D8 size=16
    let mut pc: u32 = 0x831395D8;
    'dispatch: loop {
        match pc {
            0x831395D8 => {
    //   block [0x831395D8..0x831395E8)
	// 831395D8: A963009C  lha r11, 0x9c(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as i16) as i64;
	// 831395DC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831395E0: 409A0008  bne cr6, 0x831395e8
	if !ctx.cr[6].eq {
		sub_831395E8(ctx, base);
		return;
	}
	// 831395E4: 4BFFFDF4  b 0x831393d8
	sub_831393D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831395E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831395E8 size=12
    let mut pc: u32 = 0x831395E8;
    'dispatch: loop {
        match pc {
            0x831395E8 => {
    //   block [0x831395E8..0x831395F4)
	// 831395E8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831395EC: 409A0008  bne cr6, 0x831395f4
	if !ctx.cr[6].eq {
		sub_831395F4(ctx, base);
		return;
	}
	// 831395F0: 4BFFFC70  b 0x83139260
	sub_83139260(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831395F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831395F4 size=8
    let mut pc: u32 = 0x831395F4;
    'dispatch: loop {
        match pc {
            0x831395F4 => {
    //   block [0x831395F4..0x831395FC)
	// 831395F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831395F8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831395FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831395FC size=8
    let mut pc: u32 = 0x831395FC;
    'dispatch: loop {
        match pc {
            0x831395FC => {
    //   block [0x831395FC..0x83139604)
	// 831395FC: 4BFFFAE4  b 0x831390e0
	sub_831390E0(ctx, base);
	return;
	// 83139600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139608 size=136
    let mut pc: u32 = 0x83139608;
    'dispatch: loop {
        match pc {
            0x83139608 => {
    //   block [0x83139608..0x83139690)
	// 83139608: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8313960C: 3484FFFD  addic. r4, r4, -3
	ctx.xer.ca = (ctx.r[4].u32 > (!(-3 as u32)));
	ctx.r[4].s64 = ctx.r[4].s64 + -3;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83139610: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83139614: 4081004C  ble 0x83139660
	if !ctx.cr[0].gt {
	pc = 0x83139660; continue 'dispatch;
	}
	// 83139618: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313961C: 38CB2828  addi r6, r11, 0x2828
	ctx.r[6].s64 = ctx.r[11].s64 + 10280;
	// 83139620: 7D691A14  add r11, r9, r3
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 83139624: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 83139628: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 8313962C: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139630: 8BEA0000  lbz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139634: 7CFF3851  subf. r7, r31, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[31].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83139638: 40820014  bne 0x8313964c
	if !ctx.cr[0].eq {
	pc = 0x8313964C; continue 'dispatch;
	}
	// 8313963C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83139640: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83139644: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83139648: 409AFFE4  bne cr6, 0x8313962c
	if !ctx.cr[6].eq {
	pc = 0x8313962C; continue 'dispatch;
	}
	// 8313964C: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139650: 41820024  beq 0x83139674
	if ctx.cr[0].eq {
	pc = 0x83139674; continue 'dispatch;
	}
	// 83139654: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83139658: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8313965C: 4198FFC4  blt cr6, 0x83139620
	if ctx.cr[6].lt {
	pc = 0x83139620; continue 'dispatch;
	}
	// 83139660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83139664: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83139668: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 8313966C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83139670: 4E800020  blr
	return;
	// 83139674: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83139678: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8313967C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83139680: 4098FFE0  bge cr6, 0x83139660
	if !ctx.cr[6].lt {
	pc = 0x83139660; continue 'dispatch;
	}
	// 83139684: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83139688: B1250000  sth r9, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8313968C: 4BFFFFE0  b 0x8313966c
	pc = 0x8313966C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139690 size=20
    let mut pc: u32 = 0x83139690;
    'dispatch: loop {
        match pc {
            0x83139690 => {
    //   block [0x83139690..0x831396A4)
	// 83139690: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 83139694: 38A00780  li r5, 0x780
	ctx.r[5].s64 = 1920;
	// 83139698: 386BF1A0  addi r3, r11, -0xe60
	ctx.r[3].s64 = ctx.r[11].s64 + -3680;
	// 8313969C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831396A0: 4806EB40  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831396A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831396A8 size=168
    let mut pc: u32 = 0x831396A8;
    'dispatch: loop {
        match pc {
            0x831396A8 => {
    //   block [0x831396A8..0x83139750)
	// 831396A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831396AC: 4806EAC1  bl 0x831a816c
	ctx.lr = 0x831396B0;
	sub_831A8130(ctx, base);
	// 831396B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831396B4: 3D608343  lis r11, -0x7cbd
	ctx.r[11].s64 = -2092761088;
	// 831396B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831396BC: 396BF1A0  addi r11, r11, -0xe60
	ctx.r[11].s64 = ctx.r[11].s64 + -3680;
	// 831396C0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 831396C4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831396C8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831396CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831396D0: 419A0018  beq cr6, 0x831396e8
	if ctx.cr[6].eq {
	pc = 0x831396E8; continue 'dispatch;
	}
	// 831396D4: 394A003C  addi r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 + 60;
	// 831396D8: 392B0780  addi r9, r11, 0x780
	ctx.r[9].s64 = ctx.r[11].s64 + 1920;
	// 831396DC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831396E0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831396E4: 4198FFE4  blt cr6, 0x831396c8
	if ctx.cr[6].lt {
	pc = 0x831396C8; continue 'dispatch;
	}
	// 831396E8: 2F1E0020  cmpwi cr6, r30, 0x20
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32, &mut ctx.xer);
	// 831396EC: 409A000C  bne cr6, 0x831396f8
	if !ctx.cr[6].eq {
	pc = 0x831396F8; continue 'dispatch;
	}
	// 831396F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831396F4: 48000054  b 0x83139748
	pc = 0x83139748; continue 'dispatch;
	// 831396F8: 1D5E003C  mulli r10, r30, 0x3c
	ctx.r[10].s64 = ctx.r[30].s64 * 60;
	// 831396FC: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83139700: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 83139704: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83139708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313970C: 4806EAD5  bl 0x831a81e0
	ctx.lr = 0x83139710;
	sub_831A81E0(ctx, base);
	// 83139710: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83139714: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 83139718: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8313971C: 38DF0032  addi r6, r31, 0x32
	ctx.r[6].s64 = ctx.r[31].s64 + 50;
	// 83139720: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83139724: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 83139728: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8313972C: 6084AC44  ori r4, r4, 0xac44
	ctx.r[4].u64 = ctx.r[4].u64 | 44100;
	// 83139730: 386001F4  li r3, 0x1f4
	ctx.r[3].s64 = 500;
	// 83139734: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83139738: 4BFFC5F1  bl 0x83135d28
	ctx.lr = 0x8313973C;
	sub_83135D28(ctx, base);
	// 8313973C: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83139740: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83139744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83139748: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313974C: 4806EA70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139750 size=20
    let mut pc: u32 = 0x83139750;
    'dispatch: loop {
        match pc {
            0x83139750 => {
    //   block [0x83139750..0x83139764)
	// 83139750: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83139754: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83139758: 38CB0032  addi r6, r11, 0x32
	ctx.r[6].s64 = ctx.r[11].s64 + 50;
	// 8313975C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 83139760: 4BFFC5C8  b 0x83135d28
	sub_83135D28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139768 size=36
    let mut pc: u32 = 0x83139768;
    'dispatch: loop {
        match pc {
            0x83139768 => {
    //   block [0x83139768..0x8313978C)
	// 83139768: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313976C: B1630028  sth r11, 0x28(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u16 ) };
	// 83139770: A1650000  lhz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139774: B163002A  sth r11, 0x2a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 83139778: A1640002  lhz r11, 2(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 8313977C: B163002C  sth r11, 0x2c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u16 ) };
	// 83139780: A1650002  lhz r11, 2(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(2 as u32) ) } as u64;
	// 83139784: B163002E  sth r11, 0x2e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(46 as u32), ctx.r[11].u16 ) };
	// 83139788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139790 size=36
    let mut pc: u32 = 0x83139790;
    'dispatch: loop {
        match pc {
            0x83139790 => {
    //   block [0x83139790..0x831397B4)
	// 83139790: A1630028  lhz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83139794: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83139798: A163002A  lhz r11, 0x2a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(42 as u32) ) } as u64;
	// 8313979C: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831397A0: A163002C  lhz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 831397A4: B1640002  sth r11, 2(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 831397A8: A163002E  lhz r11, 0x2e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(46 as u32) ) } as u64;
	// 831397AC: B1650002  sth r11, 2(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 831397B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831397B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831397B8 size=16
    let mut pc: u32 = 0x831397B8;
    'dispatch: loop {
        match pc {
            0x831397B8 => {
    //   block [0x831397B8..0x831397C8)
	// 831397B8: B0830034  sth r4, 0x34(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[4].u16 ) };
	// 831397BC: B0A30036  sth r5, 0x36(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(54 as u32), ctx.r[5].u16 ) };
	// 831397C0: B0C30038  sth r6, 0x38(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[6].u16 ) };
	// 831397C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831397C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831397C8 size=28
    let mut pc: u32 = 0x831397C8;
    'dispatch: loop {
        match pc {
            0x831397C8 => {
    //   block [0x831397C8..0x831397E4)
	// 831397C8: A1630034  lhz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 831397CC: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831397D0: A1630036  lhz r11, 0x36(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(54 as u32) ) } as u64;
	// 831397D4: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831397D8: A1630038  lhz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 831397DC: B1660000  sth r11, 0(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 831397E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831397E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831397E8 size=8
    let mut pc: u32 = 0x831397E8;
    'dispatch: loop {
        match pc {
            0x831397E8 => {
    //   block [0x831397E8..0x831397F0)
	// 831397E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831397EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831397F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831397F0 size=20
    let mut pc: u32 = 0x831397F0;
    'dispatch: loop {
        match pc {
            0x831397F0 => {
    //   block [0x831397F0..0x83139804)
	// 831397F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831397F4: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 831397F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831397FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83139800: 4806E9E0  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139804 size=4
    let mut pc: u32 = 0x83139804;
    'dispatch: loop {
        match pc {
            0x83139804 => {
    //   block [0x83139804..0x83139808)
	// 83139804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139808 size=48
    let mut pc: u32 = 0x83139808;
    'dispatch: loop {
        match pc {
            0x83139808 => {
    //   block [0x83139808..0x83139838)
	// 83139808: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8313980C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83139810: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83139814: 409A0024  bne cr6, 0x83139838
	if !ctx.cr[6].eq {
		sub_83139838(ctx, base);
		return;
	}
	// 83139818: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8313981C: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83139820: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83139824: 90AB001C  stw r5, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 83139828: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 8313982C: 90EB0024  stw r7, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 83139830: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83139834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139838 size=8
    let mut pc: u32 = 0x83139838;
    'dispatch: loop {
        match pc {
            0x83139838 => {
    //   block [0x83139838..0x83139840)
	// 83139838: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8313983C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139840 size=48
    let mut pc: u32 = 0x83139840;
    'dispatch: loop {
        match pc {
            0x83139840 => {
    //   block [0x83139840..0x83139870)
	// 83139840: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83139844: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83139848: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8313984C: 409A0024  bne cr6, 0x83139870
	if !ctx.cr[6].eq {
		sub_83139870(ctx, base);
		return;
	}
	// 83139850: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83139854: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83139858: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8313985C: 90AB001C  stw r5, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 83139860: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 83139864: 90EB0024  stw r7, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 83139868: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8313986C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139870 size=8
    let mut pc: u32 = 0x83139870;
    'dispatch: loop {
        match pc {
            0x83139870 => {
    //   block [0x83139870..0x83139878)
	// 83139870: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83139874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139878 size=12
    let mut pc: u32 = 0x83139878;
    'dispatch: loop {
        match pc {
            0x83139878 => {
    //   block [0x83139878..0x83139884)
	// 83139878: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8313987C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83139880: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139884 size=16
    let mut pc: u32 = 0x83139884;
    'dispatch: loop {
        match pc {
            0x83139884 => {
    //   block [0x83139884..0x83139894)
	// 83139884: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83139888: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8313988C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83139890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139898 size=20
    let mut pc: u32 = 0x83139898;
    'dispatch: loop {
        match pc {
            0x83139898 => {
    //   block [0x83139898..0x831398AC)
	// 83139898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8313989C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831398A0: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 831398A4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 831398A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831398B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831398B0 size=12
    let mut pc: u32 = 0x831398B0;
    'dispatch: loop {
        match pc {
            0x831398B0 => {
    //   block [0x831398B0..0x831398BC)
	// 831398B0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831398B4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 831398B8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831398BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831398BC size=12
    let mut pc: u32 = 0x831398BC;
    'dispatch: loop {
        match pc {
            0x831398BC => {
    //   block [0x831398BC..0x831398C8)
	// 831398BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831398C0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831398C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831398C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831398C8 size=204
    let mut pc: u32 = 0x831398C8;
    'dispatch: loop {
        match pc {
            0x831398C8 => {
    //   block [0x831398C8..0x83139994)
	// 831398C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831398CC: 4806E8A1  bl 0x831a816c
	ctx.lr = 0x831398D0;
	sub_831A8130(ctx, base);
	// 831398D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831398D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831398D8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831398DC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831398E0: 409A000C  bne cr6, 0x831398ec
	if !ctx.cr[6].eq {
	pc = 0x831398EC; continue 'dispatch;
	}
	// 831398E4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831398E8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831398EC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831398F0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831398F4: 409A0098  bne cr6, 0x8313998c
	if !ctx.cr[6].eq {
	pc = 0x8313998C; continue 'dispatch;
	}
	// 831398F8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831398FC: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 83139900: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83139904: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83139908: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313990C: A17F0038  lhz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83139910: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83139914: 409A0024  bne cr6, 0x83139938
	if !ctx.cr[6].eq {
	pc = 0x83139938; continue 'dispatch;
	}
	// 83139918: 393F0034  addi r9, r31, 0x34
	ctx.r[9].s64 = ctx.r[31].s64 + 52;
	// 8313991C: A15F0036  lhz r10, 0x36(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 83139920: A11F0032  lhz r8, 0x32(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83139924: A0FF0030  lhz r7, 0x30(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83139928: B1610056  sth r11, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[11].u16 ) };
	// 8313992C: 480012BD  bl 0x8313abe8
	ctx.lr = 0x83139930;
	sub_8313ABE8(ctx, base);
	// 83139930: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83139934: 48000050  b 0x83139984
	pc = 0x83139984; continue 'dispatch;
	// 83139938: A3BF0036  lhz r29, 0x36(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 8313993C: 3BDF0034  addi r30, r31, 0x34
	ctx.r[30].s64 = ctx.r[31].s64 + 52;
	// 83139940: 391F002C  addi r8, r31, 0x2c
	ctx.r[8].s64 = ctx.r[31].s64 + 44;
	// 83139944: A15F0032  lhz r10, 0x32(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83139948: A13F0030  lhz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8313994C: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83139950: B1610066  sth r11, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u16 ) };
	// 83139954: B3A1005E  sth r29, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[29].u16 ) };
	// 83139958: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8313995C: 48001945  bl 0x8313b2a0
	ctx.lr = 0x83139960;
	sub_8313B2A0(ctx, base);
	// 83139960: 7C6B0E70  srawi r11, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 83139964: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83139968: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8313996C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83139970: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 83139974: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83139978: 409A000C  bne cr6, 0x83139984
	if !ctx.cr[6].eq {
	pc = 0x83139984; continue 'dispatch;
	}
	// 8313997C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83139980: 916A7F00  stw r11, 0x7f00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32512 as u32), ctx.r[11].u32 ) };
	// 83139984: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83139988: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8313998C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83139990: 4806E82C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83139998 size=600
    let mut pc: u32 = 0x83139998;
    'dispatch: loop {
        match pc {
            0x83139998 => {
    //   block [0x83139998..0x83139BF0)
	// 83139998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313999C: 4806E7CD  bl 0x831a8168
	ctx.lr = 0x831399A0;
	sub_831A8130(ctx, base);
	// 831399A0: 89430003  lbz r10, 3(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 831399A4: 3D600064  lis r11, 0x64
	ctx.r[11].s64 = 6553600;
	// 831399A8: 8BC30002  lbz r30, 2(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 831399AC: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 831399B0: 8BA30001  lbz r29, 1(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 831399B4: 8B830000  lbz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831399B8: 617F732E  ori r31, r11, 0x732e
	ctx.r[31].u64 = ctx.r[11].u64 | 29486;
	// 831399BC: 7D4AF378  or r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	// 831399C0: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 831399C4: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831399C8: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 831399CC: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831399D0: 7D4AE378  or r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[28].u64;
	// 831399D4: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831399D8: 419A001C  beq cr6, 0x831399f4
	if ctx.cr[6].eq {
	pc = 0x831399F4; continue 'dispatch;
	}
	// 831399DC: 3FE0646E  lis r31, 0x646e
	ctx.r[31].s64 = 1684930560;
	// 831399E0: 63FF732E  ori r31, r31, 0x732e
	ctx.r[31].u64 = ctx.r[31].u64 | 29486;
	// 831399E4: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831399E8: 419A000C  beq cr6, 0x831399f4
	if ctx.cr[6].eq {
	pc = 0x831399F4; continue 'dispatch;
	}
	// 831399EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831399F0: 480001FC  b 0x83139bec
	pc = 0x83139BEC; continue 'dispatch;
	// 831399F4: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 831399F8: 8BEB0002  lbz r31, 2(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 831399FC: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83139A00: 8BCB0001  lbz r30, 1(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83139A04: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139A08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83139A0C: 7D4AFB78  or r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 83139A10: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139A14: 7D4AF378  or r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[30].u64;
	// 83139A18: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139A1C: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 83139A20: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 83139A24: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 83139A28: 515F843E  rlwimi r31, r10, 0x10, 0x10, 0x1f
	ctx.r[31].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[31].u64 & 0xFFFFFFFFFFFF0000);
	// 83139A2C: 515E801E  rlwimi r30, r10, 0x10, 0, 0xf
	ctx.r[30].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[30].u64 & 0xFFFFFFFF0000FFFF);
	// 83139A30: 57EAC43E  rlwinm r10, r31, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 83139A34: 57DF401E  rlwinm r31, r30, 8, 0, 0xf
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x00FFFFFFu64;
	// 83139A38: 7D5EFB78  or r30, r10, r31
	ctx.r[30].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 83139A3C: 7F1E2000  cmpw cr6, r30, r4
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83139A40: 4199FFAC  bgt cr6, 0x831399ec
	if ctx.cr[6].gt {
	pc = 0x831399EC; continue 'dispatch;
	}
	// 83139A44: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83139A48: 888B0002  lbz r4, 2(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83139A4C: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83139A50: 8BEB0001  lbz r31, 1(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83139A54: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139A58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83139A5C: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 83139A60: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139A64: 888B0003  lbz r4, 3(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83139A68: 7D4AFB78  or r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 83139A6C: 8BEB0002  lbz r31, 2(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83139A70: 5484403E  rotlwi r4, r4, 8
	ctx.r[4].u64 = ((ctx.r[4].u32).rotate_left(8)) as u64;
	// 83139A74: 8B8B0000  lbz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139A78: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139A7C: 7C84FB78  or r4, r4, r31
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[31].u64;
	// 83139A80: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 83139A84: 8BAB0001  lbz r29, 1(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83139A88: 5484402E  slwi r4, r4, 8
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83139A8C: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 83139A90: 7C84EB78  or r4, r4, r29
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[29].u64;
	// 83139A94: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 83139A98: 515F843E  rlwimi r31, r10, 0x10, 0x10, 0x1f
	ctx.r[31].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[31].u64 & 0xFFFFFFFFFFFF0000);
	// 83139A9C: 515D801E  rlwimi r29, r10, 0x10, 0, 0xf
	ctx.r[29].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[29].u64 & 0xFFFFFFFF0000FFFF);
	// 83139AA0: 5484402E  slwi r4, r4, 8
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83139AA4: 57EAC43E  rlwinm r10, r31, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 83139AA8: 57BF401E  rlwinm r31, r29, 8, 0, 0xf
	ctx.r[31].u64 = ctx.r[29].u32 as u64 & 0x00FFFFFFu64;
	// 83139AAC: 7C84E378  or r4, r4, r28
	ctx.r[4].u64 = ctx.r[4].u64 | ctx.r[28].u64;
	// 83139AB0: 7D5FFB78  or r31, r10, r31
	ctx.r[31].u64 = ctx.r[10].u64 | ctx.r[31].u64;
	// 83139AB4: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 83139AB8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83139ABC: 508A843E  rlwimi r10, r4, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 83139AC0: 509D801E  rlwimi r29, r4, 0x10, 0, 0xf
	ctx.r[29].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[29].u64 & 0xFFFFFFFF0000FFFF);
	// 83139AC4: 554AC43E  rlwinm r10, r10, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 83139AC8: 57A4401E  rlwinm r4, r29, 8, 0, 0xf
	ctx.r[4].u64 = ctx.r[29].u32 as u64 & 0x00FFFFFFu64;
	// 83139ACC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83139AD0: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 83139AD4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 83139AD8: 419A0028  beq cr6, 0x83139b00
	if ctx.cr[6].eq {
	pc = 0x83139B00; continue 'dispatch;
	}
	// 83139ADC: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 83139AE0: 419A0018  beq cr6, 0x83139af8
	if ctx.cr[6].eq {
	pc = 0x83139AF8; continue 'dispatch;
	}
	// 83139AE4: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 83139AE8: 409AFF04  bne cr6, 0x831399ec
	if !ctx.cr[6].eq {
	pc = 0x831399EC; continue 'dispatch;
	}
	// 83139AEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83139AF0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83139AF4: 48000014  b 0x83139b08
	pc = 0x83139B08; continue 'dispatch;
	// 83139AF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83139AFC: 48000008  b 0x83139b04
	pc = 0x83139B04; continue 'dispatch;
	// 83139B00: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83139B04: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 83139B08: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83139B0C: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83139B10: 894B0003  lbz r10, 3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 83139B14: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83139B18: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83139B1C: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83139B20: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139B24: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 83139B28: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139B2C: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 83139B30: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139B34: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 83139B38: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 83139B3C: 7D44C670  srawi r4, r10, 0x18
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 24) as i64;
	// 83139B40: 5147801E  rlwimi r7, r10, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 83139B44: 7D4A4670  srawi r10, r10, 8
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 8) as i64;
	// 83139B48: 54E7002E  rlwinm r7, r7, 0, 0, 0x17
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 83139B4C: 554A042E  rlwinm r10, r10, 0, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 83139B50: 50E4402E  rlwimi r4, r7, 8, 0, 0x17
	ctx.r[4].u64 = (((ctx.r[7].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[4].u64 & 0xFFFFFFFF000000FF);
	// 83139B54: 7C8A5378  or r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 | ctx.r[10].u64;
	// 83139B58: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83139B5C: 894B0007  lbz r10, 7(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(7 as u32) ) } as u64;
	// 83139B60: 88EB0006  lbz r7, 6(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 83139B64: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83139B68: 88AB0005  lbz r5, 5(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 83139B6C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139B70: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 83139B74: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139B78: 7D4A2B78  or r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[5].u64;
	// 83139B7C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83139B80: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 83139B84: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83139B88: 7D67C670  srawi r7, r11, 0x18
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 24) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 24) as i64;
	// 83139B8C: 516A801E  rlwimi r10, r11, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 83139B90: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83139B94: 554A002E  rlwinm r10, r10, 0, 0, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 83139B98: 5147402E  rlwimi r7, r10, 8, 0, 0x17
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[7].u64 & 0xFFFFFFFF000000FF);
	// 83139B9C: 7D6A4670  srawi r10, r11, 8
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 8) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 8) as i64;
	// 83139BA0: 554A042E  rlwinm r10, r10, 0, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 83139BA4: 7CEA5378  or r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 | ctx.r[10].u64;
	// 83139BA8: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83139BAC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139BB0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83139BB4: 409A000C  bne cr6, 0x83139bc0
	if !ctx.cr[6].eq {
	pc = 0x83139BC0; continue 'dispatch;
	}
	// 83139BB8: 7D7F53D6  divw r11, r31, r10
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[10].s32;
	// 83139BBC: 48000028  b 0x83139be4
	pc = 0x83139BE4; continue 'dispatch;
	// 83139BC0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83139BC4: 419AFFF4  beq cr6, 0x83139bb8
	if ctx.cr[6].eq {
	pc = 0x83139BB8; continue 'dispatch;
	}
	// 83139BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83139BCC: 409A0014  bne cr6, 0x83139be0
	if !ctx.cr[6].eq {
	pc = 0x83139BE0; continue 'dispatch;
	}
	// 83139BD0: 7FEB0E70  srawi r11, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 83139BD4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83139BD8: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 83139BDC: 48000008  b 0x83139be4
	pc = 0x83139BE4; continue 'dispatch;
	// 83139BE0: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83139BE4: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83139BE8: 7C7E1A14  add r3, r30, r3
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 83139BEC: 4806E5CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139BF0 size=116
    let mut pc: u32 = 0x83139BF0;
    'dispatch: loop {
        match pc {
            0x83139BF0 => {
    //   block [0x83139BF0..0x83139C64)
	// 83139BF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83139BF4: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83139BF8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83139BFC: 39292834  addi r9, r9, 0x2834
	ctx.r[9].s64 = ctx.r[9].s64 + 10292;
	// 83139C00: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 83139C04: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139C08: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139C0C: 7CE63851  subf. r7, r6, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83139C10: 40820014  bne 0x83139c24
	if !ctx.cr[0].eq {
	pc = 0x83139C24; continue 'dispatch;
	}
	// 83139C14: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83139C18: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83139C1C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83139C20: 409AFFE4  bne cr6, 0x83139c04
	if !ctx.cr[6].eq {
	pc = 0x83139C04; continue 'dispatch;
	}
	// 83139C24: 2C070000  cmpwi r7, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139C28: 4182003C  beq 0x83139c64
	if ctx.cr[0].eq {
		sub_83139C64(ctx, base);
		return;
	}
	// 83139C2C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83139C30: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 83139C34: 394A2830  addi r10, r10, 0x2830
	ctx.r[10].s64 = ctx.r[10].s64 + 10288;
	// 83139C38: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139C3C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139C40: 7D074051  subf. r8, r7, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83139C44: 40820014  bne 0x83139c58
	if !ctx.cr[0].eq {
	pc = 0x83139C58; continue 'dispatch;
	}
	// 83139C48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83139C4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83139C50: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83139C54: 409AFFE4  bne cr6, 0x83139c38
	if !ctx.cr[6].eq {
	pc = 0x83139C38; continue 'dispatch;
	}
	// 83139C58: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139C5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83139C60: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139C64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83139C64 size=8
    let mut pc: u32 = 0x83139C64;
    'dispatch: loop {
        match pc {
            0x83139C64 => {
    //   block [0x83139C64..0x83139C6C)
	// 83139C64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83139C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83139C70 size=240
    let mut pc: u32 = 0x83139C70;
    'dispatch: loop {
        match pc {
            0x83139C70 => {
    //   block [0x83139C70..0x83139D60)
	// 83139C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83139C74: 4806E4E9  bl 0x831a815c
	ctx.lr = 0x83139C78;
	sub_831A8130(ctx, base);
	// 83139C78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83139C7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83139C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83139C84: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83139C88: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83139C8C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83139C90: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 83139C94: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83139C98: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 83139C9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83139CA0: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83139CA4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83139CA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83139CAC: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83139CB0: 40980010  bge cr6, 0x83139cc0
	if !ctx.cr[6].lt {
	pc = 0x83139CC0; continue 'dispatch;
	}
	// 83139CB4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83139CB8: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83139CBC: 4800009C  b 0x83139d58
	pc = 0x83139D58; continue 'dispatch;
	// 83139CC0: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 83139CC4: 81210104  lwz r9, 0x104(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 83139CC8: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 83139CCC: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83139CD0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83139CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83139CD8: 4BFFFCC1  bl 0x83139998
	ctx.lr = 0x83139CDC;
	sub_83139998(ctx, base);
	// 83139CDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83139CE0: 4082000C  bne 0x83139cec
	if !ctx.cr[0].eq {
	pc = 0x83139CEC; continue 'dispatch;
	}
	// 83139CE4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83139CE8: 48000070  b 0x83139d58
	pc = 0x83139D58; continue 'dispatch;
	// 83139CEC: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 83139CF0: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83139CF4: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83139CF8: 4081FFEC  ble 0x83139ce4
	if !ctx.cr[0].gt {
	pc = 0x83139CE4; continue 'dispatch;
	}
	// 83139CFC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83139D00: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 83139D04: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83139D08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83139D0C: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83139D10: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83139D14: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83139D18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83139D1C: 995C0000  stb r10, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83139D20: 814100F4  lwz r10, 0xf4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 83139D24: 993D0000  stb r9, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83139D28: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83139D2C: 98FB0000  stb r7, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 83139D30: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139D34: 893C0000  lbz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139D38: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83139D3C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83139D40: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83139D44: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 83139D48: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83139D4C: 995A0000  stb r10, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 83139D50: 814100FC  lwz r10, 0xfc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 83139D54: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83139D58: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83139D5C: 4806E450  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83139D60 size=236
    let mut pc: u32 = 0x83139D60;
    'dispatch: loop {
        match pc {
            0x83139D60 => {
    //   block [0x83139D60..0x83139E4C)
	// 83139D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83139D64: 4806E405  bl 0x831a8168
	ctx.lr = 0x83139D68;
	sub_831A8130(ctx, base);
	// 83139D68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83139D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83139D70: 39210074  addi r9, r1, 0x74
	ctx.r[9].s64 = ctx.r[1].s64 + 116;
	// 83139D74: 391F0018  addi r8, r31, 0x18
	ctx.r[8].s64 = ctx.r[31].s64 + 24;
	// 83139D78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83139D7C: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 83139D80: 3BDF000E  addi r30, r31, 0xe
	ctx.r[30].s64 = ctx.r[31].s64 + 14;
	// 83139D84: 3BBF000F  addi r29, r31, 0xf
	ctx.r[29].s64 = ctx.r[31].s64 + 15;
	// 83139D88: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 83139D8C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83139D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83139D94: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83139D98: B17F0002  sth r11, 2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 83139D9C: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 83139DA0: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 83139DA4: 38FF000D  addi r7, r31, 0xd
	ctx.r[7].s64 = ctx.r[31].s64 + 13;
	// 83139DA8: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 83139DAC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 83139DB0: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83139DB4: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 83139DB8: 4BFFFEB9  bl 0x83139c70
	ctx.lr = 0x83139DBC;
	sub_83139C70(ctx, base);
	// 83139DBC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139DC0: 4080000C  bge 0x83139dcc
	if !ctx.cr[0].lt {
	pc = 0x83139DCC; continue 'dispatch;
	}
	// 83139DC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83139DC8: 4800007C  b 0x83139e44
	pc = 0x83139E44; continue 'dispatch;
	// 83139DCC: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83139DD4: 893D0000  lbz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139DD8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83139DDC: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139DE0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 83139DE4: 80FF003C  lwz r7, 0x3c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83139DE8: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 83139DEC: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83139DF0: 80BF0044  lwz r5, 0x44(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83139DF4: 83C10074  lwz r30, 0x74(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83139DF8: A8610070  lha r3, 0x70(r1)
	ctx.r[3].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as i16) as i64;
	// 83139DFC: B17F001C  sth r11, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 83139E00: B17F0026  sth r11, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 83139E04: B17F0024  sth r11, 0x24(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u16 ) };
	// 83139E08: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83139E0C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83139E10: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83139E14: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83139E18: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83139E1C: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 83139E20: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 83139E24: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 83139E28: 90FF005C  stw r7, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83139E2C: 90DF0060  stw r6, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 83139E30: 90BF0064  stw r5, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 83139E34: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83139E38: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83139E3C: B09F0098  sth r4, 0x98(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[4].u16 ) };
	// 83139E40: B3DF009C  sth r30, 0x9c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u16 ) };
	// 83139E44: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83139E48: 4806E370  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83139E50 size=392
    let mut pc: u32 = 0x83139E50;
    'dispatch: loop {
        match pc {
            0x83139E50 => {
    //   block [0x83139E50..0x83139FD8)
	// 83139E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83139E54: 4806E319  bl 0x831a816c
	ctx.lr = 0x83139E58;
	sub_831A8130(ctx, base);
	// 83139E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83139E5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83139E60: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 83139E64: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139E68: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139E6C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83139E70: 409A0134  bne cr6, 0x83139fa4
	if !ctx.cr[6].eq {
	pc = 0x83139FA4; continue 'dispatch;
	}
	// 83139E74: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83139E78: 4BECE2B1  bl 0x83008128
	ctx.lr = 0x83139E7C;
	sub_83008128(ctx, base);
	// 83139E7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83139E80: 40820124  bne 0x83139fa4
	if !ctx.cr[0].eq {
	pc = 0x83139FA4; continue 'dispatch;
	}
	// 83139E84: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 83139E88: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 83139E8C: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 83139E90: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 83139E94: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 83139E98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83139E9C: 4E800421  bctrl
	ctx.lr = 0x83139EA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83139EA0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83139EA4: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83139EA8: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83139EAC: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83139EB0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83139EB4: 40990008  ble cr6, 0x83139ebc
	if !ctx.cr[6].gt {
	pc = 0x83139EBC; continue 'dispatch;
	}
	// 83139EB8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83139EBC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139EC0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83139EC4: 40990008  ble cr6, 0x83139ecc
	if !ctx.cr[6].gt {
	pc = 0x83139ECC; continue 'dispatch;
	}
	// 83139EC8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83139ECC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83139ED0: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83139ED4: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 83139ED8: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 83139EDC: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 83139EE0: 409A006C  bne cr6, 0x83139f4c
	if !ctx.cr[6].eq {
	pc = 0x83139F4C; continue 'dispatch;
	}
	// 83139EE4: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83139EE8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83139EEC: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 83139EF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83139EF4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83139EF8: 4099008C  ble cr6, 0x83139f84
	if !ctx.cr[6].gt {
	pc = 0x83139F84; continue 'dispatch;
	}
	// 83139EFC: 7CCA4050  subf r6, r10, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 83139F00: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83139F04: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 83139F08: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139F0C: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83139F10: 54E5403E  rotlwi r5, r7, 8
	ctx.r[5].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 83139F14: 54E7C23E  srwi r7, r7, 8
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83139F18: 54A5043E  clrlwi r5, r5, 0x10
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 83139F1C: 7CA73B78  or r7, r5, r7
	ctx.r[7].u64 = ctx.r[5].u64 | ctx.r[7].u64;
	// 83139F20: 7CE6532E  sthx r7, r6, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u16) };
	// 83139F24: A0EB0002  lhz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83139F28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83139F2C: 54E5403E  rotlwi r5, r7, 8
	ctx.r[5].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 83139F30: 54E7C23E  srwi r7, r7, 8
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83139F34: 54A5043E  clrlwi r5, r5, 0x10
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 83139F38: 7CA73B78  or r7, r5, r7
	ctx.r[7].u64 = ctx.r[5].u64 | ctx.r[7].u64;
	// 83139F3C: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 83139F40: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83139F44: 4082FFC4  bne 0x83139f08
	if !ctx.cr[0].eq {
	pc = 0x83139F08; continue 'dispatch;
	}
	// 83139F48: 4800003C  b 0x83139f84
	pc = 0x83139F84; continue 'dispatch;
	// 83139F4C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83139F50: 40990034  ble cr6, 0x83139f84
	if !ctx.cr[6].gt {
	pc = 0x83139F84; continue 'dispatch;
	}
	// 83139F54: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83139F58: 7CE8E850  subf r7, r8, r29
	ctx.r[7].s64 = ctx.r[29].s64 - ctx.r[8].s64;
	// 83139F5C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83139F60: 7D075A2E  lhzx r8, r7, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83139F64: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83139F68: 5506403E  rotlwi r6, r8, 8
	ctx.r[6].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 83139F6C: 5508C23E  srwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83139F70: 54C6043E  clrlwi r6, r6, 0x10
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 83139F74: 7CC84378  or r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 83139F78: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 83139F7C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83139F80: 4082FFE0  bne 0x83139f60
	if !ctx.cr[0].eq {
	pc = 0x83139F60; continue 'dispatch;
	}
	// 83139F84: 897E000E  lbz r11, 0xe(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 83139F88: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83139F8C: 913E0090  stw r9, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 83139F90: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83139F94: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 83139F98: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83139F9C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83139FA0: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83139FA4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139FA8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83139FAC: 409A0024  bne cr6, 0x83139fd0
	if !ctx.cr[6].eq {
	pc = 0x83139FD0; continue 'dispatch;
	}
	// 83139FB0: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 83139FB4: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 83139FB8: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 83139FBC: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 83139FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83139FC4: 4E800421  bctrl
	ctx.lr = 0x83139FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83139FC8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83139FCC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83139FD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83139FD4: 4806E1E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83139FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83139FD8 size=352
    let mut pc: u32 = 0x83139FD8;
    'dispatch: loop {
        match pc {
            0x83139FD8 => {
    //   block [0x83139FD8..0x8313A138)
	// 83139FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83139FDC: 4806E191  bl 0x831a816c
	ctx.lr = 0x83139FE0;
	sub_831A8130(ctx, base);
	// 83139FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83139FE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83139FE8: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 83139FEC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83139FF0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83139FF4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83139FF8: 409A010C  bne cr6, 0x8313a104
	if !ctx.cr[6].eq {
	pc = 0x8313A104; continue 'dispatch;
	}
	// 83139FFC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8313A000: 4BECE129  bl 0x83008128
	ctx.lr = 0x8313A004;
	sub_83008128(ctx, base);
	// 8313A004: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8313A008: 408200FC  bne 0x8313a104
	if !ctx.cr[0].eq {
	pc = 0x8313A104; continue 'dispatch;
	}
	// 8313A00C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8313A010: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8313A014: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 8313A018: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8313A01C: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 8313A020: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313A024: 4E800421  bctrl
	ctx.lr = 0x8313A028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313A028: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8313A02C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8313A030: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8313A034: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8313A038: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8313A03C: 40990008  ble cr6, 0x8313a044
	if !ctx.cr[6].gt {
	pc = 0x8313A044; continue 'dispatch;
	}
	// 8313A040: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8313A044: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313A048: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8313A04C: 40990008  ble cr6, 0x8313a054
	if !ctx.cr[6].gt {
	pc = 0x8313A054; continue 'dispatch;
	}
	// 8313A050: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8313A054: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8313A058: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8313A05C: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8313A060: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8313A064: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8313A068: 409A0054  bne cr6, 0x8313a0bc
	if !ctx.cr[6].eq {
	pc = 0x8313A0BC; continue 'dispatch;
	}
	// 8313A06C: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313A070: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313A074: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8313A078: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8313A07C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8313A080: 40990068  ble cr6, 0x8313a0e8
	if !ctx.cr[6].gt {
	pc = 0x8313A0E8; continue 'dispatch;
	}
	// 8313A084: 7D1D3850  subf r8, r29, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 8313A088: 7CFD4850  subf r7, r29, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 8313A08C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8313A090: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8313A094: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313A098: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8313A09C: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 8313A0A0: 7CC8532E  sthx r6, r8, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[6].u16) };
	// 8313A0A4: 88CA0001  lbz r6, 1(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 8313A0A8: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 8313A0AC: 7CC7532E  sthx r6, r7, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32), ctx.r[6].u16) };
	// 8313A0B0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8313A0B4: 4082FFE0  bne 0x8313a094
	if !ctx.cr[0].eq {
	pc = 0x8313A094; continue 'dispatch;
	}
	// 8313A0B8: 48000030  b 0x8313a0e8
	pc = 0x8313A0E8; continue 'dispatch;
	// 8313A0BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8313A0C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313A0C4: 40990024  ble cr6, 0x8313a0e8
	if !ctx.cr[6].gt {
	pc = 0x8313A0E8; continue 'dispatch;
	}
	// 8313A0C8: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8313A0CC: 7D0AE8AE  lbzx r8, r10, r29
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8313A0D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8313A0D4: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 8313A0D8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8313A0DC: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8313A0E0: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8313A0E4: 4198FFE8  blt cr6, 0x8313a0cc
	if ctx.cr[6].lt {
	pc = 0x8313A0CC; continue 'dispatch;
	}
	// 8313A0E8: 895E000E  lbz r10, 0xe(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8313A0EC: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8313A0F0: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8313A0F4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8313A0F8: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8313A0FC: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8313A100: 915E0094  stw r10, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 8313A104: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313A108: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8313A10C: 409A0024  bne cr6, 0x8313a130
	if !ctx.cr[6].eq {
	pc = 0x8313A130; continue 'dispatch;
	}
	// 8313A110: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8313A114: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 8313A118: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8313A11C: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8313A120: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313A124: 4E800421  bctrl
	ctx.lr = 0x8313A128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313A128: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8313A12C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8313A130: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313A134: 4806E088  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8313A138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8313A138 size=380
    let mut pc: u32 = 0x8313A138;
    'dispatch: loop {
        match pc {
            0x8313A138 => {
    //   block [0x8313A138..0x8313A2B4)
	// 8313A138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313A13C: 4806E031  bl 0x831a816c
	ctx.lr = 0x8313A140;
	sub_831A8130(ctx, base);
	// 8313A140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313A144: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8313A148: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 8313A14C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313A150: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313A154: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8313A158: 409A0128  bne cr6, 0x8313a280
	if !ctx.cr[6].eq {
	pc = 0x8313A280; continue 'dispatch;
	}
	// 8313A15C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8313A160: 4BECDFC9  bl 0x83008128
	ctx.lr = 0x8313A164;
	sub_83008128(ctx, base);
	// 8313A164: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8313A168: 40820118  bne 0x8313a280
	if !ctx.cr[0].eq {
	pc = 0x8313A280; continue 'dispatch;
	}
	// 8313A16C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8313A170: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 8313A174: 817E0078  lwz r11, 0x78(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 8313A178: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8313A17C: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 8313A180: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313A184: 4E800421  bctrl
	ctx.lr = 0x8313A188;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313A188: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8313A18C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8313A190: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8313A194: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8313A198: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8313A19C: 40990008  ble cr6, 0x8313a1a4
	if !ctx.cr[6].gt {
	pc = 0x8313A1A4; continue 'dispatch;
	}
	// 8313A1A0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8313A1A4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313A1A8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8313A1AC: 40990008  ble cr6, 0x8313a1b4
	if !ctx.cr[6].gt {
	pc = 0x8313A1B4; continue 'dispatch;
	}
	// 8313A1B0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8313A1B4: 88FE000E  lbz r7, 0xe(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8313A1B8: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8313A1BC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8313A1C0: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 8313A1C4: 7CE84A14  add r7, r8, r9
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8313A1C8: 409A0064  bne cr6, 0x8313a22c
	if !ctx.cr[6].eq {
	pc = 0x8313A22C; continue 'dispatch;
	}
	// 8313A1CC: 811F001C  lwz r8, 0x1c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313A1D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8313A1D4: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8313A1D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8313A1DC: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8313A1E0: 40990084  ble cr6, 0x8313a264
	if !ctx.cr[6].gt {
	pc = 0x8313A264; continue 'dispatch;
	}
	// 8313A1E4: 3D008334  lis r8, -0x7ccc
	ctx.r[8].s64 = -2093744128;
	// 8313A1E8: 7CDD4850  subf r6, r29, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 8313A1EC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8313A1F0: 7CFD3850  subf r7, r29, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[29].s64;
	// 8313A1F4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8313A1F8: 39085CE8  addi r8, r8, 0x5ce8
	ctx.r[8].s64 = ctx.r[8].s64 + 23784;
	// 8313A1FC: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313A200: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8313A204: 54A5083E  rotlwi r5, r5, 1
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(1)) as u64;
	// 8313A208: 7CA5422E  lhzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8313A20C: 7CA75B2E  sthx r5, r7, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 8313A210: 88AB0001  lbz r5, 1(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 8313A214: 54A5083E  rotlwi r5, r5, 1
	ctx.r[5].u64 = ((ctx.r[5].u32).rotate_left(1)) as u64;
	// 8313A218: 7CA5422E  lhzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8313A21C: 7CA65B2E  sthx r5, r6, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u16) };
	// 8313A220: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8313A224: 4082FFD8  bne 0x8313a1fc
	if !ctx.cr[0].eq {
	pc = 0x8313A1FC; continue 'dispatch;
	}
	// 8313A228: 4800003C  b 0x8313a264
	pc = 0x8313A264; continue 'dispatch;
	// 8313A22C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8313A230: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8313A234: 40990030  ble cr6, 0x8313a264
	if !ctx.cr[6].gt {
	pc = 0x8313A264; continue 'dispatch;
	}
	// 8313A238: 3D008334  lis r8, -0x7ccc
	ctx.r[8].s64 = -2093744128;
	// 8313A23C: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8313A240: 39085CE8  addi r8, r8, 0x5ce8
	ctx.r[8].s64 = ctx.r[8].s64 + 23784;
	// 8313A244: 7CEBE8AE  lbzx r7, r11, r29
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8313A248: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8313A24C: 54E7083E  rotlwi r7, r7, 1
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(1)) as u64;
	// 8313A250: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8313A254: 7CE7422E  lhzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8313A258: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 8313A25C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8313A260: 4198FFE4  blt cr6, 0x8313a244
	if ctx.cr[6].lt {
	pc = 0x8313A244; continue 'dispatch;
	}
	// 8313A264: 897E000E  lbz r11, 0xe(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(14 as u32) ) } as u64;
	// 8313A268: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8313A26C: 915E0090  stw r10, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8313A270: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8313A274: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8313A278: 913E0004  stw r9, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8313A27C: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8313A280: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313A284: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8313A288: 409A0024  bne cr6, 0x8313a2ac
	if !ctx.cr[6].eq {
	pc = 0x8313A2AC; continue 'dispatch;
	}
	// 8313A28C: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8313A290: 809E0094  lwz r4, 0x94(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 8313A294: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8313A298: 817E0080  lwz r11, 0x80(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8313A29C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313A2A0: 4E800421  bctrl
	ctx.lr = 0x8313A2A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313A2A4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8313A2A8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8313A2AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313A2B0: 4806DF0C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


