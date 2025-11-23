pub fn sub_824640B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824640B8 size=96
    let mut pc: u32 = 0x824640B8;
    'dispatch: loop {
        match pc {
            0x824640B8 => {
    //   block [0x824640B8..0x824640E0)
	// 824640B8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824640BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824640C0: 2F052000  cmpwi cr6, r5, 0x2000
	ctx.cr[6].compare_i32(ctx.r[5].s32, 8192, &mut ctx.xer);
	// 824640C4: 4199007C  bgt cr6, 0x82464140
	if ctx.cr[6].gt {
		sub_82464140(ctx, base);
		return;
	}
	// 824640C8: 2F050200  cmpwi cr6, r5, 0x200
	ctx.cr[6].compare_i32(ctx.r[5].s32, 512, &mut ctx.xer);
	// 824640CC: 41990014  bgt cr6, 0x824640e0
	if ctx.cr[6].gt {
	pc = 0x824640E0; continue 'dispatch;
	}
	// 824640D0: 7D651A14  add r11, r5, r3
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[3].u64;
	// 824640D4: 896B0104  lbz r11, 0x104(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 824640D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824640DC: 48000018  b 0x824640f4
	pc = 0x824640F4; continue 'dispatch;
            }
            0x824640E0 => {
    //   block [0x824640E0..0x824640F4)
	// 824640E0: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 824640E4: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 824640E8: 396B00C2  addi r11, r11, 0xc2
	ctx.r[11].s64 = ctx.r[11].s64 + 194;
	// 824640EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824640F0: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	pc = 0x824640F4; continue 'dispatch;
            }
            0x824640F4 => {
    //   block [0x824640F4..0x82464118)
	// 824640F4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824640F8: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 824640FC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82464100: 814A003C  lwz r10, 0x3c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82464104: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82464108: 41980010  blt cr6, 0x82464118
	if ctx.cr[6].lt {
		sub_82464118(ctx, base);
		return;
	}
	// 8246410C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82464110: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82464114: 4BFFFE04  b 0x82463f18
	sub_82463F18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464118 size=40
    let mut pc: u32 = 0x82464118;
    'dispatch: loop {
        match pc {
            0x82464118 => {
    //   block [0x82464118..0x82464140)
	// 82464118: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 8246411C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82464120: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82464124: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82464128: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246412C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82464130: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82464134: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82464138: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8246413C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464140 size=20
    let mut pc: u32 = 0x82464140;
    'dispatch: loop {
        match pc {
            0x82464140 => {
    //   block [0x82464140..0x82464154)
	// 82464140: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82464144: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464148: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246414C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464150: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464158 size=220
    let mut pc: u32 = 0x82464158;
    'dispatch: loop {
        match pc {
            0x82464158 => {
    //   block [0x82464158..0x824641D4)
	// 82464158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246415C: 480D0F61  bl 0x825350bc
	ctx.lr = 0x82464160;
	sub_82535080(ctx, base);
	// 82464160: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82464164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82464168: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246416C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82464170: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82464174: 409A0060  bne cr6, 0x824641d4
	if !ctx.cr[6].eq {
	pc = 0x824641D4; continue 'dispatch;
	}
	// 82464178: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8246417C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82464180: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82464184: 48004C75  bl 0x82468df8
	ctx.lr = 0x82464188;
	sub_82468DF8(ctx, base);
	// 82464188: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246418C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82464190: 388B6CE8  addi r4, r11, 0x6ce8
	ctx.r[4].s64 = ctx.r[11].s64 + 27880;
	// 82464194: 4800410D  bl 0x824682a0
	ctx.lr = 0x82464198;
	sub_824682A0(ctx, base);
	// 82464198: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8246419C: 3CA02BEB  lis r5, 0x2beb
	ctx.r[5].s64 = 736821248;
	// 824641A0: 3900012D  li r8, 0x12d
	ctx.r[8].s64 = 301;
	// 824641A4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824641A8: 60A5BA62  ori r5, r5, 0xba62
	ctx.r[5].u64 = ctx.r[5].u64 | 47714;
	// 824641AC: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824641B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824641B4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 824641B8: 38EB6CCC  addi r7, r11, 0x6ccc
	ctx.r[7].s64 = ctx.r[11].s64 + 27852;
	// 824641BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824641C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824641C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824641C8: 4E800421  bctrl
	ctx.lr = 0x824641CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824641CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824641D0: 48004711  bl 0x824688e0
	ctx.lr = 0x824641D4;
	sub_824688E0(ctx, base);
            }
            0x824641D4 => {
    //   block [0x824641D4..0x824641E4)
	// 824641D4: 3BDD0400  addi r30, r29, 0x400
	ctx.r[30].s64 = ctx.r[29].s64 + 1024;
	// 824641D8: 2F1E1000  cmpwi cr6, r30, 0x1000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4096, &mut ctx.xer);
	// 824641DC: 41990008  bgt cr6, 0x824641e4
	if ctx.cr[6].gt {
	pc = 0x824641E4; continue 'dispatch;
	}
	// 824641E0: 3BC01000  li r30, 0x1000
	ctx.r[30].s64 = 4096;
	pc = 0x824641E4; continue 'dispatch;
            }
            0x824641E4 => {
    //   block [0x824641E4..0x82464234)
	// 824641E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824641E8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824641EC: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824641F0: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 824641F4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824641F8: 4BFFFE41  bl 0x82464038
	ctx.lr = 0x824641FC;
	sub_82464038(ctx, base);
	// 824641FC: E95F0020  ld r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 82464200: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82464204: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82464208: 7D23EA14  add r9, r3, r29
	ctx.r[9].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 8246420C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82464210: 7D03F214  add r8, r3, r30
	ctx.r[8].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82464214: E95F0028  ld r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 82464218: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8246421C: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82464220: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82464224: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 82464228: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8246422C: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82464230: 480D0EDC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464238 size=48
    let mut pc: u32 = 0x82464238;
    'dispatch: loop {
        match pc {
            0x82464238 => {
    //   block [0x82464238..0x82464268)
	// 82464238: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8246423C: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82464240: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82464244: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82464248: 388BFFF0  addi r4, r11, -0x10
	ctx.r[4].s64 = ctx.r[11].s64 + -16;
	// 8246424C: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82464250: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82464254: E9640000  ld r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82464258: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8246425C: E9640008  ld r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82464260: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82464264: 4BFFFE54  b 0x824640b8
	sub_824640B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464268 size=88
    let mut pc: u32 = 0x82464268;
    'dispatch: loop {
        match pc {
            0x82464268 => {
    //   block [0x82464268..0x824642C0)
	// 82464268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246426C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82464270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82464274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82464278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246427C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82464280: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82464284: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82464288: 4BFFFDB1  bl 0x82464038
	ctx.lr = 0x8246428C;
	sub_82464038(ctx, base);
	// 8246428C: 3D400234  lis r10, 0x234
	ctx.r[10].s64 = 36962304;
	// 82464290: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82464294: 614A5656  ori r10, r10, 0x5656
	ctx.r[10].u64 = ctx.r[10].u64 | 22102;
	// 82464298: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8246429C: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824642A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824642A4: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 824642A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824642AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824642B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824642B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824642B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824642BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824642C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824642C0 size=40
    let mut pc: u32 = 0x824642C0;
    'dispatch: loop {
        match pc {
            0x824642C0 => {
    //   block [0x824642C0..0x824642E8)
	// 824642C0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824642C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824642C8: 3884FFF0  addi r4, r4, -0x10
	ctx.r[4].s64 = ctx.r[4].s64 + -16;
	// 824642CC: 3D60DEAD  lis r11, -0x2153
	ctx.r[11].s64 = -559087616;
	// 824642D0: 616ABEEF  ori r10, r11, 0xbeef
	ctx.r[10].u64 = ctx.r[11].u64 | 48879;
	// 824642D4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824642D8: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 824642DC: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 824642E0: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824642E4: 4BFFFDD4  b 0x824640b8
	sub_824640B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824642F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824642F0 size=92
    let mut pc: u32 = 0x824642F0;
    'dispatch: loop {
        match pc {
            0x824642F0 => {
    //   block [0x824642F0..0x8246434C)
	// 824642F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824642F4: 480D0DC9  bl 0x825350bc
	ctx.lr = 0x824642F8;
	sub_82535080(ctx, base);
	// 824642F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824642FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82464300: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82464304: 7FBF2A14  add r29, r31, r5
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[5].u64;
	// 82464308: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246430C: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 82464310: 4BFFFD29  bl 0x82464038
	ctx.lr = 0x82464314;
	sub_82464038(ctx, base);
	// 82464314: 3D600234  lis r11, 0x234
	ctx.r[11].s64 = 36962304;
	// 82464318: 395FFFFF  addi r10, r31, -1
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	// 8246431C: 61695656  ori r9, r11, 0x5656
	ctx.r[9].u64 = ctx.r[11].u64 | 22102;
	// 82464320: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82464324: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82464328: 7D6B5078  andc r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[10].u64;
	// 8246432C: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82464330: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82464334: 912BFFF0  stw r9, -0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), ctx.r[9].u32 ) };
	// 82464338: 93ABFFF4  stw r29, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[29].u32 ) };
	// 8246433C: 93CBFFF8  stw r30, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[30].u32 ) };
	// 82464340: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 82464344: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82464348: 480D0DC4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464350 size=48
    let mut pc: u32 = 0x82464350;
    'dispatch: loop {
        match pc {
            0x82464350 => {
    //   block [0x82464350..0x82464380)
	// 82464350: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82464354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82464358: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246435C: 3D40DEAD  lis r10, -0x2153
	ctx.r[10].s64 = -559087616;
	// 82464360: 810BFFFC  lwz r8, -4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82464364: 80CBFFF8  lwz r6, -8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82464368: 6149BEEF  ori r9, r10, 0xbeef
	ctx.r[9].u64 = ctx.r[10].u64 | 48879;
	// 8246436C: 814BFFF4  lwz r10, -0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82464370: 7C885850  subf r4, r8, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82464374: 38AA0010  addi r5, r10, 0x10
	ctx.r[5].s64 = ctx.r[10].s64 + 16;
	// 82464378: 912BFFF0  stw r9, -0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), ctx.r[9].u32 ) };
	// 8246437C: 4BFFFD3C  b 0x824640b8
	sub_824640B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464388 size=104
    let mut pc: u32 = 0x82464388;
    'dispatch: loop {
        match pc {
            0x82464388 => {
    //   block [0x82464388..0x824643D8)
	// 82464388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246438C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82464390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82464394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82464398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246439C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824643A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824643A4: 396B6CB0  addi r11, r11, 0x6cb0
	ctx.r[11].s64 = ctx.r[11].s64 + 27824;
	// 824643A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824643AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824643B0: 4BFFFA49  bl 0x82463df8
	ctx.lr = 0x824643B4;
	sub_82463DF8(ctx, base);
	// 824643B4: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824643B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824643BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824643C0: 419A0018  beq cr6, 0x824643d8
	if ctx.cr[6].eq {
	pc = 0x824643D8; continue 'dispatch;
	}
	// 824643C4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 824643C8: 816B49B0  lwz r11, 0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18864 as u32) ) } as u64;
	// 824643CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824643D0: 4E800421  bctrl
	ctx.lr = 0x824643D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824643D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            0x824643D8 => {
    //   block [0x824643D8..0x824643F0)
	// 824643D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824643DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824643E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824643E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824643E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824643EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824643F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824643F0 size=4
    let mut pc: u32 = 0x824643F0;
    'dispatch: loop {
        match pc {
            0x824643F0 => {
    //   block [0x824643F0..0x824643F4)
	// 824643F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824643F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824643F8 size=4
    let mut pc: u32 = 0x824643F8;
    'dispatch: loop {
        match pc {
            0x824643F8 => {
    //   block [0x824643F8..0x824643FC)
	// 824643F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464410 size=152
    let mut pc: u32 = 0x82464410;
    'dispatch: loop {
        match pc {
            0x82464410 => {
    //   block [0x82464410..0x82464464)
	// 82464410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82464414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82464418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246441C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82464420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82464424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82464428: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246442C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82464430: 48005589  bl 0x824699b8
	ctx.lr = 0x82464434;
	sub_824699B8(ctx, base);
	// 82464434: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82464438: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246443C: 409A0028  bne cr6, 0x82464464
	if !ctx.cr[6].eq {
	pc = 0x82464464; continue 'dispatch;
	}
	// 82464440: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464444: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82464448: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8246444C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82464450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82464454: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82464458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246445C: 4E800421  bctrl
	ctx.lr = 0x82464460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464460: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
            }
            0x82464464 => {
    //   block [0x82464464..0x824644A8)
	// 82464464: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464468: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246446C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464470: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82464474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464478: 4E800421  bctrl
	ctx.lr = 0x8246447C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246447C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82464484: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82464488: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246448C: 4E800421  bctrl
	ctx.lr = 0x82464490;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82464494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82464498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246449C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824644A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824644A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824644A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824644A8 size=40
    let mut pc: u32 = 0x824644A8;
    'dispatch: loop {
        match pc {
            0x824644A8 => {
    //   block [0x824644A8..0x824644D0)
	// 824644A8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 824644AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824644B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824644B4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824644B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824644BC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824644C0: 480053F8  b 0x824698b8
	sub_824698B8(ctx, base);
	return;
	// 824644C4: 4E800020  blr
	return;
	// 824644C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824644D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824644D0 size=4
    let mut pc: u32 = 0x824644D0;
    'dispatch: loop {
        match pc {
            0x824644D0 => {
    //   block [0x824644D0..0x824644D4)
	// 824644D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824644D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824644D8 size=4
    let mut pc: u32 = 0x824644D8;
    'dispatch: loop {
        match pc {
            0x824644D8 => {
    //   block [0x824644D8..0x824644DC)
	// 824644D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824644E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824644E0 size=32
    let mut pc: u32 = 0x824644E0;
    'dispatch: loop {
        match pc {
            0x824644E0 => {
    //   block [0x824644E0..0x82464500)
	// 824644E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824644E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824644E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824644EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824644F0: 396B7374  addi r11, r11, 0x7374
	ctx.r[11].s64 = ctx.r[11].s64 + 29556;
	// 824644F4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824644F8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824644FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464500 size=20
    let mut pc: u32 = 0x82464500;
    'dispatch: loop {
        match pc {
            0x82464500 => {
    //   block [0x82464500..0x82464514)
	// 82464500: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464504: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82464508: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246450C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464510: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464518 size=12
    let mut pc: u32 = 0x82464518;
    'dispatch: loop {
        match pc {
            0x82464518 => {
    //   block [0x82464518..0x82464524)
	// 82464518: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246451C: 386B7374  addi r3, r11, 0x7374
	ctx.r[3].s64 = ctx.r[11].s64 + 29556;
	// 82464520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464528 size=4
    let mut pc: u32 = 0x82464528;
    'dispatch: loop {
        match pc {
            0x82464528 => {
    //   block [0x82464528..0x8246452C)
	// 82464528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464530 size=44
    let mut pc: u32 = 0x82464530;
    'dispatch: loop {
        match pc {
            0x82464530 => {
    //   block [0x82464530..0x8246455C)
	// 82464530: 2F042000  cmpwi cr6, r4, 0x2000
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8192, &mut ctx.xer);
	// 82464534: 4199004C  bgt cr6, 0x82464580
	if ctx.cr[6].gt {
		sub_82464580(ctx, base);
		return;
	}
	// 82464538: 2F040200  cmpwi cr6, r4, 0x200
	ctx.cr[6].compare_i32(ctx.r[4].s32, 512, &mut ctx.xer);
	// 8246453C: 41990020  bgt cr6, 0x8246455c
	if ctx.cr[6].gt {
		sub_8246455C(ctx, base);
		return;
	}
	// 82464540: 7D641A14  add r11, r4, r3
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 82464544: 896B0AF0  lbz r11, 0xaf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82464548: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246454C: 396B02AB  addi r11, r11, 0x2ab
	ctx.r[11].s64 = ctx.r[11].s64 + 683;
	// 82464550: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82464554: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82464558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246455C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246455C size=36
    let mut pc: u32 = 0x8246455C;
    'dispatch: loop {
        match pc {
            0x8246455C => {
    //   block [0x8246455C..0x82464580)
	// 8246455C: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82464560: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82464564: 396B033D  addi r11, r11, 0x33d
	ctx.r[11].s64 = ctx.r[11].s64 + 829;
	// 82464568: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246456C: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82464570: 396B02AB  addi r11, r11, 0x2ab
	ctx.r[11].s64 = ctx.r[11].s64 + 683;
	// 82464574: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82464578: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8246457C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464580 size=4
    let mut pc: u32 = 0x82464580;
    'dispatch: loop {
        match pc {
            0x82464580 => {
    //   block [0x82464580..0x82464584)
	// 82464580: 480021A0  b 0x82466720
	sub_82466720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464588 size=8
    let mut pc: u32 = 0x82464588;
    'dispatch: loop {
        match pc {
            0x82464588 => {
    //   block [0x82464588..0x82464590)
	// 82464588: 80630D74  lwz r3, 0xd74(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3444 as u32) ) } as u64;
	// 8246458C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464590 size=76
    let mut pc: u32 = 0x82464590;
    'dispatch: loop {
        match pc {
            0x82464590 => {
    //   block [0x82464590..0x8246459C)
	// 82464590: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82464594: 39630D18  addi r11, r3, 0xd18
	ctx.r[11].s64 = ctx.r[3].s64 + 3352;
	// 82464598: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	pc = 0x8246459C; continue 'dispatch;
            }
            0x8246459C => {
    //   block [0x8246459C..0x824645A8)
	// 8246459C: 812BFD54  lwz r9, -0x2ac(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-684 as u32) ) } as u64;
	// 824645A0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824645A4: 419A0010  beq cr6, 0x824645b4
	if ctx.cr[6].eq {
	pc = 0x824645B4; continue 'dispatch;
	}
	pc = 0x824645A8; continue 'dispatch;
            }
            0x824645A8 => {
    //   block [0x824645A8..0x824645B4)
	// 824645A8: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824645AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824645B0: 409AFFF8  bne cr6, 0x824645a8
	if !ctx.cr[6].eq {
	pc = 0x824645A8; continue 'dispatch;
	}
	pc = 0x824645B4; continue 'dispatch;
            }
            0x824645B4 => {
    //   block [0x824645B4..0x824645DC)
	// 824645B4: 812BFD98  lwz r9, -0x268(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-616 as u32) ) } as u64;
	// 824645B8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824645BC: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824645C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824645C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824645C8: 7D2939D6  mullw r9, r9, r7
	ctx.r[9].s32 = ((ctx.r[9].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 824645CC: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824645D0: 409AFFCC  bne cr6, 0x8246459c
	if !ctx.cr[6].eq {
	pc = 0x8246459C; continue 'dispatch;
	}
	// 824645D4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824645D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824645E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824645E0 size=104
    let mut pc: u32 = 0x824645E0;
    'dispatch: loop {
        match pc {
            0x824645E0 => {
    //   block [0x824645E0..0x82464610)
	// 824645E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824645E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824645E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824645EC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824645F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824645F4: 409A001C  bne cr6, 0x82464610
	if !ctx.cr[6].eq {
	pc = 0x82464610; continue 'dispatch;
	}
	// 824645F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824645FC: 91660D78  stw r11, 0xd78(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(3448 as u32), ctx.r[11].u32 ) };
	// 82464600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82464604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82464608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246460C: 4E800020  blr
	return;
            }
            0x82464610 => {
    //   block [0x82464610..0x82464638)
	// 82464610: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82464614: 4BFFFF7D  bl 0x82464590
	ctx.lr = 0x82464618;
	sub_82464590(ctx, base);
	// 82464618: 81660D5C  lwz r11, 0xd5c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3420 as u32) ) } as u64;
	// 8246461C: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82464620: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82464624: 41980014  blt cr6, 0x82464638
	if ctx.cr[6].lt {
	pc = 0x82464638; continue 'dispatch;
	}
	// 82464628: 81660D7C  lwz r11, 0xd7c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3452 as u32) ) } as u64;
	// 8246462C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82464630: 41980008  blt cr6, 0x82464638
	if ctx.cr[6].lt {
	pc = 0x82464638; continue 'dispatch;
	}
	// 82464634: 90860D78  stw r4, 0xd78(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(3448 as u32), ctx.r[4].u32 ) };
	pc = 0x82464638; continue 'dispatch;
            }
            0x82464638 => {
    //   block [0x82464638..0x82464648)
	// 82464638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246463C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82464640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82464644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464648 size=20
    let mut pc: u32 = 0x82464648;
    'dispatch: loop {
        match pc {
            0x82464648 => {
    //   block [0x82464648..0x8246465C)
	// 82464648: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8246464C: 409A0010  bne cr6, 0x8246465c
	if !ctx.cr[6].eq {
		sub_8246465C(ctx, base);
		return;
	}
	// 82464650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82464654: 91630D7C  stw r11, 0xd7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3452 as u32), ctx.r[11].u32 ) };
	// 82464658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246465C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246465C size=28
    let mut pc: u32 = 0x8246465C;
    'dispatch: loop {
        match pc {
            0x8246465C => {
    //   block [0x8246465C..0x82464670)
	// 8246465C: 81630D78  lwz r11, 0xd78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82464660: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82464664: 419A000C  beq cr6, 0x82464670
	if ctx.cr[6].eq {
	pc = 0x82464670; continue 'dispatch;
	}
	// 82464668: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8246466C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	pc = 0x82464670; continue 'dispatch;
            }
            0x82464670 => {
    //   block [0x82464670..0x82464678)
	// 82464670: 90830D7C  stw r4, 0xd7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3452 as u32), ctx.r[4].u32 ) };
	// 82464674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464678 size=8
    let mut pc: u32 = 0x82464678;
    'dispatch: loop {
        match pc {
            0x82464678 => {
    //   block [0x82464678..0x82464680)
	// 82464678: 80630D78  lwz r3, 0xd78(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3448 as u32) ) } as u64;
	// 8246467C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464680 size=8
    let mut pc: u32 = 0x82464680;
    'dispatch: loop {
        match pc {
            0x82464680 => {
    //   block [0x82464680..0x82464688)
	// 82464680: 80630D7C  lwz r3, 0xd7c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3452 as u32) ) } as u64;
	// 82464684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464688 size=16
    let mut pc: u32 = 0x82464688;
    'dispatch: loop {
        match pc {
            0x82464688 => {
    //   block [0x82464688..0x82464698)
	// 82464688: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246468C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82464690: 388B7488  addi r4, r11, 0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + 29832;
	// 82464694: 48003F4C  b 0x824685e0
	sub_824685E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464698 size=4
    let mut pc: u32 = 0x82464698;
    'dispatch: loop {
        match pc {
            0x82464698 => {
    //   block [0x82464698..0x8246469C)
	// 82464698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824646A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824646A0 size=44
    let mut pc: u32 = 0x824646A0;
    'dispatch: loop {
        match pc {
            0x824646A0 => {
    //   block [0x824646A0..0x824646CC)
	// 824646A0: 39650345  addi r11, r5, 0x345
	ctx.r[11].s64 = ctx.r[5].s64 + 837;
	// 824646A4: 3925029A  addi r9, r5, 0x29a
	ctx.r[9].s64 = ctx.r[5].s64 + 666;
	// 824646A8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824646AC: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824646B0: 7D2A182E  lwzx r9, r10, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824646B4: 7D0B182E  lwzx r8, r11, r3
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824646B8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824646BC: 7D2A192E  stwx r9, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 824646C0: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824646C4: 7C8B192E  stwx r4, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u32) };
	// 824646C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824646D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824646D0 size=4
    let mut pc: u32 = 0x824646D0;
    'dispatch: loop {
        match pc {
            0x824646D0 => {
    //   block [0x824646D0..0x824646D4)
	// 824646D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824646D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824646D8 size=4
    let mut pc: u32 = 0x824646D8;
    'dispatch: loop {
        match pc {
            0x824646D8 => {
    //   block [0x824646D8..0x824646DC)
	// 824646D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824646E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824646E0 size=8
    let mut pc: u32 = 0x824646E0;
    'dispatch: loop {
        match pc {
            0x824646E0 => {
    //   block [0x824646E0..0x824646E8)
	// 824646E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824646E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824646E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824646E8 size=104
    let mut pc: u32 = 0x824646E8;
    'dispatch: loop {
        match pc {
            0x824646E8 => {
    //   block [0x824646E8..0x82464714)
	// 824646E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824646EC: 480D09C9  bl 0x825350b4
	ctx.lr = 0x824646F0;
	sub_82535080(ctx, base);
	// 824646F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824646F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824646F8: 39602100  li r11, 0x2100
	ctx.r[11].s64 = 8448;
	// 824646FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82464700: 7FA45BD6  divw r29, r4, r11
	ctx.r[29].s32 = ctx.r[4].s32 / ctx.r[11].s32;
	// 82464704: 83DC0A1C  lwz r30, 0xa1c(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82464708: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8246470C: 40990038  ble cr6, 0x82464744
	if !ctx.cr[6].gt {
	pc = 0x82464744; continue 'dispatch;
	}
	// 82464710: 3F608273  lis r27, -0x7d8d
	ctx.r[27].s64 = -2106392576;
	pc = 0x82464714; continue 'dispatch;
            }
            0x82464714 => {
    //   block [0x82464714..0x82464744)
	// 82464714: 817B49AC  lwz r11, 0x49ac(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464718: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 8246471C: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82464720: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464724: 4E800421  bctrl
	ctx.lr = 0x82464728;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464728: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246472C: 419A0018  beq cr6, 0x82464744
	if ctx.cr[6].eq {
	pc = 0x82464744; continue 'dispatch;
	}
	// 82464730: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82464734: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82464738: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246473C: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82464740: 4198FFD4  blt cr6, 0x82464714
	if ctx.cr[6].lt {
	pc = 0x82464714; continue 'dispatch;
	}
            }
            0x82464744 => {
    //   block [0x82464744..0x82464750)
	// 82464744: 93DC0A1C  stw r30, 0xa1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2588 as u32), ctx.r[30].u32 ) };
	// 82464748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246474C: 480D09B8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82464750 size=72
    let mut pc: u32 = 0x82464750;
    'dispatch: loop {
        match pc {
            0x82464750 => {
    //   block [0x82464750..0x82464764)
	// 82464750: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82464754: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82464758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246475C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82464760: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	pc = 0x82464764; continue 'dispatch;
            }
            0x82464764 => {
    //   block [0x82464764..0x82464784)
	// 82464764: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82464768: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8246476C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82464770: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82464774: 7F054040  cmplw cr6, r5, r8
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82464778: 4098000C  bge cr6, 0x82464784
	if !ctx.cr[6].lt {
	pc = 0x82464784; continue 'dispatch;
	}
	// 8246477C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82464780: 48000008  b 0x82464788
	pc = 0x82464788; continue 'dispatch;
            }
            0x82464784 => {
    //   block [0x82464784..0x82464788)
	// 82464784: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	pc = 0x82464788; continue 'dispatch;
            }
            0x82464788 => {
    //   block [0x82464788..0x82464798)
	// 82464788: 7D635050  subf r11, r3, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 8246478C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82464790: 4199FFD4  bgt cr6, 0x82464764
	if ctx.cr[6].gt {
	pc = 0x82464764; continue 'dispatch;
	}
	// 82464794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464798 size=76
    let mut pc: u32 = 0x82464798;
    'dispatch: loop {
        match pc {
            0x82464798 => {
    //   block [0x82464798..0x824647E4)
	// 82464798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246479C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824647A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824647A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824647A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824647AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824647B0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824647B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824647B8: 4E800421  bctrl
	ctx.lr = 0x824647BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824647BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824647C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824647C4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 824647C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824647CC: 4E800421  bctrl
	ctx.lr = 0x824647D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824647D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824647D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824647D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824647DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824647E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824647E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824647E8 size=60
    let mut pc: u32 = 0x824647E8;
    'dispatch: loop {
        match pc {
            0x824647E8 => {
    //   block [0x824647E8..0x82464824)
	// 824647E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824647EC: 480D08D1  bl 0x825350bc
	ctx.lr = 0x824647F0;
	sub_82535080(ctx, base);
	// 824647F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824647F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824647F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824647FC: 3BFE0A30  addi r31, r30, 0xa30
	ctx.r[31].s64 = ctx.r[30].s64 + 2608;
	// 82464800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82464804: 48001975  bl 0x82466178
	ctx.lr = 0x82464808;
	sub_82466178(ctx, base);
	// 82464808: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8246480C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82464810: 93BE0D74  stw r29, 0xd74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(3444 as u32), ctx.r[29].u32 ) };
	// 82464814: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82464818: 482A8A55  bl 0x8270d26c
	ctx.lr = 0x8246481C;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8246481C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82464820: 480D08EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82464828 size=192
    let mut pc: u32 = 0x82464828;
    'dispatch: loop {
        match pc {
            0x82464828 => {
    //   block [0x82464828..0x82464858)
	// 82464828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246482C: 480D0885  bl 0x825350b0
	ctx.lr = 0x82464830;
	sub_82535080(ctx, base);
	// 82464830: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82464834: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82464838: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246483C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82464840: 83BF0D7C  lwz r29, 0xd7c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3452 as u32) ) } as u64;
	// 82464844: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464848: 409A0024  bne cr6, 0x8246486c
	if !ctx.cr[6].eq {
	pc = 0x8246486C; continue 'dispatch;
	}
	// 8246484C: 83BF0D78  lwz r29, 0xd78(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82464850: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464854: 409A0018  bne cr6, 0x8246486c
	if !ctx.cr[6].eq {
	pc = 0x8246486C; continue 'dispatch;
	}
	pc = 0x82464858; continue 'dispatch;
            }
            0x82464858 => {
    //   block [0x82464858..0x8246486C)
	// 82464858: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246485C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82464860: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82464864: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82464868: 480D0898  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            0x8246486C => {
    //   block [0x8246486C..0x82464878)
	// 8246486C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82464870: 409A0008  bne cr6, 0x82464878
	if !ctx.cr[6].eq {
	pc = 0x82464878; continue 'dispatch;
	}
	// 82464874: 3B601000  li r27, 0x1000
	ctx.r[27].s64 = 4096;
	pc = 0x82464878; continue 'dispatch;
            }
            0x82464878 => {
    //   block [0x82464878..0x824648E8)
	// 82464878: 817F0D68  lwz r11, 0xd68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3432 as u32) ) } as u64;
	// 8246487C: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82464880: 815F0D5C  lwz r10, 0xd5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82464884: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82464888: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246488C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82464890: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82464894: 4198FFC4  blt cr6, 0x82464858
	if ctx.cr[6].lt {
	pc = 0x82464858; continue 'dispatch;
	}
	// 82464898: 3BDF0A30  addi r30, r31, 0xa30
	ctx.r[30].s64 = ctx.r[31].s64 + 2608;
	// 8246489C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824648A0: 480018D9  bl 0x82466178
	ctx.lr = 0x824648A4;
	sub_82466178(ctx, base);
	// 824648A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824648A8: 4BFFFCE9  bl 0x82464590
	ctx.lr = 0x824648AC;
	sub_82464590(ctx, base);
	// 824648AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824648B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824648B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824648B8: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 824648BC: 482A89B1  bl 0x8270d26c
	ctx.lr = 0x824648C0;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 824648C0: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 824648C4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824648C8: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 824648CC: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 824648D0: 7D7D5810  subfc r11, r29, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[29].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 824648D4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 824648D8: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 824648DC: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824648E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824648E4: 480D081C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824648E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824648E8 size=116
    let mut pc: u32 = 0x824648E8;
    'dispatch: loop {
        match pc {
            0x824648E8 => {
    //   block [0x824648E8..0x8246491C)
	// 824648E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824648EC: 480D07D1  bl 0x825350bc
	ctx.lr = 0x824648F0;
	sub_82535080(ctx, base);
	// 824648F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824648F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824648F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824648FC: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 82464900: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82464904: 48001875  bl 0x82466178
	ctx.lr = 0x82464908;
	sub_82466178(ctx, base);
	// 82464908: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8246490C: 409A0010  bne cr6, 0x8246491c
	if !ctx.cr[6].eq {
	pc = 0x8246491C; continue 'dispatch;
	}
	// 82464910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82464914: 917F0D78  stw r11, 0xd78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3448 as u32), ctx.r[11].u32 ) };
	// 82464918: 4800002C  b 0x82464944
	pc = 0x82464944; continue 'dispatch;
            }
            0x8246491C => {
    //   block [0x8246491C..0x82464944)
	// 8246491C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82464920: 4BFFFC71  bl 0x82464590
	ctx.lr = 0x82464924;
	sub_82464590(ctx, base);
	// 82464924: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82464928: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 8246492C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82464930: 41980014  blt cr6, 0x82464944
	if ctx.cr[6].lt {
	pc = 0x82464944; continue 'dispatch;
	}
	// 82464934: 817F0D7C  lwz r11, 0xd7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3452 as u32) ) } as u64;
	// 82464938: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8246493C: 41980008  blt cr6, 0x82464944
	if ctx.cr[6].lt {
	pc = 0x82464944; continue 'dispatch;
	}
	// 82464940: 93DF0D78  stw r30, 0xd78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3448 as u32), ctx.r[30].u32 ) };
	pc = 0x82464944; continue 'dispatch;
            }
            0x82464944 => {
    //   block [0x82464944..0x8246495C)
	// 82464944: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82464948: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246494C: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82464950: 482A891D  bl 0x8270d26c
	ctx.lr = 0x82464954;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82464954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82464958: 480D07B4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464960 size=100
    let mut pc: u32 = 0x82464960;
    'dispatch: loop {
        match pc {
            0x82464960 => {
    //   block [0x82464960..0x82464994)
	// 82464960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82464964: 480D0759  bl 0x825350bc
	ctx.lr = 0x82464968;
	sub_82535080(ctx, base);
	// 82464968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246496C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82464970: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82464974: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 82464978: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246497C: 480017FD  bl 0x82466178
	ctx.lr = 0x82464980;
	sub_82466178(ctx, base);
	// 82464980: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82464984: 409A0010  bne cr6, 0x82464994
	if !ctx.cr[6].eq {
	pc = 0x82464994; continue 'dispatch;
	}
	// 82464988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246498C: 917F0D7C  stw r11, 0xd7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3452 as u32), ctx.r[11].u32 ) };
	// 82464990: 4800001C  b 0x824649ac
	pc = 0x824649AC; continue 'dispatch;
            }
            0x82464994 => {
    //   block [0x82464994..0x824649A8)
	// 82464994: 817F0D78  lwz r11, 0xd78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82464998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246499C: 419A000C  beq cr6, 0x824649a8
	if ctx.cr[6].eq {
	pc = 0x824649A8; continue 'dispatch;
	}
	// 824649A0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824649A4: 41990008  bgt cr6, 0x824649ac
	if ctx.cr[6].gt {
	pc = 0x824649AC; continue 'dispatch;
	}
	pc = 0x824649A8; continue 'dispatch;
            }
            0x824649A8 => {
    //   block [0x824649A8..0x824649AC)
	// 824649A8: 93DF0D7C  stw r30, 0xd7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3452 as u32), ctx.r[30].u32 ) };
	pc = 0x824649AC; continue 'dispatch;
            }
            0x824649AC => {
    //   block [0x824649AC..0x824649C4)
	// 824649AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824649B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824649B4: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 824649B8: 482A88B5  bl 0x8270d26c
	ctx.lr = 0x824649BC;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 824649BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824649C0: 480D074C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824649C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824649C8 size=192
    let mut pc: u32 = 0x824649C8;
    'dispatch: loop {
        match pc {
            0x824649C8 => {
    //   block [0x824649C8..0x82464A44)
	// 824649C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824649CC: 480D06F1  bl 0x825350bc
	ctx.lr = 0x824649D0;
	sub_82535080(ctx, base);
	// 824649D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824649D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824649D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824649DC: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 824649E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824649E4: 48001795  bl 0x82466178
	ctx.lr = 0x824649E8;
	sub_82466178(ctx, base);
	// 824649E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824649EC: 4BFFFBA5  bl 0x82464590
	ctx.lr = 0x824649F0;
	sub_82464590(ctx, base);
	// 824649F0: 817F0D64  lwz r11, 0xd64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 824649F4: 815F0D68  lwz r10, 0xd68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3432 as u32) ) } as u64;
	// 824649F8: 811F0D6C  lwz r8, 0xd6c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3436 as u32) ) } as u64;
	// 824649FC: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[11].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82464A00: 813F0D5C  lwz r9, 0xd5c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82464A04: 7D6859D6  mullw r11, r8, r11
	ctx.r[11].s32 = ((ctx.r[8].s32 as i64 * ctx.r[11].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82464A08: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82464A0C: 7D435050  subf r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 82464A10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82464A14: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82464A18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82464A1C: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82464A20: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82464A24: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82464A28: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82464A2C: 817F0D78  lwz r11, 0xd78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82464A30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82464A34: 409A0010  bne cr6, 0x82464a44
	if !ctx.cr[6].eq {
	pc = 0x82464A44; continue 'dispatch;
	}
	// 82464A38: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82464A3C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82464A40: 4800002C  b 0x82464a6c
	pc = 0x82464A6C; continue 'dispatch;
            }
            0x82464A44 => {
    //   block [0x82464A44..0x82464A6C)
	// 82464A44: 817F0D78  lwz r11, 0xd78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82464A48: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82464A4C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82464A50: 815F0D68  lwz r10, 0xd68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3432 as u32) ) } as u64;
	// 82464A54: 817F0D64  lwz r11, 0xd64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82464A58: 813F0D78  lwz r9, 0xd78(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 82464A5C: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82464A60: 815F0D5C  lwz r10, 0xd5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82464A64: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82464A68: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	pc = 0x82464A6C; continue 'dispatch;
            }
            0x82464A6C => {
    //   block [0x82464A6C..0x82464A88)
	// 82464A6C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82464A70: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82464A74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82464A78: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82464A7C: 482A87F1  bl 0x8270d26c
	ctx.lr = 0x82464A80;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82464A80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82464A84: 480D0688  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464A88 size=1128
    let mut pc: u32 = 0x82464A88;
    'dispatch: loop {
        match pc {
            0x82464A88 => {
    //   block [0x82464A88..0x82464AC8)
	// 82464A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82464A8C: 480D0619  bl 0x825350a4
	ctx.lr = 0x82464A90;
	sub_82535080(ctx, base);
	// 82464A90: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82464A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82464A98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82464A9C: 3AFF0A30  addi r23, r31, 0xa30
	ctx.r[23].s64 = ctx.r[31].s64 + 2608;
	// 82464AA0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82464AA4: 480016D5  bl 0x82466178
	ctx.lr = 0x82464AA8;
	sub_82466178(ctx, base);
	// 82464AA8: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82464AAC: 4199037C  bgt cr6, 0x82464e28
	if ctx.cr[6].gt {
	pc = 0x82464E28; continue 'dispatch;
	}
	// 82464AB0: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82464AB4: 41990014  bgt cr6, 0x82464ac8
	if ctx.cr[6].gt {
	pc = 0x82464AC8; continue 'dispatch;
	}
	// 82464AB8: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82464ABC: 896B0AF0  lbz r11, 0xaf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82464AC0: 7D780774  extsb r24, r11
	ctx.r[24].s64 = ctx.r[11].s8 as i64;
	// 82464AC4: 48000018  b 0x82464adc
	pc = 0x82464ADC; continue 'dispatch;
            }
            0x82464AC8 => {
    //   block [0x82464AC8..0x82464ADC)
	// 82464AC8: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82464ACC: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82464AD0: 396B033D  addi r11, r11, 0x33d
	ctx.r[11].s64 = ctx.r[11].s64 + 829;
	// 82464AD4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82464AD8: 7F0BF82E  lwzx r24, r11, r31
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	pc = 0x82464ADC; continue 'dispatch;
            }
            0x82464ADC => {
    //   block [0x82464ADC..0x82464B28)
	// 82464ADC: 39780345  addi r11, r24, 0x345
	ctx.r[11].s64 = ctx.r[24].s64 + 837;
	// 82464AE0: 395802AB  addi r10, r24, 0x2ab
	ctx.r[10].s64 = ctx.r[24].s64 + 683;
	// 82464AE4: 5579103A  slwi r25, r11, 2
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82464AE8: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82464AEC: 3958029A  addi r10, r24, 0x29a
	ctx.r[10].s64 = ctx.r[24].s64 + 666;
	// 82464AF0: 555A103A  slwi r26, r10, 2
	ctx.r[26].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82464AF4: 7D39F82E  lwzx r9, r25, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464AF8: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464AFC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82464B00: 7D39F92E  stwx r9, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	// 82464B04: 813F0D70  lwz r9, 0xd70(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 82464B08: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82464B0C: 915F0D70  stw r10, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[10].u32 ) };
	// 82464B10: 7FBAF82E  lwzx r29, r26, r31
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464B14: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464B18: 419A0010  beq cr6, 0x82464b28
	if ctx.cr[6].eq {
	pc = 0x82464B28; continue 'dispatch;
	}
	// 82464B1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464B20: 7D7AF92E  stwx r11, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82464B24: 480003B0  b 0x82464ed4
	pc = 0x82464ED4; continue 'dispatch;
            }
            0x82464B28 => {
    //   block [0x82464B28..0x82464B64)
	// 82464B28: 7FCBF82E  lwzx r30, r11, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464B2C: 2F18000D  cmpwi cr6, r24, 0xd
	ctx.cr[6].compare_i32(ctx.r[24].s32, 13, &mut ctx.xer);
	// 82464B30: 409801D0  bge cr6, 0x82464d00
	if !ctx.cr[6].lt {
	pc = 0x82464D00; continue 'dispatch;
	}
	// 82464B34: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82464B38: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82464B3C: 815F0A24  lwz r10, 0xa24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82464B40: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82464B44: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82464B48: 409900D4  ble cr6, 0x82464c1c
	if !ctx.cr[6].gt {
	pc = 0x82464C1C; continue 'dispatch;
	}
	// 82464B4C: 817F0A1C  lwz r11, 0xa1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82464B50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82464B54: 419A0010  beq cr6, 0x82464b64
	if ctx.cr[6].eq {
	pc = 0x82464B64; continue 'dispatch;
	}
	// 82464B58: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464B5C: 915F0A1C  stw r10, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[10].u32 ) };
	// 82464B60: 48000090  b 0x82464bf0
	pc = 0x82464BF0; continue 'dispatch;
            }
            0x82464B64 => {
    //   block [0x82464B64..0x82464BE8)
	// 82464B64: 3F608273  lis r27, -0x7d8d
	ctx.r[27].s64 = -2106392576;
	// 82464B68: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82464B6C: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82464B70: 817B49AC  lwz r11, 0x49ac(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464B74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464B78: 4E800421  bctrl
	ctx.lr = 0x82464B7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464B7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82464B80: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464B84: 409A0068  bne cr6, 0x82464bec
	if !ctx.cr[6].eq {
	pc = 0x82464BEC; continue 'dispatch;
	}
	// 82464B88: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82464B8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82464B90: 419A0058  beq cr6, 0x82464be8
	if ctx.cr[6].eq {
	pc = 0x82464BE8; continue 'dispatch;
	}
	// 82464B94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464B98: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82464B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464BA0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82464BA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464BA8: 4E800421  bctrl
	ctx.lr = 0x82464BAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464BAC: 817B49AC  lwz r11, 0x49ac(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464BB0: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82464BB4: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82464BB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464BBC: 4E800421  bctrl
	ctx.lr = 0x82464BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464BC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82464BC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464BC8: 409A0024  bne cr6, 0x82464bec
	if !ctx.cr[6].eq {
	pc = 0x82464BEC; continue 'dispatch;
	}
	// 82464BCC: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82464BD0: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82464BD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464BD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464BDC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82464BE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464BE4: 4E800421  bctrl
	ctx.lr = 0x82464BE8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82464BE8 => {
    //   block [0x82464BE8..0x82464BEC)
	// 82464BE8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82464BEC; continue 'dispatch;
            }
            0x82464BEC => {
    //   block [0x82464BEC..0x82464BF0)
	// 82464BEC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x82464BF0; continue 'dispatch;
            }
            0x82464BF0 => {
    //   block [0x82464BF0..0x82464C1C)
	// 82464BF0: 813F0A18  lwz r9, 0xa18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82464BF4: 394B0100  addi r10, r11, 0x100
	ctx.r[10].s64 = ctx.r[11].s64 + 256;
	// 82464BF8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82464BFC: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82464C00: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	// 82464C04: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82464C08: 915F0A20  stw r10, 0xa20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2592 as u32), ctx.r[10].u32 ) };
	// 82464C0C: 915F0A28  stw r10, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[10].u32 ) };
	// 82464C10: 917F0D64  stw r11, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[11].u32 ) };
	// 82464C14: 396A2000  addi r11, r10, 0x2000
	ctx.r[11].s64 = ctx.r[10].s64 + 8192;
	// 82464C18: 917F0A24  stw r11, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[11].u32 ) };
	pc = 0x82464C1C; continue 'dispatch;
            }
            0x82464C1C => {
    //   block [0x82464C1C..0x82464C2C)
	// 82464C1C: 83BF0A28  lwz r29, 0xa28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82464C20: 7D7DF214  add r11, r29, r30
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82464C24: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 82464C28: 7CD9F82E  lwzx r6, r25, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	pc = 0x82464C2C; continue 'dispatch;
            }
            0x82464C2C => {
    //   block [0x82464C2C..0x82464C40)
	// 82464C2C: 2F1C0200  cmpwi cr6, r28, 0x200
	ctx.cr[6].compare_i32(ctx.r[28].s32, 512, &mut ctx.xer);
	// 82464C30: 41980010  blt cr6, 0x82464c40
	if ctx.cr[6].lt {
	pc = 0x82464C40; continue 'dispatch;
	}
	// 82464C34: 578A063E  clrlwi r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82464C38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82464C3C: 419A00BC  beq cr6, 0x82464cf8
	if ctx.cr[6].eq {
	pc = 0x82464CF8; continue 'dispatch;
	}
	pc = 0x82464C40; continue 'dispatch;
            }
            0x82464C40 => {
    //   block [0x82464C40..0x82464C7C)
	// 82464C40: 815F0A24  lwz r10, 0xa24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82464C44: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82464C48: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82464C4C: 40980030  bge cr6, 0x82464c7c
	if !ctx.cr[6].lt {
	pc = 0x82464C7C; continue 'dispatch;
	}
	// 82464C50: 7D59F82E  lwzx r10, r25, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464C54: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82464C58: 7D3AF82E  lwzx r9, r26, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464C5C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82464C60: 7D59F92E  stwx r10, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82464C64: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82464C68: 7D7AF92E  stwx r11, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82464C6C: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82464C70: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82464C74: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 82464C78: 4BFFFFB4  b 0x82464c2c
	pc = 0x82464C2C; continue 'dispatch;
            }
            0x82464C7C => {
    //   block [0x82464C7C..0x82464C94)
	// 82464C7C: 38F8FFFF  addi r7, r24, -1
	ctx.r[7].s64 = ctx.r[24].s64 + -1;
	// 82464C80: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82464C84: 40990074  ble cr6, 0x82464cf8
	if !ctx.cr[6].gt {
	pc = 0x82464CF8; continue 'dispatch;
	}
	// 82464C88: 39670345  addi r11, r7, 0x345
	ctx.r[11].s64 = ctx.r[7].s64 + 837;
	// 82464C8C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82464C90: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	pc = 0x82464C94; continue 'dispatch;
            }
            0x82464C94 => {
    //   block [0x82464C94..0x82464CAC)
	// 82464C94: 812BFD98  lwz r9, -0x268(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-616 as u32) ) } as u64;
	// 82464C98: 815F0A28  lwz r10, 0xa28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82464C9C: 811F0A24  lwz r8, 0xa24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82464CA0: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82464CA4: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82464CA8: 40980040  bge cr6, 0x82464ce8
	if !ctx.cr[6].lt {
	pc = 0x82464CE8; continue 'dispatch;
	}
	pc = 0x82464CAC; continue 'dispatch;
            }
            0x82464CAC => {
    //   block [0x82464CAC..0x82464CE8)
	// 82464CAC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464CB0: 815F0A28  lwz r10, 0xa28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82464CB4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82464CB8: 80ABFD54  lwz r5, -0x2ac(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-684 as u32) ) } as u64;
	// 82464CBC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82464CC0: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82464CC4: 914BFD54  stw r10, -0x2ac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-684 as u32), ctx.r[10].u32 ) };
	// 82464CC8: 815F0A28  lwz r10, 0xa28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82464CCC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82464CD0: 915F0A28  stw r10, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[10].u32 ) };
	// 82464CD4: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82464CD8: 811F0A24  lwz r8, 0xa24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82464CDC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82464CE0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82464CE4: 4198FFC8  blt cr6, 0x82464cac
	if ctx.cr[6].lt {
	pc = 0x82464CAC; continue 'dispatch;
	}
	pc = 0x82464CE8; continue 'dispatch;
            }
            0x82464CE8 => {
    //   block [0x82464CE8..0x82464CF8)
	// 82464CE8: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82464CEC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82464CF0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82464CF4: 4199FFA0  bgt cr6, 0x82464c94
	if ctx.cr[6].gt {
	pc = 0x82464C94; continue 'dispatch;
	}
	pc = 0x82464CF8; continue 'dispatch;
            }
            0x82464CF8 => {
    //   block [0x82464CF8..0x82464D00)
	// 82464CF8: 7CD9F92E  stwx r6, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u32) };
	// 82464CFC: 480001D8  b 0x82464ed4
	pc = 0x82464ED4; continue 'dispatch;
            }
            0x82464D00 => {
    //   block [0x82464D00..0x82464D1C)
	// 82464D00: 815F0A1C  lwz r10, 0xa1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82464D04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82464D08: 419A0014  beq cr6, 0x82464d1c
	if ctx.cr[6].eq {
	pc = 0x82464D1C; continue 'dispatch;
	}
	// 82464D0C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464D10: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82464D14: 913F0A1C  stw r9, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[9].u32 ) };
	// 82464D18: 48000090  b 0x82464da8
	pc = 0x82464DA8; continue 'dispatch;
            }
            0x82464D1C => {
    //   block [0x82464D1C..0x82464DA0)
	// 82464D1C: 3F808273  lis r28, -0x7d8d
	ctx.r[28].s64 = -2106392576;
	// 82464D20: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82464D24: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82464D28: 817C49AC  lwz r11, 0x49ac(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464D2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464D30: 4E800421  bctrl
	ctx.lr = 0x82464D34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464D34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82464D38: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464D3C: 409A0068  bne cr6, 0x82464da4
	if !ctx.cr[6].eq {
	pc = 0x82464DA4; continue 'dispatch;
	}
	// 82464D40: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82464D44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82464D48: 419A0058  beq cr6, 0x82464da0
	if ctx.cr[6].eq {
	pc = 0x82464DA0; continue 'dispatch;
	}
	// 82464D4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464D50: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82464D54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464D58: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82464D5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464D60: 4E800421  bctrl
	ctx.lr = 0x82464D64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464D64: 817C49AC  lwz r11, 0x49ac(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464D68: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82464D6C: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82464D70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464D74: 4E800421  bctrl
	ctx.lr = 0x82464D78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464D78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82464D7C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464D80: 409A0024  bne cr6, 0x82464da4
	if !ctx.cr[6].eq {
	pc = 0x82464DA4; continue 'dispatch;
	}
	// 82464D84: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82464D88: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82464D8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464D90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464D94: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82464D98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464D9C: 4E800421  bctrl
	ctx.lr = 0x82464DA0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82464DA0 => {
    //   block [0x82464DA0..0x82464DA4)
	// 82464DA0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82464DA4; continue 'dispatch;
            }
            0x82464DA4 => {
    //   block [0x82464DA4..0x82464DA8)
	// 82464DA4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x82464DA8; continue 'dispatch;
            }
            0x82464DA8 => {
    //   block [0x82464DA8..0x82464DC8)
	// 82464DA8: 815F0A18  lwz r10, 0xa18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82464DAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82464DB0: 419A0018  beq cr6, 0x82464dc8
	if ctx.cr[6].eq {
	pc = 0x82464DC8; continue 'dispatch;
	}
	// 82464DB4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464DB8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82464DBC: 815F0A18  lwz r10, 0xa18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82464DC0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82464DC4: 48000010  b 0x82464dd4
	pc = 0x82464DD4; continue 'dispatch;
            }
            0x82464DC8 => {
    //   block [0x82464DC8..0x82464DD4)
	// 82464DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82464DCC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82464DD0: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	pc = 0x82464DD4; continue 'dispatch;
            }
            0x82464DD4 => {
    //   block [0x82464DD4..0x82464DF8)
	// 82464DD4: 3BAB0100  addi r29, r11, 0x100
	ctx.r[29].s64 = ctx.r[11].s64 + 256;
	// 82464DD8: 817F0D64  lwz r11, 0xd64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82464DDC: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82464DE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82464DE4: 917F0D64  stw r11, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[11].u32 ) };
	// 82464DE8: 7D7DF214  add r11, r29, r30
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82464DEC: 7D19F82E  lwzx r8, r25, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464DF0: 40980030  bge cr6, 0x82464e20
	if !ctx.cr[6].lt {
	pc = 0x82464E20; continue 'dispatch;
	}
	// 82464DF4: 7D5D5850  subf r10, r29, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	pc = 0x82464DF8; continue 'dispatch;
            }
            0x82464DF8 => {
    //   block [0x82464DF8..0x82464E20)
	// 82464DF8: 7D39F82E  lwzx r9, r25, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464DFC: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82464E00: 7CFAF82E  lwzx r7, r26, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464E04: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82464E08: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82464E0C: 7D39F92E  stwx r9, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	// 82464E10: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82464E14: 7D7AF92E  stwx r11, r26, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82464E18: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82464E1C: 4198FFDC  blt cr6, 0x82464df8
	if ctx.cr[6].lt {
	pc = 0x82464DF8; continue 'dispatch;
	}
	pc = 0x82464E20; continue 'dispatch;
            }
            0x82464E20 => {
    //   block [0x82464E20..0x82464E28)
	// 82464E20: 7D19F92E  stwx r8, r25, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u32) };
	// 82464E24: 480000B0  b 0x82464ed4
	pc = 0x82464ED4; continue 'dispatch;
            }
            0x82464E28 => {
    //   block [0x82464E28..0x82464E50)
	// 82464E28: 817F0D5C  lwz r11, 0xd5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82464E2C: 815F0D58  lwz r10, 0xd58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3416 as u32) ) } as u64;
	// 82464E30: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82464E34: 813F0D60  lwz r9, 0xd60(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3424 as u32) ) } as u64;
	// 82464E38: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82464E3C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82464E40: 917F0D5C  stw r11, 0xd5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3420 as u32), ctx.r[11].u32 ) };
	// 82464E44: 915F0D58  stw r10, 0xd58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3416 as u32), ctx.r[10].u32 ) };
	// 82464E48: 40990008  ble cr6, 0x82464e50
	if !ctx.cr[6].gt {
	pc = 0x82464E50; continue 'dispatch;
	}
	// 82464E4C: 917F0D60  stw r11, 0xd60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3424 as u32), ctx.r[11].u32 ) };
	pc = 0x82464E50; continue 'dispatch;
            }
            0x82464E50 => {
    //   block [0x82464E50..0x82464ED4)
	// 82464E50: 3F808273  lis r28, -0x7d8d
	ctx.r[28].s64 = -2106392576;
	// 82464E54: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82464E58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82464E5C: 817C49AC  lwz r11, 0x49ac(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464E60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464E64: 4E800421  bctrl
	ctx.lr = 0x82464E68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464E68: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82464E6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464E70: 409A0064  bne cr6, 0x82464ed4
	if !ctx.cr[6].eq {
	pc = 0x82464ED4; continue 'dispatch;
	}
	// 82464E74: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82464E78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82464E7C: 419A0058  beq cr6, 0x82464ed4
	if ctx.cr[6].eq {
	pc = 0x82464ED4; continue 'dispatch;
	}
	// 82464E80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464E84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82464E88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464E8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82464E90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464E94: 4E800421  bctrl
	ctx.lr = 0x82464E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464E98: 817C49AC  lwz r11, 0x49ac(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464E9C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82464EA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82464EA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464EA8: 4E800421  bctrl
	ctx.lr = 0x82464EAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464EAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82464EB0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464EB4: 409A0020  bne cr6, 0x82464ed4
	if !ctx.cr[6].eq {
	pc = 0x82464ED4; continue 'dispatch;
	}
	// 82464EB8: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82464EBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82464EC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464EC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464EC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82464ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464ED0: 4E800421  bctrl
	ctx.lr = 0x82464ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82464ED4 => {
    //   block [0x82464ED4..0x82464EF0)
	// 82464ED4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82464ED8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82464EDC: F9770020  std r11, 0x20(r23)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[23].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82464EE0: 482A838D  bl 0x8270d26c
	ctx.lr = 0x82464EE4;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82464EE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82464EE8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82464EEC: 480D0208  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82464EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82464EF0 size=1024
    let mut pc: u32 = 0x82464EF0;
    'dispatch: loop {
        match pc {
            0x82464EF0 => {
    //   block [0x82464EF0..0x82464F30)
	// 82464EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82464EF4: 480D01A5  bl 0x82535098
	ctx.lr = 0x82464EF8;
	sub_82535080(ctx, base);
	// 82464EF8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82464EFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82464F00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82464F04: 3A9F0A30  addi r20, r31, 0xa30
	ctx.r[20].s64 = ctx.r[31].s64 + 2608;
	// 82464F08: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82464F0C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82464F10: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82464F14: 48001265  bl 0x82466178
	ctx.lr = 0x82464F18;
	sub_82466178(ctx, base);
	// 82464F18: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82464F1C: 41990014  bgt cr6, 0x82464f30
	if ctx.cr[6].gt {
	pc = 0x82464F30; continue 'dispatch;
	}
	// 82464F20: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82464F24: 896B0AF0  lbz r11, 0xaf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82464F28: 7D7A0774  extsb r26, r11
	ctx.r[26].s64 = ctx.r[11].s8 as i64;
	// 82464F2C: 48000018  b 0x82464f44
	pc = 0x82464F44; continue 'dispatch;
            }
            0x82464F30 => {
    //   block [0x82464F30..0x82464F44)
	// 82464F30: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82464F34: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82464F38: 396B033D  addi r11, r11, 0x33d
	ctx.r[11].s64 = ctx.r[11].s64 + 829;
	// 82464F3C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82464F40: 7F4BF82E  lwzx r26, r11, r31
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	pc = 0x82464F44; continue 'dispatch;
            }
            0x82464F44 => {
    //   block [0x82464F44..0x82464F68)
	// 82464F44: 397A029A  addi r11, r26, 0x29a
	ctx.r[11].s64 = ctx.r[26].s64 + 666;
	// 82464F48: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82464F4C: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82464F50: 7D5BF82E  lwzx r10, r27, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464F54: 40990350  ble cr6, 0x824652a4
	if !ctx.cr[6].gt {
	pc = 0x824652A4; continue 'dispatch;
	}
	// 82464F58: 7FB8EB78  mr r24, r29
	ctx.r[24].u64 = ctx.r[29].u64;
	// 82464F5C: 7ED5B378  mr r21, r22
	ctx.r[21].u64 = ctx.r[22].u64;
	// 82464F60: 3F208273  lis r25, -0x7d8d
	ctx.r[25].s64 = -2106392576;
	// 82464F64: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	pc = 0x82464F68; continue 'dispatch;
            }
            0x82464F68 => {
    //   block [0x82464F68..0x82464F7C)
	// 82464F68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82464F6C: 419A0010  beq cr6, 0x82464f7c
	if ctx.cr[6].eq {
	pc = 0x82464F7C; continue 'dispatch;
	}
	// 82464F70: 91580000  stw r10, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82464F74: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464F78: 4800031C  b 0x82465294
	pc = 0x82465294; continue 'dispatch;
            }
            0x82464F7C => {
    //   block [0x82464F7C..0x82464FC4)
	// 82464F7C: 395A02AB  addi r10, r26, 0x2ab
	ctx.r[10].s64 = ctx.r[26].s64 + 683;
	// 82464F80: 7EFBF92E  stwx r23, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[23].u32) };
	// 82464F84: 2F1A000D  cmpwi cr6, r26, 0xd
	ctx.cr[6].compare_i32(ctx.r[26].s32, 13, &mut ctx.xer);
	// 82464F88: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82464F8C: 7FCAF82E  lwzx r30, r10, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82464F90: 409801D8  bge cr6, 0x82465168
	if !ctx.cr[6].lt {
	pc = 0x82465168; continue 'dispatch;
	}
	// 82464F94: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82464F98: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82464F9C: 813F0A24  lwz r9, 0xa24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82464FA0: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82464FA4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82464FA8: 409900D0  ble cr6, 0x82465078
	if !ctx.cr[6].gt {
	pc = 0x82465078; continue 'dispatch;
	}
	// 82464FAC: 817F0A1C  lwz r11, 0xa1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82464FB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82464FB4: 419A0010  beq cr6, 0x82464fc4
	if ctx.cr[6].eq {
	pc = 0x82464FC4; continue 'dispatch;
	}
	// 82464FB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464FBC: 915F0A1C  stw r10, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[10].u32 ) };
	// 82464FC0: 4800008C  b 0x8246504c
	pc = 0x8246504C; continue 'dispatch;
            }
            0x82464FC4 => {
    //   block [0x82464FC4..0x82465044)
	// 82464FC4: 817949AC  lwz r11, 0x49ac(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82464FC8: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82464FCC: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82464FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82464FD4: 4E800421  bctrl
	ctx.lr = 0x82464FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82464FD8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82464FDC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82464FE0: 409A0068  bne cr6, 0x82465048
	if !ctx.cr[6].eq {
	pc = 0x82465048; continue 'dispatch;
	}
	// 82464FE4: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 82464FE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82464FEC: 419A0058  beq cr6, 0x82465044
	if ctx.cr[6].eq {
	pc = 0x82465044; continue 'dispatch;
	}
	// 82464FF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82464FF4: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82464FF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82464FFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82465000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465004: 4E800421  bctrl
	ctx.lr = 0x82465008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465008: 817949AC  lwz r11, 0x49ac(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(18860 as u32) ) } as u64;
	// 8246500C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82465010: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 82465014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465018: 4E800421  bctrl
	ctx.lr = 0x8246501C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246501C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82465020: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82465024: 409A0024  bne cr6, 0x82465048
	if !ctx.cr[6].eq {
	pc = 0x82465048; continue 'dispatch;
	}
	// 82465028: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 8246502C: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 82465030: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82465034: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465038: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246503C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465040: 4E800421  bctrl
	ctx.lr = 0x82465044;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82465044 => {
    //   block [0x82465044..0x82465048)
	// 82465044: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82465048; continue 'dispatch;
            }
            0x82465048 => {
    //   block [0x82465048..0x8246504C)
	// 82465048: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8246504C; continue 'dispatch;
            }
            0x8246504C => {
    //   block [0x8246504C..0x82465078)
	// 8246504C: 813F0A18  lwz r9, 0xa18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82465050: 394B0100  addi r10, r11, 0x100
	ctx.r[10].s64 = ctx.r[11].s64 + 256;
	// 82465054: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82465058: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 8246505C: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	// 82465060: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82465064: 915F0A20  stw r10, 0xa20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2592 as u32), ctx.r[10].u32 ) };
	// 82465068: 915F0A28  stw r10, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[10].u32 ) };
	// 8246506C: 917F0D64  stw r11, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[11].u32 ) };
	// 82465070: 396A2000  addi r11, r10, 0x2000
	ctx.r[11].s64 = ctx.r[10].s64 + 8192;
	// 82465074: 917F0A24  stw r11, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[11].u32 ) };
	pc = 0x82465078; continue 'dispatch;
            }
            0x82465078 => {
    //   block [0x82465078..0x82465090)
	// 82465078: 397A0345  addi r11, r26, 0x345
	ctx.r[11].s64 = ctx.r[26].s64 + 837;
	// 8246507C: 80BF0A28  lwz r5, 0xa28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82465080: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82465084: 7D65F214  add r11, r5, r30
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[30].u64;
	// 82465088: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 8246508C: 7C87F82E  lwzx r4, r7, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	pc = 0x82465090; continue 'dispatch;
            }
            0x82465090 => {
    //   block [0x82465090..0x824650A4)
	// 82465090: 2F1C0200  cmpwi cr6, r28, 0x200
	ctx.cr[6].compare_i32(ctx.r[28].s32, 512, &mut ctx.xer);
	// 82465094: 41980010  blt cr6, 0x824650a4
	if ctx.cr[6].lt {
	pc = 0x824650A4; continue 'dispatch;
	}
	// 82465098: 578A063E  clrlwi r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8246509C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824650A0: 419A00BC  beq cr6, 0x8246515c
	if ctx.cr[6].eq {
	pc = 0x8246515C; continue 'dispatch;
	}
	pc = 0x824650A4; continue 'dispatch;
            }
            0x824650A4 => {
    //   block [0x824650A4..0x824650E0)
	// 824650A4: 815F0A24  lwz r10, 0xa24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824650A8: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824650AC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824650B0: 40980030  bge cr6, 0x824650e0
	if !ctx.cr[6].lt {
	pc = 0x824650E0; continue 'dispatch;
	}
	// 824650B4: 7D47F82E  lwzx r10, r7, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824650B8: 7F9CF214  add r28, r28, r30
	ctx.r[28].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 824650BC: 7D3BF82E  lwzx r9, r27, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824650C0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824650C4: 7D47F92E  stwx r10, r7, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 824650C8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824650CC: 7D7BF92E  stwx r11, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 824650D0: 817F0A28  lwz r11, 0xa28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 824650D4: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 824650D8: 917F0A28  stw r11, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[11].u32 ) };
	// 824650DC: 4BFFFFB4  b 0x82465090
	pc = 0x82465090; continue 'dispatch;
            }
            0x824650E0 => {
    //   block [0x824650E0..0x824650F8)
	// 824650E0: 38DAFFFF  addi r6, r26, -1
	ctx.r[6].s64 = ctx.r[26].s64 + -1;
	// 824650E4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824650E8: 40990074  ble cr6, 0x8246515c
	if !ctx.cr[6].gt {
	pc = 0x8246515C; continue 'dispatch;
	}
	// 824650EC: 39660345  addi r11, r6, 0x345
	ctx.r[11].s64 = ctx.r[6].s64 + 837;
	// 824650F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824650F4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	pc = 0x824650F8; continue 'dispatch;
            }
            0x824650F8 => {
    //   block [0x824650F8..0x82465110)
	// 824650F8: 812BFD98  lwz r9, -0x268(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-616 as u32) ) } as u64;
	// 824650FC: 815F0A28  lwz r10, 0xa28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82465100: 811F0A24  lwz r8, 0xa24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82465104: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82465108: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8246510C: 40980040  bge cr6, 0x8246514c
	if !ctx.cr[6].lt {
	pc = 0x8246514C; continue 'dispatch;
	}
	pc = 0x82465110; continue 'dispatch;
            }
            0x82465110 => {
    //   block [0x82465110..0x8246514C)
	// 82465110: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465114: 815F0A28  lwz r10, 0xa28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82465118: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8246511C: 806BFD54  lwz r3, -0x2ac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-684 as u32) ) } as u64;
	// 82465120: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82465124: 906A0000  stw r3, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82465128: 914BFD54  stw r10, -0x2ac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-684 as u32), ctx.r[10].u32 ) };
	// 8246512C: 815F0A28  lwz r10, 0xa28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2600 as u32) ) } as u64;
	// 82465130: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82465134: 915F0A28  stw r10, 0xa28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2600 as u32), ctx.r[10].u32 ) };
	// 82465138: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246513C: 811F0A24  lwz r8, 0xa24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82465140: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82465144: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82465148: 4198FFC8  blt cr6, 0x82465110
	if ctx.cr[6].lt {
	pc = 0x82465110; continue 'dispatch;
	}
	pc = 0x8246514C; continue 'dispatch;
            }
            0x8246514C => {
    //   block [0x8246514C..0x8246515C)
	// 8246514C: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 82465150: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82465154: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82465158: 4199FFA0  bgt cr6, 0x824650f8
	if ctx.cr[6].gt {
	pc = 0x824650F8; continue 'dispatch;
	}
	pc = 0x8246515C; continue 'dispatch;
            }
            0x8246515C => {
    //   block [0x8246515C..0x82465168)
	// 8246515C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82465160: 7C87F92E  stwx r4, r7, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[31].u32), ctx.r[4].u32) };
	// 82465164: 48000128  b 0x8246528c
	pc = 0x8246528C; continue 'dispatch;
            }
            0x82465168 => {
    //   block [0x82465168..0x82465180)
	// 82465168: 817F0A1C  lwz r11, 0xa1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2588 as u32) ) } as u64;
	// 8246516C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82465170: 419A0010  beq cr6, 0x82465180
	if ctx.cr[6].eq {
	pc = 0x82465180; continue 'dispatch;
	}
	// 82465174: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465178: 915F0A1C  stw r10, 0xa1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2588 as u32), ctx.r[10].u32 ) };
	// 8246517C: 4800008C  b 0x82465208
	pc = 0x82465208; continue 'dispatch;
            }
            0x82465180 => {
    //   block [0x82465180..0x82465200)
	// 82465180: 817949AC  lwz r11, 0x49ac(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(18860 as u32) ) } as u64;
	// 82465184: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82465188: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 8246518C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465190: 4E800421  bctrl
	ctx.lr = 0x82465194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465194: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82465198: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8246519C: 409A0068  bne cr6, 0x82465204
	if !ctx.cr[6].eq {
	pc = 0x82465204; continue 'dispatch;
	}
	// 824651A0: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 824651A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824651A8: 419A0058  beq cr6, 0x82465200
	if ctx.cr[6].eq {
	pc = 0x82465200; continue 'dispatch;
	}
	// 824651AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824651B0: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 824651B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824651B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824651BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824651C0: 4E800421  bctrl
	ctx.lr = 0x824651C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824651C4: 817949AC  lwz r11, 0x49ac(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(18860 as u32) ) } as u64;
	// 824651C8: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 824651CC: 38602100  li r3, 0x2100
	ctx.r[3].s64 = 8448;
	// 824651D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824651D4: 4E800421  bctrl
	ctx.lr = 0x824651D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824651D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824651DC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824651E0: 409A0024  bne cr6, 0x82465204
	if !ctx.cr[6].eq {
	pc = 0x82465204; continue 'dispatch;
	}
	// 824651E4: 807F0D74  lwz r3, 0xd74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 824651E8: 38A02100  li r5, 0x2100
	ctx.r[5].s64 = 8448;
	// 824651EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824651F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824651F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824651F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824651FC: 4E800421  bctrl
	ctx.lr = 0x82465200;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82465200 => {
    //   block [0x82465200..0x82465204)
	// 82465200: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	pc = 0x82465204; continue 'dispatch;
            }
            0x82465204 => {
    //   block [0x82465204..0x82465208)
	// 82465204: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x82465208; continue 'dispatch;
            }
            0x82465208 => {
    //   block [0x82465208..0x82465228)
	// 82465208: 815F0A18  lwz r10, 0xa18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 8246520C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82465210: 419A0018  beq cr6, 0x82465228
	if ctx.cr[6].eq {
	pc = 0x82465228; continue 'dispatch;
	}
	// 82465214: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465218: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246521C: 815F0A18  lwz r10, 0xa18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82465220: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82465224: 4800000C  b 0x82465230
	pc = 0x82465230; continue 'dispatch;
            }
            0x82465228 => {
    //   block [0x82465228..0x82465230)
	// 82465228: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8246522C: 917F0A18  stw r11, 0xa18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	pc = 0x82465230; continue 'dispatch;
            }
            0x82465230 => {
    //   block [0x82465230..0x8246525C)
	// 82465230: 813F0D64  lwz r9, 0xd64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82465234: 395A0345  addi r10, r26, 0x345
	ctx.r[10].s64 = ctx.r[26].s64 + 837;
	// 82465238: 38EB0100  addi r7, r11, 0x100
	ctx.r[7].s64 = ctx.r[11].s64 + 256;
	// 8246523C: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82465240: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82465244: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82465248: 917F0D64  stw r11, 0xd64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3428 as u32), ctx.r[11].u32 ) };
	// 8246524C: 7D67F214  add r11, r7, r30
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[30].u64;
	// 82465250: 7CCAF82E  lwzx r6, r10, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465254: 40980030  bge cr6, 0x82465284
	if !ctx.cr[6].lt {
	pc = 0x82465284; continue 'dispatch;
	}
	// 82465258: 7D275850  subf r9, r7, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	pc = 0x8246525C; continue 'dispatch;
            }
            0x8246525C => {
    //   block [0x8246525C..0x82465284)
	// 8246525C: 7D0AF82E  lwzx r8, r10, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465260: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82465264: 7CBBF82E  lwzx r5, r27, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465268: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8246526C: 2F092000  cmpwi cr6, r9, 0x2000
	ctx.cr[6].compare_i32(ctx.r[9].s32, 8192, &mut ctx.xer);
	// 82465270: 7D0AF92E  stwx r8, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[8].u32) };
	// 82465274: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82465278: 7D7BF92E  stwx r11, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 8246527C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82465280: 4198FFDC  blt cr6, 0x8246525c
	if ctx.cr[6].lt {
	pc = 0x8246525C; continue 'dispatch;
	}
	pc = 0x82465284; continue 'dispatch;
            }
            0x82465284 => {
    //   block [0x82465284..0x8246528C)
	// 82465284: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82465288: 7CCAF92E  stwx r6, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u32) };
	pc = 0x8246528C; continue 'dispatch;
            }
            0x8246528C => {
    //   block [0x8246528C..0x82465294)
	// 8246528C: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82465290: 7D5BF82E  lwzx r10, r27, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	pc = 0x82465294; continue 'dispatch;
            }
            0x82465294 => {
    //   block [0x82465294..0x824652A4)
	// 82465294: 3AB5FFFF  addi r21, r21, -1
	ctx.r[21].s64 = ctx.r[21].s64 + -1;
	// 82465298: 3B180004  addi r24, r24, 4
	ctx.r[24].s64 = ctx.r[24].s64 + 4;
	// 8246529C: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 824652A0: 409AFCC8  bne cr6, 0x82464f68
	if !ctx.cr[6].eq {
	pc = 0x82464F68; continue 'dispatch;
	}
	pc = 0x824652A4; continue 'dispatch;
            }
            0x824652A4 => {
    //   block [0x824652A4..0x824652F0)
	// 824652A4: 393A02AB  addi r9, r26, 0x2ab
	ctx.r[9].s64 = ctx.r[26].s64 + 683;
	// 824652A8: 7D5BF92E  stwx r10, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 824652AC: 397A0345  addi r11, r26, 0x345
	ctx.r[11].s64 = ctx.r[26].s64 + 837;
	// 824652B0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824652B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824652B8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 824652BC: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 824652C0: 7D29F82E  lwzx r9, r9, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824652C4: 7D49B1D6  mullw r10, r9, r22
	ctx.r[10].s32 = ((ctx.r[9].s32 as i64 * ctx.r[22].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 824652C8: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824652CC: 7D29B214  add r9, r9, r22
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[22].u64;
	// 824652D0: 7D2BF92E  stwx r9, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	// 824652D4: 817F0D70  lwz r11, 0xd70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 824652D8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824652DC: 917F0D70  stw r11, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[11].u32 ) };
	// 824652E0: F9140020  std r8, 0x20(r20)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[20].u32.wrapping_add(32 as u32), ctx.r[8].u64 ) };
	// 824652E4: 482A7F89  bl 0x8270d26c
	ctx.lr = 0x824652E8;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 824652E8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824652EC: 480CFDFC  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824652F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824652F0 size=220
    let mut pc: u32 = 0x824652F0;
    'dispatch: loop {
        match pc {
            0x824652F0 => {
    //   block [0x824652F0..0x8246533C)
	// 824652F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824652F4: 480CFDC5  bl 0x825350b8
	ctx.lr = 0x824652F8;
	sub_82535080(ctx, base);
	// 824652F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824652FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82465300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82465304: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82465308: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8246530C: 419A00B8  beq cr6, 0x824653c4
	if ctx.cr[6].eq {
	pc = 0x824653C4; continue 'dispatch;
	}
	// 82465310: 3B9F0A30  addi r28, r31, 0xa30
	ctx.r[28].s64 = ctx.r[31].s64 + 2608;
	// 82465314: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82465318: 48000E61  bl 0x82466178
	ctx.lr = 0x8246531C;
	sub_82466178(ctx, base);
	// 8246531C: 2F1E2000  cmpwi cr6, r30, 0x2000
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8192, &mut ctx.xer);
	// 82465320: 41990074  bgt cr6, 0x82465394
	if ctx.cr[6].gt {
	pc = 0x82465394; continue 'dispatch;
	}
	// 82465324: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 82465328: 41990014  bgt cr6, 0x8246533c
	if ctx.cr[6].gt {
	pc = 0x8246533C; continue 'dispatch;
	}
	// 8246532C: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82465330: 896B0AF0  lbz r11, 0xaf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82465334: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82465338: 48000018  b 0x82465350
	pc = 0x82465350; continue 'dispatch;
            }
            0x8246533C => {
    //   block [0x8246533C..0x82465350)
	// 8246533C: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82465340: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82465344: 396B033D  addi r11, r11, 0x33d
	ctx.r[11].s64 = ctx.r[11].s64 + 829;
	// 82465348: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246534C: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	pc = 0x82465350; continue 'dispatch;
            }
            0x82465350 => {
    //   block [0x82465350..0x82465394)
	// 82465350: 394B0345  addi r10, r11, 0x345
	ctx.r[10].s64 = ctx.r[11].s64 + 837;
	// 82465354: 811F0D70  lwz r8, 0xd70(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 82465358: 38EB029A  addi r7, r11, 0x29a
	ctx.r[7].s64 = ctx.r[11].s64 + 666;
	// 8246535C: 396B02AB  addi r11, r11, 0x2ab
	ctx.r[11].s64 = ctx.r[11].s64 + 683;
	// 82465360: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82465364: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82465368: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246536C: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465370: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82465374: 917F0D70  stw r11, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[11].u32 ) };
	// 82465378: 7D69F82E  lwzx r11, r9, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8246537C: 7D0AF82E  lwzx r8, r10, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465380: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82465384: 7D69F92E  stwx r11, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u32) };
	// 82465388: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8246538C: 7FAAF92E  stwx r29, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 82465390: 48000024  b 0x824653b4
	pc = 0x824653B4; continue 'dispatch;
            }
            0x82465394 => {
    //   block [0x82465394..0x824653B4)
	// 82465394: 815F0D5C  lwz r10, 0xd5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82465398: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246539C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824653A0: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 824653A4: 915F0D5C  stw r10, 0xd5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3420 as u32), ctx.r[10].u32 ) };
	// 824653A8: 816B49B0  lwz r11, 0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18864 as u32) ) } as u64;
	// 824653AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824653B0: 4E800421  bctrl
	ctx.lr = 0x824653B4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824653B4 => {
    //   block [0x824653B4..0x824653C4)
	// 824653B4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824653B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824653BC: F97C0020  std r11, 0x20(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 824653C0: 482A7EAD  bl 0x8270d26c
	ctx.lr = 0x824653C4;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x824653C4; continue 'dispatch;
            }
            0x824653C4 => {
    //   block [0x824653C4..0x824653CC)
	// 824653C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824653C8: 480CFD40  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824653D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824653D0 size=220
    let mut pc: u32 = 0x824653D0;
    'dispatch: loop {
        match pc {
            0x824653D0 => {
    //   block [0x824653D0..0x82465410)
	// 824653D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824653D4: 480CFCE1  bl 0x825350b4
	ctx.lr = 0x824653D8;
	sub_82535080(ctx, base);
	// 824653D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824653DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824653E0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824653E4: 3B7F0A30  addi r27, r31, 0xa30
	ctx.r[27].s64 = ctx.r[31].s64 + 2608;
	// 824653E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824653EC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824653F0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824653F4: 48000D85  bl 0x82466178
	ctx.lr = 0x824653F8;
	sub_82466178(ctx, base);
	// 824653F8: 2F1E0200  cmpwi cr6, r30, 0x200
	ctx.cr[6].compare_i32(ctx.r[30].s32, 512, &mut ctx.xer);
	// 824653FC: 41990014  bgt cr6, 0x82465410
	if ctx.cr[6].gt {
	pc = 0x82465410; continue 'dispatch;
	}
	// 82465400: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 82465404: 896B0AF0  lbz r11, 0xaf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2800 as u32) ) } as u64;
	// 82465408: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 8246540C: 48000018  b 0x82465424
	pc = 0x82465424; continue 'dispatch;
            }
            0x82465410 => {
    //   block [0x82465410..0x82465424)
	// 82465410: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82465414: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82465418: 396B033D  addi r11, r11, 0x33d
	ctx.r[11].s64 = ctx.r[11].s64 + 829;
	// 8246541C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82465420: 7D0BF82E  lwzx r8, r11, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	pc = 0x82465424; continue 'dispatch;
            }
            0x82465424 => {
    //   block [0x82465424..0x82465440)
	// 82465424: 396802AB  addi r11, r8, 0x2ab
	ctx.r[11].s64 = ctx.r[8].s64 + 683;
	// 82465428: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8246542C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82465430: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82465434: 7CABF82E  lwzx r5, r11, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465438: 40990050  ble cr6, 0x82465488
	if !ctx.cr[6].gt {
	pc = 0x82465488; continue 'dispatch;
	}
	// 8246543C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	pc = 0x82465440; continue 'dispatch;
            }
            0x82465440 => {
    //   block [0x82465440..0x82465478)
	// 82465440: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465444: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82465448: 419A0030  beq cr6, 0x82465478
	if ctx.cr[6].eq {
	pc = 0x82465478; continue 'dispatch;
	}
	// 8246544C: 39680345  addi r11, r8, 0x345
	ctx.r[11].s64 = ctx.r[8].s64 + 837;
	// 82465450: 38E8029A  addi r7, r8, 0x29a
	ctx.r[7].s64 = ctx.r[8].s64 + 666;
	// 82465454: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82465458: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246545C: 7CC62A14  add r6, r6, r5
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82465460: 7CEAF82E  lwzx r7, r10, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465464: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465468: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8246546C: 7CEAF92E  stwx r7, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[7].u32) };
	// 82465470: 90690000  stw r3, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82465474: 7D2BF92E  stwx r9, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	pc = 0x82465478; continue 'dispatch;
            }
            0x82465478 => {
    //   block [0x82465478..0x82465488)
	// 82465478: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 8246547C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82465480: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82465484: 409AFFBC  bne cr6, 0x82465440
	if !ctx.cr[6].eq {
	pc = 0x82465440; continue 'dispatch;
	}
	pc = 0x82465488; continue 'dispatch;
            }
            0x82465488 => {
    //   block [0x82465488..0x824654AC)
	// 82465488: 817F0D70  lwz r11, 0xd70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3440 as u32) ) } as u64;
	// 8246548C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82465490: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82465494: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82465498: 917F0D70  stw r11, 0xd70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3440 as u32), ctx.r[11].u32 ) };
	// 8246549C: F95B0020  std r10, 0x20(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 824654A0: 482A7DCD  bl 0x8270d26c
	ctx.lr = 0x824654A4;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 824654A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824654A8: 480CFC5C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824654B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824654B0 size=144
    let mut pc: u32 = 0x824654B0;
    'dispatch: loop {
        match pc {
            0x824654B0 => {
    //   block [0x824654B0..0x82465540)
	// 824654B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824654B4: 480CFC01  bl 0x825350b4
	ctx.lr = 0x824654B8;
	sub_82535080(ctx, base);
	// 824654B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824654BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824654C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824654C4: 3BBF0A30  addi r29, r31, 0xa30
	ctx.r[29].s64 = ctx.r[31].s64 + 2608;
	// 824654C8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824654CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824654D0: 48000CA9  bl 0x82466178
	ctx.lr = 0x824654D4;
	sub_82466178(ctx, base);
	// 824654D4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824654D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824654DC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824654E0: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 824654E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824654E8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824654EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824654F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824654F4: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 824654F8: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 824654FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465500: 993E0018  stb r9, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 82465504: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82465508: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246550C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82465510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465514: 4E800421  bctrl
	ctx.lr = 0x82465518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246551C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82465520: 907E0020  stw r3, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82465524: 937E0024  stw r27, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 82465528: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246552C: 997E0028  stb r11, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82465530: F95D0020  std r10, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82465534: 482A7D39  bl 0x8270d26c
	ctx.lr = 0x82465538;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82465538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246553C: 480CFBC8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82465540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82465540 size=144
    let mut pc: u32 = 0x82465540;
    'dispatch: loop {
        match pc {
            0x82465540 => {
    //   block [0x82465540..0x82465570)
	// 82465540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82465544: 480CFB75  bl 0x825350b8
	ctx.lr = 0x82465548;
	sub_82535080(ctx, base);
	// 82465548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246554C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82465550: 3B9F0A30  addi r28, r31, 0xa30
	ctx.r[28].s64 = ctx.r[31].s64 + 2608;
	// 82465554: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82465558: 48000C21  bl 0x82466178
	ctx.lr = 0x8246555C;
	sub_82466178(ctx, base);
	// 8246555C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82465560: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82465564: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82465568: 40990048  ble cr6, 0x824655b0
	if !ctx.cr[6].gt {
	pc = 0x824655B0; continue 'dispatch;
	}
	// 8246556C: 3BDF0024  addi r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 + 36;
	pc = 0x82465570; continue 'dispatch;
            }
            0x82465570 => {
    //   block [0x82465570..0x8246559C)
	// 82465570: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82465574: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82465578: 409A0024  bne cr6, 0x8246559c
	if !ctx.cr[6].eq {
	pc = 0x8246559C; continue 'dispatch;
	}
	// 8246557C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465584: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465588: 80BEFFF8  lwz r5, -8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246558C: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82465590: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82465594: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465598: 4E800421  bctrl
	ctx.lr = 0x8246559C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246559C => {
    //   block [0x8246559C..0x824655B0)
	// 8246559C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824655A0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824655A4: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 824655A8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824655AC: 4198FFC4  blt cr6, 0x82465570
	if ctx.cr[6].lt {
	pc = 0x82465570; continue 'dispatch;
	}
	pc = 0x824655B0; continue 'dispatch;
            }
            0x824655B0 => {
    //   block [0x824655B0..0x824655D0)
	// 824655B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824655B4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824655B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824655BC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824655C0: F97C0020  std r11, 0x20(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 824655C4: 482A7CA9  bl 0x8270d26c
	ctx.lr = 0x824655C8;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 824655C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824655CC: 480CFB3C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824655D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824655D0 size=592
    let mut pc: u32 = 0x824655D0;
    'dispatch: loop {
        match pc {
            0x824655D0 => {
    //   block [0x824655D0..0x82465608)
	// 824655D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824655D4: 480CFAD9  bl 0x825350ac
	ctx.lr = 0x824655D8;
	sub_82535080(ctx, base);
	// 824655D8: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824655DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824655E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824655E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824655E8: 2F1C2000  cmpwi cr6, r28, 0x2000
	ctx.cr[6].compare_i32(ctx.r[28].s32, 8192, &mut ctx.xer);
	// 824655EC: 4098001C  bge cr6, 0x82465608
	if !ctx.cr[6].lt {
	pc = 0x82465608; continue 'dispatch;
	}
	// 824655F0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824655F4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824655F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824655FC: 4BFFEA3D  bl 0x82464038
	ctx.lr = 0x82465600;
	sub_82464038(ctx, base);
	// 82465600: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82465604: 480CFAF8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            0x82465608 => {
    //   block [0x82465608..0x82465630)
	// 82465608: 3B3E0A30  addi r25, r30, 0xa30
	ctx.r[25].s64 = ctx.r[30].s64 + 2608;
	// 8246560C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82465610: 48000B69  bl 0x82466178
	ctx.lr = 0x82465614;
	sub_82466178(ctx, base);
	// 82465614: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82465618: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8246561C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82465620: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82465624: 40990060  ble cr6, 0x82465684
	if !ctx.cr[6].gt {
	pc = 0x82465684; continue 'dispatch;
	}
	// 82465628: 54E8003E  slwi r8, r7, 0
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8246562C: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	pc = 0x82465630; continue 'dispatch;
            }
            0x82465630 => {
    //   block [0x82465630..0x8246565C)
	// 82465630: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465634: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82465638: 409A0028  bne cr6, 0x82465660
	if !ctx.cr[6].eq {
	pc = 0x82465660; continue 'dispatch;
	}
	// 8246563C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82465640: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82465644: 4198001C  blt cr6, 0x82465660
	if ctx.cr[6].lt {
	pc = 0x82465660; continue 'dispatch;
	}
	// 82465648: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246564C: 419A0010  beq cr6, 0x8246565c
	if ctx.cr[6].eq {
	pc = 0x8246565C; continue 'dispatch;
	}
	// 82465650: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82465654: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82465658: 40980008  bge cr6, 0x82465660
	if !ctx.cr[6].lt {
	pc = 0x82465660; continue 'dispatch;
	}
	pc = 0x8246565C; continue 'dispatch;
            }
            0x8246565C => {
    //   block [0x8246565C..0x82465660)
	// 8246565C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	pc = 0x82465660; continue 'dispatch;
            }
            0x82465660 => {
    //   block [0x82465660..0x82465684)
	// 82465660: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82465664: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82465668: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8246566C: 409AFFC4  bne cr6, 0x82465630
	if !ctx.cr[6].eq {
	pc = 0x82465630; continue 'dispatch;
	}
	// 82465670: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82465674: 419A0010  beq cr6, 0x82465684
	if ctx.cr[6].eq {
	pc = 0x82465684; continue 'dispatch;
	}
	// 82465678: 83E90008  lwz r31, 8(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246567C: 9B490000  stb r26, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 82465680: 48000184  b 0x82465804
	pc = 0x82465804; continue 'dispatch;
            }
            0x82465684 => {
    //   block [0x82465684..0x82465694)
	// 82465684: 397C0040  addi r11, r28, 0x40
	ctx.r[11].s64 = ctx.r[28].s64 + 64;
	// 82465688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246568C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82465690: 40990014  ble cr6, 0x824656a4
	if !ctx.cr[6].gt {
	pc = 0x824656A4; continue 'dispatch;
	}
	pc = 0x82465694; continue 'dispatch;
            }
            0x82465694 => {
    //   block [0x82465694..0x824656A4)
	// 82465694: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82465698: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8246569C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824656A0: 4199FFF4  bgt cr6, 0x82465694
	if ctx.cr[6].gt {
	pc = 0x82465694; continue 'dispatch;
	}
	pc = 0x824656A4; continue 'dispatch;
            }
            0x824656A4 => {
    //   block [0x824656A4..0x82465778)
	// 824656A4: 3BAAFFC0  addi r29, r10, -0x40
	ctx.r[29].s64 = ctx.r[10].s64 + -64;
	// 824656A8: 2F070080  cmpwi cr6, r7, 0x80
	ctx.cr[6].compare_i32(ctx.r[7].s32, 128, &mut ctx.xer);
	// 824656AC: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 824656B0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 824656B4: 409800C4  bge cr6, 0x82465778
	if !ctx.cr[6].lt {
	pc = 0x82465778; continue 'dispatch;
	}
	// 824656B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824656BC: 4800373D  bl 0x82468df8
	ctx.lr = 0x824656C0;
	sub_82468DF8(ctx, base);
	// 824656C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824656C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824656C8: 388B75B4  addi r4, r11, 0x75b4
	ctx.r[4].s64 = ctx.r[11].s64 + 30132;
	// 824656CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824656D0: 3BEB7574  addi r31, r11, 0x7574
	ctx.r[31].s64 = ctx.r[11].s64 + 30068;
	// 824656D4: 48002BCD  bl 0x824682a0
	ctx.lr = 0x824656D8;
	sub_824682A0(ctx, base);
	// 824656D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824656DC: 48002CFD  bl 0x824683d8
	ctx.lr = 0x824656E0;
	sub_824683D8(ctx, base);
	// 824656E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824656E4: 48002BBD  bl 0x824682a0
	ctx.lr = 0x824656E8;
	sub_824682A0(ctx, base);
	// 824656E8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824656EC: 3CA0AF55  lis r5, -0x50ab
	ctx.r[5].s64 = -1353383936;
	// 824656F0: 390003B6  li r8, 0x3b6
	ctx.r[8].s64 = 950;
	// 824656F4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 824656F8: 60A5ADDE  ori r5, r5, 0xadde
	ctx.r[5].u64 = ctx.r[5].u64 | 44510;
	// 824656FC: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82465700: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82465704: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82465708: 38EB754C  addi r7, r11, 0x754c
	ctx.r[7].s64 = ctx.r[11].s64 + 30028;
	// 8246570C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465710: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82465714: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465718: 4E800421  bctrl
	ctx.lr = 0x8246571C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246571C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82465720: 480031C1  bl 0x824688e0
	ctx.lr = 0x82465724;
	sub_824688E0(ctx, base);
	// 82465724: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82465728: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8246572C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82465730: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82465734: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82465738: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246573C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82465740: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465744: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82465748: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8246574C: 9B5F0018  stb r26, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u8 ) };
	// 82465750: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82465754: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465758: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246575C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465760: 4E800421  bctrl
	ctx.lr = 0x82465764;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465764: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82465768: 937F0024  stw r27, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 8246576C: 9B5F0028  stb r26, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[26].u8 ) };
	// 82465770: 83FF0020  lwz r31, 0x20(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82465774: 48000090  b 0x82465804
	pc = 0x82465804; continue 'dispatch;
            }
            0x82465778 => {
    //   block [0x82465778..0x82465804)
	// 82465778: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246577C: 4800367D  bl 0x82468df8
	ctx.lr = 0x82465780;
	sub_82468DF8(ctx, base);
	// 82465780: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82465784: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82465788: 388B75B4  addi r4, r11, 0x75b4
	ctx.r[4].s64 = ctx.r[11].s64 + 30132;
	// 8246578C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82465790: 3BEB74F0  addi r31, r11, 0x74f0
	ctx.r[31].s64 = ctx.r[11].s64 + 29936;
	// 82465794: 48002B0D  bl 0x824682a0
	ctx.lr = 0x82465798;
	sub_824682A0(ctx, base);
	// 82465798: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246579C: 48002C3D  bl 0x824683d8
	ctx.lr = 0x824657A0;
	sub_824683D8(ctx, base);
	// 824657A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824657A4: 48002AFD  bl 0x824682a0
	ctx.lr = 0x824657A8;
	sub_824682A0(ctx, base);
	// 824657A8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824657AC: 3CA0AF55  lis r5, -0x50ab
	ctx.r[5].s64 = -1353383936;
	// 824657B0: 390003C3  li r8, 0x3c3
	ctx.r[8].s64 = 963;
	// 824657B4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 824657B8: 60A5ADDE  ori r5, r5, 0xadde
	ctx.r[5].u64 = ctx.r[5].u64 | 44510;
	// 824657BC: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824657C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824657C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824657C8: 38EB754C  addi r7, r11, 0x754c
	ctx.r[7].s64 = ctx.r[11].s64 + 30028;
	// 824657CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824657D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824657D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824657D8: 4E800421  bctrl
	ctx.lr = 0x824657DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824657DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824657E0: 48003101  bl 0x824688e0
	ctx.lr = 0x824657E4;
	sub_824688E0(ctx, base);
	// 824657E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824657E8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824657EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824657F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824657F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824657F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824657FC: 4E800421  bctrl
	ctx.lr = 0x82465800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465800: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            0x82465804 => {
    //   block [0x82465804..0x82465820)
	// 82465804: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82465808: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8246580C: F9790020  std r11, 0x20(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82465810: 482A7A5D  bl 0x8270d26c
	ctx.lr = 0x82465814;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82465814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465818: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 8246581C: 480CF8E0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82465820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82465820 size=300
    let mut pc: u32 = 0x82465820;
    'dispatch: loop {
        match pc {
            0x82465820 => {
    //   block [0x82465820..0x8246585C)
	// 82465820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82465824: 480CF891  bl 0x825350b4
	ctx.lr = 0x82465828;
	sub_82535080(ctx, base);
	// 82465828: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246582C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82465830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82465834: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82465838: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8246583C: 2F1C2000  cmpwi cr6, r28, 0x2000
	ctx.cr[6].compare_i32(ctx.r[28].s32, 8192, &mut ctx.xer);
	// 82465840: 4098001C  bge cr6, 0x8246585c
	if !ctx.cr[6].lt {
	pc = 0x8246585C; continue 'dispatch;
	}
	// 82465844: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465848: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246584C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82465850: 4BFFE869  bl 0x824640b8
	ctx.lr = 0x82465854;
	sub_824640B8(ctx, base);
	// 82465854: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82465858: 480CF8AC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246585C => {
    //   block [0x8246585C..0x8246587C)
	// 8246585C: 3BDF0A30  addi r30, r31, 0xa30
	ctx.r[30].s64 = ctx.r[31].s64 + 2608;
	// 82465860: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465864: 48000915  bl 0x82466178
	ctx.lr = 0x82465868;
	sub_82466178(ctx, base);
	// 82465868: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246586C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82465870: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82465874: 40990024  ble cr6, 0x82465898
	if !ctx.cr[6].gt {
	pc = 0x82465898; continue 'dispatch;
	}
	// 82465878: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	pc = 0x8246587C; continue 'dispatch;
            }
            0x8246587C => {
    //   block [0x8246587C..0x82465898)
	// 8246587C: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82465880: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82465884: 419A00A8  beq cr6, 0x8246592c
	if ctx.cr[6].eq {
	pc = 0x8246592C; continue 'dispatch;
	}
	// 82465888: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246588C: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82465890: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82465894: 4198FFE8  blt cr6, 0x8246587c
	if ctx.cr[6].lt {
	pc = 0x8246587C; continue 'dispatch;
	}
	pc = 0x82465898; continue 'dispatch;
            }
            0x82465898 => {
    //   block [0x82465898..0x8246592C)
	// 82465898: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8246589C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 824658A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824658A4: 48003555  bl 0x82468df8
	ctx.lr = 0x824658A8;
	sub_82468DF8(ctx, base);
	// 824658A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824658AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824658B0: 388B75D0  addi r4, r11, 0x75d0
	ctx.r[4].s64 = ctx.r[11].s64 + 30160;
	// 824658B4: 480029ED  bl 0x824682a0
	ctx.lr = 0x824658B8;
	sub_824682A0(ctx, base);
	// 824658B8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824658BC: 3CA0AF55  lis r5, -0x50ab
	ctx.r[5].s64 = -1353383936;
	// 824658C0: 390003DF  li r8, 0x3df
	ctx.r[8].s64 = 991;
	// 824658C4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824658C8: 60A5ADDF  ori r5, r5, 0xaddf
	ctx.r[5].u64 = ctx.r[5].u64 | 44511;
	// 824658CC: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824658D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824658D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824658D8: 38EB754C  addi r7, r11, 0x754c
	ctx.r[7].s64 = ctx.r[11].s64 + 30028;
	// 824658DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824658E0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824658E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824658E8: 4E800421  bctrl
	ctx.lr = 0x824658EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824658EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824658F0: 48002FF1  bl 0x824688e0
	ctx.lr = 0x824658F4;
	sub_824688E0(ctx, base);
	// 824658F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824658F8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824658FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82465900: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82465904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465908: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246590C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465910: 4E800421  bctrl
	ctx.lr = 0x82465914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465914: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82465918: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246591C: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82465920: 482A794D  bl 0x8270d26c
	ctx.lr = 0x82465924;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82465924: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82465928: 480CF7DC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x8246592C => {
    //   block [0x8246592C..0x8246594C)
	// 8246592C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82465930: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82465934: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465938: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 8246593C: F93E0020  std r9, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[9].u64 ) };
	// 82465940: 482A792D  bl 0x8270d26c
	ctx.lr = 0x82465944;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82465944: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82465948: 480CF7BC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82465950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82465950 size=116
    let mut pc: u32 = 0x82465950;
    'dispatch: loop {
        match pc {
            0x82465950 => {
    //   block [0x82465950..0x824659C4)
	// 82465950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82465954: 480CF761  bl 0x825350b4
	ctx.lr = 0x82465958;
	sub_82535080(ctx, base);
	// 82465958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246595C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82465960: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82465964: 3BDF0A30  addi r30, r31, 0xa30
	ctx.r[30].s64 = ctx.r[31].s64 + 2608;
	// 82465968: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8246596C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465970: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82465974: 48000805  bl 0x82466178
	ctx.lr = 0x82465978;
	sub_82466178(ctx, base);
	// 82465978: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246597C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82465980: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82465984: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82465988: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246598C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82465990: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82465994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465998: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8246599C: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824659A0: 992B0018  stb r9, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 824659A4: 938B001C  stw r28, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 824659A8: 93AB0020  stw r29, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 824659AC: 936B0024  stw r27, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 824659B0: 992B0028  stb r9, 0x28(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u8 ) };
	// 824659B4: F91E0020  std r8, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[8].u64 ) };
	// 824659B8: 482A78B5  bl 0x8270d26c
	ctx.lr = 0x824659BC;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 824659BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824659C0: 480CF744  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824659C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824659C8 size=108
    let mut pc: u32 = 0x824659C8;
    'dispatch: loop {
        match pc {
            0x824659C8 => {
    //   block [0x824659C8..0x82465A00)
	// 824659C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824659CC: 480CF6F1  bl 0x825350bc
	ctx.lr = 0x824659D0;
	sub_82535080(ctx, base);
	// 824659D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824659D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824659D8: 3BBE0A30  addi r29, r30, 0xa30
	ctx.r[29].s64 = ctx.r[30].s64 + 2608;
	// 824659DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824659E0: 48000799  bl 0x82466178
	ctx.lr = 0x824659E4;
	sub_82466178(ctx, base);
	// 824659E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824659E8: 83FE0A1C  lwz r31, 0xa1c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2588 as u32) ) } as u64;
	// 824659EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824659F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824659F4: 917E0A1C  stw r11, 0xa1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(2588 as u32), ctx.r[11].u32 ) };
	// 824659F8: 419A0024  beq cr6, 0x82465a1c
	if ctx.cr[6].eq {
	pc = 0x82465A1C; continue 'dispatch;
	}
	// 824659FC: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	pc = 0x82465A00; continue 'dispatch;
            }
            0x82465A00 => {
    //   block [0x82465A00..0x82465A1C)
	// 82465A00: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465A04: 817E49B0  lwz r11, 0x49b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(18864 as u32) ) } as u64;
	// 82465A08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465A0C: 4E800421  bctrl
	ctx.lr = 0x82465A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465A10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465A14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82465A18: 409AFFE8  bne cr6, 0x82465a00
	if !ctx.cr[6].eq {
	pc = 0x82465A00; continue 'dispatch;
	}
            }
            0x82465A1C => {
    //   block [0x82465A1C..0x82465A34)
	// 82465A1C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82465A20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82465A24: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82465A28: 482A7845  bl 0x8270d26c
	ctx.lr = 0x82465A2C;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82465A2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82465A30: 480CF6DC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82465A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82465A38 size=864
    let mut pc: u32 = 0x82465A38;
    'dispatch: loop {
        match pc {
            0x82465A38 => {
    //   block [0x82465A38..0x82465A9C)
	// 82465A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82465A3C: 480CF679  bl 0x825350b4
	ctx.lr = 0x82465A40;
	sub_82535080(ctx, base);
	// 82465A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82465A44: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82465A48: 48000D09  bl 0x82466750
	ctx.lr = 0x82465A4C;
	sub_82466750(ctx, base);
	// 82465A4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82465A50: 3BBC0A30  addi r29, r28, 0xa30
	ctx.r[29].s64 = ctx.r[28].s64 + 2608;
	// 82465A54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82465A58: 396B75FC  addi r11, r11, 0x75fc
	ctx.r[11].s64 = ctx.r[11].s64 + 30204;
	// 82465A5C: 3BDD0028  addi r30, r29, 0x28
	ctx.r[30].s64 = ctx.r[29].s64 + 40;
	// 82465A60: 39200FA0  li r9, 0xfa0
	ctx.r[9].s64 = 4000;
	// 82465A64: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82465A68: 3BEA9198  addi r31, r10, -0x6e68
	ctx.r[31].s64 = ctx.r[10].s64 + -28264;
	// 82465A6C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82465A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465A74: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82465A78: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82465A7C: 937E0008  stw r27, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82465A80: 480006F9  bl 0x82466178
	ctx.lr = 0x82465A84;
	sub_82466178(ctx, base);
	// 82465A84: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82465A88: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82465A8C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82465A90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82465A94: 419A0008  beq cr6, 0x82465a9c
	if ctx.cr[6].eq {
	pc = 0x82465A9C; continue 'dispatch;
	}
	// 82465A98: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	pc = 0x82465A9C; continue 'dispatch;
            }
            0x82465A9C => {
    //   block [0x82465A9C..0x82465AF0)
	// 82465A9C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82465AA0: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82465AA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465AA8: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82465AAC: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82465AB0: 482A77BD  bl 0x8270d26c
	ctx.lr = 0x82465AB4;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82465AB4: 38800FA0  li r4, 0xfa0
	ctx.r[4].s64 = 4000;
	// 82465AB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82465ABC: 482A8261  bl 0x8270dd1c
	ctx.lr = 0x82465AC0;
	// extern call 0x8270DD1C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82465AC0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82465AC4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82465AC8: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82465ACC: 397C0D54  addi r11, r28, 0xd54
	ctx.r[11].s64 = ctx.r[28].s64 + 3412;
	// 82465AD0: 937C0A20  stw r27, 0xa20(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2592 as u32), ctx.r[27].u32 ) };
	// 82465AD4: 937C0A24  stw r27, 0xa24(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2596 as u32), ctx.r[27].u32 ) };
	// 82465AD8: 937C0A28  stw r27, 0xa28(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2600 as u32), ctx.r[27].u32 ) };
	// 82465ADC: 937C0A1C  stw r27, 0xa1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2588 as u32), ctx.r[27].u32 ) };
	// 82465AE0: 937C0A18  stw r27, 0xa18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(2584 as u32), ctx.r[27].u32 ) };
	// 82465AE4: 937C0D74  stw r27, 0xd74(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3444 as u32), ctx.r[27].u32 ) };
	// 82465AE8: 937C0D7C  stw r27, 0xd7c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3452 as u32), ctx.r[27].u32 ) };
	// 82465AEC: 937C0D78  stw r27, 0xd78(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3448 as u32), ctx.r[27].u32 ) };
	pc = 0x82465AF0; continue 'dispatch;
            }
            0x82465AF0 => {
    //   block [0x82465AF0..0x82465B10)
	// 82465AF0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82465AF4: 936BFD54  stw r27, -0x2ac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-684 as u32), ctx.r[27].u32 ) };
	// 82465AF8: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82465AFC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82465B00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82465B04: 4098FFEC  bge cr6, 0x82465af0
	if !ctx.cr[6].lt {
	pc = 0x82465AF0; continue 'dispatch;
	}
	// 82465B08: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82465B0C: 393C0AF0  addi r9, r28, 0xaf0
	ctx.r[9].s64 = ctx.r[28].s64 + 2800;
	pc = 0x82465B10; continue 'dispatch;
            }
            0x82465B10 => {
    //   block [0x82465B10..0x82465B20)
	// 82465B10: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82465B14: 4199000C  bgt cr6, 0x82465b20
	if ctx.cr[6].gt {
	pc = 0x82465B20; continue 'dispatch;
	}
	// 82465B18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82465B1C: 480000FC  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B20 => {
    //   block [0x82465B20..0x82465B30)
	// 82465B20: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82465B24: 4199000C  bgt cr6, 0x82465b30
	if ctx.cr[6].gt {
	pc = 0x82465B30; continue 'dispatch;
	}
	// 82465B28: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82465B2C: 480000EC  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B30 => {
    //   block [0x82465B30..0x82465B40)
	// 82465B30: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82465B34: 4199000C  bgt cr6, 0x82465b40
	if ctx.cr[6].gt {
	pc = 0x82465B40; continue 'dispatch;
	}
	// 82465B38: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82465B3C: 480000DC  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B40 => {
    //   block [0x82465B40..0x82465B50)
	// 82465B40: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 82465B44: 4199000C  bgt cr6, 0x82465b50
	if ctx.cr[6].gt {
	pc = 0x82465B50; continue 'dispatch;
	}
	// 82465B48: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82465B4C: 480000CC  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B50 => {
    //   block [0x82465B50..0x82465B60)
	// 82465B50: 2F0A0040  cmpwi cr6, r10, 0x40
	ctx.cr[6].compare_i32(ctx.r[10].s32, 64, &mut ctx.xer);
	// 82465B54: 4199000C  bgt cr6, 0x82465b60
	if ctx.cr[6].gt {
	pc = 0x82465B60; continue 'dispatch;
	}
	// 82465B58: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82465B5C: 480000BC  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B60 => {
    //   block [0x82465B60..0x82465B70)
	// 82465B60: 2F0A0060  cmpwi cr6, r10, 0x60
	ctx.cr[6].compare_i32(ctx.r[10].s32, 96, &mut ctx.xer);
	// 82465B64: 4199000C  bgt cr6, 0x82465b70
	if ctx.cr[6].gt {
	pc = 0x82465B70; continue 'dispatch;
	}
	// 82465B68: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82465B6C: 480000AC  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B70 => {
    //   block [0x82465B70..0x82465B80)
	// 82465B70: 2F0A0080  cmpwi cr6, r10, 0x80
	ctx.cr[6].compare_i32(ctx.r[10].s32, 128, &mut ctx.xer);
	// 82465B74: 4199000C  bgt cr6, 0x82465b80
	if ctx.cr[6].gt {
	pc = 0x82465B80; continue 'dispatch;
	}
	// 82465B78: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82465B7C: 4800009C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B80 => {
    //   block [0x82465B80..0x82465B90)
	// 82465B80: 2F0A00A0  cmpwi cr6, r10, 0xa0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 160, &mut ctx.xer);
	// 82465B84: 4199000C  bgt cr6, 0x82465b90
	if ctx.cr[6].gt {
	pc = 0x82465B90; continue 'dispatch;
	}
	// 82465B88: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82465B8C: 4800008C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465B90 => {
    //   block [0x82465B90..0x82465BA0)
	// 82465B90: 2F0A00C0  cmpwi cr6, r10, 0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 192, &mut ctx.xer);
	// 82465B94: 4199000C  bgt cr6, 0x82465ba0
	if ctx.cr[6].gt {
	pc = 0x82465BA0; continue 'dispatch;
	}
	// 82465B98: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82465B9C: 4800007C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465BA0 => {
    //   block [0x82465BA0..0x82465BB0)
	// 82465BA0: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 82465BA4: 4199000C  bgt cr6, 0x82465bb0
	if ctx.cr[6].gt {
	pc = 0x82465BB0; continue 'dispatch;
	}
	// 82465BA8: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82465BAC: 4800006C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465BB0 => {
    //   block [0x82465BB0..0x82465BC0)
	// 82465BB0: 2F0A0140  cmpwi cr6, r10, 0x140
	ctx.cr[6].compare_i32(ctx.r[10].s32, 320, &mut ctx.xer);
	// 82465BB4: 4199000C  bgt cr6, 0x82465bc0
	if ctx.cr[6].gt {
	pc = 0x82465BC0; continue 'dispatch;
	}
	// 82465BB8: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82465BBC: 4800005C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465BC0 => {
    //   block [0x82465BC0..0x82465BD0)
	// 82465BC0: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 82465BC4: 4199000C  bgt cr6, 0x82465bd0
	if ctx.cr[6].gt {
	pc = 0x82465BD0; continue 'dispatch;
	}
	// 82465BC8: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82465BCC: 4800004C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465BD0 => {
    //   block [0x82465BD0..0x82465BE0)
	// 82465BD0: 2F0A0400  cmpwi cr6, r10, 0x400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1024, &mut ctx.xer);
	// 82465BD4: 4199000C  bgt cr6, 0x82465be0
	if ctx.cr[6].gt {
	pc = 0x82465BE0; continue 'dispatch;
	}
	// 82465BD8: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82465BDC: 4800003C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465BE0 => {
    //   block [0x82465BE0..0x82465BF0)
	// 82465BE0: 2F0A0800  cmpwi cr6, r10, 0x800
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2048, &mut ctx.xer);
	// 82465BE4: 4199000C  bgt cr6, 0x82465bf0
	if ctx.cr[6].gt {
	pc = 0x82465BF0; continue 'dispatch;
	}
	// 82465BE8: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 82465BEC: 4800002C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465BF0 => {
    //   block [0x82465BF0..0x82465C00)
	// 82465BF0: 2F0A1000  cmpwi cr6, r10, 0x1000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4096, &mut ctx.xer);
	// 82465BF4: 4199000C  bgt cr6, 0x82465c00
	if ctx.cr[6].gt {
	pc = 0x82465C00; continue 'dispatch;
	}
	// 82465BF8: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82465BFC: 4800001C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465C00 => {
    //   block [0x82465C00..0x82465C10)
	// 82465C00: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82465C04: 4199000C  bgt cr6, 0x82465c10
	if ctx.cr[6].gt {
	pc = 0x82465C10; continue 'dispatch;
	}
	// 82465C08: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82465C0C: 4800000C  b 0x82465c18
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465C10 => {
    //   block [0x82465C10..0x82465C18)
	// 82465C10: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82465C14: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82465C18; continue 'dispatch;
            }
            0x82465C18 => {
    //   block [0x82465C18..0x82465C3C)
	// 82465C18: 390B02AB  addi r8, r11, 0x2ab
	ctx.r[8].s64 = ctx.r[11].s64 + 683;
	// 82465C1C: 7D6951AE  stbx r11, r9, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 82465C20: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82465C24: 7D48E12E  stwx r10, r8, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82465C28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82465C2C: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 82465C30: 4099FEE0  ble cr6, 0x82465b10
	if !ctx.cr[6].gt {
	pc = 0x82465B10; continue 'dispatch;
	}
	// 82465C34: 39400400  li r10, 0x400
	ctx.r[10].s64 = 1024;
	// 82465C38: 393C0CF4  addi r9, r28, 0xcf4
	ctx.r[9].s64 = ctx.r[28].s64 + 3316;
	pc = 0x82465C3C; continue 'dispatch;
            }
            0x82465C3C => {
    //   block [0x82465C3C..0x82465C4C)
	// 82465C3C: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82465C40: 4199000C  bgt cr6, 0x82465c4c
	if ctx.cr[6].gt {
	pc = 0x82465C4C; continue 'dispatch;
	}
	// 82465C44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82465C48: 480000FC  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465C4C => {
    //   block [0x82465C4C..0x82465C5C)
	// 82465C4C: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82465C50: 4199000C  bgt cr6, 0x82465c5c
	if ctx.cr[6].gt {
	pc = 0x82465C5C; continue 'dispatch;
	}
	// 82465C54: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82465C58: 480000EC  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465C5C => {
    //   block [0x82465C5C..0x82465C6C)
	// 82465C5C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82465C60: 4199000C  bgt cr6, 0x82465c6c
	if ctx.cr[6].gt {
	pc = 0x82465C6C; continue 'dispatch;
	}
	// 82465C64: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82465C68: 480000DC  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465C6C => {
    //   block [0x82465C6C..0x82465C7C)
	// 82465C6C: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 82465C70: 4199000C  bgt cr6, 0x82465c7c
	if ctx.cr[6].gt {
	pc = 0x82465C7C; continue 'dispatch;
	}
	// 82465C74: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82465C78: 480000CC  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465C7C => {
    //   block [0x82465C7C..0x82465C8C)
	// 82465C7C: 2F0A0040  cmpwi cr6, r10, 0x40
	ctx.cr[6].compare_i32(ctx.r[10].s32, 64, &mut ctx.xer);
	// 82465C80: 4199000C  bgt cr6, 0x82465c8c
	if ctx.cr[6].gt {
	pc = 0x82465C8C; continue 'dispatch;
	}
	// 82465C84: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82465C88: 480000BC  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465C8C => {
    //   block [0x82465C8C..0x82465C9C)
	// 82465C8C: 2F0A0060  cmpwi cr6, r10, 0x60
	ctx.cr[6].compare_i32(ctx.r[10].s32, 96, &mut ctx.xer);
	// 82465C90: 4199000C  bgt cr6, 0x82465c9c
	if ctx.cr[6].gt {
	pc = 0x82465C9C; continue 'dispatch;
	}
	// 82465C94: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82465C98: 480000AC  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465C9C => {
    //   block [0x82465C9C..0x82465CAC)
	// 82465C9C: 2F0A0080  cmpwi cr6, r10, 0x80
	ctx.cr[6].compare_i32(ctx.r[10].s32, 128, &mut ctx.xer);
	// 82465CA0: 4199000C  bgt cr6, 0x82465cac
	if ctx.cr[6].gt {
	pc = 0x82465CAC; continue 'dispatch;
	}
	// 82465CA4: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82465CA8: 4800009C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465CAC => {
    //   block [0x82465CAC..0x82465CBC)
	// 82465CAC: 2F0A00A0  cmpwi cr6, r10, 0xa0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 160, &mut ctx.xer);
	// 82465CB0: 4199000C  bgt cr6, 0x82465cbc
	if ctx.cr[6].gt {
	pc = 0x82465CBC; continue 'dispatch;
	}
	// 82465CB4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82465CB8: 4800008C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465CBC => {
    //   block [0x82465CBC..0x82465CCC)
	// 82465CBC: 2F0A00C0  cmpwi cr6, r10, 0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 192, &mut ctx.xer);
	// 82465CC0: 4199000C  bgt cr6, 0x82465ccc
	if ctx.cr[6].gt {
	pc = 0x82465CCC; continue 'dispatch;
	}
	// 82465CC4: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82465CC8: 4800007C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465CCC => {
    //   block [0x82465CCC..0x82465CDC)
	// 82465CCC: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 82465CD0: 4199000C  bgt cr6, 0x82465cdc
	if ctx.cr[6].gt {
	pc = 0x82465CDC; continue 'dispatch;
	}
	// 82465CD4: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82465CD8: 4800006C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465CDC => {
    //   block [0x82465CDC..0x82465CEC)
	// 82465CDC: 2F0A0140  cmpwi cr6, r10, 0x140
	ctx.cr[6].compare_i32(ctx.r[10].s32, 320, &mut ctx.xer);
	// 82465CE0: 4199000C  bgt cr6, 0x82465cec
	if ctx.cr[6].gt {
	pc = 0x82465CEC; continue 'dispatch;
	}
	// 82465CE4: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82465CE8: 4800005C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465CEC => {
    //   block [0x82465CEC..0x82465CFC)
	// 82465CEC: 2F0A0200  cmpwi cr6, r10, 0x200
	ctx.cr[6].compare_i32(ctx.r[10].s32, 512, &mut ctx.xer);
	// 82465CF0: 4199000C  bgt cr6, 0x82465cfc
	if ctx.cr[6].gt {
	pc = 0x82465CFC; continue 'dispatch;
	}
	// 82465CF4: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82465CF8: 4800004C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465CFC => {
    //   block [0x82465CFC..0x82465D0C)
	// 82465CFC: 2F0A0400  cmpwi cr6, r10, 0x400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1024, &mut ctx.xer);
	// 82465D00: 4199000C  bgt cr6, 0x82465d0c
	if ctx.cr[6].gt {
	pc = 0x82465D0C; continue 'dispatch;
	}
	// 82465D04: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82465D08: 4800003C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465D0C => {
    //   block [0x82465D0C..0x82465D1C)
	// 82465D0C: 2F0A0800  cmpwi cr6, r10, 0x800
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2048, &mut ctx.xer);
	// 82465D10: 4199000C  bgt cr6, 0x82465d1c
	if ctx.cr[6].gt {
	pc = 0x82465D1C; continue 'dispatch;
	}
	// 82465D14: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 82465D18: 4800002C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465D1C => {
    //   block [0x82465D1C..0x82465D2C)
	// 82465D1C: 2F0A1000  cmpwi cr6, r10, 0x1000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4096, &mut ctx.xer);
	// 82465D20: 4199000C  bgt cr6, 0x82465d2c
	if ctx.cr[6].gt {
	pc = 0x82465D2C; continue 'dispatch;
	}
	// 82465D24: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 82465D28: 4800001C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465D2C => {
    //   block [0x82465D2C..0x82465D3C)
	// 82465D2C: 2F0A2000  cmpwi cr6, r10, 0x2000
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8192, &mut ctx.xer);
	// 82465D30: 4199000C  bgt cr6, 0x82465d3c
	if ctx.cr[6].gt {
	pc = 0x82465D3C; continue 'dispatch;
	}
	// 82465D34: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82465D38: 4800000C  b 0x82465d44
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465D3C => {
    //   block [0x82465D3C..0x82465D44)
	// 82465D3C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82465D40: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82465D44; continue 'dispatch;
            }
            0x82465D44 => {
    //   block [0x82465D44..0x82465D98)
	// 82465D44: 390B02AB  addi r8, r11, 0x2ab
	ctx.r[8].s64 = ctx.r[11].s64 + 683;
	// 82465D48: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82465D4C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82465D50: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82465D54: 7D48E12E  stwx r10, r8, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82465D58: 394A0400  addi r10, r10, 0x400
	ctx.r[10].s64 = ctx.r[10].s64 + 1024;
	// 82465D5C: 2F0A2400  cmpwi cr6, r10, 0x2400
	ctx.cr[6].compare_i32(ctx.r[10].s32, 9216, &mut ctx.xer);
	// 82465D60: 4198FEDC  blt cr6, 0x82465c3c
	if ctx.cr[6].lt {
	pc = 0x82465C3C; continue 'dispatch;
	}
	// 82465D64: 39602000  li r11, 0x2000
	ctx.r[11].s64 = 8192;
	// 82465D68: 937C0D58  stw r27, 0xd58(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3416 as u32), ctx.r[27].u32 ) };
	// 82465D6C: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 82465D70: 937C0D5C  stw r27, 0xd5c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3420 as u32), ctx.r[27].u32 ) };
	// 82465D74: 937C0D60  stw r27, 0xd60(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3424 as u32), ctx.r[27].u32 ) };
	// 82465D78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82465D7C: 937C0D64  stw r27, 0xd64(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3428 as u32), ctx.r[27].u32 ) };
	// 82465D80: 937C0D70  stw r27, 0xd70(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3440 as u32), ctx.r[27].u32 ) };
	// 82465D84: 917C0D68  stw r11, 0xd68(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3432 as u32), ctx.r[11].u32 ) };
	// 82465D88: 915C0D6C  stw r10, 0xd6c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(3436 as u32), ctx.r[10].u32 ) };
	// 82465D8C: 937C0014  stw r27, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82465D90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82465D94: 480CF370  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82465D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82465D98 size=216
    let mut pc: u32 = 0x82465D98;
    'dispatch: loop {
        match pc {
            0x82465D98 => {
    //   block [0x82465D98..0x82465DD4)
	// 82465D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82465D9C: 480CF319  bl 0x825350b4
	ctx.lr = 0x82465DA0;
	sub_82535080(ctx, base);
	// 82465DA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82465DA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82465DA8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82465DAC: 396B75FC  addi r11, r11, 0x75fc
	ctx.r[11].s64 = ctx.r[11].s64 + 30204;
	// 82465DB0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82465DB4: 4BFFF78D  bl 0x82465540
	ctx.lr = 0x82465DB8;
	sub_82465540(ctx, base);
	// 82465DB8: 817B0A18  lwz r11, 0xa18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82465DBC: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82465DC0: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 82465DC4: 3FA08273  lis r29, -0x7d8d
	ctx.r[29].s64 = -2106392576;
	// 82465DC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82465DCC: 817B0A1C  lwz r11, 0xa1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82465DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	pc = 0x82465DD4; continue 'dispatch;
            }
            0x82465DD4 => {
    //   block [0x82465DD4..0x82465DE4)
	// 82465DD4: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465DDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82465DE0: 419A0020  beq cr6, 0x82465e00
	if ctx.cr[6].eq {
	pc = 0x82465E00; continue 'dispatch;
	}
	pc = 0x82465DE4; continue 'dispatch;
            }
            0x82465DE4 => {
    //   block [0x82465DE4..0x82465E00)
	// 82465DE4: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465DE8: 817D49B0  lwz r11, 0x49b0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(18864 as u32) ) } as u64;
	// 82465DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465DF0: 4E800421  bctrl
	ctx.lr = 0x82465DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82465DF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82465DFC: 409AFFE8  bne cr6, 0x82465de4
	if !ctx.cr[6].eq {
	pc = 0x82465DE4; continue 'dispatch;
	}
            }
            0x82465E00 => {
    //   block [0x82465E00..0x82465E50)
	// 82465E00: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82465E04: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82465E08: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82465E0C: 409AFFC8  bne cr6, 0x82465dd4
	if !ctx.cr[6].eq {
	pc = 0x82465DD4; continue 'dispatch;
	}
	// 82465E10: 3BFB0A58  addi r31, r27, 0xa58
	ctx.r[31].s64 = ctx.r[27].s64 + 2648;
	// 82465E14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82465E18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82465E1C: 419A0044  beq cr6, 0x82465e60
	if ctx.cr[6].eq {
	pc = 0x82465E60; continue 'dispatch;
	}
	// 82465E20: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82465E24: 3BCB9198  addi r30, r11, -0x6e68
	ctx.r[30].s64 = ctx.r[11].s64 + -28264;
	// 82465E28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465E2C: 4800034D  bl 0x82466178
	ctx.lr = 0x82465E30;
	sub_82466178(ctx, base);
	// 82465E30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82465E34: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82465E38: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82465E3C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82465E40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82465E44: 419A000C  beq cr6, 0x82465e50
	if ctx.cr[6].eq {
	pc = 0x82465E50; continue 'dispatch;
	}
	// 82465E48: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82465E4C: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	pc = 0x82465E50; continue 'dispatch;
            }
            0x82465E50 => {
    //   block [0x82465E50..0x82465E60)
	// 82465E50: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82465E54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465E58: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82465E5C: 482A7411  bl 0x8270d26c
	ctx.lr = 0x82465E60;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x82465E60; continue 'dispatch;
            }
            0x82465E60 => {
    //   block [0x82465E60..0x82465E70)
	// 82465E60: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82465E64: 48000815  bl 0x82466678
	ctx.lr = 0x82465E68;
	sub_82466678(ctx, base);
	// 82465E68: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82465E6C: 480CF298  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82465E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82465E70 size=776
    let mut pc: u32 = 0x82465E70;
    'dispatch: loop {
        match pc {
            0x82465E70 => {
    //   block [0x82465E70..0x82465EC4)
	// 82465E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82465E74: 480CF231  bl 0x825350a4
	ctx.lr = 0x82465E78;
	sub_82535080(ctx, base);
	// 82465E78: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82465E7C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82465E80: 3AFB0A30  addi r23, r27, 0xa30
	ctx.r[23].s64 = ctx.r[27].s64 + 2608;
	// 82465E84: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82465E88: 480002F1  bl 0x82466178
	ctx.lr = 0x82465E8C;
	sub_82466178(ctx, base);
	// 82465E8C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465E90: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 82465E94: 83BB0D64  lwz r29, 0xd64(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82465E98: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82465E9C: 557F1036  rlwinm r31, r11, 2, 0, 0x1b
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82465EA0: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82465EA4: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82465EA8: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82465EAC: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82465EB0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82465EB4: 41990010  bgt cr6, 0x82465ec4
	if ctx.cr[6].gt {
	pc = 0x82465EC4; continue 'dispatch;
	}
	// 82465EB8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82465EBC: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82465EC0: 4800001C  b 0x82465edc
	pc = 0x82465EDC; continue 'dispatch;
            }
            0x82465EC4 => {
    //   block [0x82465EC4..0x82465EDC)
	// 82465EC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465EC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82465ECC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82465ED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465ED4: 4E800421  bctrl
	ctx.lr = 0x82465ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465ED8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
            }
            0x82465EDC => {
    //   block [0x82465EDC..0x82465F00)
	// 82465EDC: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82465EE0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82465EE4: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82465EE8: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82465EEC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82465EF0: 41990010  bgt cr6, 0x82465f00
	if ctx.cr[6].gt {
	pc = 0x82465F00; continue 'dispatch;
	}
	// 82465EF4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82465EF8: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82465EFC: 4800001C  b 0x82465f18
	pc = 0x82465F18; continue 'dispatch;
            }
            0x82465F00 => {
    //   block [0x82465F00..0x82465F18)
	// 82465F00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465F04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82465F08: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82465F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82465F10: 4E800421  bctrl
	ctx.lr = 0x82465F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82465F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            0x82465F18 => {
    //   block [0x82465F18..0x82465F40)
	// 82465F18: 3B5DFFFF  addi r26, r29, -1
	ctx.r[26].s64 = ctx.r[29].s64 + -1;
	// 82465F1C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82465F20: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82465F24: 41980028  blt cr6, 0x82465f4c
	if ctx.cr[6].lt {
	pc = 0x82465F4C; continue 'dispatch;
	}
	// 82465F28: 395A0001  addi r10, r26, 1
	ctx.r[10].s64 = ctx.r[26].s64 + 1;
	// 82465F2C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82465F30: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82465F34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82465F38: 419A0014  beq cr6, 0x82465f4c
	if ctx.cr[6].eq {
	pc = 0x82465F4C; continue 'dispatch;
	}
	// 82465F3C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82465F40; continue 'dispatch;
            }
            0x82465F40 => {
    //   block [0x82465F40..0x82465F4C)
	// 82465F40: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82465F44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82465F48: 4200FFF8  bdnz 0x82465f40
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82465F40; continue 'dispatch;
	}
	pc = 0x82465F4C; continue 'dispatch;
            }
            0x82465F4C => {
    //   block [0x82465F4C..0x82465F5C)
	// 82465F4C: 817B0A18  lwz r11, 0xa18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82465F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82465F54: 419A001C  beq cr6, 0x82465f70
	if ctx.cr[6].eq {
	pc = 0x82465F70; continue 'dispatch;
	}
	// 82465F58: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	pc = 0x82465F5C; continue 'dispatch;
            }
            0x82465F5C => {
    //   block [0x82465F5C..0x82465F70)
	// 82465F5C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82465F60: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82465F64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465F68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82465F6C: 409AFFF0  bne cr6, 0x82465f5c
	if !ctx.cr[6].eq {
	pc = 0x82465F5C; continue 'dispatch;
	}
	pc = 0x82465F70; continue 'dispatch;
            }
            0x82465F70 => {
    //   block [0x82465F70..0x82465F94)
	// 82465F70: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82465F74: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82465F78: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82465F7C: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82465F80: 40990014  ble cr6, 0x82465f94
	if !ctx.cr[6].gt {
	pc = 0x82465F94; continue 'dispatch;
	}
	// 82465F84: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82465F88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82465F8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82465F90: 480002B1  bl 0x82466240
	ctx.lr = 0x82465F94;
	sub_82466240(ctx, base);
	pc = 0x82465F94; continue 'dispatch;
            }
            0x82465F94 => {
    //   block [0x82465F94..0x82465FA0)
	// 82465F94: 38DB0A6C  addi r6, r27, 0xa6c
	ctx.r[6].s64 = ctx.r[27].s64 + 2668;
	// 82465F98: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82465F9C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	pc = 0x82465FA0; continue 'dispatch;
            }
            0x82465FA0 => {
    //   block [0x82465FA0..0x82465FB0)
	// 82465FA0: 81050000  lwz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82465FA4: 80E50044  lwz r7, 0x44(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(68 as u32) ) } as u64;
	// 82465FA8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82465FAC: 419A0060  beq cr6, 0x8246600c
	if ctx.cr[6].eq {
	pc = 0x8246600C; continue 'dispatch;
	}
	pc = 0x82465FB0; continue 'dispatch;
            }
            0x82465FB0 => {
    //   block [0x82465FB0..0x82465FC0)
	// 82465FB0: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82465FB4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82465FB8: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82465FBC: 40990034  ble cr6, 0x82465ff0
	if !ctx.cr[6].gt {
	pc = 0x82465FF0; continue 'dispatch;
	}
	pc = 0x82465FC0; continue 'dispatch;
            }
            0x82465FC0 => {
    //   block [0x82465FC0..0x82465FE0)
	// 82465FC0: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82465FC4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82465FC8: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82465FCC: 7C63F02E  lwzx r3, r3, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82465FD0: 7F081840  cmplw cr6, r8, r3
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82465FD4: 4098000C  bge cr6, 0x82465fe0
	if !ctx.cr[6].lt {
	pc = 0x82465FE0; continue 'dispatch;
	}
	// 82465FD8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82465FDC: 48000008  b 0x82465fe4
	pc = 0x82465FE4; continue 'dispatch;
            }
            0x82465FE0 => {
    //   block [0x82465FE0..0x82465FE4)
	// 82465FE0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x82465FE4; continue 'dispatch;
            }
            0x82465FE4 => {
    //   block [0x82465FE4..0x82465FF0)
	// 82465FE4: 7D6A4850  subf r11, r10, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82465FE8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82465FEC: 4199FFD4  bgt cr6, 0x82465fc0
	if ctx.cr[6].gt {
	pc = 0x82465FC0; continue 'dispatch;
	}
	pc = 0x82465FF0; continue 'dispatch;
            }
            0x82465FF0 => {
    //   block [0x82465FF0..0x8246600C)
	// 82465FF0: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82465FF4: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82465FF8: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82465FFC: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82466000: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466004: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82466008: 409AFFA8  bne cr6, 0x82465fb0
	if !ctx.cr[6].eq {
	pc = 0x82465FB0; continue 'dispatch;
	}
	pc = 0x8246600C; continue 'dispatch;
            }
            0x8246600C => {
    //   block [0x8246600C..0x82466020)
	// 8246600C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 82466010: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 82466014: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82466018: 409AFF88  bne cr6, 0x82465fa0
	if !ctx.cr[6].eq {
	pc = 0x82465FA0; continue 'dispatch;
	}
	// 8246601C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	pc = 0x82466020; continue 'dispatch;
            }
            0x82466020 => {
    //   block [0x82466020..0x82466030)
	// 82466020: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466024: 93860000  stw r28, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82466028: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8246602C: 419A0070  beq cr6, 0x8246609c
	if ctx.cr[6].eq {
	pc = 0x8246609C; continue 'dispatch;
	}
	pc = 0x82466030; continue 'dispatch;
            }
            0x82466030 => {
    //   block [0x82466030..0x82466044)
	// 82466030: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466034: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82466038: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8246603C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82466040: 40990034  ble cr6, 0x82466074
	if !ctx.cr[6].gt {
	pc = 0x82466074; continue 'dispatch;
	}
	pc = 0x82466044; continue 'dispatch;
            }
            0x82466044 => {
    //   block [0x82466044..0x82466064)
	// 82466044: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82466048: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8246604C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82466050: 7C84F02E  lwzx r4, r4, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82466054: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82466058: 4098000C  bge cr6, 0x82466064
	if !ctx.cr[6].lt {
	pc = 0x82466064; continue 'dispatch;
	}
	// 8246605C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82466060: 48000008  b 0x82466068
	pc = 0x82466068; continue 'dispatch;
            }
            0x82466064 => {
    //   block [0x82466064..0x82466068)
	// 82466064: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x82466068; continue 'dispatch;
            }
            0x82466068 => {
    //   block [0x82466068..0x82466074)
	// 82466068: 7D6A4850  subf r11, r10, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 8246606C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82466070: 4199FFD4  bgt cr6, 0x82466044
	if ctx.cr[6].gt {
	pc = 0x82466044; continue 'dispatch;
	}
	pc = 0x82466074; continue 'dispatch;
            }
            0x82466074 => {
    //   block [0x82466074..0x82466090)
	// 82466074: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466078: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8246607C: 2F0B2000  cmpwi cr6, r11, 0x2000
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8192, &mut ctx.xer);
	// 82466080: 419A0010  beq cr6, 0x82466090
	if ctx.cr[6].eq {
	pc = 0x82466090; continue 'dispatch;
	}
	// 82466084: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466088: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246608C: 91060000  stw r8, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	pc = 0x82466090; continue 'dispatch;
            }
            0x82466090 => {
    //   block [0x82466090..0x8246609C)
	// 82466090: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82466094: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82466098: 409AFF98  bne cr6, 0x82466030
	if !ctx.cr[6].eq {
	pc = 0x82466030; continue 'dispatch;
	}
	pc = 0x8246609C; continue 'dispatch;
            }
            0x8246609C => {
    //   block [0x8246609C..0x824660C8)
	// 8246609C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 824660A0: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 824660A4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 824660A8: 409AFF78  bne cr6, 0x82466020
	if !ctx.cr[6].eq {
	pc = 0x82466020; continue 'dispatch;
	}
	// 824660AC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 824660B0: 939B0A18  stw r28, 0xa18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2584 as u32), ctx.r[28].u32 ) };
	// 824660B4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824660B8: 41980058  blt cr6, 0x82466110
	if ctx.cr[6].lt {
	pc = 0x82466110; continue 'dispatch;
	}
	// 824660BC: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824660C0: 7CFFF050  subf r7, r31, r30
	ctx.r[7].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 824660C4: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	pc = 0x824660C8; continue 'dispatch;
            }
            0x824660C8 => {
    //   block [0x824660C8..0x824660F4)
	// 824660C8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824660CC: 7D67502E  lwzx r11, r7, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824660D0: 2F092000  cmpwi cr6, r9, 0x2000
	ctx.cr[6].compare_i32(ctx.r[9].s32, 8192, &mut ctx.xer);
	// 824660D4: 409A0020  bne cr6, 0x824660f4
	if !ctx.cr[6].eq {
	pc = 0x824660F4; continue 'dispatch;
	}
	// 824660D8: 813B0A1C  lwz r9, 0xa1c(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2588 as u32) ) } as u64;
	// 824660DC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824660E0: 813B0D64  lwz r9, 0xd64(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(3428 as u32) ) } as u64;
	// 824660E4: 917B0A1C  stw r11, 0xa1c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2588 as u32), ctx.r[11].u32 ) };
	// 824660E8: 3969FFFF  addi r11, r9, -1
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	// 824660EC: 917B0D64  stw r11, 0xd64(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(3428 as u32), ctx.r[11].u32 ) };
	// 824660F0: 48000010  b 0x82466100
	pc = 0x82466100; continue 'dispatch;
            }
            0x824660F4 => {
    //   block [0x824660F4..0x82466100)
	// 824660F4: 813B0A18  lwz r9, 0xa18(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(2584 as u32) ) } as u64;
	// 824660F8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824660FC: 917B0A18  stw r11, 0xa18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2584 as u32), ctx.r[11].u32 ) };
	pc = 0x82466100; continue 'dispatch;
            }
            0x82466100 => {
    //   block [0x82466100..0x82466110)
	// 82466100: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82466104: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82466108: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8246610C: 4098FFBC  bge cr6, 0x824660c8
	if !ctx.cr[6].lt {
	pc = 0x824660C8; continue 'dispatch;
	}
	pc = 0x82466110; continue 'dispatch;
            }
            0x82466110 => {
    //   block [0x82466110..0x82466148)
	// 82466110: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82466114: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82466118: F9770020  std r11, 0x20(r23)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[23].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 8246611C: 482A7151  bl 0x8270d26c
	ctx.lr = 0x82466120;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82466120: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82466124: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82466128: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8246612C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82466130: 409A0018  bne cr6, 0x82466148
	if !ctx.cr[6].eq {
	pc = 0x82466148; continue 'dispatch;
	}
	// 82466134: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466138: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246613C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466140: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82466144: 4E800421  bctrl
	ctx.lr = 0x82466148;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82466148 => {
    //   block [0x82466148..0x82466170)
	// 82466148: 7C79C02E  lwzx r3, r25, r24
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246614C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82466150: 93C30020  stw r30, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82466154: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82466158: 409A0018  bne cr6, 0x82466170
	if !ctx.cr[6].eq {
	pc = 0x82466170; continue 'dispatch;
	}
	// 8246615C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466160: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82466164: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246616C: 4E800421  bctrl
	ctx.lr = 0x82466170;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82466170 => {
    //   block [0x82466170..0x82466178)
	// 82466170: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82466174: 480CEF80  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466178 size=196
    let mut pc: u32 = 0x82466178;
    'dispatch: loop {
        match pc {
            0x82466178 => {
    //   block [0x82466178..0x824661DC)
	// 82466178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246617C: 480CEF41  bl 0x825350bc
	ctx.lr = 0x82466180;
	sub_82535080(ctx, base);
	// 82466180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466184: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82466188: 482A70F5  bl 0x8270d27c
	ctx.lr = 0x8246618C;
	// extern call 0x8270D27C → crate::xboxkrnl::RtlTryEnterCriticalSection
	crate::xboxkrnl::RtlTryEnterCriticalSection(ctx, base);
	// 8246618C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82466190: 409A009C  bne cr6, 0x8246622c
	if !ctx.cr[6].eq {
	pc = 0x8246622C; continue 'dispatch;
	}
	// 82466194: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466198: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 8246619C: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824661A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824661A4: 419A0080  beq cr6, 0x82466224
	if ctx.cr[6].eq {
	pc = 0x82466224; continue 'dispatch;
	}
	// 824661A8: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824661AC: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824661B0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824661B4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824661B8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824661BC: 40980020  bge cr6, 0x824661dc
	if !ctx.cr[6].lt {
	pc = 0x824661DC; continue 'dispatch;
	}
	// 824661C0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824661C4: 392974E0  addi r9, r9, 0x74e0
	ctx.r[9].s64 = ctx.r[9].s64 + 29920;
	// 824661C8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824661CC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824661D0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824661D4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824661D8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x824661DC; continue 'dispatch;
            }
            0x824661DC => {
    //   block [0x824661DC..0x82466224)
	// 824661DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824661E0: 482A707D  bl 0x8270d25c
	ctx.lr = 0x824661E4;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 824661E4: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 824661E8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824661EC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824661F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824661F4: 40980038  bge cr6, 0x8246622c
	if !ctx.cr[6].lt {
	pc = 0x8246622C; continue 'dispatch;
	}
	// 824661F8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824661FC: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 82466200: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82466204: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82466208: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8246620C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82466210: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82466214: 48003C6D  bl 0x82469e80
	ctx.lr = 0x82466218;
	sub_82469E80(ctx, base);
	// 82466218: F87D0020  std r3, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[3].u64 ) };
	// 8246621C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466220: 480CEEEC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82466224 => {
    //   block [0x82466224..0x8246622C)
	// 82466224: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82466228: 482A7035  bl 0x8270d25c
	ctx.lr = 0x8246622C;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	pc = 0x8246622C; continue 'dispatch;
            }
            0x8246622C => {
    //   block [0x8246622C..0x8246623C)
	// 8246622C: 48003C55  bl 0x82469e80
	ctx.lr = 0x82466230;
	sub_82469E80(ctx, base);
	// 82466230: F87D0020  std r3, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[3].u64 ) };
	// 82466234: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466238: 480CEED4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466240 size=224
    let mut pc: u32 = 0x82466240;
    'dispatch: loop {
        match pc {
            0x82466240 => {
    //   block [0x82466240..0x82466258)
	// 82466240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466244: 480CEE75  bl 0x825350b8
	ctx.lr = 0x82466248;
	sub_82535080(ctx, base);
	// 82466248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246624C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82466250: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82466254: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	pc = 0x82466258; continue 'dispatch;
            }
            0x82466258 => {
    //   block [0x82466258..0x82466270)
	// 82466258: 7D64EA14  add r11, r4, r29
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 8246625C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82466260: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82466264: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82466268: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246626C: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	pc = 0x82466270; continue 'dispatch;
            }
            0x82466270 => {
    //   block [0x82466270..0x82466284)
	// 82466270: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466274: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82466278: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246627C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82466280: 40980018  bge cr6, 0x82466298
	if !ctx.cr[6].lt {
	pc = 0x82466298; continue 'dispatch;
	}
	pc = 0x82466284; continue 'dispatch;
            }
            0x82466284 => {
    //   block [0x82466284..0x82466298)
	// 82466284: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82466288: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8246628C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466290: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82466294: 4198FFF0  blt cr6, 0x82466284
	if ctx.cr[6].lt {
	pc = 0x82466284; continue 'dispatch;
	}
	pc = 0x82466298; continue 'dispatch;
            }
            0x82466298 => {
    //   block [0x82466298..0x824662AC)
	// 82466298: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246629C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824662A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824662A4: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824662A8: 40980018  bge cr6, 0x824662c0
	if !ctx.cr[6].lt {
	pc = 0x824662C0; continue 'dispatch;
	}
	pc = 0x824662AC; continue 'dispatch;
            }
            0x824662AC => {
    //   block [0x824662AC..0x824662C0)
	// 824662AC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 824662B0: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 824662B4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824662B8: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824662BC: 4198FFF0  blt cr6, 0x824662ac
	if ctx.cr[6].lt {
	pc = 0x824662AC; continue 'dispatch;
	}
	pc = 0x824662C0; continue 'dispatch;
            }
            0x824662C0 => {
    //   block [0x824662C0..0x824662E4)
	// 824662C0: 7F05F800  cmpw cr6, r5, r31
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824662C4: 41980030  blt cr6, 0x824662f4
	if ctx.cr[6].lt {
	pc = 0x824662F4; continue 'dispatch;
	}
	// 824662C8: 419A001C  beq cr6, 0x824662e4
	if ctx.cr[6].eq {
	pc = 0x824662E4; continue 'dispatch;
	}
	// 824662CC: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824662D0: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824662D4: 7CEBF02E  lwzx r7, r11, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824662D8: 7D2AF02E  lwzx r9, r10, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824662DC: 7CEAF12E  stwx r7, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[7].u32) };
	// 824662E0: 7D2BF12E  stwx r9, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u32) };
	pc = 0x824662E4; continue 'dispatch;
            }
            0x824662E4 => {
    //   block [0x824662E4..0x824662F4)
	// 824662E4: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 824662E8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824662EC: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 824662F0: 4099FF80  ble cr6, 0x82466270
	if !ctx.cr[6].gt {
	pc = 0x82466270; continue 'dispatch;
	}
	pc = 0x824662F4; continue 'dispatch;
            }
            0x824662F4 => {
    //   block [0x824662F4..0x82466308)
	// 824662F4: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 824662F8: 40980010  bge cr6, 0x82466308
	if !ctx.cr[6].lt {
	pc = 0x82466308; continue 'dispatch;
	}
	// 824662FC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82466300: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466304: 4BFFFF3D  bl 0x82466240
	ctx.lr = 0x82466308;
	sub_82466240(ctx, base);
	pc = 0x82466308; continue 'dispatch;
            }
            0x82466308 => {
    //   block [0x82466308..0x82466318)
	// 82466308: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8246630C: 4098000C  bge cr6, 0x82466318
	if !ctx.cr[6].lt {
	pc = 0x82466318; continue 'dispatch;
	}
	// 82466310: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82466314: 4BFFFF44  b 0x82466258
	pc = 0x82466258; continue 'dispatch;
            }
            0x82466318 => {
    //   block [0x82466318..0x82466320)
	// 82466318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246631C: 480CEDEC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466320 size=124
    let mut pc: u32 = 0x82466320;
    'dispatch: loop {
        match pc {
            0x82466320 => {
    //   block [0x82466320..0x82466374)
	// 82466320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82466328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246632C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82466330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82466338: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246633C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466340: 419A0044  beq cr6, 0x82466384
	if ctx.cr[6].eq {
	pc = 0x82466384; continue 'dispatch;
	}
	// 82466344: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82466348: 3BCB9198  addi r30, r11, -0x6e68
	ctx.r[30].s64 = ctx.r[11].s64 + -28264;
	// 8246634C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466350: 4BFFFE29  bl 0x82466178
	ctx.lr = 0x82466354;
	sub_82466178(ctx, base);
	// 82466354: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466358: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246635C: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82466360: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82466364: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466368: 419A000C  beq cr6, 0x82466374
	if ctx.cr[6].eq {
	pc = 0x82466374; continue 'dispatch;
	}
	// 8246636C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466370: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	pc = 0x82466374; continue 'dispatch;
            }
            0x82466374 => {
    //   block [0x82466374..0x82466384)
	// 82466374: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82466378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246637C: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82466380: 482A6EED  bl 0x8270d26c
	ctx.lr = 0x82466384;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x82466384; continue 'dispatch;
            }
            0x82466384 => {
    //   block [0x82466384..0x8246639C)
	// 82466384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82466390: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82466394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82466398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824663A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824663A0 size=144
    let mut pc: u32 = 0x824663A0;
    'dispatch: loop {
        match pc {
            0x824663A0 => {
    //   block [0x824663A0..0x824663F8)
	// 824663A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824663A4: 480CED15  bl 0x825350b8
	ctx.lr = 0x824663A8;
	sub_82535080(ctx, base);
	// 824663A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824663AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824663B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824663B4: 3BFD0028  addi r31, r29, 0x28
	ctx.r[31].s64 = ctx.r[29].s64 + 40;
	// 824663B8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824663BC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824663C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824663C4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 824663C8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824663CC: 419A0044  beq cr6, 0x82466410
	if ctx.cr[6].eq {
	pc = 0x82466410; continue 'dispatch;
	}
	// 824663D0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824663D4: 3BCB9198  addi r30, r11, -0x6e68
	ctx.r[30].s64 = ctx.r[11].s64 + -28264;
	// 824663D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824663DC: 4BFFFD9D  bl 0x82466178
	ctx.lr = 0x824663E0;
	sub_82466178(ctx, base);
	// 824663E0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 824663E4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824663E8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824663EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824663F0: 419A0008  beq cr6, 0x824663f8
	if ctx.cr[6].eq {
	pc = 0x824663F8; continue 'dispatch;
	}
	// 824663F4: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	pc = 0x824663F8; continue 'dispatch;
            }
            0x824663F8 => {
    //   block [0x824663F8..0x82466410)
	// 824663F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 824663FC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82466400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466404: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82466408: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 8246640C: 482A6E61  bl 0x8270d26c
	ctx.lr = 0x82466410;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	pc = 0x82466410; continue 'dispatch;
            }
            0x82466410 => {
    //   block [0x82466410..0x82466430)
	// 82466410: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82466414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82466418: 482A7905  bl 0x8270dd1c
	ctx.lr = 0x8246641C;
	// extern call 0x8270DD1C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 8246641C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82466420: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82466424: F97D0020  std r11, 0x20(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82466428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246642C: 480CECDC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466430 size=92
    let mut pc: u32 = 0x82466430;
    'dispatch: loop {
        match pc {
            0x82466430 => {
    //   block [0x82466430..0x82466474)
	// 82466430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82466438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246643C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82466440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82466448: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246644C: 4BFFF94D  bl 0x82465d98
	ctx.lr = 0x82466450;
	sub_82465D98(ctx, base);
	// 82466450: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82466454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246645C: 419A0018  beq cr6, 0x82466474
	if ctx.cr[6].eq {
	pc = 0x82466474; continue 'dispatch;
	}
	// 82466460: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82466464: 816B49B0  lwz r11, 0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18864 as u32) ) } as u64;
	// 82466468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246646C: 4E800421  bctrl
	ctx.lr = 0x82466470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82466470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            0x82466474 => {
    //   block [0x82466474..0x8246648C)
	// 82466474: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246647C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82466480: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82466484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82466488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466490 size=4
    let mut pc: u32 = 0x82466490;
    'dispatch: loop {
        match pc {
            0x82466490 => {
    //   block [0x82466490..0x82466494)
	// 82466490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466498 size=4
    let mut pc: u32 = 0x82466498;
    'dispatch: loop {
        match pc {
            0x82466498 => {
    //   block [0x82466498..0x8246649C)
	// 82466498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824664A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824664A0 size=8
    let mut pc: u32 = 0x824664A0;
    'dispatch: loop {
        match pc {
            0x824664A0 => {
    //   block [0x824664A0..0x824664A8)
	// 824664A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824664A4: 48000004  b 0x824664a8
	sub_824664A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824664A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824664A8 size=184
    let mut pc: u32 = 0x824664A8;
    'dispatch: loop {
        match pc {
            0x824664A8 => {
    //   block [0x824664A8..0x824664F0)
	// 824664A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824664AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824664B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824664B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824664B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824664BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824664C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824664C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824664C8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824664CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824664D0: 409A0020  bne cr6, 0x824664f0
	if !ctx.cr[6].eq {
	pc = 0x824664F0; continue 'dispatch;
	}
	// 824664D4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824664D8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824664DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824664E0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824664E4: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824664E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824664EC: 4BFFDBCD  bl 0x824640b8
	ctx.lr = 0x824664F0;
	sub_824640B8(ctx, base);
	pc = 0x824664F0; continue 'dispatch;
            }
            0x824664F0 => {
    //   block [0x824664F0..0x8246652C)
	// 824664F0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824664F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824664F8: 419A004C  beq cr6, 0x82466544
	if ctx.cr[6].eq {
	pc = 0x82466544; continue 'dispatch;
	}
	// 824664FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466500: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82466504: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82466508: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8246650C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82466510: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82466514: 41980018  blt cr6, 0x8246652c
	if ctx.cr[6].lt {
	pc = 0x8246652C; continue 'dispatch;
	}
	// 82466518: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246651C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82466520: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82466524: 4BFFD9F5  bl 0x82463f18
	ctx.lr = 0x82466528;
	sub_82463F18(ctx, base);
	// 82466528: 4800001C  b 0x82466544
	pc = 0x82466544; continue 'dispatch;
            }
            0x8246652C => {
    //   block [0x8246652C..0x82466544)
	// 8246652C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82466530: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82466534: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82466538: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8246653C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82466540: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	pc = 0x82466544; continue 'dispatch;
            }
            0x82466544 => {
    //   block [0x82466544..0x82466560)
	// 82466544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466548: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246654C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82466550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82466554: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82466558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246655C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466560 size=4
    let mut pc: u32 = 0x82466560;
    'dispatch: loop {
        match pc {
            0x82466560 => {
    //   block [0x82466560..0x82466564)
	// 82466560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466568 size=4
    let mut pc: u32 = 0x82466568;
    'dispatch: loop {
        match pc {
            0x82466568 => {
    //   block [0x82466568..0x8246656C)
	// 82466568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466570 size=4
    let mut pc: u32 = 0x82466570;
    'dispatch: loop {
        match pc {
            0x82466570 => {
    //   block [0x82466570..0x82466574)
	// 82466570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466578 size=24
    let mut pc: u32 = 0x82466578;
    'dispatch: loop {
        match pc {
            0x82466578 => {
    //   block [0x82466578..0x82466590)
	// 82466578: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8246657C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82466580: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82466584: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82466588: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246658C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466590 size=8
    let mut pc: u32 = 0x82466590;
    'dispatch: loop {
        match pc {
            0x82466590 => {
    //   block [0x82466590..0x82466598)
	// 82466590: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82466594: 48000004  b 0x82466598
	sub_82466598(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466598 size=196
    let mut pc: u32 = 0x82466598;
    'dispatch: loop {
        match pc {
            0x82466598 => {
    //   block [0x82466598..0x824665EC)
	// 82466598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246659C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824665A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824665A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824665A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824665AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824665B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824665B4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824665B8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824665BC: 419A0030  beq cr6, 0x824665ec
	if ctx.cr[6].eq {
	pc = 0x824665EC; continue 'dispatch;
	}
	// 824665C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824665C4: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824665C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824665CC: 409A0020  bne cr6, 0x824665ec
	if !ctx.cr[6].eq {
	pc = 0x824665EC; continue 'dispatch;
	}
	// 824665D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824665D4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824665D8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824665DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824665E0: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824665E4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824665E8: 4BFFDAD1  bl 0x824640b8
	ctx.lr = 0x824665EC;
	sub_824640B8(ctx, base);
	pc = 0x824665EC; continue 'dispatch;
            }
            0x824665EC => {
    //   block [0x824665EC..0x82466628)
	// 824665EC: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824665F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824665F4: 419A004C  beq cr6, 0x82466640
	if ctx.cr[6].eq {
	pc = 0x82466640; continue 'dispatch;
	}
	// 824665F8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824665FC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82466600: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82466604: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82466608: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8246660C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82466610: 41980018  blt cr6, 0x82466628
	if ctx.cr[6].lt {
	pc = 0x82466628; continue 'dispatch;
	}
	// 82466614: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82466618: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8246661C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82466620: 4BFFD8F9  bl 0x82463f18
	ctx.lr = 0x82466624;
	sub_82463F18(ctx, base);
	// 82466624: 4800001C  b 0x82466640
	pc = 0x82466640; continue 'dispatch;
            }
            0x82466628 => {
    //   block [0x82466628..0x82466640)
	// 82466628: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8246662C: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82466630: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82466634: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82466638: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8246663C: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	pc = 0x82466640; continue 'dispatch;
            }
            0x82466640 => {
    //   block [0x82466640..0x8246665C)
	// 82466640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246664C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82466650: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82466654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82466658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466660 size=4
    let mut pc: u32 = 0x82466660;
    'dispatch: loop {
        match pc {
            0x82466660 => {
    //   block [0x82466660..0x82466664)
	// 82466660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466668 size=4
    let mut pc: u32 = 0x82466668;
    'dispatch: loop {
        match pc {
            0x82466668 => {
    //   block [0x82466668..0x8246666C)
	// 82466668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466678 size=16
    let mut pc: u32 = 0x82466678;
    'dispatch: loop {
        match pc {
            0x82466678 => {
    //   block [0x82466678..0x82466688)
	// 82466678: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246667C: 396B7A34  addi r11, r11, 0x7a34
	ctx.r[11].s64 = ctx.r[11].s64 + 31284;
	// 82466680: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82466684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466698 size=132
    let mut pc: u32 = 0x82466698;
    'dispatch: loop {
        match pc {
            0x82466698 => {
    //   block [0x82466698..0x824666C4)
	// 82466698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246669C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824666A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824666A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824666A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824666AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824666B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824666B4: 419A0010  beq cr6, 0x824666c4
	if ctx.cr[6].eq {
	pc = 0x824666C4; continue 'dispatch;
	}
	// 824666B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824666BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824666C0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	pc = 0x824666C4; continue 'dispatch;
            }
            0x824666C4 => {
    //   block [0x824666C4..0x82466700)
	// 824666C4: 3FC08293  lis r30, -0x7d6d
	ctx.r[30].s64 = -2104295424;
	// 824666C8: 807E9004  lwz r3, -0x6ffc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824666CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824666D0: 419A0030  beq cr6, 0x82466700
	if ctx.cr[6].eq {
	pc = 0x82466700; continue 'dispatch;
	}
	// 824666D4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824666D8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824666DC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824666E0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824666E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824666E8: 409A0018  bne cr6, 0x82466700
	if !ctx.cr[6].eq {
	pc = 0x82466700; continue 'dispatch;
	}
	// 824666EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824666F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824666F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824666F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824666FC: 4E800421  bctrl
	ctx.lr = 0x82466700;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82466700 => {
    //   block [0x82466700..0x8246671C)
	// 82466700: 93FE9004  stw r31, -0x6ffc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-28668 as u32), ctx.r[31].u32 ) };
	// 82466704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246670C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82466710: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82466714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82466718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466720 size=24
    let mut pc: u32 = 0x82466720;
    'dispatch: loop {
        match pc {
            0x82466720 => {
    //   block [0x82466720..0x82466738)
	// 82466720: 2F040010  cmpwi cr6, r4, 0x10
	ctx.cr[6].compare_i32(ctx.r[4].s32, 16, &mut ctx.xer);
	// 82466724: 40990014  ble cr6, 0x82466738
	if !ctx.cr[6].gt {
		crate::recompiler::externs::call(ctx, base, 0x82466738);
		return;
	}
	// 82466728: 3964000F  addi r11, r4, 0xf
	ctx.r[11].s64 = ctx.r[4].s64 + 15;
	// 8246672C: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82466730: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82466734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466740 size=12
    let mut pc: u32 = 0x82466740;
    'dispatch: loop {
        match pc {
            0x82466740 => {
    //   block [0x82466740..0x8246674C)
	// 82466740: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82466744: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82466748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466750 size=48
    let mut pc: u32 = 0x82466750;
    'dispatch: loop {
        match pc {
            0x82466750 => {
    //   block [0x82466750..0x82466780)
	// 82466750: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82466754: 3D007FFF  lis r8, 0x7fff
	ctx.r[8].s64 = 2147418112;
	// 82466758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246675C: 396B7A34  addi r11, r11, 0x7a34
	ctx.r[11].s64 = ctx.r[11].s64 + 31284;
	// 82466760: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82466764: 6108FFFF  ori r8, r8, 0xffff
	ctx.r[8].u64 = ctx.r[8].u64 | 65535;
	// 82466768: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8246676C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82466770: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82466774: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82466778: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8246677C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466780 size=84
    let mut pc: u32 = 0x82466780;
    'dispatch: loop {
        match pc {
            0x82466780 => {
    //   block [0x82466780..0x824667C0)
	// 82466780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82466788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246678C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466790: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82466794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82466798: 396B7A34  addi r11, r11, 0x7a34
	ctx.r[11].s64 = ctx.r[11].s64 + 31284;
	// 8246679C: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824667A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824667A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824667A8: 419A0018  beq cr6, 0x824667c0
	if ctx.cr[6].eq {
	pc = 0x824667C0; continue 'dispatch;
	}
	// 824667AC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 824667B0: 816B49B0  lwz r11, 0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18864 as u32) ) } as u64;
	// 824667B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824667B8: 4E800421  bctrl
	ctx.lr = 0x824667BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824667BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            0x824667C0 => {
    //   block [0x824667C0..0x824667D4)
	// 824667C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824667C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824667C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824667CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824667D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824667D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824667D8 size=128
    let mut pc: u32 = 0x824667D8;
    'dispatch: loop {
        match pc {
            0x824667D8 => {
    //   block [0x824667D8..0x82466800)
	// 824667D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824667DC: 480CE8DD  bl 0x825350b8
	ctx.lr = 0x824667E0;
	sub_82535080(ctx, base);
	// 824667E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824667E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824667E8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824667EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824667F0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824667F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824667F8: 40990034  ble cr6, 0x8246682c
	if !ctx.cr[6].gt {
	pc = 0x8246682C; continue 'dispatch;
	}
	// 824667FC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82466800; continue 'dispatch;
            }
            0x82466800 => {
    //   block [0x82466800..0x8246682C)
	// 82466800: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466804: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82466808: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246680C: 480037C5  bl 0x82469fd0
	ctx.lr = 0x82466810;
	sub_82469FD0(ctx, base);
	// 82466810: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82466814: 419A0024  beq cr6, 0x82466838
	if ctx.cr[6].eq {
	pc = 0x82466838; continue 'dispatch;
	}
	// 82466818: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246681C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82466820: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82466824: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82466828: 4198FFD8  blt cr6, 0x82466800
	if ctx.cr[6].lt {
	pc = 0x82466800; continue 'dispatch;
	}
	pc = 0x8246682C; continue 'dispatch;
            }
            0x8246682C => {
    //   block [0x8246682C..0x82466838)
	// 8246682C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82466830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466834: 480CE8D4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82466838 => {
    //   block [0x82466838..0x82466858)
	// 82466838: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246683C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466840: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82466844: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466848: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246684C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82466850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466854: 480CE8B4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466858 size=8
    let mut pc: u32 = 0x82466858;
    'dispatch: loop {
        match pc {
            0x82466858 => {
    //   block [0x82466858..0x82466860)
	// 82466858: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246685C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466860 size=8
    let mut pc: u32 = 0x82466860;
    'dispatch: loop {
        match pc {
            0x82466860 => {
    //   block [0x82466860..0x82466868)
	// 82466860: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466868 size=8
    let mut pc: u32 = 0x82466868;
    'dispatch: loop {
        match pc {
            0x82466868 => {
    //   block [0x82466868..0x82466870)
	// 82466868: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246686C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466870 size=36
    let mut pc: u32 = 0x82466870;
    'dispatch: loop {
        match pc {
            0x82466870 => {
    //   block [0x82466870..0x82466880)
	// 82466870: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82466874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82466878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246687C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82466880; continue 'dispatch;
            }
            0x82466880 => {
    //   block [0x82466880..0x82466894)
	// 82466880: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466884: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82466888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246688C: 409AFFF4  bne cr6, 0x82466880
	if !ctx.cr[6].eq {
	pc = 0x82466880; continue 'dispatch;
	}
	// 82466890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466898 size=40
    let mut pc: u32 = 0x82466898;
    'dispatch: loop {
        match pc {
            0x82466898 => {
    //   block [0x82466898..0x824668A0)
	// 82466898: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246689C: 419A0018  beq cr6, 0x824668b4
	if ctx.cr[6].eq {
	pc = 0x824668B4; continue 'dispatch;
	}
	pc = 0x824668A0; continue 'dispatch;
            }
            0x824668A0 => {
    //   block [0x824668A0..0x824668B4)
	// 824668A0: 7F052040  cmplw cr6, r5, r4
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824668A4: 419A001C  beq cr6, 0x824668c0
	if ctx.cr[6].eq {
		sub_824668C0(ctx, base);
		return;
	}
	// 824668A8: 80A50004  lwz r5, 4(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 824668AC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 824668B0: 409AFFF0  bne cr6, 0x824668a0
	if !ctx.cr[6].eq {
	pc = 0x824668A0; continue 'dispatch;
	}
	pc = 0x824668B4; continue 'dispatch;
            }
            0x824668B4 => {
    //   block [0x824668B4..0x824668C0)
	// 824668B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824668B8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824668BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824668C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824668C0 size=12
    let mut pc: u32 = 0x824668C0;
    'dispatch: loop {
        match pc {
            0x824668C0 => {
    //   block [0x824668C0..0x824668CC)
	// 824668C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824668C4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824668C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824668D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824668D0 size=40
    let mut pc: u32 = 0x824668D0;
    'dispatch: loop {
        match pc {
            0x824668D0 => {
    //   block [0x824668D0..0x824668E0)
	// 824668D0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824668D4: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824668D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824668DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x824668E0; continue 'dispatch;
            }
            0x824668E0 => {
    //   block [0x824668E0..0x824668F8)
	// 824668E0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824668E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824668E8: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 824668EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824668F0: 409AFFF0  bne cr6, 0x824668e0
	if !ctx.cr[6].eq {
	pc = 0x824668E0; continue 'dispatch;
	}
	// 824668F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824668F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824668F8 size=8
    let mut pc: u32 = 0x824668F8;
    'dispatch: loop {
        match pc {
            0x824668F8 => {
    //   block [0x824668F8..0x82466900)
	// 824668F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824668FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466900 size=8
    let mut pc: u32 = 0x82466900;
    'dispatch: loop {
        match pc {
            0x82466900 => {
    //   block [0x82466900..0x82466908)
	// 82466900: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82466904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466908 size=8
    let mut pc: u32 = 0x82466908;
    'dispatch: loop {
        match pc {
            0x82466908 => {
    //   block [0x82466908..0x82466910)
	// 82466908: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246690C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466910 size=40
    let mut pc: u32 = 0x82466910;
    'dispatch: loop {
        match pc {
            0x82466910 => {
    //   block [0x82466910..0x82466920)
	// 82466910: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466914: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246691C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82466920; continue 'dispatch;
            }
            0x82466920 => {
    //   block [0x82466920..0x82466938)
	// 82466920: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466924: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466928: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8246692C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466930: 409AFFF0  bne cr6, 0x82466920
	if !ctx.cr[6].eq {
	pc = 0x82466920; continue 'dispatch;
	}
	// 82466934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466938 size=80
    let mut pc: u32 = 0x82466938;
    'dispatch: loop {
        match pc {
            0x82466938 => {
    //   block [0x82466938..0x82466948)
	// 82466938: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246693C: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466940: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466944: 419A0018  beq cr6, 0x8246695c
	if ctx.cr[6].eq {
	pc = 0x8246695C; continue 'dispatch;
	}
	pc = 0x82466948; continue 'dispatch;
            }
            0x82466948 => {
    //   block [0x82466948..0x8246695C)
	// 82466948: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246694C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466950: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82466954: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466958: 409AFFF0  bne cr6, 0x82466948
	if !ctx.cr[6].eq {
	pc = 0x82466948; continue 'dispatch;
	}
	pc = 0x8246695C; continue 'dispatch;
            }
            0x8246695C => {
    //   block [0x8246695C..0x82466964)
	// 8246695C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82466960: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	pc = 0x82466964; continue 'dispatch;
            }
            0x82466964 => {
    //   block [0x82466964..0x82466988)
	// 82466964: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466968: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8246696C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82466970: 40980018  bge cr6, 0x82466988
	if !ctx.cr[6].lt {
		sub_82466988(ctx, base);
		return;
	}
	// 82466974: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466978: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246697C: 409AFFE8  bne cr6, 0x82466964
	if !ctx.cr[6].eq {
	pc = 0x82466964; continue 'dispatch;
	}
	// 82466980: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82466984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466988 size=24
    let mut pc: u32 = 0x82466988;
    'dispatch: loop {
        match pc {
            0x82466988 => {
    //   block [0x82466988..0x824669A0)
	// 82466988: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8246698C: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82466990: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82466994: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466998: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246699C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824669A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824669A0 size=200
    let mut pc: u32 = 0x824669A0;
    'dispatch: loop {
        match pc {
            0x824669A0 => {
    //   block [0x824669A0..0x824669C4)
	// 824669A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824669A4: 480CE715  bl 0x825350b8
	ctx.lr = 0x824669A8;
	sub_82535080(ctx, base);
	// 824669A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824669AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824669B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824669B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824669B8: 4BFFFF59  bl 0x82466910
	ctx.lr = 0x824669BC;
	sub_82466910(ctx, base);
	// 824669BC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824669C0: 40990078  ble cr6, 0x82466a38
	if !ctx.cr[6].gt {
	pc = 0x82466A38; continue 'dispatch;
	}
	pc = 0x824669C4; continue 'dispatch;
            }
            0x824669C4 => {
    //   block [0x824669C4..0x824669D4)
	// 824669C4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824669C8: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 824669CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824669D0: 419A0018  beq cr6, 0x824669e8
	if ctx.cr[6].eq {
	pc = 0x824669E8; continue 'dispatch;
	}
	pc = 0x824669D4; continue 'dispatch;
            }
            0x824669D4 => {
    //   block [0x824669D4..0x824669E8)
	// 824669D4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824669D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824669DC: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824669E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824669E4: 409AFFF0  bne cr6, 0x824669d4
	if !ctx.cr[6].eq {
	pc = 0x824669D4; continue 'dispatch;
	}
	pc = 0x824669E8; continue 'dispatch;
            }
            0x824669E8 => {
    //   block [0x824669E8..0x824669F0)
	// 824669E8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 824669EC: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	pc = 0x824669F0; continue 'dispatch;
            }
            0x824669F0 => {
    //   block [0x824669F0..0x82466A10)
	// 824669F0: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 824669F4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824669F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824669FC: 40980048  bge cr6, 0x82466a44
	if !ctx.cr[6].lt {
	pc = 0x82466A44; continue 'dispatch;
	}
	// 82466A00: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466A04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82466A08: 409AFFE8  bne cr6, 0x824669f0
	if !ctx.cr[6].eq {
	pc = 0x824669F0; continue 'dispatch;
	}
	// 82466A0C: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	pc = 0x82466A10; continue 'dispatch;
            }
            0x82466A10 => {
    //   block [0x82466A10..0x82466A38)
	// 82466A10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82466A14: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466A18: 480035B9  bl 0x82469fd0
	ctx.lr = 0x82466A1C;
	sub_82469FD0(ctx, base);
	// 82466A1C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82466A20: 419A003C  beq cr6, 0x82466a5c
	if ctx.cr[6].eq {
	pc = 0x82466A5C; continue 'dispatch;
	}
	// 82466A24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466A28: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82466A2C: 4BFFFEE5  bl 0x82466910
	ctx.lr = 0x82466A30;
	sub_82466910(ctx, base);
	// 82466A30: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82466A34: 4198FF90  blt cr6, 0x824669c4
	if ctx.cr[6].lt {
	pc = 0x824669C4; continue 'dispatch;
	}
	pc = 0x82466A38; continue 'dispatch;
            }
            0x82466A38 => {
    //   block [0x82466A38..0x82466A44)
	// 82466A38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82466A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466A40: 480CE6C8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82466A44 => {
    //   block [0x82466A44..0x82466A5C)
	// 82466A44: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82466A48: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82466A4C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82466A50: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466A54: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82466A58: 4BFFFFB8  b 0x82466a10
	pc = 0x82466A10; continue 'dispatch;
            }
            0x82466A5C => {
    //   block [0x82466A5C..0x82466A68)
	// 82466A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466A64: 480CE6A4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466A68 size=8
    let mut pc: u32 = 0x82466A68;
    'dispatch: loop {
        match pc {
            0x82466A68 => {
    //   block [0x82466A68..0x82466A70)
	// 82466A68: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466A70 size=40
    let mut pc: u32 = 0x82466A70;
    'dispatch: loop {
        match pc {
            0x82466A70 => {
    //   block [0x82466A70..0x82466A80)
	// 82466A70: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466A74: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466A78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466A7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x82466A80; continue 'dispatch;
            }
            0x82466A80 => {
    //   block [0x82466A80..0x82466A98)
	// 82466A80: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466A84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466A88: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82466A8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466A90: 409AFFF0  bne cr6, 0x82466a80
	if !ctx.cr[6].eq {
	pc = 0x82466A80; continue 'dispatch;
	}
	// 82466A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466A98 size=80
    let mut pc: u32 = 0x82466A98;
    'dispatch: loop {
        match pc {
            0x82466A98 => {
    //   block [0x82466A98..0x82466AA8)
	// 82466A98: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466A9C: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466AA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466AA4: 419A0018  beq cr6, 0x82466abc
	if ctx.cr[6].eq {
	pc = 0x82466ABC; continue 'dispatch;
	}
	pc = 0x82466AA8; continue 'dispatch;
            }
            0x82466AA8 => {
    //   block [0x82466AA8..0x82466ABC)
	// 82466AA8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466AAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466AB0: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82466AB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466AB8: 409AFFF0  bne cr6, 0x82466aa8
	if !ctx.cr[6].eq {
	pc = 0x82466AA8; continue 'dispatch;
	}
	pc = 0x82466ABC; continue 'dispatch;
            }
            0x82466ABC => {
    //   block [0x82466ABC..0x82466AC4)
	// 82466ABC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82466AC0: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	pc = 0x82466AC4; continue 'dispatch;
            }
            0x82466AC4 => {
    //   block [0x82466AC4..0x82466AE8)
	// 82466AC4: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466AC8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82466ACC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82466AD0: 40980018  bge cr6, 0x82466ae8
	if !ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x82466AE8);
		return;
	}
	// 82466AD4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466AD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82466ADC: 409AFFE8  bne cr6, 0x82466ac4
	if !ctx.cr[6].eq {
	pc = 0x82466AC4; continue 'dispatch;
	}
	// 82466AE0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466B00 size=4
    let mut pc: u32 = 0x82466B00;
    'dispatch: loop {
        match pc {
            0x82466B00 => {
    //   block [0x82466B00..0x82466B04)
	// 82466B00: 4BFFFF98  b 0x82466a98
	sub_82466A98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466B08 size=8
    let mut pc: u32 = 0x82466B08;
    'dispatch: loop {
        match pc {
            0x82466B08 => {
    //   block [0x82466B08..0x82466B10)
	// 82466B08: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466B10 size=24
    let mut pc: u32 = 0x82466B10;
    'dispatch: loop {
        match pc {
            0x82466B10 => {
    //   block [0x82466B10..0x82466B28)
	// 82466B10: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466B14: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466B18: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82466B1C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466B20: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82466B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466B28 size=200
    let mut pc: u32 = 0x82466B28;
    'dispatch: loop {
        match pc {
            0x82466B28 => {
    //   block [0x82466B28..0x82466B4C)
	// 82466B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466B2C: 480CE58D  bl 0x825350b8
	ctx.lr = 0x82466B30;
	sub_82535080(ctx, base);
	// 82466B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466B34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82466B38: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82466B3C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82466B40: 4BFFFF31  bl 0x82466a70
	ctx.lr = 0x82466B44;
	sub_82466A70(ctx, base);
	// 82466B44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82466B48: 40990078  ble cr6, 0x82466bc0
	if !ctx.cr[6].gt {
	pc = 0x82466BC0; continue 'dispatch;
	}
	pc = 0x82466B4C; continue 'dispatch;
            }
            0x82466B4C => {
    //   block [0x82466B4C..0x82466B5C)
	// 82466B4C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466B50: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466B58: 419A0018  beq cr6, 0x82466b70
	if ctx.cr[6].eq {
	pc = 0x82466B70; continue 'dispatch;
	}
	pc = 0x82466B5C; continue 'dispatch;
            }
            0x82466B5C => {
    //   block [0x82466B5C..0x82466B70)
	// 82466B5C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466B60: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466B64: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82466B68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466B6C: 409AFFF0  bne cr6, 0x82466b5c
	if !ctx.cr[6].eq {
	pc = 0x82466B5C; continue 'dispatch;
	}
	pc = 0x82466B70; continue 'dispatch;
            }
            0x82466B70 => {
    //   block [0x82466B70..0x82466B78)
	// 82466B70: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82466B74: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	pc = 0x82466B78; continue 'dispatch;
            }
            0x82466B78 => {
    //   block [0x82466B78..0x82466B98)
	// 82466B78: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466B7C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82466B80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82466B84: 40980048  bge cr6, 0x82466bcc
	if !ctx.cr[6].lt {
	pc = 0x82466BCC; continue 'dispatch;
	}
	// 82466B88: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466B8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82466B90: 409AFFE8  bne cr6, 0x82466b78
	if !ctx.cr[6].eq {
	pc = 0x82466B78; continue 'dispatch;
	}
	// 82466B94: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	pc = 0x82466B98; continue 'dispatch;
            }
            0x82466B98 => {
    //   block [0x82466B98..0x82466BC0)
	// 82466B98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82466B9C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466BA0: 48003431  bl 0x82469fd0
	ctx.lr = 0x82466BA4;
	sub_82469FD0(ctx, base);
	// 82466BA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82466BA8: 419A003C  beq cr6, 0x82466be4
	if ctx.cr[6].eq {
	pc = 0x82466BE4; continue 'dispatch;
	}
	// 82466BAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466BB0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82466BB4: 4BFFFEBD  bl 0x82466a70
	ctx.lr = 0x82466BB8;
	sub_82466A70(ctx, base);
	// 82466BB8: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82466BBC: 4198FF90  blt cr6, 0x82466b4c
	if ctx.cr[6].lt {
	pc = 0x82466B4C; continue 'dispatch;
	}
	pc = 0x82466BC0; continue 'dispatch;
            }
            0x82466BC0 => {
    //   block [0x82466BC0..0x82466BCC)
	// 82466BC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82466BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466BC8: 480CE540  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82466BCC => {
    //   block [0x82466BCC..0x82466BE4)
	// 82466BCC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82466BD0: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466BD4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82466BD8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466BDC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82466BE0: 4BFFFFB8  b 0x82466b98
	pc = 0x82466B98; continue 'dispatch;
            }
            0x82466BE4 => {
    //   block [0x82466BE4..0x82466BF0)
	// 82466BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466BEC: 480CE51C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466BF0 size=200
    let mut pc: u32 = 0x82466BF0;
    'dispatch: loop {
        match pc {
            0x82466BF0 => {
    //   block [0x82466BF0..0x82466C14)
	// 82466BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466BF4: 480CE4C9  bl 0x825350bc
	ctx.lr = 0x82466BF8;
	sub_82535080(ctx, base);
	// 82466BF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466BFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82466C00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82466C04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82466C08: 4BFFFE69  bl 0x82466a70
	ctx.lr = 0x82466C0C;
	sub_82466A70(ctx, base);
	// 82466C0C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82466C10: 40990078  ble cr6, 0x82466c88
	if !ctx.cr[6].gt {
	pc = 0x82466C88; continue 'dispatch;
	}
	pc = 0x82466C14; continue 'dispatch;
            }
            0x82466C14 => {
    //   block [0x82466C14..0x82466C24)
	// 82466C14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466C18: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466C1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466C20: 419A0018  beq cr6, 0x82466c38
	if ctx.cr[6].eq {
	pc = 0x82466C38; continue 'dispatch;
	}
	pc = 0x82466C24; continue 'dispatch;
            }
            0x82466C24 => {
    //   block [0x82466C24..0x82466C38)
	// 82466C24: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466C28: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466C2C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82466C30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466C34: 409AFFF0  bne cr6, 0x82466c24
	if !ctx.cr[6].eq {
	pc = 0x82466C24; continue 'dispatch;
	}
	pc = 0x82466C38; continue 'dispatch;
            }
            0x82466C38 => {
    //   block [0x82466C38..0x82466C40)
	// 82466C38: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82466C3C: 7D69F050  subf r11, r9, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	pc = 0x82466C40; continue 'dispatch;
            }
            0x82466C40 => {
    //   block [0x82466C40..0x82466C60)
	// 82466C40: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466C44: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82466C48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82466C4C: 40980048  bge cr6, 0x82466c94
	if !ctx.cr[6].lt {
	pc = 0x82466C94; continue 'dispatch;
	}
	// 82466C50: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466C54: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82466C58: 409AFFE8  bne cr6, 0x82466c40
	if !ctx.cr[6].eq {
	pc = 0x82466C40; continue 'dispatch;
	}
	// 82466C5C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	pc = 0x82466C60; continue 'dispatch;
            }
            0x82466C60 => {
    //   block [0x82466C60..0x82466C88)
	// 82466C60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82466C64: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466C68: 48003369  bl 0x82469fd0
	ctx.lr = 0x82466C6C;
	sub_82469FD0(ctx, base);
	// 82466C6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82466C70: 419A003C  beq cr6, 0x82466cac
	if ctx.cr[6].eq {
	pc = 0x82466CAC; continue 'dispatch;
	}
	// 82466C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466C78: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82466C7C: 4BFFFDF5  bl 0x82466a70
	ctx.lr = 0x82466C80;
	sub_82466A70(ctx, base);
	// 82466C80: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82466C84: 4198FF90  blt cr6, 0x82466c14
	if ctx.cr[6].lt {
	pc = 0x82466C14; continue 'dispatch;
	}
	pc = 0x82466C88; continue 'dispatch;
            }
            0x82466C88 => {
    //   block [0x82466C88..0x82466C94)
	// 82466C88: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82466C8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466C90: 480CE47C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82466C94 => {
    //   block [0x82466C94..0x82466CAC)
	// 82466C94: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82466C98: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466C9C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82466CA0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466CA4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82466CA8: 4BFFFFB8  b 0x82466c60
	pc = 0x82466C60; continue 'dispatch;
            }
            0x82466CAC => {
    //   block [0x82466CAC..0x82466CB8)
	// 82466CAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466CB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82466CB4: 480CE458  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466CB8 size=8
    let mut pc: u32 = 0x82466CB8;
    'dispatch: loop {
        match pc {
            0x82466CB8 => {
    //   block [0x82466CB8..0x82466CC0)
	// 82466CB8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82466CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466CC0 size=8
    let mut pc: u32 = 0x82466CC0;
    'dispatch: loop {
        match pc {
            0x82466CC0 => {
    //   block [0x82466CC0..0x82466CC8)
	// 82466CC0: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82466CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466CC8 size=56
    let mut pc: u32 = 0x82466CC8;
    'dispatch: loop {
        match pc {
            0x82466CC8 => {
    //   block [0x82466CC8..0x82466CD8)
	// 82466CC8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82466CCC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466CD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82466CD4: 419A0014  beq cr6, 0x82466ce8
	if ctx.cr[6].eq {
	pc = 0x82466CE8; continue 'dispatch;
	}
	pc = 0x82466CD8; continue 'dispatch;
            }
            0x82466CD8 => {
    //   block [0x82466CD8..0x82466CE8)
	// 82466CD8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82466CDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466CE0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82466CE4: 409AFFF4  bne cr6, 0x82466cd8
	if !ctx.cr[6].eq {
	pc = 0x82466CD8; continue 'dispatch;
	}
	pc = 0x82466CE8; continue 'dispatch;
            }
            0x82466CE8 => {
    //   block [0x82466CE8..0x82466D00)
	// 82466CE8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82466CEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82466CF0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82466CF4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82466CF8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82466CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466D08 size=80
    let mut pc: u32 = 0x82466D08;
    'dispatch: loop {
        match pc {
            0x82466D08 => {
    //   block [0x82466D08..0x82466D1C)
	// 82466D08: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82466D0C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466D10: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466D14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466D18: 419A0018  beq cr6, 0x82466d30
	if ctx.cr[6].eq {
	pc = 0x82466D30; continue 'dispatch;
	}
	pc = 0x82466D1C; continue 'dispatch;
            }
            0x82466D1C => {
    //   block [0x82466D1C..0x82466D30)
	// 82466D1C: 810B001C  lwz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466D20: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466D24: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82466D28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82466D2C: 409AFFF0  bne cr6, 0x82466d1c
	if !ctx.cr[6].eq {
	pc = 0x82466D1C; continue 'dispatch;
	}
	pc = 0x82466D30; continue 'dispatch;
            }
            0x82466D30 => {
    //   block [0x82466D30..0x82466D34)
	// 82466D30: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	pc = 0x82466D34; continue 'dispatch;
            }
            0x82466D34 => {
    //   block [0x82466D34..0x82466D58)
	// 82466D34: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466D38: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82466D3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82466D40: 40980018  bge cr6, 0x82466d58
	if !ctx.cr[6].lt {
		sub_82466D58(ctx, base);
		return;
	}
	// 82466D44: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82466D48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82466D4C: 409AFFE8  bne cr6, 0x82466d34
	if !ctx.cr[6].eq {
	pc = 0x82466D34; continue 'dispatch;
	}
	// 82466D50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82466D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466D58 size=68
    let mut pc: u32 = 0x82466D58;
    'dispatch: loop {
        match pc {
            0x82466D58 => {
    //   block [0x82466D58..0x82466D9C)
	// 82466D58: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82466D5C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82466D60: 419AFFF0  beq cr6, 0x82466d50
	if ctx.cr[6].eq {
		sub_82466D08(ctx, base);
		return;
	}
	// 82466D64: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82466D68: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82466D6C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82466D70: 4198FFE0  blt cr6, 0x82466d50
	if ctx.cr[6].lt {
		sub_82466D08(ctx, base);
		return;
	}
	// 82466D74: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82466D78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82466D7C: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82466D80: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82466D84: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466D88: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82466D8C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466D90: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82466D94: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82466D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466DA0 size=88
    let mut pc: u32 = 0x82466DA0;
    'dispatch: loop {
        match pc {
            0x82466DA0 => {
    //   block [0x82466DA0..0x82466DEC)
	// 82466DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466DA4: 480CE319  bl 0x825350bc
	ctx.lr = 0x82466DA8;
	sub_82535080(ctx, base);
	// 82466DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466DAC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82466DB0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82466DB4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82466DB8: 4BFFFF51  bl 0x82466d08
	ctx.lr = 0x82466DBC;
	sub_82466D08(ctx, base);
	// 82466DBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82466DC0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82466DC4: 409A0028  bne cr6, 0x82466dec
	if !ctx.cr[6].eq {
	pc = 0x82466DEC; continue 'dispatch;
	}
	// 82466DC8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466DCC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82466DD0: 48004451  bl 0x8246b220
	ctx.lr = 0x82466DD4;
	sub_8246B220(ctx, base);
	// 82466DD4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82466DD8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82466DDC: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82466DE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466DE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82466DE8: 4E800421  bctrl
	ctx.lr = 0x82466DEC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82466DEC => {
    //   block [0x82466DEC..0x82466DF8)
	// 82466DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466DF4: 480CE318  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466E20 size=76
    let mut pc: u32 = 0x82466E20;
    'dispatch: loop {
        match pc {
            0x82466E20 => {
    //   block [0x82466E20..0x82466E6C)
	// 82466E20: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82466E24: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82466E28: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82466E2C: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82466E30: 80C10064  lwz r6, 0x64(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82466E34: 8101006C  lwz r8, 0x6c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82466E38: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82466E3C: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82466E40: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82466E44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82466E48: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82466E4C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82466E50: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82466E54: 90E3001C  stw r7, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 82466E58: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82466E5C: 91030024  stw r8, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 82466E60: 91230028  stw r9, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82466E64: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82466E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82466E70 size=24
    let mut pc: u32 = 0x82466E70;
    'dispatch: loop {
        match pc {
            0x82466E70 => {
    //   block [0x82466E70..0x82466E88)
	// 82466E70: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466E74: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82466E78: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82466E7C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82466E80: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82466E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466E88 size=184
    let mut pc: u32 = 0x82466E88;
    'dispatch: loop {
        match pc {
            0x82466E88 => {
    //   block [0x82466E88..0x82466EE0)
	// 82466E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466E8C: 480CE22D  bl 0x825350b8
	ctx.lr = 0x82466E90;
	sub_82535080(ctx, base);
	// 82466E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466E94: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82466E98: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82466E9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82466EA0: 4BFFFE69  bl 0x82466d08
	ctx.lr = 0x82466EA4;
	sub_82466D08(ctx, base);
	// 82466EA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82466EA8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82466EAC: 409A0034  bne cr6, 0x82466ee0
	if !ctx.cr[6].eq {
	pc = 0x82466EE0; continue 'dispatch;
	}
	// 82466EB0: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82466EB4: 88BE000C  lbz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82466EB8: 2B050014  cmplwi cr6, r5, 0x14
	ctx.cr[6].compare_u32(ctx.r[5].u32, 20 as u32, &mut ctx.xer);
	// 82466EBC: 409A0030  bne cr6, 0x82466eec
	if !ctx.cr[6].eq {
	pc = 0x82466EEC; continue 'dispatch;
	}
	// 82466EC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466EC4: 4800431D  bl 0x8246b1e0
	ctx.lr = 0x82466EC8;
	sub_8246B1E0(ctx, base);
	// 82466EC8: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82466ECC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82466ED0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82466ED4: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82466ED8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82466EDC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82466EE0; continue 'dispatch;
            }
            0x82466EE0 => {
    //   block [0x82466EE0..0x82466EEC)
	// 82466EE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82466EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466EE8: 480CE220  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82466EEC => {
    //   block [0x82466EEC..0x82466F2C)
	// 82466EEC: 2B050018  cmplwi cr6, r5, 0x18
	ctx.cr[6].compare_u32(ctx.r[5].u32, 24 as u32, &mut ctx.xer);
	// 82466EF0: 409A003C  bne cr6, 0x82466f2c
	if !ctx.cr[6].eq {
	pc = 0x82466F2C; continue 'dispatch;
	}
	// 82466EF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466EF8: 480042F1  bl 0x8246b1e8
	ctx.lr = 0x82466EFC;
	sub_8246B1E8(ctx, base);
	// 82466EFC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82466F00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82466F04: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82466F08: 480045E9  bl 0x8246b4f0
	ctx.lr = 0x82466F0C;
	sub_8246B4F0(ctx, base);
	// 82466F0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82466F10: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82466F14: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82466F18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82466F1C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82466F20: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82466F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466F28: 480CE1E0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82466F2C => {
    //   block [0x82466F2C..0x82466F40)
	// 82466F2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82466F30: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82466F34: 480002E5  bl 0x82467218
	ctx.lr = 0x82466F38;
	sub_82467218(ctx, base);
	// 82466F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82466F3C: 480CE1CC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82466F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82466F40 size=588
    let mut pc: u32 = 0x82466F40;
    'dispatch: loop {
        match pc {
            0x82466F40 => {
    //   block [0x82466F40..0x82466F8C)
	// 82466F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82466F44: 480CE15D  bl 0x825350a0
	ctx.lr = 0x82466F48;
	sub_82535080(ctx, base);
	// 82466F48: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82466F4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82466F50: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82466F54: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82466F58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82466F5C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82466F60: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82466F64: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466F68: 48005429  bl 0x8246c390
	ctx.lr = 0x82466F6C;
	sub_8246C390(ctx, base);
	// 82466F6C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82466F70: 809A000C  lwz r4, 0xc(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82466F74: 480056A5  bl 0x8246c618
	ctx.lr = 0x82466F78;
	sub_8246C618(ctx, base);
	// 82466F78: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466F7C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82466F80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82466F84: 4099002C  ble cr6, 0x82466fb0
	if !ctx.cr[6].gt {
	pc = 0x82466FB0; continue 'dispatch;
	}
	// 82466F88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82466F8C; continue 'dispatch;
            }
            0x82466F8C => {
    //   block [0x82466F8C..0x82466FB0)
	// 82466F8C: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82466F90: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82466F94: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82466F98: 48005AC9  bl 0x8246ca60
	ctx.lr = 0x82466F9C;
	sub_8246CA60(ctx, base);
	// 82466F9C: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466FA0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82466FA4: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 82466FA8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82466FAC: 4198FFE0  blt cr6, 0x82466f8c
	if ctx.cr[6].lt {
	pc = 0x82466F8C; continue 'dispatch;
	}
	pc = 0x82466FB0; continue 'dispatch;
            }
            0x82466FB0 => {
    //   block [0x82466FB0..0x82466FD0)
	// 82466FB0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82466FB4: 809A0014  lwz r4, 0x14(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82466FB8: 48005661  bl 0x8246c618
	ctx.lr = 0x82466FBC;
	sub_8246C618(ctx, base);
	// 82466FBC: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82466FC0: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82466FC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82466FC8: 409901A8  ble cr6, 0x82467170
	if !ctx.cr[6].gt {
	pc = 0x82467170; continue 'dispatch;
	}
	// 82466FCC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	pc = 0x82466FD0; continue 'dispatch;
            }
            0x82466FD0 => {
    //   block [0x82466FD0..0x82466FE8)
	// 82466FD0: 817A0018  lwz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82466FD4: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82466FD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82466FDC: 7F6BC214  add r27, r11, r24
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82466FE0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82466FE4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82466FE8; continue 'dispatch;
            }
            0x82466FE8 => {
    //   block [0x82466FE8..0x82467014)
	// 82466FE8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82466FEC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82466FF0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82466FF4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82466FF8: 4200FFF0  bdnz 0x82466fe8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82466FE8; continue 'dispatch;
	}
	// 82466FFC: 8B21007C  lbz r25, 0x7c(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82467000: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82467004: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82467008: 419A000C  beq cr6, 0x82467014
	if ctx.cr[6].eq {
	pc = 0x82467014; continue 'dispatch;
	}
	// 8246700C: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 82467010: 409A0034  bne cr6, 0x82467044
	if !ctx.cr[6].eq {
	pc = 0x82467044; continue 'dispatch;
	}
	pc = 0x82467014; continue 'dispatch;
            }
            0x82467014 => {
    //   block [0x82467014..0x82467044)
	// 82467014: 8861007D  lbz r3, 0x7d(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(125 as u32) ) } as u64;
	// 82467018: 480041A1  bl 0x8246b1b8
	ctx.lr = 0x8246701C;
	sub_8246B1B8(ctx, base);
	// 8246701C: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82467020: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82467024: A1610080  lhz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82467028: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 8246702C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82467030: 9BC1007D  stb r30, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[30].u8 ) };
	// 82467034: 7D4B5A78  xor r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82467038: 557C043E  clrlwi r28, r11, 0x10
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8246703C: B3810080  sth r28, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u16 ) };
	// 82467040: 4800000C  b 0x8246704c
	pc = 0x8246704C; continue 'dispatch;
            }
            0x82467044 => {
    //   block [0x82467044..0x8246704C)
	// 82467044: A3810080  lhz r28, 0x80(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82467048: 8BC1007D  lbz r30, 0x7d(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(125 as u32) ) } as u64;
	pc = 0x8246704C; continue 'dispatch;
            }
            0x8246704C => {
    //   block [0x8246704C..0x8246707C)
	// 8246704C: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82467050: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82467054: 556A056A  rlwinm r10, r11, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82467058: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246705C: 419A0030  beq cr6, 0x8246708c
	if ctx.cr[6].eq {
	pc = 0x8246708C; continue 'dispatch;
	}
	// 82467060: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82467064: 57CA063E  clrlwi r10, r30, 0x18
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82467068: 697C0400  xori r28, r11, 0x400
	ctx.r[28].u64 = ctx.r[11].u64 ^ 1024;
	// 8246706C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82467070: B3810080  sth r28, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u16 ) };
	// 82467074: 419A0008  beq cr6, 0x8246707c
	if ctx.cr[6].eq {
	pc = 0x8246707C; continue 'dispatch;
	}
	// 82467078: 57DD063E  clrlwi r29, r30, 0x18
	ctx.r[29].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	pc = 0x8246707C; continue 'dispatch;
            }
            0x8246707C => {
    //   block [0x8246707C..0x8246708C)
	// 8246707C: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82467080: 3B200013  li r25, 0x13
	ctx.r[25].s64 = 19;
	// 82467084: 9BC1007D  stb r30, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[30].u8 ) };
	// 82467088: 9B21007C  stb r25, 0x7c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[25].u8 ) };
	pc = 0x8246708C; continue 'dispatch;
            }
            0x8246708C => {
    //   block [0x8246708C..0x824670C4)
	// 8246708C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82467090: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467094: 419A0048  beq cr6, 0x824670dc
	if ctx.cr[6].eq {
	pc = 0x824670DC; continue 'dispatch;
	}
	// 82467098: 897B000C  lbz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246709C: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 824670A0: 419A003C  beq cr6, 0x824670dc
	if ctx.cr[6].eq {
	pc = 0x824670DC; continue 'dispatch;
	}
	// 824670A4: 897B000D  lbz r11, 0xd(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(13 as u32) ) } as u64;
	// 824670A8: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 824670AC: 419A0030  beq cr6, 0x824670dc
	if ctx.cr[6].eq {
	pc = 0x824670DC; continue 'dispatch;
	}
	// 824670B0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824670B4: 4800412D  bl 0x8246b1e0
	ctx.lr = 0x824670B8;
	sub_8246B1E0(ctx, base);
	// 824670B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824670BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824670C0: 419A001C  beq cr6, 0x824670dc
	if ctx.cr[6].eq {
	pc = 0x824670DC; continue 'dispatch;
	}
	pc = 0x824670C4; continue 'dispatch;
            }
            0x824670C4 => {
    //   block [0x824670C4..0x824670DC)
	// 824670C4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 824670C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824670CC: 4BFFFE75  bl 0x82466f40
	ctx.lr = 0x824670D0;
	sub_82466F40(ctx, base);
	// 824670D0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824670D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824670D8: 409AFFEC  bne cr6, 0x824670c4
	if !ctx.cr[6].eq {
	pc = 0x824670C4; continue 'dispatch;
	}
	pc = 0x824670DC; continue 'dispatch;
            }
            0x824670DC => {
    //   block [0x824670DC..0x824670F8)
	// 824670DC: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 824670E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824670E4: 419A0014  beq cr6, 0x824670f8
	if ctx.cr[6].eq {
	pc = 0x824670F8; continue 'dispatch;
	}
	// 824670E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824670EC: 480040FD  bl 0x8246b1e8
	ctx.lr = 0x824670F0;
	sub_8246B1E8(ctx, base);
	// 824670F0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 824670F4: 4800596D  bl 0x8246ca60
	ctx.lr = 0x824670F8;
	sub_8246CA60(ctx, base);
	pc = 0x824670F8; continue 'dispatch;
            }
            0x824670F8 => {
    //   block [0x824670F8..0x82467144)
	// 824670F8: 83E10070  lwz r31, 0x70(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 824670FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467100: 48003121  bl 0x8246a220
	ctx.lr = 0x82467104;
	sub_8246A220(ctx, base);
	// 82467104: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82467108: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246710C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467110: 48005179  bl 0x8246c288
	ctx.lr = 0x82467114;
	sub_8246C288(ctx, base);
	// 82467114: 5724063E  clrlwi r4, r25, 0x18
	ctx.r[4].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 82467118: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246711C: 4800549D  bl 0x8246c5b8
	ctx.lr = 0x82467120;
	sub_8246C5B8(ctx, base);
	// 82467120: 57C4063E  clrlwi r4, r30, 0x18
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82467124: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467128: 48005491  bl 0x8246c5b8
	ctx.lr = 0x8246712C;
	sub_8246C5B8(ctx, base);
	// 8246712C: 7FAB0734  extsh r11, r29
	ctx.r[11].s64 = ctx.r[29].s16 as i64;
	// 82467130: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82467134: 419A0010  beq cr6, 0x82467144
	if ctx.cr[6].eq {
	pc = 0x82467144; continue 'dispatch;
	}
	// 82467138: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246713C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467140: 48005479  bl 0x8246c5b8
	ctx.lr = 0x82467144;
	sub_8246C5B8(ctx, base);
	pc = 0x82467144; continue 'dispatch;
            }
            0x82467144 => {
    //   block [0x82467144..0x82467170)
	// 82467144: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467148: A081007E  lhz r4, 0x7e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(126 as u32) ) } as u64;
	// 8246714C: 4800546D  bl 0x8246c5b8
	ctx.lr = 0x82467150;
	sub_8246C5B8(ctx, base);
	// 82467150: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82467154: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467158: 48005461  bl 0x8246c5b8
	ctx.lr = 0x8246715C;
	sub_8246C5B8(ctx, base);
	// 8246715C: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82467160: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82467164: 3B180018  addi r24, r24, 0x18
	ctx.r[24].s64 = ctx.r[24].s64 + 24;
	// 82467168: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246716C: 4198FE64  blt cr6, 0x82466fd0
	if ctx.cr[6].lt {
	pc = 0x82466FD0; continue 'dispatch;
	}
	pc = 0x82467170; continue 'dispatch;
            }
            0x82467170 => {
    //   block [0x82467170..0x8246718C)
	// 82467170: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467174: 809A001C  lwz r4, 0x1c(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82467178: 480054A1  bl 0x8246c618
	ctx.lr = 0x8246717C;
	sub_8246C618(ctx, base);
	// 8246717C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467180: 480053B9  bl 0x8246c538
	ctx.lr = 0x82467184;
	sub_8246C538(ctx, base);
	// 82467184: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82467188: 480CDF68  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467190 size=132
    let mut pc: u32 = 0x82467190;
    'dispatch: loop {
        match pc {
            0x82467190 => {
    //   block [0x82467190..0x824671CC)
	// 82467190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246719C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824671A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824671A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824671A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824671AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824671B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824671B4: 48005BB5  bl 0x8246cd68
	ctx.lr = 0x824671B8;
	sub_8246CD68(ctx, base);
	// 824671B8: 7FCBF0F8  nor r11, r30, r30
	ctx.r[11].u64 = !(ctx.r[30].u64 | ctx.r[30].u64);
	// 824671BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824671C0: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 824671C4: 419A0030  beq cr6, 0x824671f4
	if ctx.cr[6].eq {
	pc = 0x824671F4; continue 'dispatch;
	}
	// 824671C8: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	pc = 0x824671CC; continue 'dispatch;
            }
            0x824671CC => {
    //   block [0x824671CC..0x824671E8)
	// 824671CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824671D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824671D4: 4BFFFD6D  bl 0x82466f40
	ctx.lr = 0x824671D8;
	sub_82466F40(ctx, base);
	// 824671D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824671DC: 419A000C  beq cr6, 0x824671e8
	if ctx.cr[6].eq {
	pc = 0x824671E8; continue 'dispatch;
	}
	// 824671E0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824671E4: 48000008  b 0x824671ec
	pc = 0x824671EC; continue 'dispatch;
            }
            0x824671E8 => {
    //   block [0x824671E8..0x824671EC)
	// 824671E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x824671EC; continue 'dispatch;
            }
            0x824671EC => {
    //   block [0x824671EC..0x824671F4)
	// 824671EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824671F0: 409AFFDC  bne cr6, 0x824671cc
	if !ctx.cr[6].eq {
	pc = 0x824671CC; continue 'dispatch;
	}
	pc = 0x824671F4; continue 'dispatch;
            }
            0x824671F4 => {
    //   block [0x824671F4..0x82467214)
	// 824671F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824671F8: 48005B61  bl 0x8246cd58
	ctx.lr = 0x824671FC;
	sub_8246CD58(ctx, base);
	// 824671FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82467200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82467204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82467208: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246720C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82467210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467218 size=96
    let mut pc: u32 = 0x82467218;
    'dispatch: loop {
        match pc {
            0x82467218 => {
    //   block [0x82467218..0x8246726C)
	// 82467218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246721C: 480CDEA1  bl 0x825350bc
	ctx.lr = 0x82467220;
	sub_82535080(ctx, base);
	// 82467220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467224: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82467228: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246722C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82467230: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82467234: 48003F85  bl 0x8246b1b8
	ctx.lr = 0x82467238;
	sub_8246B1B8(ctx, base);
	// 82467238: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246723C: A14B0008  lhz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82467240: 2B0A0040  cmplwi cr6, r10, 0x40
	ctx.cr[6].compare_u32(ctx.r[10].u32, 64 as u32, &mut ctx.xer);
	// 82467244: 41990028  bgt cr6, 0x8246726c
	if ctx.cr[6].gt {
	pc = 0x8246726C; continue 'dispatch;
	}
	// 82467248: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8246724C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82467250: A16B0008  lhz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82467254: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82467258: 7D650734  extsh r5, r11
	ctx.r[5].s64 = ctx.r[11].s16 as i64;
	// 8246725C: 480030CD  bl 0x8246a328
	ctx.lr = 0x82467260;
	sub_8246A328(ctx, base);
	// 82467260: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82467264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82467268: 480CDEA4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8246726C => {
    //   block [0x8246726C..0x82467278)
	// 8246726C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82467270: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82467274: 480CDE98  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82467278 size=4
    let mut pc: u32 = 0x82467278;
    'dispatch: loop {
        match pc {
            0x82467278 => {
    //   block [0x82467278..0x8246727C)
	// 82467278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82467280 size=4
    let mut pc: u32 = 0x82467280;
    'dispatch: loop {
        match pc {
            0x82467280 => {
    //   block [0x82467280..0x82467284)
	// 82467280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467290 size=40
    let mut pc: u32 = 0x82467290;
    'dispatch: loop {
        match pc {
            0x82467290 => {
    //   block [0x82467290..0x824672B8)
	// 82467290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246729C: 4BFFC7ED  bl 0x82463a88
	ctx.lr = 0x824672A0;
	sub_82463A88(ctx, base);
	// 824672A0: 48002A61  bl 0x82469d00
	ctx.lr = 0x824672A4;
	sub_82469D00(ctx, base);
	// 824672A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824672A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824672AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824672B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824672B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824672B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824672B8 size=88
    let mut pc: u32 = 0x824672B8;
    'dispatch: loop {
        match pc {
            0x824672B8 => {
    //   block [0x824672B8..0x82467310)
	// 824672B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824672BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824672C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824672C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824672C8: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824672CC: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 824672D0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824672D4: 48002875  bl 0x82469b48
	ctx.lr = 0x824672D8;
	sub_82469B48(ctx, base);
	// 824672D8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824672DC: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824672E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824672E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824672E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824672EC: 4E800421  bctrl
	ctx.lr = 0x824672F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824672F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824672F4: 4BFFC795  bl 0x82463a88
	ctx.lr = 0x824672F8;
	sub_82463A88(ctx, base);
	// 824672F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824672FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82467300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82467304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82467308: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467320 size=80
    let mut pc: u32 = 0x82467320;
    'dispatch: loop {
        match pc {
            0x82467320 => {
    //   block [0x82467320..0x82467370)
	// 82467320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246732C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467330: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82467334: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82467338: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8246733C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82467340: 4BFFCCF9  bl 0x82464038
	ctx.lr = 0x82467344;
	sub_82464038(ctx, base);
	// 82467344: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467348: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 8246734C: 396B7BE8  addi r11, r11, 0x7be8
	ctx.r[11].s64 = ctx.r[11].s64 + 31720;
	// 82467350: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82467354: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82467358: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246735C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82467360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82467364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82467368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246736C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467370 size=372
    let mut pc: u32 = 0x82467370;
    'dispatch: loop {
        match pc {
            0x82467370 => {
    //   block [0x82467370..0x824673A4)
	// 82467370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467374: 480CDD45  bl 0x825350b8
	ctx.lr = 0x82467378;
	sub_82535080(ctx, base);
	// 82467378: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246737C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82467380: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82467384: 3BCB91D0  addi r30, r11, -0x6e30
	ctx.r[30].s64 = ctx.r[11].s64 + -28208;
	// 82467388: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8246738C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82467390: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467394: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82467398: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8246739C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824673A0: 419A00C0  beq cr6, 0x82467460
	if ctx.cr[6].eq {
	pc = 0x82467460; continue 'dispatch;
	}
	pc = 0x824673A4; continue 'dispatch;
            }
            0x824673A4 => {
    //   block [0x824673A4..0x824673D0)
	// 824673A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824673A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824673AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824673B0: 409A0020  bne cr6, 0x824673d0
	if !ctx.cr[6].eq {
	pc = 0x824673D0; continue 'dispatch;
	}
	// 824673B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824673B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824673BC: 4E800421  bctrl
	ctx.lr = 0x824673C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824673C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824673C4: 419A00D0  beq cr6, 0x82467494
	if ctx.cr[6].eq {
	pc = 0x82467494; continue 'dispatch;
	}
	// 824673C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824673CC: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
            }
            0x824673D0 => {
    //   block [0x824673D0..0x824673D8)
	// 824673D0: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 824673D4: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x824673D8; continue 'dispatch;
            }
            0x824673D8 => {
    //   block [0x824673D8..0x824673EC)
	// 824673D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824673DC: 409AFFC8  bne cr6, 0x824673a4
	if !ctx.cr[6].eq {
	pc = 0x824673A4; continue 'dispatch;
	}
	// 824673E0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824673E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824673E8: 419A0078  beq cr6, 0x82467460
	if ctx.cr[6].eq {
	pc = 0x82467460; continue 'dispatch;
	}
	pc = 0x824673EC; continue 'dispatch;
            }
            0x824673EC => {
    //   block [0x824673EC..0x824673FC)
	// 824673EC: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 824673F0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824673F4: 41980060  blt cr6, 0x82467454
	if ctx.cr[6].lt {
	pc = 0x82467454; continue 'dispatch;
	}
	// 824673F8: 579D103A  slwi r29, r28, 2
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	pc = 0x824673FC; continue 'dispatch;
            }
            0x824673FC => {
    //   block [0x824673FC..0x82467444)
	// 824673FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82467400: 7FFD582E  lwzx r31, r29, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82467404: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246740C: 4E800421  bctrl
	ctx.lr = 0x82467410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82467414: 419A0030  beq cr6, 0x82467444
	if ctx.cr[6].eq {
	pc = 0x82467444; continue 'dispatch;
	}
	// 82467418: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246741C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82467420: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82467424: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82467428: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8246742C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82467430: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82467434: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82467438: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246743C: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82467440: 7D5D592E  stwx r10, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
            }
            0x82467444 => {
    //   block [0x82467444..0x82467454)
	// 82467444: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82467448: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 8246744C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82467450: 4098FFAC  bge cr6, 0x824673fc
	if !ctx.cr[6].lt {
	pc = 0x824673FC; continue 'dispatch;
	}
	pc = 0x82467454; continue 'dispatch;
            }
            0x82467454 => {
    //   block [0x82467454..0x82467460)
	// 82467454: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82467458: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246745C: 409AFF90  bne cr6, 0x824673ec
	if !ctx.cr[6].eq {
	pc = 0x824673EC; continue 'dispatch;
	}
	pc = 0x82467460; continue 'dispatch;
            }
            0x82467460 => {
    //   block [0x82467460..0x8246748C)
	// 82467460: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82467464: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82467468: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246746C: 409A0020  bne cr6, 0x8246748c
	if !ctx.cr[6].eq {
	pc = 0x8246748C; continue 'dispatch;
	}
	// 82467470: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467474: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82467478: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8246747C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82467480: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82467484: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82467488: 4BFFCC31  bl 0x824640b8
	ctx.lr = 0x8246748C;
	sub_824640B8(ctx, base);
	pc = 0x8246748C; continue 'dispatch;
            }
            0x8246748C => {
    //   block [0x8246748C..0x82467494)
	// 8246748C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82467490: 480CDC78  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82467494 => {
    //   block [0x82467494..0x824674B4)
	// 82467494: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82467498: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8246749C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824674A0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824674A4: 409A0010  bne cr6, 0x824674b4
	if !ctx.cr[6].eq {
	pc = 0x824674B4; continue 'dispatch;
	}
	// 824674A8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824674AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824674B0: 48006EA1  bl 0x8246e350
	ctx.lr = 0x824674B4;
	sub_8246E350(ctx, base);
	pc = 0x824674B4; continue 'dispatch;
            }
            0x824674B4 => {
    //   block [0x824674B4..0x824674E4)
	// 824674B4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824674B8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824674BC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824674C0: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 824674C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824674C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824674CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824674D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824674D4: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824674D8: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824674DC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824674E0: 4BFFFEF8  b 0x824673d8
	pc = 0x824673D8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824674E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824674E8 size=300
    let mut pc: u32 = 0x824674E8;
    'dispatch: loop {
        match pc {
            0x824674E8 => {
    //   block [0x824674E8..0x82467524)
	// 824674E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824674EC: 480CDBD1  bl 0x825350bc
	ctx.lr = 0x824674F0;
	sub_82535080(ctx, base);
	// 824674F0: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824674F4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824674F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824674FC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82467500: 614A0080  ori r10, r10, 0x80
	ctx.r[10].u64 = ctx.r[10].u64 | 128;
	// 82467504: 83EB91D0  lwz r31, -0x6e30(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 82467508: 3961005C  addi r11, r1, 0x5c
	ctx.r[11].s64 = ctx.r[1].s64 + 92;
	// 8246750C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82467510: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82467514: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82467518: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8246751C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82467520: 419A0048  beq cr6, 0x82467568
	if ctx.cr[6].eq {
	pc = 0x82467568; continue 'dispatch;
	}
	pc = 0x82467524; continue 'dispatch;
            }
            0x82467524 => {
    //   block [0x82467524..0x82467544)
	// 82467524: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82467528: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8246752C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82467530: 409A0014  bne cr6, 0x82467544
	if !ctx.cr[6].eq {
	pc = 0x82467544; continue 'dispatch;
	}
	// 82467534: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82467538: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246753C: 48006E15  bl 0x8246e350
	ctx.lr = 0x82467540;
	sub_8246E350(ctx, base);
	// 82467540: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	pc = 0x82467544; continue 'dispatch;
            }
            0x82467544 => {
    //   block [0x82467544..0x82467568)
	// 82467544: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82467548: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246754C: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82467550: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82467554: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82467558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8246755C: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82467560: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82467564: 409AFFC0  bne cr6, 0x82467524
	if !ctx.cr[6].eq {
	pc = 0x82467524; continue 'dispatch;
	}
	pc = 0x82467568; continue 'dispatch;
            }
            0x82467568 => {
    //   block [0x82467568..0x82467578)
	// 82467568: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 8246756C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82467570: 41980070  blt cr6, 0x824675e0
	if ctx.cr[6].lt {
	pc = 0x824675E0; continue 'dispatch;
	}
	// 82467574: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	pc = 0x82467578; continue 'dispatch;
            }
            0x82467578 => {
    //   block [0x82467578..0x824675C0)
	// 82467578: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8246757C: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82467580: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82467584: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467588: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246758C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467590: 419A0030  beq cr6, 0x824675c0
	if ctx.cr[6].eq {
	pc = 0x824675C0; continue 'dispatch;
	}
	// 82467594: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82467598: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246759C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824675A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824675A4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824675A8: 409A0018  bne cr6, 0x824675c0
	if !ctx.cr[6].eq {
	pc = 0x824675C0; continue 'dispatch;
	}
	// 824675AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824675B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824675B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824675B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824675BC: 4E800421  bctrl
	ctx.lr = 0x824675C0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824675C0 => {
    //   block [0x824675C0..0x824675E0)
	// 824675C0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824675C4: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824675C8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824675CC: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824675D0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 824675D4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824675D8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824675DC: 4098FF9C  bge cr6, 0x82467578
	if !ctx.cr[6].lt {
	pc = 0x82467578; continue 'dispatch;
	}
	pc = 0x824675E0; continue 'dispatch;
            }
            0x824675E0 => {
    //   block [0x824675E0..0x8246760C)
	// 824675E0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824675E4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824675E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824675EC: 409A0020  bne cr6, 0x8246760c
	if !ctx.cr[6].eq {
	pc = 0x8246760C; continue 'dispatch;
	}
	// 824675F0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824675F4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824675F8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824675FC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82467600: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82467604: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82467608: 4BFFCAB1  bl 0x824640b8
	ctx.lr = 0x8246760C;
	sub_824640B8(ctx, base);
	pc = 0x8246760C; continue 'dispatch;
            }
            0x8246760C => {
    //   block [0x8246760C..0x82467614)
	// 8246760C: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82467610: 480CDAFC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467618 size=312
    let mut pc: u32 = 0x82467618;
    'dispatch: loop {
        match pc {
            0x82467618 => {
    //   block [0x82467618..0x824676E4)
	// 82467618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246761C: 480CDA99  bl 0x825350b4
	ctx.lr = 0x82467620;
	sub_82535080(ctx, base);
	// 82467620: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467624: 3F608293  lis r27, -0x7d6d
	ctx.r[27].s64 = -2104295424;
	// 82467628: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246762C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82467630: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82467634: 897B906C  lbz r11, -0x6f94(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(-28564 as u32) ) } as u64;
	// 82467638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246763C: 409A00A8  bne cr6, 0x824676e4
	if !ctx.cr[6].eq {
	pc = 0x824676E4; continue 'dispatch;
	}
	// 82467640: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82467644: 419A00DC  beq cr6, 0x82467720
	if ctx.cr[6].eq {
	pc = 0x82467720; continue 'dispatch;
	}
	// 82467648: 4BFFF051  bl 0x82466698
	ctx.lr = 0x8246764C;
	sub_82466698(ctx, base);
	// 8246764C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82467650: 419A00A0  beq cr6, 0x824676f0
	if ctx.cr[6].eq {
	pc = 0x824676F0; continue 'dispatch;
	}
	// 82467654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467658: 4BFFC431  bl 0x82463a88
	ctx.lr = 0x8246765C;
	sub_82463A88(ctx, base);
	// 8246765C: 480026A5  bl 0x82469d00
	ctx.lr = 0x82467660;
	sub_82469D00(ctx, base);
	// 82467660: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467664: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82467668: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8246766C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82467670: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82467674: 4BFFC9C5  bl 0x82464038
	ctx.lr = 0x82467678;
	sub_82464038(ctx, base);
	// 82467678: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 8246767C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467680: 396B7BCC  addi r11, r11, 0x7bcc
	ctx.r[11].s64 = ctx.r[11].s64 + 31692;
	// 82467684: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82467688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8246768C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82467690: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82467694: 4800026D  bl 0x82467900
	ctx.lr = 0x82467698;
	sub_82467900(ctx, base);
	// 82467698: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 8246769C: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824676A0: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 824676A4: 4BFFC995  bl 0x82464038
	ctx.lr = 0x824676A8;
	sub_82464038(ctx, base);
	// 824676A8: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 824676AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824676B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824676B4: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 824676B8: 48000541  bl 0x82467bf8
	ctx.lr = 0x824676BC;
	sub_82467BF8(ctx, base);
	// 824676BC: 480001C5  bl 0x82467880
	ctx.lr = 0x824676C0;
	sub_82467880(ctx, base);
	// 824676C0: 4BFFFCB1  bl 0x82467370
	ctx.lr = 0x824676C4;
	sub_82467370(ctx, base);
	// 824676C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824676C8: 806B9068  lwz r3, -0x6f98(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28568 as u32) ) } as u64;
	// 824676CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824676D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824676D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824676D8: 4E800421  bctrl
	ctx.lr = 0x824676DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824676DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824676E0: 997B906C  stb r11, -0x6f94(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(-28564 as u32), ctx.r[11].u8 ) };
            }
            0x824676E4 => {
    //   block [0x824676E4..0x824676F0)
	// 824676E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824676E8: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 824676EC: 480CDA18  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x824676F0 => {
    //   block [0x824676F0..0x82467720)
	// 824676F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824676F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824676F8: 388B7D50  addi r4, r11, 0x7d50
	ctx.r[4].s64 = ctx.r[11].s64 + 32080;
	// 824676FC: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 82467700: 480CD451  bl 0x82534b50
	ctx.lr = 0x82467704;
	sub_82534B50(ctx, base);
	// 82467704: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82467708: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246770C: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82467710: 4E800421  bctrl
	ctx.lr = 0x82467714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467714: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82467718: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8246771C: 480CD9E8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82467720 => {
    //   block [0x82467720..0x82467750)
	// 82467720: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467724: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82467728: 388B7C70  addi r4, r11, 0x7c70
	ctx.r[4].s64 = ctx.r[11].s64 + 31856;
	// 8246772C: 38A000DC  li r5, 0xdc
	ctx.r[5].s64 = 220;
	// 82467730: 480CD421  bl 0x82534b50
	ctx.lr = 0x82467734;
	sub_82534B50(ctx, base);
	// 82467734: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82467738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246773C: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82467740: 4E800421  bctrl
	ctx.lr = 0x82467744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467744: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82467748: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8246774C: 480CD9B8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467750 size=300
    let mut pc: u32 = 0x82467750;
    'dispatch: loop {
        match pc {
            0x82467750 => {
    //   block [0x82467750..0x824677AC)
	// 82467750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467754: 480CD965  bl 0x825350b8
	ctx.lr = 0x82467758;
	sub_82535080(ctx, base);
	// 82467758: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246775C: 3F808293  lis r28, -0x7d6d
	ctx.r[28].s64 = -2104295424;
	// 82467760: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82467764: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82467768: 897C906C  lbz r11, -0x6f94(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(-28564 as u32) ) } as u64;
	// 8246776C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82467770: 409A0100  bne cr6, 0x82467870
	if !ctx.cr[6].eq {
	pc = 0x82467870; continue 'dispatch;
	}
	// 82467774: 4BFFFD75  bl 0x824674e8
	ctx.lr = 0x82467778;
	sub_824674E8(ctx, base);
	// 82467778: 3FE08293  lis r31, -0x7d6d
	ctx.r[31].s64 = -2104295424;
	// 8246777C: 897F91D8  lbz r11, -0x6e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-28200 as u32) ) } as u64;
	// 82467780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467784: 419A0028  beq cr6, 0x824677ac
	if ctx.cr[6].eq {
	pc = 0x824677AC; continue 'dispatch;
	}
	// 82467788: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8246778C: 814B4EA8  lwz r10, 0x4ea8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20136 as u32) ) } as u64;
	// 82467790: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82467794: 419A0018  beq cr6, 0x824677ac
	if ctx.cr[6].eq {
	pc = 0x824677AC; continue 'dispatch;
	}
	// 82467798: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246779C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824677A0: 4E800421  bctrl
	ctx.lr = 0x824677A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824677A4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824677A8: 997F91D8  stb r11, -0x6e28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-28200 as u32), ctx.r[11].u8 ) };
            }
            0x824677AC => {
    //   block [0x824677AC..0x82467858)
	// 824677AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824677B0: 480000D1  bl 0x82467880
	ctx.lr = 0x824677B4;
	sub_82467880(ctx, base);
	// 824677B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824677B8: 48000149  bl 0x82467900
	ctx.lr = 0x824677BC;
	sub_82467900(ctx, base);
	// 824677BC: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824677C0: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 824677C4: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824677C8: 48002381  bl 0x82469b48
	ctx.lr = 0x824677CC;
	sub_82469B48(ctx, base);
	// 824677CC: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 824677D0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824677D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824677D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824677DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824677E0: 4E800421  bctrl
	ctx.lr = 0x824677E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824677E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824677E8: 4BFFC2A1  bl 0x82463a88
	ctx.lr = 0x824677EC;
	sub_82463A88(ctx, base);
	// 824677EC: 3FE08293  lis r31, -0x7d6d
	ctx.r[31].s64 = -2104295424;
	// 824677F0: 807F9004  lwz r3, -0x6ffc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824677F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824677F8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 824677FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467800: 4E800421  bctrl
	ctx.lr = 0x82467804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467804: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82467808: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246780C: 419A0054  beq cr6, 0x82467860
	if ctx.cr[6].eq {
	pc = 0x82467860; continue 'dispatch;
	}
	// 82467810: 807F9004  lwz r3, -0x6ffc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 82467814: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82467818: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8246781C: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82467820: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82467824: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82467828: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8246782C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82467830: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467834: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82467838: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246783C: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 82467840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467844: 4E800421  bctrl
	ctx.lr = 0x82467848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467848: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8246784C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82467850: 41990008  bgt cr6, 0x82467858
	if ctx.cr[6].gt {
	pc = 0x82467858; continue 'dispatch;
	}
	// 82467854: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
            }
            0x82467858 => {
    //   block [0x82467858..0x82467860)
	// 82467858: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8246785C: 4800555D  bl 0x8246cdb8
	ctx.lr = 0x82467860;
	sub_8246CDB8(ctx, base);
	pc = 0x82467860; continue 'dispatch;
            }
            0x82467860 => {
    //   block [0x82467860..0x82467870)
	// 82467860: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82467864: 4BFFEE35  bl 0x82466698
	ctx.lr = 0x82467868;
	sub_82466698(ctx, base);
	// 82467868: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8246786C: 997C906C  stb r11, -0x6f94(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(-28564 as u32), ctx.r[11].u8 ) };
	pc = 0x82467870; continue 'dispatch;
            }
            0x82467870 => {
    //   block [0x82467870..0x8246787C)
	// 82467870: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82467874: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82467878: 480CD890  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467880 size=124
    let mut pc: u32 = 0x82467880;
    'dispatch: loop {
        match pc {
            0x82467880 => {
    //   block [0x82467880..0x824678E0)
	// 82467880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467888: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246788C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82467890: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467894: 3FE08293  lis r31, -0x7d6d
	ctx.r[31].s64 = -2104295424;
	// 82467898: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246789C: 807F9190  lwz r3, -0x6e70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 824678A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824678A4: 419A003C  beq cr6, 0x824678e0
	if ctx.cr[6].eq {
	pc = 0x824678E0; continue 'dispatch;
	}
	// 824678A8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824678AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824678B0: 419A0030  beq cr6, 0x824678e0
	if ctx.cr[6].eq {
	pc = 0x824678E0; continue 'dispatch;
	}
	// 824678B4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824678B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824678BC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824678C0: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 824678C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824678C8: 409A0018  bne cr6, 0x824678e0
	if !ctx.cr[6].eq {
	pc = 0x824678E0; continue 'dispatch;
	}
	// 824678CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824678D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824678D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824678D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824678DC: 4E800421  bctrl
	ctx.lr = 0x824678E0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824678E0 => {
    //   block [0x824678E0..0x824678FC)
	// 824678E0: 93DF9190  stw r30, -0x6e70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28272 as u32), ctx.r[30].u32 ) };
	// 824678E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824678E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824678EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824678F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824678F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824678F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467900 size=124
    let mut pc: u32 = 0x82467900;
    'dispatch: loop {
        match pc {
            0x82467900 => {
    //   block [0x82467900..0x82467960)
	// 82467900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246790C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82467910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467914: 3FE08293  lis r31, -0x7d6d
	ctx.r[31].s64 = -2104295424;
	// 82467918: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246791C: 807F91D4  lwz r3, -0x6e2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-28204 as u32) ) } as u64;
	// 82467920: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82467924: 419A003C  beq cr6, 0x82467960
	if ctx.cr[6].eq {
	pc = 0x82467960; continue 'dispatch;
	}
	// 82467928: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246792C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467930: 419A0030  beq cr6, 0x82467960
	if ctx.cr[6].eq {
	pc = 0x82467960; continue 'dispatch;
	}
	// 82467934: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82467938: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246793C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82467940: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82467944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467948: 409A0018  bne cr6, 0x82467960
	if !ctx.cr[6].eq {
	pc = 0x82467960; continue 'dispatch;
	}
	// 8246794C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467950: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82467954: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246795C: 4E800421  bctrl
	ctx.lr = 0x82467960;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82467960 => {
    //   block [0x82467960..0x8246797C)
	// 82467960: 93DF91D4  stw r30, -0x6e2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-28204 as u32), ctx.r[30].u32 ) };
	// 82467964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82467968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246796C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82467970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82467974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82467978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82467980 size=24
    let mut pc: u32 = 0x82467980;
    'dispatch: loop {
        match pc {
            0x82467980 => {
    //   block [0x82467980..0x82467998)
	// 82467980: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82467984: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82467988: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8246798C: 816B4D9C  lwz r11, 0x4d9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19868 as u32) ) } as u64;
	// 82467990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467994: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467998 size=344
    let mut pc: u32 = 0x82467998;
    'dispatch: loop {
        match pc {
            0x82467998 => {
    //   block [0x82467998..0x82467A90)
	// 82467998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246799C: 480CD71D  bl 0x825350b8
	ctx.lr = 0x824679A0;
	sub_82535080(ctx, base);
	// 824679A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824679A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824679A8: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824679AC: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 824679B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824679B4: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 824679B8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 824679BC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824679C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824679C4: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 824679C8: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824679CC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824679D0: 4BFFC669  bl 0x82464038
	ctx.lr = 0x824679D4;
	sub_82464038(ctx, base);
	// 824679D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824679D8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 824679DC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 824679E0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 824679E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824679E8: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 824679EC: 816B4DA0  lwz r11, 0x4da0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19872 as u32) ) } as u64;
	// 824679F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824679F4: 4E800421  bctrl
	ctx.lr = 0x824679F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824679F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824679FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467A00: 480071F9  bl 0x8246ebf8
	ctx.lr = 0x82467A04;
	sub_8246EBF8(ctx, base);
	// 82467A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82467A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82467A0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82467A10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467A14: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82467A18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467A1C: 4E800421  bctrl
	ctx.lr = 0x82467A20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467A20: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467A24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467A28: 409A0098  bne cr6, 0x82467ac0
	if !ctx.cr[6].eq {
	pc = 0x82467AC0; continue 'dispatch;
	}
	// 82467A2C: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82467A30: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82467A34: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82467A38: 4BFFC601  bl 0x82464038
	ctx.lr = 0x82467A3C;
	sub_82464038(ctx, base);
	// 82467A3C: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82467A40: 38A01000  li r5, 0x1000
	ctx.r[5].s64 = 4096;
	// 82467A44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82467A48: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82467A4C: 48006F85  bl 0x8246e9d0
	ctx.lr = 0x82467A50;
	sub_8246E9D0(ctx, base);
	// 82467A50: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82467A54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82467A58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467A5C: 419A0034  beq cr6, 0x82467a90
	if ctx.cr[6].eq {
	pc = 0x82467A90; continue 'dispatch;
	}
	// 82467A60: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82467A64: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82467A68: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82467A6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82467A70: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82467A74: 409A001C  bne cr6, 0x82467a90
	if !ctx.cr[6].eq {
	pc = 0x82467A90; continue 'dispatch;
	}
	// 82467A78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467A7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82467A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467A84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467A88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467A8C: 4E800421  bctrl
	ctx.lr = 0x82467A90;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82467A90 => {
    //   block [0x82467A90..0x82467AB4)
	// 82467A90: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82467A94: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82467A98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82467A9C: 409A0018  bne cr6, 0x82467ab4
	if !ctx.cr[6].eq {
	pc = 0x82467AB4; continue 'dispatch;
	}
	// 82467AA0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82467AA4: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82467AA8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82467AAC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82467AB0: 4BFFC609  bl 0x824640b8
	ctx.lr = 0x82467AB4;
	sub_824640B8(ctx, base);
	pc = 0x82467AB4; continue 'dispatch;
            }
            0x82467AB4 => {
    //   block [0x82467AB4..0x82467AC0)
	// 82467AB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82467AB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82467ABC: 480CD64C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82467AC0 => {
    //   block [0x82467AC0..0x82467AE4)
	// 82467AC0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82467AC4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82467AC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82467ACC: 409A0018  bne cr6, 0x82467ae4
	if !ctx.cr[6].eq {
	pc = 0x82467AE4; continue 'dispatch;
	}
	// 82467AD0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82467AD4: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82467AD8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82467ADC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82467AE0: 4BFFC5D9  bl 0x824640b8
	ctx.lr = 0x82467AE4;
	sub_824640B8(ctx, base);
	pc = 0x82467AE4; continue 'dispatch;
            }
            0x82467AE4 => {
    //   block [0x82467AE4..0x82467AF0)
	// 82467AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467AE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82467AEC: 480CD61C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467AF0 size=264
    let mut pc: u32 = 0x82467AF0;
    'dispatch: loop {
        match pc {
            0x82467AF0 => {
    //   block [0x82467AF0..0x82467BC8)
	// 82467AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467AF4: 480CD5C5  bl 0x825350b8
	ctx.lr = 0x82467AF8;
	sub_82535080(ctx, base);
	// 82467AF8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467AFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82467B00: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467B04: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82467B08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82467B0C: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82467B10: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82467B14: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82467B18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82467B1C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82467B20: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82467B24: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82467B28: 4BFFC511  bl 0x82464038
	ctx.lr = 0x82467B2C;
	sub_82464038(ctx, base);
	// 82467B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82467B30: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82467B34: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82467B38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82467B3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82467B40: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82467B44: 816B4DA0  lwz r11, 0x4da0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19872 as u32) ) } as u64;
	// 82467B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467B4C: 4E800421  bctrl
	ctx.lr = 0x82467B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467B50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82467B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467B58: 48007A71  bl 0x8246f5c8
	ctx.lr = 0x82467B5C;
	sub_8246F5C8(ctx, base);
	// 82467B5C: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82467B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82467B64: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82467B68: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82467B6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82467B70: 4BFFC4C9  bl 0x82464038
	ctx.lr = 0x82467B74;
	sub_82464038(ctx, base);
	// 82467B74: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82467B78: 38A01000  li r5, 0x1000
	ctx.r[5].s64 = 4096;
	// 82467B7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82467B80: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82467B84: 48007655  bl 0x8246f1d8
	ctx.lr = 0x82467B88;
	sub_8246F1D8(ctx, base);
	// 82467B88: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82467B8C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82467B90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467B94: 419A0034  beq cr6, 0x82467bc8
	if ctx.cr[6].eq {
	pc = 0x82467BC8; continue 'dispatch;
	}
	// 82467B98: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82467B9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82467BA0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82467BA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82467BA8: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82467BAC: 409A001C  bne cr6, 0x82467bc8
	if !ctx.cr[6].eq {
	pc = 0x82467BC8; continue 'dispatch;
	}
	// 82467BB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467BB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82467BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467BBC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467BC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467BC4: 4E800421  bctrl
	ctx.lr = 0x82467BC8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82467BC8 => {
    //   block [0x82467BC8..0x82467BEC)
	// 82467BC8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82467BCC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82467BD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82467BD4: 409A0018  bne cr6, 0x82467bec
	if !ctx.cr[6].eq {
	pc = 0x82467BEC; continue 'dispatch;
	}
	// 82467BD8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82467BDC: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82467BE0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82467BE4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82467BE8: 4BFFC4D1  bl 0x824640b8
	ctx.lr = 0x82467BEC;
	sub_824640B8(ctx, base);
	pc = 0x82467BEC; continue 'dispatch;
            }
            0x82467BEC => {
    //   block [0x82467BEC..0x82467BF8)
	// 82467BEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82467BF0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82467BF4: 480CD514  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467BF8 size=92
    let mut pc: u32 = 0x82467BF8;
    'dispatch: loop {
        match pc {
            0x82467BF8 => {
    //   block [0x82467BF8..0x82467C54)
	// 82467BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467BFC: 480CD4C1  bl 0x825350bc
	ctx.lr = 0x82467C00;
	sub_82535080(ctx, base);
	// 82467C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467C04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82467C0C: 396B7BFC  addi r11, r11, 0x7bfc
	ctx.r[11].s64 = ctx.r[11].s64 + 31740;
	// 82467C10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82467C14: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82467C18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82467C1C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82467C20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82467C24: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82467C28: 48005719  bl 0x8246d340
	ctx.lr = 0x82467C2C;
	sub_8246D340(ctx, base);
	// 82467C2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82467C30: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82467C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467C38: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82467C3C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82467C40: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82467C44: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82467C48: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82467C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82467C50: 480CD4BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467C58 size=364
    let mut pc: u32 = 0x82467C58;
    'dispatch: loop {
        match pc {
            0x82467C58 => {
    //   block [0x82467C58..0x82467DA4)
	// 82467C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467C5C: 480CD445  bl 0x825350a0
	ctx.lr = 0x82467C60;
	sub_82535080(ctx, base);
	// 82467C60: 9421FCB0  stwu r1, -0x350(r1)
	ea = ctx.r[1].u32.wrapping_add(-848 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467C64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82467C6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82467C70: 388B7C4C  addi r4, r11, 0x7c4c
	ctx.r[4].s64 = ctx.r[11].s64 + 31820;
	// 82467C74: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82467C78: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82467C7C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82467C80: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82467C84: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82467C88: 48002301  bl 0x82469f88
	ctx.lr = 0x82467C8C;
	sub_82469F88(ctx, base);
	// 82467C8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82467C90: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82467C94: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82467C98: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82467C9C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82467CA0: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82467CA4: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467CA8: 480073D1  bl 0x8246f078
	ctx.lr = 0x82467CAC;
	sub_8246F078(ctx, base);
	// 82467CAC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82467CB0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467CB4: 48000A95  bl 0x82468748
	ctx.lr = 0x82467CB8;
	sub_82468748(ctx, base);
	// 82467CB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467CBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82467CC0: 3B2B7C48  addi r25, r11, 0x7c48
	ctx.r[25].s64 = ctx.r[11].s64 + 31816;
	// 82467CC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467CC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467CCC: 3B0B7C40  addi r24, r11, 0x7c40
	ctx.r[24].s64 = ctx.r[11].s64 + 31808;
	// 82467CD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467CD4: 3B810070  addi r28, r1, 0x70
	ctx.r[28].s64 = ctx.r[1].s64 + 112;
	// 82467CD8: 3AEB7C3C  addi r23, r11, 0x7c3c
	ctx.r[23].s64 = ctx.r[11].s64 + 31804;
	// 82467CDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467CE0: 3ACB7C34  addi r22, r11, 0x7c34
	ctx.r[22].s64 = ctx.r[11].s64 + 31796;
	// 82467CE4: 480005BD  bl 0x824682a0
	ctx.lr = 0x82467CE8;
	sub_824682A0(ctx, base);
	// 82467CE8: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82467CEC: 48000565  bl 0x82468250
	ctx.lr = 0x82467CF0;
	sub_82468250(ctx, base);
	// 82467CF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82467CF4: 480006E5  bl 0x824683d8
	ctx.lr = 0x82467CF8;
	sub_824683D8(ctx, base);
	// 82467CF8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82467CFC: 480005A5  bl 0x824682a0
	ctx.lr = 0x82467D00;
	sub_824682A0(ctx, base);
	// 82467D00: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82467D04: 4800059D  bl 0x824682a0
	ctx.lr = 0x82467D08;
	sub_824682A0(ctx, base);
	// 82467D08: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82467D0C: 48000595  bl 0x824682a0
	ctx.lr = 0x82467D10;
	sub_824682A0(ctx, base);
	// 82467D10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82467D14: 4800058D  bl 0x824682a0
	ctx.lr = 0x82467D18;
	sub_824682A0(ctx, base);
	// 82467D18: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82467D1C: 48000585  bl 0x824682a0
	ctx.lr = 0x82467D20;
	sub_824682A0(ctx, base);
	// 82467D20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82467D24: 4800057D  bl 0x824682a0
	ctx.lr = 0x82467D28;
	sub_824682A0(ctx, base);
	// 82467D28: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82467D2C: 48000575  bl 0x824682a0
	ctx.lr = 0x82467D30;
	sub_824682A0(ctx, base);
	// 82467D30: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82467D34: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82467D38: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82467D3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467D40: 4E800421  bctrl
	ctx.lr = 0x82467D44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467D44: 7F4B0774  extsb r11, r26
	ctx.r[11].s64 = ctx.r[26].s8 as i64;
	// 82467D48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82467D4C: 419A0060  beq cr6, 0x82467dac
	if ctx.cr[6].eq {
	pc = 0x82467DAC; continue 'dispatch;
	}
	// 82467D50: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82467D54: 48007A0D  bl 0x8246f760
	ctx.lr = 0x82467D58;
	sub_8246F760(ctx, base);
	// 82467D58: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82467D5C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82467D60: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82467D64: 480079F5  bl 0x8246f758
	ctx.lr = 0x82467D68;
	sub_8246F758(ctx, base);
	// 82467D68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82467D6C: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82467D70: 40990034  ble cr6, 0x82467da4
	if !ctx.cr[6].gt {
	pc = 0x82467DA4; continue 'dispatch;
	}
	// 82467D74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467D78: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82467D7C: 386B7C20  addi r3, r11, 0x7c20
	ctx.r[3].s64 = ctx.r[11].s64 + 31776;
	// 82467D80: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82467D84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467D88: 4E800421  bctrl
	ctx.lr = 0x82467D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467D8C: 38BEFFFE  addi r5, r30, -2
	ctx.r[5].s64 = ctx.r[30].s64 + -2;
	// 82467D90: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82467D94: 388100A8  addi r4, r1, 0xa8
	ctx.r[4].s64 = ctx.r[1].s64 + 168;
	// 82467D98: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82467D9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82467DA0: 480079A1  bl 0x8246f740
	ctx.lr = 0x82467DA4;
	sub_8246F740(ctx, base);
            }
            0x82467DA4 => {
    //   block [0x82467DA4..0x82467DAC)
	// 82467DA4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82467DA8: 480079D1  bl 0x8246f778
	ctx.lr = 0x82467DAC;
	sub_8246F778(ctx, base);
	pc = 0x82467DAC; continue 'dispatch;
            }
            0x82467DAC => {
    //   block [0x82467DAC..0x82467DC4)
	// 82467DAC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82467DB0: 48000B31  bl 0x824688e0
	ctx.lr = 0x82467DB4;
	sub_824688E0(ctx, base);
	// 82467DB4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82467DB8: 480074D9  bl 0x8246f290
	ctx.lr = 0x82467DBC;
	sub_8246F290(ctx, base);
	// 82467DBC: 38210350  addi r1, r1, 0x350
	ctx.r[1].s64 = ctx.r[1].s64 + 848;
	// 82467DC0: 480CD330  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467DE8 size=80
    let mut pc: u32 = 0x82467DE8;
    'dispatch: loop {
        match pc {
            0x82467DE8 => {
    //   block [0x82467DE8..0x82467E38)
	// 82467DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82467DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467DF8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82467DFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82467E00: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82467E04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82467E08: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82467E0C: 48005725  bl 0x8246d530
	ctx.lr = 0x82467E10;
	sub_8246D530(ctx, base);
	// 82467E10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82467E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467E18: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82467E1C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82467E20: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82467E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82467E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82467E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82467E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82467E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467E40 size=292
    let mut pc: u32 = 0x82467E40;
    'dispatch: loop {
        match pc {
            0x82467E40 => {
    //   block [0x82467E40..0x82467E8C)
	// 82467E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467E44: 480CD26D  bl 0x825350b0
	ctx.lr = 0x82467E48;
	sub_82535080(ctx, base);
	// 82467E48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467E4C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82467E50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82467E54: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82467E58: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82467E5C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82467E60: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82467E64: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82467E68: 409A0024  bne cr6, 0x82467e8c
	if !ctx.cr[6].eq {
	pc = 0x82467E8C; continue 'dispatch;
	}
	// 82467E6C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82467E70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82467E74: 419A0018  beq cr6, 0x82467e8c
	if ctx.cr[6].eq {
	pc = 0x82467E8C; continue 'dispatch;
	}
	// 82467E78: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82467E7C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82467E80: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82467E84: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82467E88: 83CBFFFC  lwz r30, -4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	pc = 0x82467E8C; continue 'dispatch;
            }
            0x82467E8C => {
    //   block [0x82467E8C..0x82467EC0)
	// 82467E8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467E90: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82467E94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82467E98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82467E9C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82467EA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82467EA4: 4E800421  bctrl
	ctx.lr = 0x82467EA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82467EA8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467EAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82467EB0: 409A0010  bne cr6, 0x82467ec0
	if !ctx.cr[6].eq {
	pc = 0x82467EC0; continue 'dispatch;
	}
	// 82467EB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82467EB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82467EBC: 480CD244  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            0x82467EC0 => {
    //   block [0x82467EC0..0x82467EFC)
	// 82467EC0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82467EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82467EC8: 388B1160  addi r4, r11, 0x1160
	ctx.r[4].s64 = ctx.r[11].s64 + 4448;
	// 82467ECC: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 82467ED0: 4199005C  bgt cr6, 0x82467f2c
	if ctx.cr[6].gt {
	pc = 0x82467F2C; continue 'dispatch;
	}
	// 82467ED4: 3D808246  lis r12, -0x7dba
	ctx.r[12].s64 = -2109341696;
	// 82467ED8: 398C7EEC  addi r12, r12, 0x7eec
	ctx.r[12].s64 = ctx.r[12].s64 + 32492;
	// 82467EDC: 57A0103A  slwi r0, r29, 2
	ctx.r[0].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82467EE0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82467EE4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82467EE8: 4E800420  bctr
	match ctx.r[29].u64 {
		0 => {
	pc = 0x82467EFC; continue 'dispatch;
		},
		1 => {
	pc = 0x82467F08; continue 'dispatch;
		},
		2 => {
	pc = 0x82467F14; continue 'dispatch;
		},
		3 => {
	pc = 0x82467F20; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82467EEC: 82467EFC  lwz r18, 0x7efc(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32508 as u32) ) } as u64;
	// 82467EF0: 82467F08  lwz r18, 0x7f08(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32520 as u32) ) } as u64;
	// 82467EF4: 82467F14  lwz r18, 0x7f14(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32532 as u32) ) } as u64;
	// 82467EF8: 82467F20  lwz r18, 0x7f20(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32544 as u32) ) } as u64;
            }
            0x82467EFC => {
    //   block [0x82467EFC..0x82467F08)
	// 82467EFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467F00: 388B7C64  addi r4, r11, 0x7c64
	ctx.r[4].s64 = ctx.r[11].s64 + 31844;
	// 82467F04: 48000028  b 0x82467f2c
	pc = 0x82467F2C; continue 'dispatch;
            }
            0x82467F08 => {
    //   block [0x82467F08..0x82467F14)
	// 82467F08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467F0C: 388B7C5C  addi r4, r11, 0x7c5c
	ctx.r[4].s64 = ctx.r[11].s64 + 31836;
	// 82467F10: 4800001C  b 0x82467f2c
	pc = 0x82467F2C; continue 'dispatch;
            }
            0x82467F14 => {
    //   block [0x82467F14..0x82467F20)
	// 82467F14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467F18: 388B7C54  addi r4, r11, 0x7c54
	ctx.r[4].s64 = ctx.r[11].s64 + 31828;
	// 82467F1C: 4800000C  b 0x82467f28
	pc = 0x82467F28; continue 'dispatch;
            }
            0x82467F20 => {
    //   block [0x82467F20..0x82467F28)
	// 82467F20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82467F24: 388B17AC  addi r4, r11, 0x17ac
	ctx.r[4].s64 = ctx.r[11].s64 + 6060;
	pc = 0x82467F28; continue 'dispatch;
            }
            0x82467F28 => {
    //   block [0x82467F28..0x82467F2C)
	// 82467F28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	pc = 0x82467F2C; continue 'dispatch;
            }
            0x82467F2C => {
    //   block [0x82467F2C..0x82467F58)
	// 82467F2C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82467F30: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82467F34: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82467F38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82467F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467F40: 4BFFFD19  bl 0x82467c58
	ctx.lr = 0x82467F44;
	sub_82467C58(ctx, base);
	// 82467F44: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82467F48: 419A0010  beq cr6, 0x82467f58
	if ctx.cr[6].eq {
	pc = 0x82467F58; continue 'dispatch;
	}
	// 82467F4C: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 82467F50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82467F54: 409A0008  bne cr6, 0x82467f5c
	if !ctx.cr[6].eq {
	pc = 0x82467F5C; continue 'dispatch;
	}
	pc = 0x82467F58; continue 'dispatch;
            }
            0x82467F58 => {
    //   block [0x82467F58..0x82467F5C)
	// 82467F58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82467F5C; continue 'dispatch;
            }
            0x82467F5C => {
    //   block [0x82467F5C..0x82467F64)
	// 82467F5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82467F60: 480CD1A0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467F68 size=112
    let mut pc: u32 = 0x82467F68;
    'dispatch: loop {
        match pc {
            0x82467F68 => {
    //   block [0x82467F68..0x82467FA4)
	// 82467F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467F70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82467F74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82467F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467F7C: 3BE30014  addi r31, r3, 0x14
	ctx.r[31].s64 = ctx.r[3].s64 + 20;
	// 82467F80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82467F84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82467F88: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82467F8C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82467F90: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82467F94: 409A0010  bne cr6, 0x82467fa4
	if !ctx.cr[6].eq {
	pc = 0x82467FA4; continue 'dispatch;
	}
	// 82467F98: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82467F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82467FA0: 480063B1  bl 0x8246e350
	ctx.lr = 0x82467FA4;
	sub_8246E350(ctx, base);
	pc = 0x82467FA4; continue 'dispatch;
            }
            0x82467FA4 => {
    //   block [0x82467FA4..0x82467FD8)
	// 82467FA4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82467FA8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82467FAC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82467FB0: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82467FB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82467FB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82467FBC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82467FC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82467FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82467FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82467FCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82467FD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82467FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82467FD8 size=16
    let mut pc: u32 = 0x82467FD8;
    'dispatch: loop {
        match pc {
            0x82467FD8 => {
    //   block [0x82467FD8..0x82467FE8)
	// 82467FD8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82467FDC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82467FE0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82467FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82467FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82467FE8 size=160
    let mut pc: u32 = 0x82467FE8;
    'dispatch: loop {
        match pc {
            0x82467FE8 => {
    //   block [0x82467FE8..0x82468030)
	// 82467FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82467FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82467FF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82467FF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82467FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82467FFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468000: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82468004: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82468008: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8246800C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82468010: 409A0020  bne cr6, 0x82468030
	if !ctx.cr[6].eq {
	pc = 0x82468030; continue 'dispatch;
	}
	// 82468014: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468018: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246801C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82468020: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82468024: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82468028: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246802C: 4BFFC08D  bl 0x824640b8
	ctx.lr = 0x82468030;
	sub_824640B8(ctx, base);
	pc = 0x82468030; continue 'dispatch;
            }
            0x82468030 => {
    //   block [0x82468030..0x8246806C)
	// 82468030: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82468034: 48005395  bl 0x8246d3c8
	ctx.lr = 0x82468038;
	sub_8246D3C8(ctx, base);
	// 82468038: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8246803C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82468040: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82468044: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82468048: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246804C: 419A0020  beq cr6, 0x8246806c
	if ctx.cr[6].eq {
	pc = 0x8246806C; continue 'dispatch;
	}
	// 82468050: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468054: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82468058: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8246805C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468060: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82468064: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82468068: 4BFFC051  bl 0x824640b8
	ctx.lr = 0x8246806C;
	sub_824640B8(ctx, base);
	pc = 0x8246806C; continue 'dispatch;
            }
            0x8246806C => {
    //   block [0x8246806C..0x82468088)
	// 8246806C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82468074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246807C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82468080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468088 size=4
    let mut pc: u32 = 0x82468088;
    'dispatch: loop {
        match pc {
            0x82468088 => {
    //   block [0x82468088..0x8246808C)
	// 82468088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468090 size=4
    let mut pc: u32 = 0x82468090;
    'dispatch: loop {
        match pc {
            0x82468090 => {
    //   block [0x82468090..0x82468094)
	// 82468090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468098 size=4
    let mut pc: u32 = 0x82468098;
    'dispatch: loop {
        match pc {
            0x82468098 => {
    //   block [0x82468098..0x8246809C)
	// 82468098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824680A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824680A0 size=4
    let mut pc: u32 = 0x824680A0;
    'dispatch: loop {
        match pc {
            0x824680A0 => {
    //   block [0x824680A0..0x824680A4)
	// 824680A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824680A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824680A8 size=4
    let mut pc: u32 = 0x824680A8;
    'dispatch: loop {
        match pc {
            0x824680A8 => {
    //   block [0x824680A8..0x824680AC)
	// 824680A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824680B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824680B0 size=112
    let mut pc: u32 = 0x824680B0;
    'dispatch: loop {
        match pc {
            0x824680B0 => {
    //   block [0x824680B0..0x824680F8)
	// 824680B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824680B4: 480CD009  bl 0x825350bc
	ctx.lr = 0x824680B8;
	sub_82535080(ctx, base);
	// 824680B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824680BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824680C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824680C4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824680C8: 419A0030  beq cr6, 0x824680f8
	if ctx.cr[6].eq {
	pc = 0x824680F8; continue 'dispatch;
	}
	// 824680CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824680D0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824680D4: 4800214D  bl 0x8246a220
	ctx.lr = 0x824680D8;
	sub_8246A220(ctx, base);
	// 824680D8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824680DC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824680E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824680E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824680E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824680EC: 4E800421  bctrl
	ctx.lr = 0x824680F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824680F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824680F4: 480CD018  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824680F8 => {
    //   block [0x824680F8..0x82468120)
	// 824680F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824680FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468100: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82468104: 388B8300  addi r4, r11, -0x7d00
	ctx.r[4].s64 = ctx.r[11].s64 + -32000;
	// 82468108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246810C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468114: 4E800421  bctrl
	ctx.lr = 0x82468118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246811C: 480CCFF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468120 size=64
    let mut pc: u32 = 0x82468120;
    'dispatch: loop {
        match pc {
            0x82468120 => {
    //   block [0x82468120..0x82468160)
	// 82468120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246812C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468130: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468138: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246813C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468140: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468144: 4E800421  bctrl
	ctx.lr = 0x82468148;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246814C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82468150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82468158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246815C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468160 size=100
    let mut pc: u32 = 0x82468160;
    'dispatch: loop {
        match pc {
            0x82468160 => {
    //   block [0x82468160..0x824681C4)
	// 82468160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468164: 480CCF59  bl 0x825350bc
	ctx.lr = 0x82468168;
	sub_82535080(ctx, base);
	// 82468168: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8246816C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82468170: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468174: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246817C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82468180: 38AB8308  addi r5, r11, -0x7cf8
	ctx.r[5].s64 = ctx.r[11].s64 + -31992;
	// 82468184: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82468188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246818C: 48001DB5  bl 0x82469f40
	ctx.lr = 0x82468190;
	sub_82469F40(ctx, base);
	// 82468190: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468194: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468198: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246819C: 48002085  bl 0x8246a220
	ctx.lr = 0x824681A0;
	sub_8246A220(ctx, base);
	// 824681A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824681A4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824681A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824681AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824681B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824681B4: 4E800421  bctrl
	ctx.lr = 0x824681B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824681B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824681BC: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 824681C0: 480CCF4C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824681C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824681C8 size=132
    let mut pc: u32 = 0x824681C8;
    'dispatch: loop {
        match pc {
            0x824681C8 => {
    //   block [0x824681C8..0x824681F0)
	// 824681C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824681CC: 480CCEED  bl 0x825350b8
	ctx.lr = 0x824681D0;
	sub_82535080(ctx, base);
	// 824681D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824681D4: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 824681D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824681DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824681E0: 419A0010  beq cr6, 0x824681f0
	if ctx.cr[6].eq {
	pc = 0x824681F0; continue 'dispatch;
	}
	// 824681E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824681E8: 3BCB8314  addi r30, r11, -0x7cec
	ctx.r[30].s64 = ctx.r[11].s64 + -31980;
	// 824681EC: 4800000C  b 0x824681f8
	pc = 0x824681F8; continue 'dispatch;
            }
            0x824681F0 => {
    //   block [0x824681F0..0x824681F8)
	// 824681F0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824681F4: 3BCB830C  addi r30, r11, -0x7cf4
	ctx.r[30].s64 = ctx.r[11].s64 + -31988;
	pc = 0x824681F8; continue 'dispatch;
            }
            0x824681F8 => {
    //   block [0x824681F8..0x82468220)
	// 824681F8: 83FD0008  lwz r31, 8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824681FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82468200: 419A0020  beq cr6, 0x82468220
	if ctx.cr[6].eq {
	pc = 0x82468220; continue 'dispatch;
	}
	// 82468204: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468208: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246820C: 48002015  bl 0x8246a220
	ctx.lr = 0x82468210;
	sub_8246A220(ctx, base);
	// 82468210: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468214: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82468218: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246821C: 48000018  b 0x82468234
	pc = 0x82468234; continue 'dispatch;
            }
            0x82468220 => {
    //   block [0x82468220..0x82468234)
	// 82468220: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468224: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468228: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8246822C: 388B8300  addi r4, r11, -0x7d00
	ctx.r[4].s64 = ctx.r[11].s64 + -32000;
	// 82468230: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	pc = 0x82468234; continue 'dispatch;
            }
            0x82468234 => {
    //   block [0x82468234..0x8246824C)
	// 82468234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468238: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246823C: 4E800421  bctrl
	ctx.lr = 0x82468240;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468240: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82468244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468248: 480CCEC0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468250 size=76
    let mut pc: u32 = 0x82468250;
    'dispatch: loop {
        match pc {
            0x82468250 => {
    //   block [0x82468250..0x8246829C)
	// 82468250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246825C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468264: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82468268: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246826C: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 82468270: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468274: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468278: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246827C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468280: 4E800421  bctrl
	ctx.lr = 0x82468284;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246828C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82468294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824682A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824682A0 size=104
    let mut pc: u32 = 0x824682A0;
    'dispatch: loop {
        match pc {
            0x824682A0 => {
    //   block [0x824682A0..0x824682DC)
	// 824682A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824682A4: 480CCE15  bl 0x825350b8
	ctx.lr = 0x824682A8;
	sub_82535080(ctx, base);
	// 824682A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824682AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824682B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824682B4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824682B8: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824682BC: 419A0020  beq cr6, 0x824682dc
	if ctx.cr[6].eq {
	pc = 0x824682DC; continue 'dispatch;
	}
	// 824682C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824682C4: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824682C8: 48001F59  bl 0x8246a220
	ctx.lr = 0x824682CC;
	sub_8246A220(ctx, base);
	// 824682CC: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 824682D0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824682D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824682D8: 48000018  b 0x824682f0
	pc = 0x824682F0; continue 'dispatch;
            }
            0x824682DC => {
    //   block [0x824682DC..0x824682F0)
	// 824682DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824682E0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824682E4: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 824682E8: 388B8300  addi r4, r11, -0x7d00
	ctx.r[4].s64 = ctx.r[11].s64 + -32000;
	// 824682EC: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	pc = 0x824682F0; continue 'dispatch;
            }
            0x824682F0 => {
    //   block [0x824682F0..0x82468308)
	// 824682F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824682F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824682F8: 4E800421  bctrl
	ctx.lr = 0x824682FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824682FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468304: 480CCE04  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468308 size=100
    let mut pc: u32 = 0x82468308;
    'dispatch: loop {
        match pc {
            0x82468308 => {
    //   block [0x82468308..0x8246836C)
	// 82468308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246830C: 480CCDB1  bl 0x825350bc
	ctx.lr = 0x82468310;
	sub_82535080(ctx, base);
	// 82468310: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82468314: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82468318: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246831C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82468320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468324: 7C860734  extsh r6, r4
	ctx.r[6].s64 = ctx.r[4].s16 as i64;
	// 82468328: 38AB0034  addi r5, r11, 0x34
	ctx.r[5].s64 = ctx.r[11].s64 + 52;
	// 8246832C: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82468330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468334: 48001C0D  bl 0x82469f40
	ctx.lr = 0x82468338;
	sub_82469F40(ctx, base);
	// 82468338: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246833C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468340: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468344: 48001EDD  bl 0x8246a220
	ctx.lr = 0x82468348;
	sub_8246A220(ctx, base);
	// 82468348: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246834C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468350: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82468354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246835C: 4E800421  bctrl
	ctx.lr = 0x82468360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468364: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82468368: 480CCDA4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468370 size=100
    let mut pc: u32 = 0x82468370;
    'dispatch: loop {
        match pc {
            0x82468370 => {
    //   block [0x82468370..0x824683D4)
	// 82468370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468374: 480CCD49  bl 0x825350bc
	ctx.lr = 0x82468378;
	sub_82535080(ctx, base);
	// 82468378: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8246837C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82468380: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468384: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246838C: 5486043E  clrlwi r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82468390: 38AB831C  addi r5, r11, -0x7ce4
	ctx.r[5].s64 = ctx.r[11].s64 + -31972;
	// 82468394: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82468398: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246839C: 48001BA5  bl 0x82469f40
	ctx.lr = 0x824683A0;
	sub_82469F40(ctx, base);
	// 824683A0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824683A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824683A8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824683AC: 48001E75  bl 0x8246a220
	ctx.lr = 0x824683B0;
	sub_8246A220(ctx, base);
	// 824683B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824683B4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824683B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824683BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824683C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824683C4: 4E800421  bctrl
	ctx.lr = 0x824683C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824683C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824683CC: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 824683D0: 480CCD3C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824683D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824683D8 size=100
    let mut pc: u32 = 0x824683D8;
    'dispatch: loop {
        match pc {
            0x824683D8 => {
    //   block [0x824683D8..0x8246843C)
	// 824683D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824683DC: 480CCCE1  bl 0x825350bc
	ctx.lr = 0x824683E0;
	sub_82535080(ctx, base);
	// 824683E0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 824683E4: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 824683E8: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824683EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 824683F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824683F4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 824683F8: 38AB0034  addi r5, r11, 0x34
	ctx.r[5].s64 = ctx.r[11].s64 + 52;
	// 824683FC: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82468400: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468404: 48001B3D  bl 0x82469f40
	ctx.lr = 0x82468408;
	sub_82469F40(ctx, base);
	// 82468408: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246840C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468410: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468414: 48001E0D  bl 0x8246a220
	ctx.lr = 0x82468418;
	sub_8246A220(ctx, base);
	// 82468418: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246841C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468420: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82468424: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246842C: 4E800421  bctrl
	ctx.lr = 0x82468430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468434: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82468438: 480CCCD4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468440 size=100
    let mut pc: u32 = 0x82468440;
    'dispatch: loop {
        match pc {
            0x82468440 => {
    //   block [0x82468440..0x824684A4)
	// 82468440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468444: 480CCC79  bl 0x825350bc
	ctx.lr = 0x82468448;
	sub_82535080(ctx, base);
	// 82468448: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8246844C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82468450: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468454: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246845C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82468460: 38AB831C  addi r5, r11, -0x7ce4
	ctx.r[5].s64 = ctx.r[11].s64 + -31972;
	// 82468464: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82468468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246846C: 48001AD5  bl 0x82469f40
	ctx.lr = 0x82468470;
	sub_82469F40(ctx, base);
	// 82468470: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468474: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468478: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246847C: 48001DA5  bl 0x8246a220
	ctx.lr = 0x82468480;
	sub_8246A220(ctx, base);
	// 82468480: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82468484: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468488: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8246848C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468494: 4E800421  bctrl
	ctx.lr = 0x82468498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246849C: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 824684A0: 480CCC6C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824684A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824684A8 size=104
    let mut pc: u32 = 0x824684A8;
    'dispatch: loop {
        match pc {
            0x824684A8 => {
    //   block [0x824684A8..0x82468510)
	// 824684A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824684AC: 480CCC11  bl 0x825350bc
	ctx.lr = 0x824684B0;
	sub_82535080(ctx, base);
	// 824684B0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 824684B4: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 824684B8: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824684BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824684C0: D8210028  stfd f1, 0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.f[1].u64 ) };
	// 824684C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824684C8: E8C10028  ld r6, 0x28(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	// 824684CC: 38AB8320  addi r5, r11, -0x7ce0
	ctx.r[5].s64 = ctx.r[11].s64 + -31968;
	// 824684D0: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 824684D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824684D8: 48001A69  bl 0x82469f40
	ctx.lr = 0x824684DC;
	sub_82469F40(ctx, base);
	// 824684DC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824684E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824684E4: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824684E8: 48001D39  bl 0x8246a220
	ctx.lr = 0x824684EC;
	sub_8246A220(ctx, base);
	// 824684EC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824684F0: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824684F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824684F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824684FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468500: 4E800421  bctrl
	ctx.lr = 0x82468504;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468508: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 8246850C: 480CCC00  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468510 size=100
    let mut pc: u32 = 0x82468510;
    'dispatch: loop {
        match pc {
            0x82468510 => {
    //   block [0x82468510..0x82468574)
	// 82468510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468514: 480CCBA9  bl 0x825350bc
	ctx.lr = 0x82468518;
	sub_82535080(ctx, base);
	// 82468518: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8246851C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82468520: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468524: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468528: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246852C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82468530: 38AB8324  addi r5, r11, -0x7cdc
	ctx.r[5].s64 = ctx.r[11].s64 + -31964;
	// 82468534: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82468538: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8246853C: 48001A05  bl 0x82469f40
	ctx.lr = 0x82468540;
	sub_82469F40(ctx, base);
	// 82468540: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468544: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468548: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246854C: 48001CD5  bl 0x8246a220
	ctx.lr = 0x82468550;
	sub_8246A220(ctx, base);
	// 82468550: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82468554: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468558: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8246855C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468564: 4E800421  bctrl
	ctx.lr = 0x82468568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246856C: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82468570: 480CCB9C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468578 size=100
    let mut pc: u32 = 0x82468578;
    'dispatch: loop {
        match pc {
            0x82468578 => {
    //   block [0x82468578..0x824685DC)
	// 82468578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246857C: 480CCB41  bl 0x825350bc
	ctx.lr = 0x82468580;
	sub_82535080(ctx, base);
	// 82468580: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82468584: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82468588: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246858C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468594: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82468598: 38AB832C  addi r5, r11, -0x7cd4
	ctx.r[5].s64 = ctx.r[11].s64 + -31956;
	// 8246859C: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 824685A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824685A4: 4800199D  bl 0x82469f40
	ctx.lr = 0x824685A8;
	sub_82469F40(ctx, base);
	// 824685A8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824685AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824685B0: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824685B4: 48001C6D  bl 0x8246a220
	ctx.lr = 0x824685B8;
	sub_8246A220(ctx, base);
	// 824685B8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824685BC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 824685C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824685C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824685C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824685CC: 4E800421  bctrl
	ctx.lr = 0x824685D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824685D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824685D4: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 824685D8: 480CCB34  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824685E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824685E0 size=152
    let mut pc: u32 = 0x824685E0;
    'dispatch: loop {
        match pc {
            0x824685E0 => {
    //   block [0x824685E0..0x82468678)
	// 824685E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824685E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824685E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824685EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824685F0: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 824685F4: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 824685F8: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 824685FC: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82468600: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82468604: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82468608: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8246860C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82468610: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468614: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82468618: 394127C0  addi r10, r1, 0x27c0
	ctx.r[10].s64 = ctx.r[1].s64 + 10176;
	// 8246861C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468620: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82468624: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82468628: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246862C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82468630: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82468634: 48001905  bl 0x82469f38
	ctx.lr = 0x82468638;
	sub_82469F38(ctx, base);
	// 82468638: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246863C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82468640: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468644: 48001BDD  bl 0x8246a220
	ctx.lr = 0x82468648;
	sub_8246A220(ctx, base);
	// 82468648: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8246864C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468650: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82468654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468658: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246865C: 4E800421  bctrl
	ctx.lr = 0x82468660;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468660: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82468664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246866C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82468670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468690 size=20
    let mut pc: u32 = 0x82468690;
    'dispatch: loop {
        match pc {
            0x82468690 => {
    //   block [0x82468690..0x824686A4)
	// 82468690: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468694: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468698: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8246869C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824686A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824686A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824686A8 size=96
    let mut pc: u32 = 0x824686A8;
    'dispatch: loop {
        match pc {
            0x824686A8 => {
    //   block [0x824686A8..0x82468708)
	// 824686A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824686AC: 480CCA0D  bl 0x825350b8
	ctx.lr = 0x824686B0;
	sub_82535080(ctx, base);
	// 824686B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824686B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824686B8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824686BC: 3BAB8334  addi r29, r11, -0x7ccc
	ctx.r[29].s64 = ctx.r[11].s64 + -31948;
	// 824686C0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824686C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824686C8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824686CC: 48001B55  bl 0x8246a220
	ctx.lr = 0x824686D0;
	sub_8246A220(ctx, base);
	// 824686D0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824686D4: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 824686D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824686DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824686E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824686E4: 4E800421  bctrl
	ctx.lr = 0x824686E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824686E8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824686EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824686F0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824686F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824686F8: 4E800421  bctrl
	ctx.lr = 0x824686FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824686FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468704: 480CCA04  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468708 size=64
    let mut pc: u32 = 0x82468708;
    'dispatch: loop {
        match pc {
            0x82468708 => {
    //   block [0x82468708..0x82468748)
	// 82468708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246870C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246871C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468720: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468724: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82468728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246872C: 4E800421  bctrl
	ctx.lr = 0x82468730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82468738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246873C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82468740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468748 size=60
    let mut pc: u32 = 0x82468748;
    'dispatch: loop {
        match pc {
            0x82468748 => {
    //   block [0x82468748..0x82468784)
	// 82468748: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246874C: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82468750: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82468754: 396B8368  addi r11, r11, -0x7c98
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	// 82468758: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8246875C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82468760: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82468764: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82468768: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246876C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82468770: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82468774: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82468778: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246877C: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82468780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468788 size=96
    let mut pc: u32 = 0x82468788;
    'dispatch: loop {
        match pc {
            0x82468788 => {
    //   block [0x82468788..0x824687E8)
	// 82468788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246878C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468798: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8246879C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824687A0: 396B8368  addi r11, r11, -0x7c98
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	// 824687A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824687A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824687AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824687B0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824687B4: 806B91D4  lwz r3, -0x6e2c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28204 as u32) ) } as u64;
	// 824687B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824687BC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824687C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824687C4: 4E800421  bctrl
	ctx.lr = 0x824687C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824687C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824687CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824687D0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824687D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824687D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824687DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824687E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824687E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824687E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824687E8 size=116
    let mut pc: u32 = 0x824687E8;
    'dispatch: loop {
        match pc {
            0x824687E8 => {
    //   block [0x824687E8..0x8246885C)
	// 824687E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824687EC: 480CC8CD  bl 0x825350b8
	ctx.lr = 0x824687F0;
	sub_82535080(ctx, base);
	// 824687F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824687F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824687F8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824687FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468800: 396B8368  addi r11, r11, -0x7c98
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	// 82468804: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82468808: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8246880C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82468810: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82468814: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82468818: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8246881C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82468820: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82468824: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82468828: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8246882C: 4BFFB80D  bl 0x82464038
	ctx.lr = 0x82468830;
	sub_82464038(ctx, base);
	// 82468830: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82468834: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82468838: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8246883C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82468840: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82468844: 48006835  bl 0x8246f078
	ctx.lr = 0x82468848;
	sub_8246F078(ctx, base);
	// 82468848: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246884C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468850: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82468854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468858: 480CC8B0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468860 size=128
    let mut pc: u32 = 0x82468860;
    'dispatch: loop {
        match pc {
            0x82468860 => {
    //   block [0x82468860..0x824688E0)
	// 82468860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468864: 480CC855  bl 0x825350b8
	ctx.lr = 0x82468868;
	sub_82535080(ctx, base);
	// 82468868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246886C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468870: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468874: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82468878: 396B8368  addi r11, r11, -0x7c98
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	// 8246887C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82468880: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82468884: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82468888: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 8246888C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82468890: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82468894: B39E0006  sth r28, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82468898: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246889C: 4BFFB79D  bl 0x82464038
	ctx.lr = 0x824688A0;
	sub_82464038(ctx, base);
	// 824688A0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824688A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824688A8: 396B833C  addi r11, r11, -0x7cc4
	ctx.r[11].s64 = ctx.r[11].s64 + -31940;
	// 824688AC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 824688B0: B39F0006  sth r28, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 824688B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824688B8: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 824688BC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 824688C0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824688C4: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 824688C8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824688CC: 4800017D  bl 0x82468a48
	ctx.lr = 0x824688D0;
	sub_82468A48(ctx, base);
	// 824688D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824688D4: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 824688D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824688DC: 480CC82C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824688E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824688E0 size=132
    let mut pc: u32 = 0x824688E0;
    'dispatch: loop {
        match pc {
            0x824688E0 => {
    //   block [0x824688E0..0x82468944)
	// 824688E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824688E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824688E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824688EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824688F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824688F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 824688F8: 396B8368  addi r11, r11, -0x7c98
	ctx.r[11].s64 = ctx.r[11].s64 + -31896;
	// 824688FC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468900: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82468904: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82468908: 419A003C  beq cr6, 0x82468944
	if ctx.cr[6].eq {
	pc = 0x82468944; continue 'dispatch;
	}
	// 8246890C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82468914: 419A0030  beq cr6, 0x82468944
	if ctx.cr[6].eq {
	pc = 0x82468944; continue 'dispatch;
	}
	// 82468918: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8246891C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82468920: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82468924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82468928: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8246892C: 409A0018  bne cr6, 0x82468944
	if !ctx.cr[6].eq {
	pc = 0x82468944; continue 'dispatch;
	}
	// 82468930: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468934: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82468938: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246893C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468940: 4E800421  bctrl
	ctx.lr = 0x82468944;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82468944 => {
    //   block [0x82468944..0x82468964)
	// 82468944: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82468948: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8246894C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82468950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82468954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246895C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468968 size=140
    let mut pc: u32 = 0x82468968;
    'dispatch: loop {
        match pc {
            0x82468968 => {
    //   block [0x82468968..0x8246899C)
	// 82468968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246896C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468970: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82468974: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468978: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246897C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82468980: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82468984: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468988: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246898C: 419A0010  beq cr6, 0x8246899c
	if ctx.cr[6].eq {
	pc = 0x8246899C; continue 'dispatch;
	}
	// 82468990: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82468994: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82468998: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x8246899C; continue 'dispatch;
            }
            0x8246899C => {
    //   block [0x8246899C..0x824689D8)
	// 8246899C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824689A0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824689A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824689A8: 419A0030  beq cr6, 0x824689d8
	if ctx.cr[6].eq {
	pc = 0x824689D8; continue 'dispatch;
	}
	// 824689AC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824689B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824689B4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824689B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824689BC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824689C0: 409A0018  bne cr6, 0x824689d8
	if !ctx.cr[6].eq {
	pc = 0x824689D8; continue 'dispatch;
	}
	// 824689C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824689C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824689CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824689D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824689D4: 4E800421  bctrl
	ctx.lr = 0x824689D8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824689D8 => {
    //   block [0x824689D8..0x824689F4)
	// 824689D8: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 824689DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824689E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824689E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824689E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824689EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824689F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824689F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824689F8 size=76
    let mut pc: u32 = 0x824689F8;
    'dispatch: loop {
        match pc {
            0x824689F8 => {
    //   block [0x824689F8..0x82468A44)
	// 824689F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824689FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468A04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468A0C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468A10: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468A14: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82468A18: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468A1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468A20: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468A24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468A28: 4E800421  bctrl
	ctx.lr = 0x82468A2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468A2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82468A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82468A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468A48 size=116
    let mut pc: u32 = 0x82468A48;
    'dispatch: loop {
        match pc {
            0x82468A48 => {
    //   block [0x82468A48..0x82468A88)
	// 82468A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468A54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468A58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468A5C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468A60: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468A64: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468A68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82468A6C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82468A70: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82468A74: 40980020  bge cr6, 0x82468a94
	if !ctx.cr[6].lt {
	pc = 0x82468A94; continue 'dispatch;
	}
	// 82468A78: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82468A7C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82468A80: 40980008  bge cr6, 0x82468a88
	if !ctx.cr[6].lt {
	pc = 0x82468A88; continue 'dispatch;
	}
	// 82468A84: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x82468A88; continue 'dispatch;
            }
            0x82468A88 => {
    //   block [0x82468A88..0x82468A94)
	// 82468A88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82468A8C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82468A90: 48005839  bl 0x8246e2c8
	ctx.lr = 0x82468A94;
	sub_8246E2C8(ctx, base);
	pc = 0x82468A94; continue 'dispatch;
            }
            0x82468A94 => {
    //   block [0x82468A94..0x82468ABC)
	// 82468A94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82468A9C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468AA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468AA4: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82468AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82468AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82468AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468AC0 size=8
    let mut pc: u32 = 0x82468AC0;
    'dispatch: loop {
        match pc {
            0x82468AC0 => {
    //   block [0x82468AC0..0x82468AC8)
	// 82468AC0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82468AC8 size=20
    let mut pc: u32 = 0x82468AC8;
    'dispatch: loop {
        match pc {
            0x82468AC8 => {
    //   block [0x82468AC8..0x82468ADC)
	// 82468AC8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468ACC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82468AD0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82468AD4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82468AD8: 4BFFFF70  b 0x82468a48
	sub_82468A48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468AE0 size=212
    let mut pc: u32 = 0x82468AE0;
    'dispatch: loop {
        match pc {
            0x82468AE0 => {
    //   block [0x82468AE0..0x82468B3C)
	// 82468AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468AE4: 480CC5D5  bl 0x825350b8
	ctx.lr = 0x82468AE8;
	sub_82535080(ctx, base);
	// 82468AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468AEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468AF0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82468AF4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82468AF8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468AFC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468B00: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468B04: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82468B08: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82468B0C: 40990058  ble cr6, 0x82468b64
	if !ctx.cr[6].gt {
	pc = 0x82468B64; continue 'dispatch;
	}
	// 82468B10: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82468B14: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468B18: 7FABF214  add r29, r11, r30
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82468B1C: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82468B20: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82468B24: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82468B28: 40980020  bge cr6, 0x82468b48
	if !ctx.cr[6].lt {
	pc = 0x82468B48; continue 'dispatch;
	}
	// 82468B2C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82468B30: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82468B34: 40980008  bge cr6, 0x82468b3c
	if !ctx.cr[6].lt {
	pc = 0x82468B3C; continue 'dispatch;
	}
	// 82468B38: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x82468B3C; continue 'dispatch;
            }
            0x82468B3C => {
    //   block [0x82468B3C..0x82468B48)
	// 82468B3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82468B40: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82468B44: 48005785  bl 0x8246e2c8
	ctx.lr = 0x82468B48;
	sub_8246E2C8(ctx, base);
	pc = 0x82468B48; continue 'dispatch;
            }
            0x82468B48 => {
    //   block [0x82468B48..0x82468B64)
	// 82468B48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468B4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82468B50: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82468B54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468B58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468B5C: 7D4BE9AE  stbx r10, r11, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u8) };
	// 82468B60: 48000020  b 0x82468b80
	pc = 0x82468B80; continue 'dispatch;
            }
            0x82468B64 => {
    //   block [0x82468B64..0x82468B80)
	// 82468B64: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468B68: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82468B6C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82468B70: 40990010  ble cr6, 0x82468b80
	if !ctx.cr[6].gt {
	pc = 0x82468B80; continue 'dispatch;
	}
	// 82468B74: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468B78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82468B7C: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	pc = 0x82468B80; continue 'dispatch;
            }
            0x82468B80 => {
    //   block [0x82468B80..0x82468BB4)
	// 82468B80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468B84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82468B88: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468B8C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82468B90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468B94: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82468B98: 48001791  bl 0x8246a328
	ctx.lr = 0x82468B9C;
	sub_8246A328(ctx, base);
	// 82468B9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468BA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468BA4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82468BA8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82468BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468BB0: 480CC558  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468BB8 size=236
    let mut pc: u32 = 0x82468BB8;
    'dispatch: loop {
        match pc {
            0x82468BB8 => {
    //   block [0x82468BB8..0x82468BF4)
	// 82468BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468BBC: 480CC4F9  bl 0x825350b4
	ctx.lr = 0x82468BC0;
	sub_82535080(ctx, base);
	// 82468BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468BC4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82468BC8: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82468BCC: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468BD0: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82468BD4: 41980028  blt cr6, 0x82468bfc
	if ctx.cr[6].lt {
	pc = 0x82468BFC; continue 'dispatch;
	}
	// 82468BD8: 419A001C  beq cr6, 0x82468bf4
	if ctx.cr[6].eq {
	pc = 0x82468BF4; continue 'dispatch;
	}
	// 82468BDC: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 82468BE0: 40980020  bge cr6, 0x82468c00
	if !ctx.cr[6].lt {
	pc = 0x82468C00; continue 'dispatch;
	}
	// 82468BE4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468BE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468BEC: 7F845850  subf r28, r4, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82468BF0: 48000010  b 0x82468c00
	pc = 0x82468C00; continue 'dispatch;
            }
            0x82468BF4 => {
    //   block [0x82468BF4..0x82468BFC)
	// 82468BF4: 7F8B2214  add r28, r11, r4
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82468BF8: 48000008  b 0x82468c00
	pc = 0x82468C00; continue 'dispatch;
            }
            0x82468BFC => {
    //   block [0x82468BFC..0x82468C00)
	// 82468BFC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	pc = 0x82468C00; continue 'dispatch;
            }
            0x82468C00 => {
    //   block [0x82468C00..0x82468C48)
	// 82468C00: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82468C04: 41980094  blt cr6, 0x82468c98
	if ctx.cr[6].lt {
	pc = 0x82468C98; continue 'dispatch;
	}
	// 82468C08: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468C0C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468C10: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82468C14: 40990074  ble cr6, 0x82468c88
	if !ctx.cr[6].gt {
	pc = 0x82468C88; continue 'dispatch;
	}
	// 82468C18: 3BFC0001  addi r31, r28, 1
	ctx.r[31].s64 = ctx.r[28].s64 + 1;
	// 82468C1C: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82468C20: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82468C24: 40990058  ble cr6, 0x82468c7c
	if !ctx.cr[6].gt {
	pc = 0x82468C7C; continue 'dispatch;
	}
	// 82468C28: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468C2C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82468C30: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82468C34: 40980024  bge cr6, 0x82468c58
	if !ctx.cr[6].lt {
	pc = 0x82468C58; continue 'dispatch;
	}
	// 82468C38: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82468C3C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82468C40: 41980008  blt cr6, 0x82468c48
	if ctx.cr[6].lt {
	pc = 0x82468C48; continue 'dispatch;
	}
	// 82468C44: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x82468C48; continue 'dispatch;
            }
            0x82468C48 => {
    //   block [0x82468C48..0x82468C58)
	// 82468C48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82468C4C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82468C50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82468C54: 48005675  bl 0x8246e2c8
	ctx.lr = 0x82468C58;
	sub_8246E2C8(ctx, base);
	pc = 0x82468C58; continue 'dispatch;
            }
            0x82468C58 => {
    //   block [0x82468C58..0x82468C68)
	// 82468C58: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82468C5C: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82468C60: 4098001C  bge cr6, 0x82468c7c
	if !ctx.cr[6].lt {
	pc = 0x82468C7C; continue 'dispatch;
	}
	// 82468C64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82468C68; continue 'dispatch;
            }
            0x82468C68 => {
    //   block [0x82468C68..0x82468C7C)
	// 82468C68: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468C6C: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 82468C70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82468C74: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82468C78: 4198FFF0  blt cr6, 0x82468c68
	if ctx.cr[6].lt {
	pc = 0x82468C68; continue 'dispatch;
	}
	pc = 0x82468C7C; continue 'dispatch;
            }
            0x82468C7C => {
    //   block [0x82468C7C..0x82468C88)
	// 82468C7C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82468C80: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468C84: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	pc = 0x82468C88; continue 'dispatch;
            }
            0x82468C88 => {
    //   block [0x82468C88..0x82468C98)
	// 82468C88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82468C8C: 939B000C  stw r28, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82468C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468C94: 480CC470  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82468C98 => {
    //   block [0x82468C98..0x82468CA4)
	// 82468C98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82468C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468CA0: 480CC464  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468CA8 size=184
    let mut pc: u32 = 0x82468CA8;
    'dispatch: loop {
        match pc {
            0x82468CA8 => {
    //   block [0x82468CA8..0x82468CF0)
	// 82468CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468CB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82468CB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468CB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468CBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468CC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82468CC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468CC8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82468CCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82468CD0: 409A0020  bne cr6, 0x82468cf0
	if !ctx.cr[6].eq {
	pc = 0x82468CF0; continue 'dispatch;
	}
	// 82468CD4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468CD8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82468CDC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82468CE0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468CE4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82468CE8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82468CEC: 4BFFB3CD  bl 0x824640b8
	ctx.lr = 0x82468CF0;
	sub_824640B8(ctx, base);
	pc = 0x82468CF0; continue 'dispatch;
            }
            0x82468CF0 => {
    //   block [0x82468CF0..0x82468D2C)
	// 82468CF0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82468CF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82468CF8: 419A004C  beq cr6, 0x82468d44
	if ctx.cr[6].eq {
	pc = 0x82468D44; continue 'dispatch;
	}
	// 82468CFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468D00: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82468D04: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82468D08: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82468D0C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82468D10: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82468D14: 41980018  blt cr6, 0x82468d2c
	if ctx.cr[6].lt {
	pc = 0x82468D2C; continue 'dispatch;
	}
	// 82468D18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82468D1C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82468D20: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82468D24: 4BFFB1F5  bl 0x82463f18
	ctx.lr = 0x82468D28;
	sub_82463F18(ctx, base);
	// 82468D28: 4800001C  b 0x82468d44
	pc = 0x82468D44; continue 'dispatch;
            }
            0x82468D2C => {
    //   block [0x82468D2C..0x82468D44)
	// 82468D2C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82468D30: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82468D34: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82468D38: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82468D3C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82468D40: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	pc = 0x82468D44; continue 'dispatch;
            }
            0x82468D44 => {
    //   block [0x82468D44..0x82468D60)
	// 82468D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468D48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82468D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82468D54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82468D58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468D60 size=152
    let mut pc: u32 = 0x82468D60;
    'dispatch: loop {
        match pc {
            0x82468D60 => {
    //   block [0x82468D60..0x82468DA8)
	// 82468D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82468D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468D78: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468D7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82468D80: 396B833C  addi r11, r11, -0x7cc4
	ctx.r[11].s64 = ctx.r[11].s64 + -31940;
	// 82468D84: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82468D88: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82468D8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82468D90: 409A0018  bne cr6, 0x82468da8
	if !ctx.cr[6].eq {
	pc = 0x82468DA8; continue 'dispatch;
	}
	// 82468D94: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82468D98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82468D9C: 419A000C  beq cr6, 0x82468da8
	if ctx.cr[6].eq {
	pc = 0x82468DA8; continue 'dispatch;
	}
	// 82468DA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82468DA4: 4BFFFF05  bl 0x82468ca8
	ctx.lr = 0x82468DA8;
	sub_82468CA8(ctx, base);
	pc = 0x82468DA8; continue 'dispatch;
            }
            0x82468DA8 => {
    //   block [0x82468DA8..0x82468DDC)
	// 82468DA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82468DAC: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82468DB0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82468DB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82468DB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82468DBC: 419A0020  beq cr6, 0x82468ddc
	if ctx.cr[6].eq {
	pc = 0x82468DDC; continue 'dispatch;
	}
	// 82468DC0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468DC4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82468DC8: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82468DCC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468DD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82468DD4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82468DD8: 4BFFB2E1  bl 0x824640b8
	ctx.lr = 0x82468DDC;
	sub_824640B8(ctx, base);
	pc = 0x82468DDC; continue 'dispatch;
            }
            0x82468DDC => {
    //   block [0x82468DDC..0x82468DF8)
	// 82468DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468DE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82468DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82468DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82468DEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82468DF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82468DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468DF8 size=140
    let mut pc: u32 = 0x82468DF8;
    'dispatch: loop {
        match pc {
            0x82468DF8 => {
    //   block [0x82468DF8..0x82468E58)
	// 82468DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468DFC: 480CC2C1  bl 0x825350bc
	ctx.lr = 0x82468E00;
	sub_82535080(ctx, base);
	// 82468E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468E04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82468E08: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82468E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468E10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82468E14: 4BFFF935  bl 0x82468748
	ctx.lr = 0x82468E18;
	sub_82468748(ctx, base);
	// 82468E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82468E1C: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82468E20: 394A8378  addi r10, r10, -0x7c88
	ctx.r[10].s64 = ctx.r[10].s64 + -31880;
	// 82468E24: 386BFFE0  addi r3, r11, -0x20
	ctx.r[3].s64 = ctx.r[11].s64 + -32;
	// 82468E28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82468E2C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82468E30: 419A0028  beq cr6, 0x82468e58
	if ctx.cr[6].eq {
	pc = 0x82468E58; continue 'dispatch;
	}
	// 82468E34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82468E38: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82468E3C: 38BDFFE0  addi r5, r29, -0x20
	ctx.r[5].s64 = ctx.r[29].s64 + -32;
	// 82468E40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82468E44: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82468E48: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468E4C: 4800622D  bl 0x8246f078
	ctx.lr = 0x82468E50;
	sub_8246F078(ctx, base);
	// 82468E50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82468E54: 48000008  b 0x82468e5c
	pc = 0x82468E5C; continue 'dispatch;
            }
            0x82468E58 => {
    //   block [0x82468E58..0x82468E5C)
	// 82468E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82468E5C; continue 'dispatch;
            }
            0x82468E5C => {
    //   block [0x82468E5C..0x82468E7C)
	// 82468E5C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82468E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82468E64: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82468E68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82468E6C: 419A0010  beq cr6, 0x82468e7c
	if ctx.cr[6].eq {
	pc = 0x82468E7C; continue 'dispatch;
	}
	// 82468E70: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82468E74: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82468E78: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	pc = 0x82468E7C; continue 'dispatch;
            }
            0x82468E7C => {
    //   block [0x82468E7C..0x82468E84)
	// 82468E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82468E80: 480CC28C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468E88 size=264
    let mut pc: u32 = 0x82468E88;
    'dispatch: loop {
        match pc {
            0x82468E88 => {
    //   block [0x82468E88..0x82468F90)
	// 82468E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468E8C: 480CC215  bl 0x825350a0
	ctx.lr = 0x82468E90;
	sub_82535080(ctx, base);
	// 82468E90: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468E98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82468E9C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82468EA0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82468EA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468EA8: 4BFFFF51  bl 0x82468df8
	ctx.lr = 0x82468EAC;
	sub_82468DF8(ctx, base);
	// 82468EAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468EB0: A15F0006  lhz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82468EB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468EB8: 388B8480  addi r4, r11, -0x7b80
	ctx.r[4].s64 = ctx.r[11].s64 + -31616;
	// 82468EBC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468EC0: 7D560734  extsh r22, r10
	ctx.r[22].s64 = ctx.r[10].s16 as i64;
	// 82468EC4: 3BAB8448  addi r29, r11, -0x7bb8
	ctx.r[29].s64 = ctx.r[11].s64 + -31672;
	// 82468EC8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468ECC: 3B8B842C  addi r28, r11, -0x7bd4
	ctx.r[28].s64 = ctx.r[11].s64 + -31700;
	// 82468ED0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468ED4: 3B6B83F8  addi r27, r11, -0x7c08
	ctx.r[27].s64 = ctx.r[11].s64 + -31752;
	// 82468ED8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468EDC: 3B4B83C0  addi r26, r11, -0x7c40
	ctx.r[26].s64 = ctx.r[11].s64 + -31808;
	// 82468EE0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468EE4: 3B2B83BC  addi r25, r11, -0x7c44
	ctx.r[25].s64 = ctx.r[11].s64 + -31812;
	// 82468EE8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468EEC: 3B0B83B4  addi r24, r11, -0x7c4c
	ctx.r[24].s64 = ctx.r[11].s64 + -31820;
	// 82468EF0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468EF4: 3AEB83A0  addi r23, r11, -0x7c60
	ctx.r[23].s64 = ctx.r[11].s64 + -31840;
	// 82468EF8: 4BFFF3A9  bl 0x824682a0
	ctx.lr = 0x82468EFC;
	sub_824682A0(ctx, base);
	// 82468EFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82468F00: 4BFFF261  bl 0x82468160
	ctx.lr = 0x82468F04;
	sub_82468160(ctx, base);
	// 82468F04: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82468F08: 4BFFF399  bl 0x824682a0
	ctx.lr = 0x82468F0C;
	sub_824682A0(ctx, base);
	// 82468F0C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82468F10: 4BFFF4C9  bl 0x824683d8
	ctx.lr = 0x82468F14;
	sub_824683D8(ctx, base);
	// 82468F14: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82468F18: 4BFFF389  bl 0x824682a0
	ctx.lr = 0x82468F1C;
	sub_824682A0(ctx, base);
	// 82468F1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82468F20: 4BFFF381  bl 0x824682a0
	ctx.lr = 0x82468F24;
	sub_824682A0(ctx, base);
	// 82468F24: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82468F28: 4BFFF379  bl 0x824682a0
	ctx.lr = 0x82468F2C;
	sub_824682A0(ctx, base);
	// 82468F2C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82468F30: 4BFFF371  bl 0x824682a0
	ctx.lr = 0x82468F34;
	sub_824682A0(ctx, base);
	// 82468F34: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82468F38: 4BFFF369  bl 0x824682a0
	ctx.lr = 0x82468F3C;
	sub_824682A0(ctx, base);
	// 82468F3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82468F40: 4BFFF361  bl 0x824682a0
	ctx.lr = 0x82468F44;
	sub_824682A0(ctx, base);
	// 82468F44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82468F48: 4BFFF359  bl 0x824682a0
	ctx.lr = 0x82468F4C;
	sub_824682A0(ctx, base);
	// 82468F4C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82468F50: 3CA02C66  lis r5, 0x2c66
	ctx.r[5].s64 = 744882176;
	// 82468F54: 3900001E  li r8, 0x1e
	ctx.r[8].s64 = 30;
	// 82468F58: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82468F5C: 60A5F2D8  ori r5, r5, 0xf2d8
	ctx.r[5].u64 = ctx.r[5].u64 | 62168;
	// 82468F60: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82468F64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468F68: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82468F6C: 38EB8384  addi r7, r11, -0x7c7c
	ctx.r[7].s64 = ctx.r[11].s64 + -31868;
	// 82468F70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468F74: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468F78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468F7C: 4E800421  bctrl
	ctx.lr = 0x82468F80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468F80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468F84: 4BFFF95D  bl 0x824688e0
	ctx.lr = 0x82468F88;
	sub_824688E0(ctx, base);
	// 82468F88: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82468F8C: 480CC164  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82468F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82468F90 size=128
    let mut pc: u32 = 0x82468F90;
    'dispatch: loop {
        match pc {
            0x82468F90 => {
    //   block [0x82468F90..0x82469010)
	// 82468F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82468F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82468F98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82468F9C: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82468FA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82468FA4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82468FA8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82468FAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468FB0: 4BFFFE49  bl 0x82468df8
	ctx.lr = 0x82468FB4;
	sub_82468DF8(ctx, base);
	// 82468FB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82468FB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468FBC: 4BFFF2E5  bl 0x824682a0
	ctx.lr = 0x82468FC0;
	sub_824682A0(ctx, base);
	// 82468FC0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82468FC4: 3CA02636  lis r5, 0x2636
	ctx.r[5].s64 = 641073152;
	// 82468FC8: 39000025  li r8, 0x25
	ctx.r[8].s64 = 37;
	// 82468FCC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82468FD0: 60A5FE25  ori r5, r5, 0xfe25
	ctx.r[5].u64 = ctx.r[5].u64 | 65061;
	// 82468FD4: 806B9190  lwz r3, -0x6e70(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28272 as u32) ) } as u64;
	// 82468FD8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82468FDC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82468FE0: 38EB8384  addi r7, r11, -0x7c7c
	ctx.r[7].s64 = ctx.r[11].s64 + -31868;
	// 82468FE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82468FE8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82468FEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82468FF0: 4E800421  bctrl
	ctx.lr = 0x82468FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82468FF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82468FF8: 4BFFF8E9  bl 0x824688e0
	ctx.lr = 0x82468FFC;
	sub_824688E0(ctx, base);
	// 82468FFC: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82469000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246900C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469010 size=100
    let mut pc: u32 = 0x82469010;
    'dispatch: loop {
        match pc {
            0x82469010 => {
    //   block [0x82469010..0x82469058)
	// 82469010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246901C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82469020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469028: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8246902C: 4BFFF8B5  bl 0x824688e0
	ctx.lr = 0x82469030;
	sub_824688E0(ctx, base);
	// 82469030: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82469034: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82469038: 419A0020  beq cr6, 0x82469058
	if ctx.cr[6].eq {
	pc = 0x82469058; continue 'dispatch;
	}
	// 8246903C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469040: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82469044: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82469048: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246904C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469050: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82469054: 4BFFB065  bl 0x824640b8
	ctx.lr = 0x82469058;
	sub_824640B8(ctx, base);
	pc = 0x82469058; continue 'dispatch;
            }
            0x82469058 => {
    //   block [0x82469058..0x82469074)
	// 82469058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246905C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82469060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469068: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246906C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82469070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469078 size=128
    let mut pc: u32 = 0x82469078;
    'dispatch: loop {
        match pc {
            0x82469078 => {
    //   block [0x82469078..0x824690F0)
	// 82469078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246907C: 480CC039  bl 0x825350b4
	ctx.lr = 0x82469080;
	sub_82535080(ctx, base);
	// 82469080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469084: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82469088: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246908C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82469090: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82469094: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82469098: 419A0058  beq cr6, 0x824690f0
	if ctx.cr[6].eq {
	pc = 0x824690F0; continue 'dispatch;
	}
	// 8246909C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824690A0: 48001181  bl 0x8246a220
	ctx.lr = 0x824690A4;
	sub_8246A220(ctx, base);
	// 824690A4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824690A8: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 824690AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824690B0: 816B9004  lwz r11, -0x6ffc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 824690B4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824690B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824690BC: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 824690C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824690C4: 4E800421  bctrl
	ctx.lr = 0x824690C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824690C8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824690CC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 824690D0: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824690D4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 824690D8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824690DC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824690E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824690E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824690E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824690EC: 4E800421  bctrl
	ctx.lr = 0x824690F0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824690F0 => {
    //   block [0x824690F0..0x824690F8)
	// 824690F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824690F4: 480CC010  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824690F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824690F8 size=112
    let mut pc: u32 = 0x824690F8;
    'dispatch: loop {
        match pc {
            0x824690F8 => {
    //   block [0x824690F8..0x82469168)
	// 824690F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824690FC: 480CBFB5  bl 0x825350b0
	ctx.lr = 0x82469100;
	sub_82535080(ctx, base);
	// 82469100: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469104: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82469108: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246910C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82469110: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82469114: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82469118: 806B9004  lwz r3, -0x6ffc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 8246911C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82469120: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82469124: 7C86F9D6  mullw r4, r6, r31
	ctx.r[4].s32 = ((ctx.r[6].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 82469128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246912C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82469130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469134: 4E800421  bctrl
	ctx.lr = 0x82469138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469138: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246913C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82469140: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469144: 7CFBF9D6  mullw r7, r27, r31
	ctx.r[7].s32 = ((ctx.r[27].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82469148: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246914C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82469150: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82469154: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82469158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246915C: 4E800421  bctrl
	ctx.lr = 0x82469160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469160: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82469164: 480CBF9C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469168 size=212
    let mut pc: u32 = 0x82469168;
    'dispatch: loop {
        match pc {
            0x82469168 => {
    //   block [0x82469168..0x824691F0)
	// 82469168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246916C: 480CBF41  bl 0x825350ac
	ctx.lr = 0x82469170;
	sub_82535080(ctx, base);
	// 82469170: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469174: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82469178: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8246917C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82469180: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82469184: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82469188: 419A00AC  beq cr6, 0x82469234
	if ctx.cr[6].eq {
	pc = 0x82469234; continue 'dispatch;
	}
	// 8246918C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469190: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469194: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82469198: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8246919C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824691A0: 4E800421  bctrl
	ctx.lr = 0x824691A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824691A4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824691A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824691AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824691B0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824691B4: 4BFFDB15  bl 0x82466cc8
	ctx.lr = 0x824691B8;
	sub_82466CC8(ctx, base);
	// 824691B8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824691BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824691C0: 419A0030  beq cr6, 0x824691f0
	if ctx.cr[6].eq {
	pc = 0x824691F0; continue 'dispatch;
	}
	// 824691C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824691C8: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824691CC: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 824691D0: 4BFFD689  bl 0x82466858
	ctx.lr = 0x824691D4;
	sub_82466858(ctx, base);
	// 824691D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824691D8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 824691DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824691E0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824691E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824691E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824691EC: 4E800421  bctrl
	ctx.lr = 0x824691F0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824691F0 => {
    //   block [0x824691F0..0x82469234)
	// 824691F0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824691F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824691F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824691FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82469200: 48000379  bl 0x82469578
	ctx.lr = 0x82469204;
	sub_82469578(ctx, base);
	// 82469204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469208: 4BFFD659  bl 0x82466860
	ctx.lr = 0x8246920C;
	sub_82466860(ctx, base);
	// 8246920C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469210: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82469214: 409AFFDC  bne cr6, 0x824691f0
	if !ctx.cr[6].eq {
	pc = 0x824691F0; continue 'dispatch;
	}
	// 82469218: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8246921C: 419A0018  beq cr6, 0x82469234
	if ctx.cr[6].eq {
	pc = 0x82469234; continue 'dispatch;
	}
	// 82469220: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469224: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82469228: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8246922C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469230: 4E800421  bctrl
	ctx.lr = 0x82469234;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82469234 => {
    //   block [0x82469234..0x8246923C)
	// 82469234: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82469238: 480CBEC4  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469278 size=276
    let mut pc: u32 = 0x82469278;
    'dispatch: loop {
        match pc {
            0x82469278 => {
    //   block [0x82469278..0x824692FC)
	// 82469278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246927C: 480CBE2D  bl 0x825350a8
	ctx.lr = 0x82469280;
	sub_82535080(ctx, base);
	// 82469280: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469284: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82469288: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246928C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82469290: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82469294: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82469298: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8246929C: 419A00E8  beq cr6, 0x82469384
	if ctx.cr[6].eq {
	pc = 0x82469384; continue 'dispatch;
	}
	// 824692A0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824692A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824692A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824692AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824692B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824692B4: 4E800421  bctrl
	ctx.lr = 0x824692B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824692B8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 824692BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824692C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824692C4: 4BFFDA05  bl 0x82466cc8
	ctx.lr = 0x824692C8;
	sub_82466CC8(ctx, base);
	// 824692C8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824692CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824692D0: 419A002C  beq cr6, 0x824692fc
	if ctx.cr[6].eq {
	pc = 0x824692FC; continue 'dispatch;
	}
	// 824692D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824692D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 824692DC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 824692E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824692E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824692E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824692EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824692F0: 4E800421  bctrl
	ctx.lr = 0x824692F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824692F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824692F8: 480CBE00  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x824692FC => {
    //   block [0x824692FC..0x8246933C)
	// 824692FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469300: 4BFFD9B9  bl 0x82466cb8
	ctx.lr = 0x82469304;
	sub_82466CB8(ctx, base);
	// 82469304: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82469308: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8246930C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82469310: 816B9004  lwz r11, -0x6ffc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28668 as u32) ) } as u64;
	// 82469314: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82469318: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246931C: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82469320: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469324: 4E800421  bctrl
	ctx.lr = 0x82469328;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469328: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8246932C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82469330: 419A000C  beq cr6, 0x8246933c
	if ctx.cr[6].eq {
	pc = 0x8246933C; continue 'dispatch;
	}
	// 82469334: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82469338: 48000010  b 0x82469348
	pc = 0x82469348; continue 'dispatch;
            }
            0x8246933C => {
    //   block [0x8246933C..0x82469348)
	// 8246933C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469340: 4BFFD519  bl 0x82466858
	ctx.lr = 0x82469344;
	sub_82466858(ctx, base);
	// 82469344: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	pc = 0x82469348; continue 'dispatch;
            }
            0x82469348 => {
    //   block [0x82469348..0x82469384)
	// 82469348: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246934C: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82469350: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82469354: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82469358: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8246935C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82469360: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82469364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469368: 4E800421  bctrl
	ctx.lr = 0x8246936C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246936C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82469370: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82469374: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82469378: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8246937C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82469380: 4BFFFDE9  bl 0x82469168
	ctx.lr = 0x82469384;
	sub_82469168(ctx, base);
            }
            0x82469384 => {
    //   block [0x82469384..0x8246938C)
	// 82469384: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82469388: 480CBD70  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469390 size=116
    let mut pc: u32 = 0x82469390;
    'dispatch: loop {
        match pc {
            0x82469390 => {
    //   block [0x82469390..0x824693EC)
	// 82469390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246939C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824693A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824693A4: 887F000D  lbz r3, 0xd(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 824693A8: 48001E11  bl 0x8246b1b8
	ctx.lr = 0x824693AC;
	sub_8246B1B8(ctx, base);
	// 824693AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824693B0: A14B0008  lhz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824693B4: 7D430734  extsh r3, r10
	ctx.r[3].s64 = ctx.r[10].s16 as i64;
	// 824693B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824693BC: 41990034  bgt cr6, 0x824693f0
	if ctx.cr[6].gt {
	pc = 0x824693F0; continue 'dispatch;
	}
	// 824693C0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824693C4: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 824693C8: 409A0024  bne cr6, 0x824693ec
	if !ctx.cr[6].eq {
	pc = 0x824693EC; continue 'dispatch;
	}
	// 824693CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824693D0: 48001E09  bl 0x8246b1d8
	ctx.lr = 0x824693D4;
	sub_8246B1D8(ctx, base);
	// 824693D4: 4BFFD8E5  bl 0x82466cb8
	ctx.lr = 0x824693D8;
	sub_82466CB8(ctx, base);
	// 824693D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824693DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824693E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824693E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824693E8: 4E800020  blr
	return;
            }
            0x824693EC => {
    //   block [0x824693EC..0x824693F0)
	// 824693EC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x824693F0; continue 'dispatch;
            }
            0x824693F0 => {
    //   block [0x824693F0..0x82469404)
	// 824693F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824693F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824693F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824693FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82469400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469408 size=148
    let mut pc: u32 = 0x82469408;
    'dispatch: loop {
        match pc {
            0x82469408 => {
    //   block [0x82469408..0x82469458)
	// 82469408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246940C: 480CBCA1  bl 0x825350ac
	ctx.lr = 0x82469410;
	sub_82535080(ctx, base);
	// 82469410: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469414: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82469418: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8246941C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82469420: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82469424: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82469428: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8246942C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82469430: 4BFFD889  bl 0x82466cb8
	ctx.lr = 0x82469434;
	sub_82466CB8(ctx, base);
	// 82469434: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469438: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8246943C: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469440: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82469444: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8246944C: 4E800421  bctrl
	ctx.lr = 0x82469450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469450: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82469454: 4099002C  ble cr6, 0x82469480
	if !ctx.cr[6].gt {
	pc = 0x82469480; continue 'dispatch;
	}
            }
            0x82469458 => {
    //   block [0x82469458..0x82469480)
	// 82469458: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8246945C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82469460: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82469464: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246946C: 4BFFFE0D  bl 0x82469278
	ctx.lr = 0x82469470;
	sub_82469278(ctx, base);
	// 82469470: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82469474: 7FFFD214  add r31, r31, r26
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 82469478: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8246947C: 409AFFDC  bne cr6, 0x82469458
	if !ctx.cr[6].eq {
	pc = 0x82469458; continue 'dispatch;
	}
	pc = 0x82469480; continue 'dispatch;
            }
            0x82469480 => {
    //   block [0x82469480..0x8246949C)
	// 82469480: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469484: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82469488: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8246948C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469490: 4E800421  bctrl
	ctx.lr = 0x82469494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469494: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82469498: 480CBC64  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824694A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824694A0 size=212
    let mut pc: u32 = 0x824694A0;
    'dispatch: loop {
        match pc {
            0x824694A0 => {
    //   block [0x824694A0..0x82469504)
	// 824694A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824694A4: 480CBC11  bl 0x825350b4
	ctx.lr = 0x824694A8;
	sub_82535080(ctx, base);
	// 824694A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824694AC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824694B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824694B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824694B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824694BC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 824694C0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824694C4: 409900A8  ble cr6, 0x8246956c
	if !ctx.cr[6].gt {
	pc = 0x8246956C; continue 'dispatch;
	}
	// 824694C8: 897D000D  lbz r11, 0xd(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(13 as u32) ) } as u64;
	// 824694CC: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 824694D0: 419A0034  beq cr6, 0x82469504
	if ctx.cr[6].eq {
	pc = 0x82469504; continue 'dispatch;
	}
	// 824694D4: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 824694D8: 409A0094  bne cr6, 0x8246956c
	if !ctx.cr[6].eq {
	pc = 0x8246956C; continue 'dispatch;
	}
	// 824694DC: 48001CFD  bl 0x8246b1d8
	ctx.lr = 0x824694E0;
	sub_8246B1D8(ctx, base);
	// 824694E0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824694E4: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 824694E8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 824694EC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824694F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824694F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824694F8: 4BFFFF11  bl 0x82469408
	ctx.lr = 0x824694FC;
	sub_82469408(ctx, base);
	// 824694FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82469500: 480CBC04  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82469504 => {
    //   block [0x82469504..0x82469530)
	// 82469504: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469508: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246950C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469510: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469518: 4E800421  bctrl
	ctx.lr = 0x8246951C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246951C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82469520: 48001CB9  bl 0x8246b1d8
	ctx.lr = 0x82469524;
	sub_8246B1D8(ctx, base);
	// 82469524: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82469528: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8246952C: 4099002C  ble cr6, 0x82469558
	if !ctx.cr[6].gt {
	pc = 0x82469558; continue 'dispatch;
	}
            }
            0x82469530 => {
    //   block [0x82469530..0x82469558)
	// 82469530: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82469534: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469538: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8246953C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82469540: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82469544: 4BFFFD35  bl 0x82469278
	ctx.lr = 0x82469548;
	sub_82469278(ctx, base);
	// 82469548: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8246954C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82469550: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82469554: 409AFFDC  bne cr6, 0x82469530
	if !ctx.cr[6].eq {
	pc = 0x82469530; continue 'dispatch;
	}
	pc = 0x82469558; continue 'dispatch;
            }
            0x82469558 => {
    //   block [0x82469558..0x8246956C)
	// 82469558: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246955C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82469560: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82469564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469568: 4E800421  bctrl
	ctx.lr = 0x8246956C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8246956C => {
    //   block [0x8246956C..0x82469574)
	// 8246956C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82469570: 480CBB94  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469578 size=828
    let mut pc: u32 = 0x82469578;
    'dispatch: loop {
        match pc {
            0x82469578 => {
    //   block [0x82469578..0x824695AC)
	// 82469578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246957C: 480CBB21  bl 0x8253509c
	ctx.lr = 0x82469580;
	sub_82535080(ctx, base);
	// 82469580: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469584: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82469588: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8246958C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82469590: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82469594: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82469598: 419A0314  beq cr6, 0x824698ac
	if ctx.cr[6].eq {
	pc = 0x824698AC; continue 'dispatch;
	}
	// 8246959C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 824695A0: 409A000C  bne cr6, 0x824695ac
	if !ctx.cr[6].eq {
	pc = 0x824695AC; continue 'dispatch;
	}
	// 824695A4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824695A8: 832B91E0  lwz r25, -0x6e20(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28192 as u32) ) } as u64;
	pc = 0x824695AC; continue 'dispatch;
            }
            0x824695AC => {
    //   block [0x824695AC..0x824695E0)
	// 824695AC: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 824695B0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 824695B4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824695B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824695BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824695C0: 4E800421  bctrl
	ctx.lr = 0x824695C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824695C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824695C8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 824695CC: 4BFFD53D  bl 0x82466b08
	ctx.lr = 0x824695D0;
	sub_82466B08(ctx, base);
	// 824695D0: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 824695D4: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 824695D8: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 824695DC: 409902D0  ble cr6, 0x824698ac
	if !ctx.cr[6].gt {
	pc = 0x824698AC; continue 'dispatch;
	}
            }
            0x824695E0 => {
    //   block [0x824695E0..0x82469654)
	// 824695E0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 824695E4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 824695E8: 4BFFD529  bl 0x82466b10
	ctx.lr = 0x824695EC;
	sub_82466B10(ctx, base);
	// 824695EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824695F0: A17F0010  lhz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824695F4: 556BBA7E  srwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824695F8: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 824695FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82469600: 409A02A0  bne cr6, 0x824698a0
	if !ctx.cr[6].eq {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 82469604: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82469608: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 8246960C: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82469610: 41990290  bgt cr6, 0x824698a0
	if ctx.cr[6].gt {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 82469614: 3D808247  lis r12, -0x7db9
	ctx.r[12].s64 = -2109276160;
	// 82469618: 398C962C  addi r12, r12, -0x69d4
	ctx.r[12].s64 = ctx.r[12].s64 + -27092;
	// 8246961C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82469620: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82469624: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82469628: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82469840; continue 'dispatch;
		},
		1 => {
	pc = 0x824698A0; continue 'dispatch;
		},
		2 => {
	pc = 0x824696E0; continue 'dispatch;
		},
		3 => {
	pc = 0x824698A0; continue 'dispatch;
		},
		4 => {
	pc = 0x824698A0; continue 'dispatch;
		},
		5 => {
	pc = 0x824698A0; continue 'dispatch;
		},
		6 => {
	pc = 0x82469744; continue 'dispatch;
		},
		7 => {
	pc = 0x824697B8; continue 'dispatch;
		},
		8 => {
	pc = 0x82469870; continue 'dispatch;
		},
		9 => {
	pc = 0x82469654; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8246962C: 82469840  lwz r18, -0x67c0(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26560 as u32) ) } as u64;
	// 82469630: 824698A0  lwz r18, -0x6760(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26464 as u32) ) } as u64;
	// 82469634: 824696E0  lwz r18, -0x6920(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26912 as u32) ) } as u64;
	// 82469638: 824698A0  lwz r18, -0x6760(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26464 as u32) ) } as u64;
	// 8246963C: 824698A0  lwz r18, -0x6760(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26464 as u32) ) } as u64;
	// 82469640: 824698A0  lwz r18, -0x6760(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26464 as u32) ) } as u64;
	// 82469644: 82469744  lwz r18, -0x68bc(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26812 as u32) ) } as u64;
	// 82469648: 824697B8  lwz r18, -0x6848(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26696 as u32) ) } as u64;
	// 8246964C: 82469870  lwz r18, -0x6790(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-26512 as u32) ) } as u64;
	// 82469650: 82469654  lwz r18, -0x69ac(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27052 as u32) ) } as u64;
            }
            0x82469654 => {
    //   block [0x82469654..0x824696A0)
	// 82469654: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82469658: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8246965C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82469660: 48006B51  bl 0x824701b0
	ctx.lr = 0x82469664;
	sub_824701B0(ctx, base);
	// 82469664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469668: 48001B89  bl 0x8246b1f0
	ctx.lr = 0x8246966C;
	sub_8246B1F0(ctx, base);
	// 8246966C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82469670: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82469674: 4199002C  bgt cr6, 0x824696a0
	if ctx.cr[6].gt {
	pc = 0x824696A0; continue 'dispatch;
	}
	// 82469678: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8246967C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82469680: 48006C51  bl 0x824702d0
	ctx.lr = 0x82469684;
	sub_824702D0(ctx, base);
	// 82469684: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82469688: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8246968C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82469690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469694: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469698: 4BFFF9E1  bl 0x82469078
	ctx.lr = 0x8246969C;
	sub_82469078(ctx, base);
	// 8246969C: 48000204  b 0x824698a0
	pc = 0x824698A0; continue 'dispatch;
            }
            0x824696A0 => {
    //   block [0x824696A0..0x824696AC)
	// 824696A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824696A4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824696A8: 409901F8  ble cr6, 0x824698a0
	if !ctx.cr[6].gt {
	pc = 0x824698A0; continue 'dispatch;
	}
	pc = 0x824696AC; continue 'dispatch;
            }
            0x824696AC => {
    //   block [0x824696AC..0x824696E0)
	// 824696AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824696B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824696B4: 48006C1D  bl 0x824702d0
	ctx.lr = 0x824696B8;
	sub_824702D0(ctx, base);
	// 824696B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824696BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824696C0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 824696C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824696C8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824696CC: 4BFFF9AD  bl 0x82469078
	ctx.lr = 0x824696D0;
	sub_82469078(ctx, base);
	// 824696D0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824696D4: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824696D8: 4198FFD4  blt cr6, 0x824696ac
	if ctx.cr[6].lt {
	pc = 0x824696AC; continue 'dispatch;
	}
	// 824696DC: 480001C4  b 0x824698a0
	pc = 0x824698A0; continue 'dispatch;
            }
            0x824696E0 => {
    //   block [0x824696E0..0x82469744)
	// 824696E0: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 824696E4: 7FCBC214  add r30, r11, r24
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 824696E8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824696EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824696F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824696F4: 409A01AC  bne cr6, 0x824698a0
	if !ctx.cr[6].eq {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 824696F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824696FC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82469700: 557B00BE  clrlwi r27, r11, 2
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82469704: 4BFFFC8D  bl 0x82469390
	ctx.lr = 0x82469708;
	sub_82469390(ctx, base);
	// 82469708: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8246970C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82469710: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469714: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82469718: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8246971C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82469720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469724: 4BFFF9D5  bl 0x824690f8
	ctx.lr = 0x82469728;
	sub_824690F8(ctx, base);
	// 82469728: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8246972C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82469730: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469734: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82469738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246973C: 4BFFFD65  bl 0x824694a0
	ctx.lr = 0x82469740;
	sub_824694A0(ctx, base);
	// 82469740: 48000160  b 0x824698a0
	pc = 0x824698A0; continue 'dispatch;
            }
            0x82469744 => {
    //   block [0x82469744..0x824697B8)
	// 82469744: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82469748: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8246974C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82469750: 48006A61  bl 0x824701b0
	ctx.lr = 0x82469754;
	sub_824701B0(ctx, base);
	// 82469754: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82469758: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8246975C: 48006C65  bl 0x824703c0
	ctx.lr = 0x82469760;
	sub_824703C0(ctx, base);
	// 82469760: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82469764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469768: E96B0000  ld r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8246976C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82469770: 4BFFFC21  bl 0x82469390
	ctx.lr = 0x82469774;
	sub_82469390(ctx, base);
	// 82469774: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82469778: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8246977C: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82469780: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82469784: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82469788: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8246978C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82469790: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82469794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469798: 4BFFF961  bl 0x824690f8
	ctx.lr = 0x8246979C;
	sub_824690F8(ctx, base);
	// 8246979C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 824697A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824697A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824697A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824697AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824697B0: 4BFFFCF1  bl 0x824694a0
	ctx.lr = 0x824697B4;
	sub_824694A0(ctx, base);
	// 824697B4: 480000EC  b 0x824698a0
	pc = 0x824698A0; continue 'dispatch;
            }
            0x824697B8 => {
    //   block [0x824697B8..0x82469840)
	// 824697B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824697BC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 824697C0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 824697C4: 480069ED  bl 0x824701b0
	ctx.lr = 0x824697C8;
	sub_824701B0(ctx, base);
	// 824697C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824697CC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 824697D0: 48006C01  bl 0x824703d0
	ctx.lr = 0x824697D4;
	sub_824703D0(ctx, base);
	// 824697D4: 83C30008  lwz r30, 8(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824697D8: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824697DC: 83630004  lwz r27, 4(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824697E0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824697E4: 409900BC  ble cr6, 0x824698a0
	if !ctx.cr[6].gt {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 824697E8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824697EC: 419A00B4  beq cr6, 0x824698a0
	if ctx.cr[6].eq {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 824697F0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824697F4: 419A00AC  beq cr6, 0x824698a0
	if ctx.cr[6].eq {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 824697F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824697FC: 4BFFD4BD  bl 0x82466cb8
	ctx.lr = 0x82469800;
	sub_82466CB8(ctx, base);
	// 82469800: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82469804: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82469808: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246980C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82469810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469814: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82469818: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8246981C: 4BFFF8DD  bl 0x824690f8
	ctx.lr = 0x82469820;
	sub_824690F8(ctx, base);
	// 82469820: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82469824: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82469828: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8246982C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82469830: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82469834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469838: 4BFFFBD1  bl 0x82469408
	ctx.lr = 0x8246983C;
	sub_82469408(ctx, base);
	// 8246983C: 48000064  b 0x824698a0
	pc = 0x824698A0; continue 'dispatch;
            }
            0x82469840 => {
    //   block [0x82469840..0x82469870)
	// 82469840: 897F000D  lbz r11, 0xd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(13 as u32) ) } as u64;
	// 82469844: 2B0B0019  cmplwi cr6, r11, 0x19
	ctx.cr[6].compare_u32(ctx.r[11].u32, 25 as u32, &mut ctx.xer);
	// 82469848: 409A0058  bne cr6, 0x824698a0
	if !ctx.cr[6].eq {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 8246984C: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82469850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469854: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469858: 7FCBC02E  lwzx r30, r11, r24
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8246985C: 4800197D  bl 0x8246b1d8
	ctx.lr = 0x82469860;
	sub_8246B1D8(ctx, base);
	// 82469860: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82469864: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82469868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246986C: 48000028  b 0x82469894
	pc = 0x82469894; continue 'dispatch;
            }
            0x82469870 => {
    //   block [0x82469870..0x82469894)
	// 82469870: A17F0012  lhz r11, 0x12(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82469874: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82469878: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246987C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82469880: 419A0020  beq cr6, 0x824698a0
	if ctx.cr[6].eq {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 82469884: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82469888: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246988C: 419A0014  beq cr6, 0x824698a0
	if ctx.cr[6].eq {
	pc = 0x824698A0; continue 'dispatch;
	}
	// 82469890: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x82469894; continue 'dispatch;
            }
            0x82469894 => {
    //   block [0x82469894..0x824698A0)
	// 82469894: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82469898: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8246989C: 4BFFF9DD  bl 0x82469278
	ctx.lr = 0x824698A0;
	sub_82469278(ctx, base);
	pc = 0x824698A0; continue 'dispatch;
            }
            0x824698A0 => {
    //   block [0x824698A0..0x824698AC)
	// 824698A0: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 824698A4: 7F17A800  cmpw cr6, r23, r21
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[21].s32, &mut ctx.xer);
	// 824698A8: 4198FD38  blt cr6, 0x824695e0
	if ctx.cr[6].lt {
	pc = 0x824695E0; continue 'dispatch;
	}
	pc = 0x824698AC; continue 'dispatch;
            }
            0x824698AC => {
    //   block [0x824698AC..0x824698B4)
	// 824698AC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824698B0: 480CB83C  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824698B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824698B8 size=100
    let mut pc: u32 = 0x824698B8;
    'dispatch: loop {
        match pc {
            0x824698B8 => {
    //   block [0x824698B8..0x824698E8)
	// 824698B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824698BC: 480CB7FD  bl 0x825350b8
	ctx.lr = 0x824698C0;
	sub_82535080(ctx, base);
	// 824698C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824698C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824698C8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824698CC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824698D0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824698D4: 419A0040  beq cr6, 0x82469914
	if ctx.cr[6].eq {
	pc = 0x82469914; continue 'dispatch;
	}
	// 824698D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824698DC: 409A000C  bne cr6, 0x824698e8
	if !ctx.cr[6].eq {
	pc = 0x824698E8; continue 'dispatch;
	}
	// 824698E0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824698E4: 83CB91E0  lwz r30, -0x6e20(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28192 as u32) ) } as u64;
	pc = 0x824698E8; continue 'dispatch;
            }
            0x824698E8 => {
    //   block [0x824698E8..0x824698EC)
	// 824698E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	pc = 0x824698EC; continue 'dispatch;
            }
            0x824698EC => {
    //   block [0x824698EC..0x82469914)
	// 824698EC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824698F0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824698F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824698F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824698FC: 4BFFFC7D  bl 0x82469578
	ctx.lr = 0x82469900;
	sub_82469578(ctx, base);
	// 82469900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469904: 4BFFCF5D  bl 0x82466860
	ctx.lr = 0x82469908;
	sub_82466860(ctx, base);
	// 82469908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246990C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82469910: 409AFFDC  bne cr6, 0x824698ec
	if !ctx.cr[6].eq {
	pc = 0x824698EC; continue 'dispatch;
	}
	pc = 0x82469914; continue 'dispatch;
            }
            0x82469914 => {
    //   block [0x82469914..0x8246991C)
	// 82469914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82469918: 480CB7F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469948 size=108
    let mut pc: u32 = 0x82469948;
    'dispatch: loop {
        match pc {
            0x82469948 => {
    //   block [0x82469948..0x82469970)
	// 82469948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246994C: 480CB771  bl 0x825350bc
	ctx.lr = 0x82469950;
	sub_82535080(ctx, base);
	// 82469950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469954: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82469958: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246995C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82469960: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82469964: 409A000C  bne cr6, 0x82469970
	if !ctx.cr[6].eq {
	pc = 0x82469970; continue 'dispatch;
	}
	// 82469968: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8246996C: 83CB91E0  lwz r30, -0x6e20(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28192 as u32) ) } as u64;
	pc = 0x82469970; continue 'dispatch;
            }
            0x82469970 => {
    //   block [0x82469970..0x824699AC)
	// 82469970: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82469974: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469978: 806B91DC  lwz r3, -0x6e24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 8246997C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469980: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469988: 4E800421  bctrl
	ctx.lr = 0x8246998C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8246998C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82469990: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82469994: 419A0018  beq cr6, 0x824699ac
	if ctx.cr[6].eq {
	pc = 0x824699AC; continue 'dispatch;
	}
	// 82469998: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8246999C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824699A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824699A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824699A8: 4BFFF7C1  bl 0x82469168
	ctx.lr = 0x824699AC;
	sub_82469168(ctx, base);
            }
            0x824699AC => {
    //   block [0x824699AC..0x824699B4)
	// 824699AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824699B0: 480CB75C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824699B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824699B8 size=204
    let mut pc: u32 = 0x824699B8;
    'dispatch: loop {
        match pc {
            0x824699B8 => {
    //   block [0x824699B8..0x824699E8)
	// 824699B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824699BC: 480CB6F9  bl 0x825350b4
	ctx.lr = 0x824699C0;
	sub_82535080(ctx, base);
	// 824699C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824699C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824699C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824699CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824699D0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 824699D4: 419A00A4  beq cr6, 0x82469a78
	if ctx.cr[6].eq {
	pc = 0x82469A78; continue 'dispatch;
	}
	// 824699D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824699DC: 409A000C  bne cr6, 0x824699e8
	if !ctx.cr[6].eq {
	pc = 0x824699E8; continue 'dispatch;
	}
	// 824699E0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824699E4: 83CB91E0  lwz r30, -0x6e20(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28192 as u32) ) } as u64;
	pc = 0x824699E8; continue 'dispatch;
            }
            0x824699E8 => {
    //   block [0x824699E8..0x82469A78)
	// 824699E8: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 824699EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824699F0: 806B91DC  lwz r3, -0x6e24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 824699F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824699F8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824699FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469A00: 4E800421  bctrl
	ctx.lr = 0x82469A04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469A08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82469A0C: 419A006C  beq cr6, 0x82469a78
	if ctx.cr[6].eq {
	pc = 0x82469A78; continue 'dispatch;
	}
	// 82469A10: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469A14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469A18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82469A1C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82469A20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469A24: 4E800421  bctrl
	ctx.lr = 0x82469A28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469A28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82469A2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469A30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82469A34: 4BFFD295  bl 0x82466cc8
	ctx.lr = 0x82469A38;
	sub_82466CC8(ctx, base);
	// 82469A38: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469A3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82469A40: 419A0038  beq cr6, 0x82469a78
	if ctx.cr[6].eq {
	pc = 0x82469A78; continue 'dispatch;
	}
	// 82469A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469A48: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469A4C: 4BFFCE0D  bl 0x82466858
	ctx.lr = 0x82469A50;
	sub_82466858(ctx, base);
	// 82469A50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82469A54: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82469A58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82469A5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82469A60: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82469A64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469A68: 4E800421  bctrl
	ctx.lr = 0x82469A6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469A70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82469A74: 480CB690  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82469A78 => {
    //   block [0x82469A78..0x82469A84)
	// 82469A78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82469A7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82469A80: 480CB684  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469A88 size=88
    let mut pc: u32 = 0x82469A88;
    'dispatch: loop {
        match pc {
            0x82469A88 => {
    //   block [0x82469A88..0x82469AD8)
	// 82469A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469A8C: 480CB631  bl 0x825350bc
	ctx.lr = 0x82469A90;
	sub_82535080(ctx, base);
	// 82469A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469A94: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82469A98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469A9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82469AA0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82469AA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469AA8: 806B91DC  lwz r3, -0x6e24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 82469AAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469AB0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469AB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469AB8: 4E800421  bctrl
	ctx.lr = 0x82469ABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469ABC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82469AC0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82469AC4: 419A0014  beq cr6, 0x82469ad8
	if ctx.cr[6].eq {
	pc = 0x82469AD8; continue 'dispatch;
	}
	// 82469AC8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82469ACC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82469AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469AD4: 4BFFFAA5  bl 0x82469578
	ctx.lr = 0x82469AD8;
	sub_82469578(ctx, base);
            }
            0x82469AD8 => {
    //   block [0x82469AD8..0x82469AE0)
	// 82469AD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82469ADC: 480CB630  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469AE0 size=88
    let mut pc: u32 = 0x82469AE0;
    'dispatch: loop {
        match pc {
            0x82469AE0 => {
    //   block [0x82469AE0..0x82469B30)
	// 82469AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469AE4: 480CB5D9  bl 0x825350bc
	ctx.lr = 0x82469AE8;
	sub_82535080(ctx, base);
	// 82469AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469AEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82469AF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469AF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82469AF8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82469AFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82469B00: 806B91DC  lwz r3, -0x6e24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28196 as u32) ) } as u64;
	// 82469B04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469B08: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469B0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82469B10: 4E800421  bctrl
	ctx.lr = 0x82469B14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82469B14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82469B18: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82469B1C: 419A0014  beq cr6, 0x82469b30
	if ctx.cr[6].eq {
	pc = 0x82469B30; continue 'dispatch;
	}
	// 82469B20: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82469B24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82469B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469B2C: 4BFFFD8D  bl 0x824698b8
	ctx.lr = 0x82469B30;
	sub_824698B8(ctx, base);
            }
            0x82469B30 => {
    //   block [0x82469B30..0x82469B38)
	// 82469B30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82469B34: 480CB5D8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469B48 size=160
    let mut pc: u32 = 0x82469B48;
    'dispatch: loop {
        match pc {
            0x82469B48 => {
    //   block [0x82469B48..0x82469B7C)
	// 82469B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469B54: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469B58: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82469B5C: 419A0020  beq cr6, 0x82469b7c
	if ctx.cr[6].eq {
	pc = 0x82469B7C; continue 'dispatch;
	}
	// 82469B60: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82469B68: 419A0014  beq cr6, 0x82469b7c
	if ctx.cr[6].eq {
	pc = 0x82469B7C; continue 'dispatch;
	}
	// 82469B6C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469B70: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82469B74: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82469B78: 4BFFA749  bl 0x824642c0
	ctx.lr = 0x82469B7C;
	sub_824642C0(ctx, base);
	pc = 0x82469B7C; continue 'dispatch;
            }
            0x82469B7C => {
    //   block [0x82469B7C..0x82469BC0)
	// 82469B7C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469B80: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82469B84: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82469B88: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82469B8C: 419A004C  beq cr6, 0x82469bd8
	if ctx.cr[6].eq {
	pc = 0x82469BD8; continue 'dispatch;
	}
	// 82469B90: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82469B94: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82469B98: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82469B9C: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82469BA0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82469BA4: 4198001C  blt cr6, 0x82469bc0
	if ctx.cr[6].lt {
	pc = 0x82469BC0; continue 'dispatch;
	}
	// 82469BA8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82469BAC: 4BFFA36D  bl 0x82463f18
	ctx.lr = 0x82469BB0;
	sub_82463F18(ctx, base);
	// 82469BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469BBC: 4E800020  blr
	return;
            }
            0x82469BC0 => {
    //   block [0x82469BC0..0x82469BD8)
	// 82469BC0: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82469BC4: 81430050  lwz r10, 0x50(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82469BC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82469BCC: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82469BD0: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82469BD4: 90A30050  stw r5, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	pc = 0x82469BD8; continue 'dispatch;
            }
            0x82469BD8 => {
    //   block [0x82469BD8..0x82469BE8)
	// 82469BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469BE8 size=184
    let mut pc: u32 = 0x82469BE8;
    'dispatch: loop {
        match pc {
            0x82469BE8 => {
    //   block [0x82469BE8..0x82469C44)
	// 82469BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469BF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82469BF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82469BF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469BFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469C00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82469C04: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469C08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82469C0C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82469C10: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82469C14: 419A0074  beq cr6, 0x82469c88
	if ctx.cr[6].eq {
	pc = 0x82469C88; continue 'dispatch;
	}
	// 82469C18: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82469C1C: 40990064  ble cr6, 0x82469c80
	if !ctx.cr[6].gt {
	pc = 0x82469C80; continue 'dispatch;
	}
	// 82469C20: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82469C24: 419A0020  beq cr6, 0x82469c44
	if ctx.cr[6].eq {
	pc = 0x82469C44; continue 'dispatch;
	}
	// 82469C28: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469C2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82469C30: 419A0014  beq cr6, 0x82469c44
	if ctx.cr[6].eq {
	pc = 0x82469C44; continue 'dispatch;
	}
	// 82469C34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469C38: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82469C3C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82469C40: 4BFFA681  bl 0x824642c0
	ctx.lr = 0x82469C44;
	sub_824642C0(ctx, base);
	pc = 0x82469C44; continue 'dispatch;
            }
            0x82469C44 => {
    //   block [0x82469C44..0x82469C80)
	// 82469C44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82469C48: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469C4C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82469C50: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 82469C54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82469C58: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82469C5C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82469C60: 4BFFA609  bl 0x82464268
	ctx.lr = 0x82469C64;
	sub_82464268(ctx, base);
	// 82469C64: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82469C68: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82469C6C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82469C70: 394BFFE0  addi r10, r11, -0x20
	ctx.r[10].s64 = ctx.r[11].s64 + -32;
	// 82469C74: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82469C78: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82469C7C: 4800000C  b 0x82469c88
	pc = 0x82469C88; continue 'dispatch;
            }
            0x82469C80 => {
    //   block [0x82469C80..0x82469C88)
	// 82469C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82469C84: 4BFFFEC5  bl 0x82469b48
	ctx.lr = 0x82469C88;
	sub_82469B48(ctx, base);
	pc = 0x82469C88; continue 'dispatch;
            }
            0x82469C88 => {
    //   block [0x82469C88..0x82469CA0)
	// 82469C88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82469C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469C94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82469C98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82469C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469CA0 size=96
    let mut pc: u32 = 0x82469CA0;
    'dispatch: loop {
        match pc {
            0x82469CA0 => {
    //   block [0x82469CA0..0x82469CD8)
	// 82469CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469CA4: 480CB419  bl 0x825350bc
	ctx.lr = 0x82469CA8;
	sub_82535080(ctx, base);
	// 82469CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469CAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469CB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82469CB4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82469CB8: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82469CBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82469CC0: 419A0018  beq cr6, 0x82469cd8
	if ctx.cr[6].eq {
	pc = 0x82469CD8; continue 'dispatch;
	}
	// 82469CC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82469CC8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469CCC: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82469CD0: 41820008  beq 0x82469cd8
	if ctx.cr[0].eq {
	pc = 0x82469CD8; continue 'dispatch;
	}
	// 82469CD4: 4BFFFE75  bl 0x82469b48
	ctx.lr = 0x82469CD8;
	sub_82469B48(ctx, base);
	pc = 0x82469CD8; continue 'dispatch;
            }
            0x82469CD8 => {
    //   block [0x82469CD8..0x82469D00)
	// 82469CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82469CDC: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82469CE0: 995F0010  stb r10, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82469CE4: 394BFFE0  addi r10, r11, -0x20
	ctx.r[10].s64 = ctx.r[11].s64 + -32;
	// 82469CE8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82469CEC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82469CF0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82469CF4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82469CF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82469CFC: 480CB410  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469D00 size=128
    let mut pc: u32 = 0x82469D00;
    'dispatch: loop {
        match pc {
            0x82469D00 => {
    //   block [0x82469D00..0x82469D40)
	// 82469D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469D08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82469D0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469D10: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469D14: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82469D18: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82469D1C: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82469D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82469D24: 419A001C  beq cr6, 0x82469d40
	if ctx.cr[6].eq {
	pc = 0x82469D40; continue 'dispatch;
	}
	// 82469D28: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82469D2C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82469D30: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82469D34: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469D38: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82469D3C: 48000010  b 0x82469d4c
	pc = 0x82469D4C; continue 'dispatch;
            }
            0x82469D40 => {
    //   block [0x82469D40..0x82469D4C)
	// 82469D40: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82469D44: 4BFFA0FD  bl 0x82463e40
	ctx.lr = 0x82469D48;
	sub_82463E40(ctx, base);
	// 82469D48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x82469D4C; continue 'dispatch;
            }
            0x82469D4C => {
    //   block [0x82469D4C..0x82469D80)
	// 82469D4C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82469D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82469D54: 7D7F492E  stwx r11, r31, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82469D58: 994B0010  stb r10, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82469D5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82469D60: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82469D64: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82469D68: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82469D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469D78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82469D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469D90 size=64
    let mut pc: u32 = 0x82469D90;
    'dispatch: loop {
        match pc {
            0x82469D90 => {
    //   block [0x82469D90..0x82469DBC)
	// 82469D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82469D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469DA4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82469DAC: 419A0010  beq cr6, 0x82469dbc
	if ctx.cr[6].eq {
	pc = 0x82469DBC; continue 'dispatch;
	}
	// 82469DB0: 4BF56BA9  bl 0x823c0958
	ctx.lr = 0x82469DB4;
	sub_823C0958(ctx, base);
	// 82469DB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82469DB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82469DBC; continue 'dispatch;
            }
            0x82469DBC => {
    //   block [0x82469DBC..0x82469DD0)
	// 82469DBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469DC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82469DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469DD0 size=84
    let mut pc: u32 = 0x82469DD0;
    'dispatch: loop {
        match pc {
            0x82469DD0 => {
    //   block [0x82469DD0..0x82469E24)
	// 82469DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469DD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82469DDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469DE4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82469DE8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82469DEC: 391F0008  addi r8, r31, 8
	ctx.r[8].s64 = ctx.r[31].s64 + 8;
	// 82469DF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82469DF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82469DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82469DFC: 4BF57FD5  bl 0x823c1dd0
	ctx.lr = 0x82469E00;
	sub_823C1DD0(ctx, base);
	// 82469E00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82469E04: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82469E08: 5543DFFE  rlwinm r3, r10, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82469E0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82469E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469E1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82469E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469E28 size=84
    let mut pc: u32 = 0x82469E28;
    'dispatch: loop {
        match pc {
            0x82469E28 => {
    //   block [0x82469E28..0x82469E54)
	// 82469E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469E34: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469E38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82469E3C: 409A0018  bne cr6, 0x82469e54
	if !ctx.cr[6].eq {
	pc = 0x82469E54; continue 'dispatch;
	}
	// 82469E40: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82469E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469E50: 4E800020  blr
	return;
            }
            0x82469E54 => {
    //   block [0x82469E54..0x82469E7C)
	// 82469E54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82469E58: 4BF57CC9  bl 0x823c1b20
	ctx.lr = 0x82469E5C;
	sub_823C1B20(ctx, base);
	// 82469E5C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82469E60: 396BFEFD  addi r11, r11, -0x103
	ctx.r[11].s64 = ctx.r[11].s64 + -259;
	// 82469E64: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82469E68: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82469E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469E80 size=36
    let mut pc: u32 = 0x82469E80;
    'dispatch: loop {
        match pc {
            0x82469E80 => {
    //   block [0x82469E80..0x82469EA4)
	// 82469E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469E8C: 480DC1B5  bl 0x82546040
	ctx.lr = 0x82469E90;
	sub_82546040(ctx, base);
	// 82469E90: 78630020  clrldi r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82469E94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469EB8 size=64
    let mut pc: u32 = 0x82469EB8;
    'dispatch: loop {
        match pc {
            0x82469EB8 => {
    //   block [0x82469EB8..0x82469EE4)
	// 82469EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82469EC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82469ECC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469ED0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82469ED4: 419A0010  beq cr6, 0x82469ee4
	if ctx.cr[6].eq {
	pc = 0x82469EE4; continue 'dispatch;
	}
	// 82469ED8: 4BF56A81  bl 0x823c0958
	ctx.lr = 0x82469EDC;
	sub_823C0958(ctx, base);
	// 82469EDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82469EE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82469EE4; continue 'dispatch;
            }
            0x82469EE4 => {
    //   block [0x82469EE4..0x82469EF8)
	// 82469EE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469EF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82469EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469EF8 size=32
    let mut pc: u32 = 0x82469EF8;
    'dispatch: loop {
        match pc {
            0x82469EF8 => {
    //   block [0x82469EF8..0x82469F18)
	// 82469EF8: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82469EFC: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 82469F00: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82469F04: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 82469F08: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	// 82469F0C: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 82469F10: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82469F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469F18 size=32
    let mut pc: u32 = 0x82469F18;
    'dispatch: loop {
        match pc {
            0x82469F18 => {
    //   block [0x82469F18..0x82469F38)
	// 82469F18: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82469F1C: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82469F20: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82469F24: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82469F28: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
	// 82469F2C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82469F30: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82469F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469F38 size=8
    let mut pc: u32 = 0x82469F38;
    'dispatch: loop {
        match pc {
            0x82469F38 => {
    //   block [0x82469F38..0x82469F40)
	// 82469F38: 480CDFE0  b 0x82537f18
	sub_82537F18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469F40 size=68
    let mut pc: u32 = 0x82469F40;
    'dispatch: loop {
        match pc {
            0x82469F40 => {
    //   block [0x82469F40..0x82469F84)
	// 82469F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469F48: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82469F4C: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82469F50: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82469F54: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82469F58: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82469F5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469F60: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82469F64: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 82469F68: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82469F6C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82469F70: 480CDFA9  bl 0x82537f18
	ctx.lr = 0x82469F74;
	sub_82537F18(ctx, base);
	// 82469F74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82469F88 size=72
    let mut pc: u32 = 0x82469F88;
    'dispatch: loop {
        match pc {
            0x82469F88 => {
    //   block [0x82469F88..0x82469FD0)
	// 82469F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82469F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82469F90: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82469F94: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82469F98: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82469F9C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82469FA0: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82469FA4: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82469FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82469FAC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82469FB0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82469FB4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82469FB8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82469FBC: 480C8E5D  bl 0x82532e18
	ctx.lr = 0x82469FC0;
	sub_82532E18(ctx, base);
	// 82469FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82469FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82469FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82469FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82469FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82469FD0 size=44
    let mut pc: u32 = 0x82469FD0;
    'dispatch: loop {
        match pc {
            0x82469FD0 => {
    //   block [0x82469FD0..0x82469FD4)
	// 82469FD0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	pc = 0x82469FD4; continue 'dispatch;
            }
            0x82469FD4 => {
    //   block [0x82469FD4..0x82469FFC)
	// 82469FD4: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469FD8: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82469FDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82469FE0: 7C695850  subf r3, r9, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82469FE4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82469FE8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82469FEC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82469FF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82469FF4: 419AFFE0  beq cr6, 0x82469fd4
	if ctx.cr[6].eq {
	pc = 0x82469FD4; continue 'dispatch;
	}
	// 82469FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A000 size=8
    let mut pc: u32 = 0x8246A000;
    'dispatch: loop {
        match pc {
            0x8246A000 => {
    //   block [0x8246A000..0x8246A008)
	// 8246A000: 480C9190  b 0x82533190
	sub_82533190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A008 size=216
    let mut pc: u32 = 0x8246A008;
    'dispatch: loop {
        match pc {
            0x8246A008 => {
    //   block [0x8246A008..0x8246A00C)
	// 8246A008: 7CE41850  subf r7, r4, r3
	ctx.r[7].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	pc = 0x8246A00C; continue 'dispatch;
            }
            0x8246A00C => {
    //   block [0x8246A00C..0x8246A024)
	// 8246A00C: 7D4720AE  lbzx r10, r7, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8246A010: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246A014: 409A0010  bne cr6, 0x8246a024
	if !ctx.cr[6].eq {
	pc = 0x8246A024; continue 'dispatch;
	}
	// 8246A018: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A01C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A020: 419A00B8  beq cr6, 0x8246a0d8
	if ctx.cr[6].eq {
	pc = 0x8246A0D8; continue 'dispatch;
	}
	pc = 0x8246A024; continue 'dispatch;
            }
            0x8246A024 => {
    //   block [0x8246A024..0x8246A044)
	// 8246A024: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8246A028: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A02C: 41980018  blt cr6, 0x8246a044
	if ctx.cr[6].lt {
	pc = 0x8246A044; continue 'dispatch;
	}
	// 8246A030: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A034: 41990010  bgt cr6, 0x8246a044
	if ctx.cr[6].gt {
	pc = 0x8246A044; continue 'dispatch;
	}
	// 8246A038: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A03C: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 8246A040: 48000008  b 0x8246a048
	pc = 0x8246A048; continue 'dispatch;
            }
            0x8246A044 => {
    //   block [0x8246A044..0x8246A048)
	// 8246A044: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x8246A048; continue 'dispatch;
            }
            0x8246A048 => {
    //   block [0x8246A048..0x8246A06C)
	// 8246A048: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A04C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A050: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A054: 41980018  blt cr6, 0x8246a06c
	if ctx.cr[6].lt {
	pc = 0x8246A06C; continue 'dispatch;
	}
	// 8246A058: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A05C: 41990010  bgt cr6, 0x8246a06c
	if ctx.cr[6].gt {
	pc = 0x8246A06C; continue 'dispatch;
	}
	// 8246A060: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A064: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A068: 48000008  b 0x8246a070
	pc = 0x8246A070; continue 'dispatch;
            }
            0x8246A06C => {
    //   block [0x8246A06C..0x8246A070)
	// 8246A06C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x8246A070; continue 'dispatch;
            }
            0x8246A070 => {
    //   block [0x8246A070..0x8246A09C)
	// 8246A070: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A074: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 8246A078: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A07C: 41980064  blt cr6, 0x8246a0e0
	if ctx.cr[6].lt {
		sub_8246A0E0(ctx, base);
		return;
	}
	// 8246A080: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8246A084: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A088: 41980014  blt cr6, 0x8246a09c
	if ctx.cr[6].lt {
	pc = 0x8246A09C; continue 'dispatch;
	}
	// 8246A08C: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A090: 4199000C  bgt cr6, 0x8246a09c
	if ctx.cr[6].gt {
	pc = 0x8246A09C; continue 'dispatch;
	}
	// 8246A094: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A098: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	pc = 0x8246A09C; continue 'dispatch;
            }
            0x8246A09C => {
    //   block [0x8246A09C..0x8246A0BC)
	// 8246A09C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A0A0: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A0A4: 41980018  blt cr6, 0x8246a0bc
	if ctx.cr[6].lt {
	pc = 0x8246A0BC; continue 'dispatch;
	}
	// 8246A0A8: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A0AC: 41990010  bgt cr6, 0x8246a0bc
	if ctx.cr[6].gt {
	pc = 0x8246A0BC; continue 'dispatch;
	}
	// 8246A0B0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A0B4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A0B8: 48000008  b 0x8246a0c0
	pc = 0x8246A0C0; continue 'dispatch;
            }
            0x8246A0BC => {
    //   block [0x8246A0BC..0x8246A0C0)
	// 8246A0BC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x8246A0C0; continue 'dispatch;
            }
            0x8246A0C0 => {
    //   block [0x8246A0C0..0x8246A0D8)
	// 8246A0C0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A0C4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246A0C8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A0CC: 4199001C  bgt cr6, 0x8246a0e8
	if ctx.cr[6].gt {
		sub_8246A0E8(ctx, base);
		return;
	}
	// 8246A0D0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 8246A0D4: 4BFFFF38  b 0x8246a00c
	pc = 0x8246A00C; continue 'dispatch;
            }
            0x8246A0D8 => {
    //   block [0x8246A0D8..0x8246A0E0)
	// 8246A0D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246A0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A0E0 size=8
    let mut pc: u32 = 0x8246A0E0;
    'dispatch: loop {
        match pc {
            0x8246A0E0 => {
    //   block [0x8246A0E0..0x8246A0E8)
	// 8246A0E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246A0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A0E8 size=8
    let mut pc: u32 = 0x8246A0E8;
    'dispatch: loop {
        match pc {
            0x8246A0E8 => {
    //   block [0x8246A0E8..0x8246A0F0)
	// 8246A0E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246A0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A0F0 size=236
    let mut pc: u32 = 0x8246A0F0;
    'dispatch: loop {
        match pc {
            0x8246A0F0 => {
    //   block [0x8246A0F0..0x8246A0FC)
	// 8246A0F0: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 8246A0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8246A0F8: 7C841850  subf r4, r4, r3
	ctx.r[4].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	pc = 0x8246A0FC; continue 'dispatch;
            }
            0x8246A0FC => {
    //   block [0x8246A0FC..0x8246A114)
	// 8246A0FC: 7D4438AE  lbzx r10, r4, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8246A100: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246A104: 409A0010  bne cr6, 0x8246a114
	if !ctx.cr[6].eq {
	pc = 0x8246A114; continue 'dispatch;
	}
	// 8246A108: 89670000  lbz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A10C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A110: 419A00D4  beq cr6, 0x8246a1e4
	if ctx.cr[6].eq {
		sub_8246A1E4(ctx, base);
		return;
	}
	pc = 0x8246A114; continue 'dispatch;
            }
            0x8246A114 => {
    //   block [0x8246A114..0x8246A13C)
	// 8246A114: 7F062800  cmpw cr6, r6, r5
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8246A118: 409800CC  bge cr6, 0x8246a1e4
	if !ctx.cr[6].lt {
		sub_8246A1E4(ctx, base);
		return;
	}
	// 8246A11C: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8246A120: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A124: 41980018  blt cr6, 0x8246a13c
	if ctx.cr[6].lt {
	pc = 0x8246A13C; continue 'dispatch;
	}
	// 8246A128: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A12C: 41990010  bgt cr6, 0x8246a13c
	if ctx.cr[6].gt {
	pc = 0x8246A13C; continue 'dispatch;
	}
	// 8246A130: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A134: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 8246A138: 48000008  b 0x8246a140
	pc = 0x8246A140; continue 'dispatch;
            }
            0x8246A13C => {
    //   block [0x8246A13C..0x8246A140)
	// 8246A13C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x8246A140; continue 'dispatch;
            }
            0x8246A140 => {
    //   block [0x8246A140..0x8246A164)
	// 8246A140: 89270000  lbz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A144: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A148: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A14C: 41980018  blt cr6, 0x8246a164
	if ctx.cr[6].lt {
	pc = 0x8246A164; continue 'dispatch;
	}
	// 8246A150: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A154: 41990010  bgt cr6, 0x8246a164
	if ctx.cr[6].gt {
	pc = 0x8246A164; continue 'dispatch;
	}
	// 8246A158: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A15C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A160: 48000008  b 0x8246a168
	pc = 0x8246A168; continue 'dispatch;
            }
            0x8246A164 => {
    //   block [0x8246A164..0x8246A168)
	// 8246A164: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x8246A168; continue 'dispatch;
            }
            0x8246A168 => {
    //   block [0x8246A168..0x8246A194)
	// 8246A168: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A16C: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 8246A170: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A174: 41980060  blt cr6, 0x8246a1d4
	if ctx.cr[6].lt {
	pc = 0x8246A1D4; continue 'dispatch;
	}
	// 8246A178: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 8246A17C: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A180: 41980014  blt cr6, 0x8246a194
	if ctx.cr[6].lt {
	pc = 0x8246A194; continue 'dispatch;
	}
	// 8246A184: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A188: 4199000C  bgt cr6, 0x8246a194
	if ctx.cr[6].gt {
	pc = 0x8246A194; continue 'dispatch;
	}
	// 8246A18C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A190: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	pc = 0x8246A194; continue 'dispatch;
            }
            0x8246A194 => {
    //   block [0x8246A194..0x8246A1B4)
	// 8246A194: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A198: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A19C: 41980018  blt cr6, 0x8246a1b4
	if ctx.cr[6].lt {
	pc = 0x8246A1B4; continue 'dispatch;
	}
	// 8246A1A0: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A1A4: 41990010  bgt cr6, 0x8246a1b4
	if ctx.cr[6].gt {
	pc = 0x8246A1B4; continue 'dispatch;
	}
	// 8246A1A8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A1AC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A1B0: 48000008  b 0x8246a1b8
	pc = 0x8246A1B8; continue 'dispatch;
            }
            0x8246A1B4 => {
    //   block [0x8246A1B4..0x8246A1B8)
	// 8246A1B4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	pc = 0x8246A1B8; continue 'dispatch;
            }
            0x8246A1B8 => {
    //   block [0x8246A1B8..0x8246A1D4)
	// 8246A1B8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8246A1BC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246A1C0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A1C4: 41990018  bgt cr6, 0x8246a1dc
	if ctx.cr[6].gt {
		sub_8246A1DC(ctx, base);
		return;
	}
	// 8246A1C8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8246A1CC: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8246A1D0: 4BFFFF2C  b 0x8246a0fc
	pc = 0x8246A0FC; continue 'dispatch;
            }
            0x8246A1D4 => {
    //   block [0x8246A1D4..0x8246A1DC)
	// 8246A1D4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246A1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A1DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A1DC size=8
    let mut pc: u32 = 0x8246A1DC;
    'dispatch: loop {
        match pc {
            0x8246A1DC => {
    //   block [0x8246A1DC..0x8246A1E4)
	// 8246A1DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8246A1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A1E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A1E4 size=8
    let mut pc: u32 = 0x8246A1E4;
    'dispatch: loop {
        match pc {
            0x8246A1E4 => {
    //   block [0x8246A1E4..0x8246A1EC)
	// 8246A1E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246A1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A1F0 size=28
    let mut pc: u32 = 0x8246A1F0;
    'dispatch: loop {
        match pc {
            0x8246A1F0 => {
    //   block [0x8246A1F0..0x8246A1F4)
	// 8246A1F0: 7D441850  subf r10, r4, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	pc = 0x8246A1F4; continue 'dispatch;
            }
            0x8246A1F4 => {
    //   block [0x8246A1F4..0x8246A20C)
	// 8246A1F4: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A1F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A1FC: 7D6A21AE  stbx r11, r10, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[11].u8) };
	// 8246A200: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 8246A204: 409AFFF0  bne cr6, 0x8246a1f4
	if !ctx.cr[6].eq {
	pc = 0x8246A1F4; continue 'dispatch;
	}
	// 8246A208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A210 size=16
    let mut pc: u32 = 0x8246A210;
    'dispatch: loop {
        match pc {
            0x8246A210 => {
    //   block [0x8246A210..0x8246A220)
	// 8246A210: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8246A214: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246A218: 480C89A8  b 0x82532bc0
	sub_82532BC0(ctx, base);
	return;
	// 8246A21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A220 size=36
    let mut pc: u32 = 0x8246A220;
    'dispatch: loop {
        match pc {
            0x8246A220 => {
    //   block [0x8246A220..0x8246A224)
	// 8246A220: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	pc = 0x8246A224; continue 'dispatch;
            }
            0x8246A224 => {
    //   block [0x8246A224..0x8246A244)
	// 8246A224: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A228: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8246A22C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246A230: 409AFFF4  bne cr6, 0x8246a224
	if !ctx.cr[6].eq {
	pc = 0x8246A224; continue 'dispatch;
	}
	// 8246A234: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8246A238: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246A23C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8246A240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A248 size=16
    let mut pc: u32 = 0x8246A248;
    'dispatch: loop {
        match pc {
            0x8246A248 => {
    //   block [0x8246A248..0x8246A258)
	// 8246A248: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8246A24C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8246A250: 480C9E60  b 0x825340b0
	sub_825340B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246A258 size=40
    let mut pc: u32 = 0x8246A258;
    'dispatch: loop {
        match pc {
            0x8246A258 => {
    //   block [0x8246A258..0x8246A280)
	// 8246A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246A260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246A264: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8246A268: 480CDE61  bl 0x825380c8
	ctx.lr = 0x8246A26C;
	sub_825380C8(ctx, base);
	// 8246A26C: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8246A270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246A274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246A278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246A27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A280 size=8
    let mut pc: u32 = 0x8246A280;
    'dispatch: loop {
        match pc {
            0x8246A280 => {
    //   block [0x8246A280..0x8246A288)
	// 8246A280: 480CA438  b 0x825346b8
	sub_825346B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A288 size=8
    let mut pc: u32 = 0x8246A288;
    'dispatch: loop {
        match pc {
            0x8246A288 => {
    //   block [0x8246A288..0x8246A290)
	// 8246A288: 480C90E8  b 0x82533370
	sub_82533370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A290 size=8
    let mut pc: u32 = 0x8246A290;
    'dispatch: loop {
        match pc {
            0x8246A290 => {
    //   block [0x8246A290..0x8246A298)
	// 8246A290: 480CDE40  b 0x825380d0
	sub_825380D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A298 size=72
    let mut pc: u32 = 0x8246A298;
    'dispatch: loop {
        match pc {
            0x8246A298 => {
    //   block [0x8246A298..0x8246A2A8)
	// 8246A298: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A29C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A2A0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246A2A4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	pc = 0x8246A2A8; continue 'dispatch;
            }
            0x8246A2A8 => {
    //   block [0x8246A2A8..0x8246A2C8)
	// 8246A2A8: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A2AC: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A2B0: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 8246A2B4: 41980014  blt cr6, 0x8246a2c8
	if ctx.cr[6].lt {
	pc = 0x8246A2C8; continue 'dispatch;
	}
	// 8246A2B8: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 8246A2BC: 4199000C  bgt cr6, 0x8246a2c8
	if ctx.cr[6].gt {
	pc = 0x8246A2C8; continue 'dispatch;
	}
	// 8246A2C0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 8246A2C4: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	pc = 0x8246A2C8; continue 'dispatch;
            }
            0x8246A2C8 => {
    //   block [0x8246A2C8..0x8246A2E0)
	// 8246A2C8: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246A2CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246A2D0: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A2D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A2D8: 409AFFD0  bne cr6, 0x8246a2a8
	if !ctx.cr[6].eq {
	pc = 0x8246A2A8; continue 'dispatch;
	}
	// 8246A2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A2E0 size=72
    let mut pc: u32 = 0x8246A2E0;
    'dispatch: loop {
        match pc {
            0x8246A2E0 => {
    //   block [0x8246A2E0..0x8246A2F0)
	// 8246A2E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A2E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A2E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246A2EC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	pc = 0x8246A2F0; continue 'dispatch;
            }
            0x8246A2F0 => {
    //   block [0x8246A2F0..0x8246A310)
	// 8246A2F0: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A2F4: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 8246A2F8: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 8246A2FC: 41980014  blt cr6, 0x8246a310
	if ctx.cr[6].lt {
	pc = 0x8246A310; continue 'dispatch;
	}
	// 8246A300: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 8246A304: 4199000C  bgt cr6, 0x8246a310
	if ctx.cr[6].gt {
	pc = 0x8246A310; continue 'dispatch;
	}
	// 8246A308: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8246A30C: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	pc = 0x8246A310; continue 'dispatch;
            }
            0x8246A310 => {
    //   block [0x8246A310..0x8246A328)
	// 8246A310: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246A314: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246A318: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A31C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8246A320: 409AFFD0  bne cr6, 0x8246a2f0
	if !ctx.cr[6].eq {
	pc = 0x8246A2F0; continue 'dispatch;
	}
	// 8246A324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A328 size=8
    let mut pc: u32 = 0x8246A328;
    'dispatch: loop {
        match pc {
            0x8246A328 => {
    //   block [0x8246A328..0x8246A330)
	// 8246A328: 480CA828  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A330 size=8
    let mut pc: u32 = 0x8246A330;
    'dispatch: loop {
        match pc {
            0x8246A330 => {
    //   block [0x8246A330..0x8246A338)
	// 8246A330: 480CB040  b 0x82535370
	sub_82535370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A338 size=8
    let mut pc: u32 = 0x8246A338;
    'dispatch: loop {
        match pc {
            0x8246A338 => {
    //   block [0x8246A338..0x8246A340)
	// 8246A338: 480CAE98  b 0x825351d0
	sub_825351D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A340 size=60
    let mut pc: u32 = 0x8246A340;
    'dispatch: loop {
        match pc {
            0x8246A340 => {
    //   block [0x8246A340..0x8246A358)
	// 8246A340: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8246A344: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8246A348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8246A34C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8246A350: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8246A354: 7D2B2A14  add r9, r11, r5
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	pc = 0x8246A358; continue 'dispatch;
            }
            0x8246A358 => {
    //   block [0x8246A358..0x8246A37C)
	// 8246A358: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A35C: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A360: 7C674051  subf. r3, r7, r8
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246A364: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
	// 8246A368: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A36C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8246A370: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246A374: 409AFFE4  bne cr6, 0x8246a358
	if !ctx.cr[6].eq {
	pc = 0x8246A358; continue 'dispatch;
	}
	// 8246A378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246A380 size=128
    let mut pc: u32 = 0x8246A380;
    'dispatch: loop {
        match pc {
            0x8246A380 => {
    //   block [0x8246A380..0x8246A39C)
	// 8246A380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246A384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246A388: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246A38C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246A390: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246A394: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246A398: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x8246A39C; continue 'dispatch;
            }
            0x8246A39C => {
    //   block [0x8246A39C..0x8246A3D8)
	// 8246A39C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A3A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A3A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246A3A8: 409AFFF4  bne cr6, 0x8246a39c
	if !ctx.cr[6].eq {
	pc = 0x8246A39C; continue 'dispatch;
	}
	// 8246A3AC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246A3B0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A3B4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246A3B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246A3BC: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246A3C0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A3C4: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 8246A3C8: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8246A3CC: 4BFF9E9D  bl 0x82464268
	ctx.lr = 0x8246A3D0;
	sub_82464268(ctx, base);
	// 8246A3D0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246A3D4: 7D3F1850  subf r9, r31, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	pc = 0x8246A3D8; continue 'dispatch;
            }
            0x8246A3D8 => {
    //   block [0x8246A3D8..0x8246A400)
	// 8246A3D8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A3DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246A3E0: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 8246A3E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A3E8: 409AFFF0  bne cr6, 0x8246a3d8
	if !ctx.cr[6].eq {
	pc = 0x8246A3D8; continue 'dispatch;
	}
	// 8246A3EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8246A3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246A3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246A3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246A3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246A400 size=132
    let mut pc: u32 = 0x8246A400;
    'dispatch: loop {
        match pc {
            0x8246A400 => {
    //   block [0x8246A400..0x8246A418)
	// 8246A400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246A404: 480CACB9  bl 0x825350bc
	ctx.lr = 0x8246A408;
	sub_82535080(ctx, base);
	// 8246A408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246A40C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246A410: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8246A414: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x8246A418; continue 'dispatch;
            }
            0x8246A418 => {
    //   block [0x8246A418..0x8246A440)
	// 8246A418: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A41C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A420: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246A424: 409AFFF4  bne cr6, 0x8246a418
	if !ctx.cr[6].eq {
	pc = 0x8246A418; continue 'dispatch;
	}
	// 8246A428: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246A42C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246A430: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8246A434: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8246A438: 40990008  ble cr6, 0x8246a440
	if !ctx.cr[6].gt {
	pc = 0x8246A440; continue 'dispatch;
	}
	// 8246A43C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	pc = 0x8246A440; continue 'dispatch;
            }
            0x8246A440 => {
    //   block [0x8246A440..0x8246A470)
	// 8246A440: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A444: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246A448: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246A44C: 389F0001  addi r4, r31, 1
	ctx.r[4].s64 = ctx.r[31].s64 + 1;
	// 8246A450: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246A454: 4BFF9E15  bl 0x82464268
	ctx.lr = 0x8246A458;
	sub_82464268(ctx, base);
	// 8246A458: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246A45C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8246A460: 419A0010  beq cr6, 0x8246a470
	if ctx.cr[6].eq {
	pc = 0x8246A470; continue 'dispatch;
	}
	// 8246A464: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8246A468: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246A46C: 480C8755  bl 0x82532bc0
	ctx.lr = 0x8246A470;
	sub_82532BC0(ctx, base);
	pc = 0x8246A470; continue 'dispatch;
            }
            0x8246A470 => {
    //   block [0x8246A470..0x8246A484)
	// 8246A470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A474: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246A478: 7D7EF9AE  stbx r11, r30, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 8246A47C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246A480: 480CAC8C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246A488 size=216
    let mut pc: u32 = 0x8246A488;
    'dispatch: loop {
        match pc {
            0x8246A488 => {
    //   block [0x8246A488..0x8246A4B0)
	// 8246A488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246A48C: 480CAC19  bl 0x825350a4
	ctx.lr = 0x8246A490;
	sub_82535080(ctx, base);
	// 8246A490: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246A494: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8246A498: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246A49C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 8246A4A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8246A4A4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8246A4A8: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 8246A4AC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x8246A4B0; continue 'dispatch;
            }
            0x8246A4B0 => {
    //   block [0x8246A4B0..0x8246A4F0)
	// 8246A4B0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A4B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A4B8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246A4BC: 409AFFF4  bne cr6, 0x8246a4b0
	if !ctx.cr[6].eq {
	pc = 0x8246A4B0; continue 'dispatch;
	}
	// 8246A4C0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246A4C4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246A4C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246A4CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8246A4D0: 5578003E  slwi r24, r11, 0
	ctx.r[24].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 8246A4D4: 480CA1E5  bl 0x825346b8
	ctx.lr = 0x8246A4D8;
	sub_825346B8(ctx, base);
	// 8246A4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A4DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246A4E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8246A4E4: 997C0000  stb r11, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246A4E8: 419A006C  beq cr6, 0x8246a554
	if ctx.cr[6].eq {
	pc = 0x8246A554; continue 'dispatch;
	}
	// 8246A4EC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	pc = 0x8246A4F0; continue 'dispatch;
            }
            0x8246A4F0 => {
    //   block [0x8246A4F0..0x8246A518)
	// 8246A4F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A4F4: 7FBBF050  subf r29, r27, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[27].s64;
	// 8246A4F8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A4FC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A500: 9B3C0000  stb r25, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 8246A504: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A508: 409A0010  bne cr6, 0x8246a518
	if !ctx.cr[6].eq {
	pc = 0x8246A518; continue 'dispatch;
	}
	// 8246A50C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8246A510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246A514: 48003E3D  bl 0x8246e350
	ctx.lr = 0x8246A518;
	sub_8246E350(ctx, base);
	pc = 0x8246A518; continue 'dispatch;
            }
            0x8246A518 => {
    //   block [0x8246A518..0x8246A554)
	// 8246A518: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A51C: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8246A520: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A524: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A528: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 8246A52C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A530: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A534: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246A538: 419A001C  beq cr6, 0x8246a554
	if ctx.cr[6].eq {
	pc = 0x8246A554; continue 'dispatch;
	}
	// 8246A53C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8246A540: 7C7EC214  add r3, r30, r24
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 8246A544: 480CA175  bl 0x825346b8
	ctx.lr = 0x8246A548;
	sub_825346B8(ctx, base);
	// 8246A548: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246A54C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8246A550: 409AFFA0  bne cr6, 0x8246a4f0
	if !ctx.cr[6].eq {
	pc = 0x8246A4F0; continue 'dispatch;
	}
	pc = 0x8246A554; continue 'dispatch;
            }
            0x8246A554 => {
    //   block [0x8246A554..0x8246A560)
	// 8246A554: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8246A558: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8246A55C: 480CAB98  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246A560 size=344
    let mut pc: u32 = 0x8246A560;
    'dispatch: loop {
        match pc {
            0x8246A560 => {
    //   block [0x8246A560..0x8246A5B4)
	// 8246A560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246A564: 480CAB55  bl 0x825350b8
	ctx.lr = 0x8246A568;
	sub_82535080(ctx, base);
	// 8246A568: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 8246A56C: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 8246A570: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 8246A574: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 8246A578: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 8246A57C: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 8246A580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246A584: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246A588: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8246A58C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A590: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A594: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 8246A598: 4098002C  bge cr6, 0x8246a5c4
	if !ctx.cr[6].lt {
	pc = 0x8246A5C4; continue 'dispatch;
	}
	// 8246A59C: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 8246A5A0: 40980024  bge cr6, 0x8246a5c4
	if !ctx.cr[6].lt {
	pc = 0x8246A5C4; continue 'dispatch;
	}
	// 8246A5A4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A5A8: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 8246A5AC: 41990008  bgt cr6, 0x8246a5b4
	if ctx.cr[6].gt {
	pc = 0x8246A5B4; continue 'dispatch;
	}
	// 8246A5B0: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	pc = 0x8246A5B4; continue 'dispatch;
            }
            0x8246A5B4 => {
    //   block [0x8246A5B4..0x8246A5C4)
	// 8246A5B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A5B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A5BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246A5C0: 48003D09  bl 0x8246e2c8
	ctx.lr = 0x8246A5C4;
	sub_8246E2C8(ctx, base);
	pc = 0x8246A5C4; continue 'dispatch;
            }
            0x8246A5C4 => {
    //   block [0x8246A5C4..0x8246A5D4)
	// 8246A5C4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8246A5C8: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 8246A5CC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8246A5D0: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	pc = 0x8246A5D4; continue 'dispatch;
            }
            0x8246A5D4 => {
    //   block [0x8246A5D4..0x8246A624)
	// 8246A5D4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A5D8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8246A5DC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8246A5E0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A5E4: 557F00BE  clrlwi r31, r11, 2
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A5E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8246A5EC: 480CD92D  bl 0x82537f18
	ctx.lr = 0x8246A5F0;
	sub_82537F18(ctx, base);
	// 8246A5F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8246A5F4: 41980048  blt cr6, 0x8246a63c
	if ctx.cr[6].lt {
	pc = 0x8246A63C; continue 'dispatch;
	}
	// 8246A5F8: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246A5FC: 4198008C  blt cr6, 0x8246a688
	if ctx.cr[6].lt {
	pc = 0x8246A688; continue 'dispatch;
	}
	// 8246A600: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A604: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 8246A608: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A60C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246A610: 40980024  bge cr6, 0x8246a634
	if !ctx.cr[6].lt {
	pc = 0x8246A634; continue 'dispatch;
	}
	// 8246A614: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A618: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A61C: 41980008  blt cr6, 0x8246a624
	if ctx.cr[6].lt {
	pc = 0x8246A624; continue 'dispatch;
	}
	// 8246A620: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x8246A624; continue 'dispatch;
            }
            0x8246A624 => {
    //   block [0x8246A624..0x8246A634)
	// 8246A624: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A628: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246A630: 48003C99  bl 0x8246e2c8
	ctx.lr = 0x8246A634;
	sub_8246E2C8(ctx, base);
	pc = 0x8246A634; continue 'dispatch;
            }
            0x8246A634 => {
    //   block [0x8246A634..0x8246A63C)
	// 8246A634: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8246A638: 4BFFFF9C  b 0x8246a5d4
	pc = 0x8246A5D4; continue 'dispatch;
            }
            0x8246A63C => {
    //   block [0x8246A63C..0x8246A64C)
	// 8246A63C: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A640: 2F0B00FF  cmpwi cr6, r11, 0xff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 255, &mut ctx.xer);
	// 8246A644: 41990008  bgt cr6, 0x8246a64c
	if ctx.cr[6].gt {
	pc = 0x8246A64C; continue 'dispatch;
	}
	// 8246A648: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	pc = 0x8246A64C; continue 'dispatch;
            }
            0x8246A64C => {
    //   block [0x8246A64C..0x8246A688)
	// 8246A64C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A650: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 8246A654: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A658: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8246A65C: 4098FFD8  bge cr6, 0x8246a634
	if !ctx.cr[6].lt {
	pc = 0x8246A634; continue 'dispatch;
	}
	// 8246A660: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A664: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A668: 4198FFBC  blt cr6, 0x8246a624
	if ctx.cr[6].lt {
	pc = 0x8246A624; continue 'dispatch;
	}
	// 8246A66C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8246A670: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A674: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A678: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246A67C: 48003C4D  bl 0x8246e2c8
	ctx.lr = 0x8246A680;
	sub_8246E2C8(ctx, base);
	// 8246A680: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8246A684: 4BFFFF50  b 0x8246a5d4
	pc = 0x8246A5D4; continue 'dispatch;
            }
            0x8246A688 => {
    //   block [0x8246A688..0x8246A6B0)
	// 8246A688: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A68C: 38C30001  addi r6, r3, 1
	ctx.r[6].s64 = ctx.r[3].s64 + 1;
	// 8246A690: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A694: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A698: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8246A69C: 41990014  bgt cr6, 0x8246a6b0
	if ctx.cr[6].gt {
	pc = 0x8246A6B0; continue 'dispatch;
	}
	// 8246A6A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8246A6A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8246A6A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246A6AC: 48003D3D  bl 0x8246e3e8
	ctx.lr = 0x8246A6B0;
	sub_8246E3E8(ctx, base);
	pc = 0x8246A6B0; continue 'dispatch;
            }
            0x8246A6B0 => {
    //   block [0x8246A6B0..0x8246A6B8)
	// 8246A6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246A6B4: 480CAA54  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A6B8 size=72
    let mut pc: u32 = 0x8246A6B8;
    'dispatch: loop {
        match pc {
            0x8246A6B8 => {
    //   block [0x8246A6B8..0x8246A6CC)
	// 8246A6B8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A6BC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8246A6C0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246A6C4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246A6C8: 40980030  bge cr6, 0x8246a6f8
	if !ctx.cr[6].lt {
	pc = 0x8246A6F8; continue 'dispatch;
	}
	pc = 0x8246A6CC; continue 'dispatch;
            }
            0x8246A6CC => {
    //   block [0x8246A6CC..0x8246A6F8)
	// 8246A6CC: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 8246A6D0: 40980028  bge cr6, 0x8246a6f8
	if !ctx.cr[6].lt {
	pc = 0x8246A6F8; continue 'dispatch;
	}
	// 8246A6D4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A6D8: 7C880774  extsb r8, r4
	ctx.r[8].s64 = ctx.r[4].s8 as i64;
	// 8246A6DC: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246A6E0: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8246A6E4: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8246A6E8: 419A0018  beq cr6, 0x8246a700
	if ctx.cr[6].eq {
		sub_8246A700(ctx, base);
		return;
	}
	// 8246A6EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A6F0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246A6F4: 4198FFD8  blt cr6, 0x8246a6cc
	if ctx.cr[6].lt {
	pc = 0x8246A6CC; continue 'dispatch;
	}
	pc = 0x8246A6F8; continue 'dispatch;
            }
            0x8246A6F8 => {
    //   block [0x8246A6F8..0x8246A700)
	// 8246A6F8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246A6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A700 size=8
    let mut pc: u32 = 0x8246A700;
    'dispatch: loop {
        match pc {
            0x8246A700 => {
    //   block [0x8246A700..0x8246A708)
	// 8246A700: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8246A704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A708 size=76
    let mut pc: u32 = 0x8246A708;
    'dispatch: loop {
        match pc {
            0x8246A708 => {
    //   block [0x8246A708..0x8246A71C)
	// 8246A708: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A70C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246A710: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A714: 40990008  ble cr6, 0x8246a71c
	if !ctx.cr[6].gt {
	pc = 0x8246A71C; continue 'dispatch;
	}
	// 8246A718: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	pc = 0x8246A71C; continue 'dispatch;
            }
            0x8246A71C => {
    //   block [0x8246A71C..0x8246A730)
	// 8246A71C: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 8246A720: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8246A724: 41980028  blt cr6, 0x8246a74c
	if ctx.cr[6].lt {
	pc = 0x8246A74C; continue 'dispatch;
	}
	// 8246A728: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A72C: 7C890774  extsb r9, r4
	ctx.r[9].s64 = ctx.r[4].s8 as i64;
	pc = 0x8246A730; continue 'dispatch;
            }
            0x8246A730 => {
    //   block [0x8246A730..0x8246A74C)
	// 8246A730: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246A734: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 8246A738: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246A73C: 419A0018  beq cr6, 0x8246a754
	if ctx.cr[6].eq {
		sub_8246A754(ctx, base);
		return;
	}
	// 8246A740: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246A744: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8246A748: 4098FFE8  bge cr6, 0x8246a730
	if !ctx.cr[6].lt {
	pc = 0x8246A730; continue 'dispatch;
	}
	pc = 0x8246A74C; continue 'dispatch;
            }
            0x8246A74C => {
    //   block [0x8246A74C..0x8246A754)
	// 8246A74C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8246A750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A754 size=8
    let mut pc: u32 = 0x8246A754;
    'dispatch: loop {
        match pc {
            0x8246A754 => {
    //   block [0x8246A754..0x8246A75C)
	// 8246A754: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8246A758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246A760 size=128
    let mut pc: u32 = 0x8246A760;
    'dispatch: loop {
        match pc {
            0x8246A760 => {
    //   block [0x8246A760..0x8246A7AC)
	// 8246A760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246A764: 480CA951  bl 0x825350b4
	ctx.lr = 0x8246A768;
	sub_82535080(ctx, base);
	// 8246A768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246A76C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246A770: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8246A774: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A778: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A77C: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 8246A780: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A784: 3B6AFFFF  addi r27, r10, -1
	ctx.r[27].s64 = ctx.r[10].s64 + -1;
	// 8246A788: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A78C: 7D5BE214  add r10, r27, r28
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 8246A790: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 8246A794: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246A798: 40980024  bge cr6, 0x8246a7bc
	if !ctx.cr[6].lt {
	pc = 0x8246A7BC; continue 'dispatch;
	}
	// 8246A79C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A7A0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A7A4: 41980008  blt cr6, 0x8246a7ac
	if ctx.cr[6].lt {
	pc = 0x8246A7AC; continue 'dispatch;
	}
	// 8246A7A8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8246A7AC; continue 'dispatch;
            }
            0x8246A7AC => {
    //   block [0x8246A7AC..0x8246A7BC)
	// 8246A7AC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A7B0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A7B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246A7B8: 48003B11  bl 0x8246e2c8
	ctx.lr = 0x8246A7BC;
	sub_8246E2C8(ctx, base);
	pc = 0x8246A7BC; continue 'dispatch;
            }
            0x8246A7BC => {
    //   block [0x8246A7BC..0x8246A7E0)
	// 8246A7BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A7C0: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 8246A7C4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8246A7C8: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8246A7CC: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A7D0: 480CA381  bl 0x82534b50
	ctx.lr = 0x8246A7D4;
	sub_82534B50(ctx, base);
	// 8246A7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246A7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246A7DC: 480CA928  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246A7E0 size=156
    let mut pc: u32 = 0x8246A7E0;
    'dispatch: loop {
        match pc {
            0x8246A7E0 => {
    //   block [0x8246A7E0..0x8246A804)
	// 8246A7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246A7E4: 480CA8D1  bl 0x825350b4
	ctx.lr = 0x8246A7E8;
	sub_82535080(ctx, base);
	// 8246A7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246A7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246A7F0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8246A7F4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8246A7F8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A7FC: 3B8AFFFF  addi r28, r10, -1
	ctx.r[28].s64 = ctx.r[10].s64 + -1;
	// 8246A800: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x8246A804; continue 'dispatch;
            }
            0x8246A804 => {
    //   block [0x8246A804..0x8246A848)
	// 8246A804: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A808: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A80C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246A810: 409AFFF4  bne cr6, 0x8246a804
	if !ctx.cr[6].eq {
	pc = 0x8246A804; continue 'dispatch;
	}
	// 8246A814: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246A818: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8246A81C: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8246A820: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8246A824: 553D003E  slwi r29, r9, 0
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8246A828: 7D5DE214  add r10, r29, r28
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 8246A82C: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 8246A830: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8246A834: 40980024  bge cr6, 0x8246a858
	if !ctx.cr[6].lt {
	pc = 0x8246A858; continue 'dispatch;
	}
	// 8246A838: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246A83C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246A840: 41980008  blt cr6, 0x8246a848
	if ctx.cr[6].lt {
	pc = 0x8246A848; continue 'dispatch;
	}
	// 8246A844: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8246A848; continue 'dispatch;
            }
            0x8246A848 => {
    //   block [0x8246A848..0x8246A858)
	// 8246A848: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8246A84C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8246A850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246A854: 48003A75  bl 0x8246e2c8
	ctx.lr = 0x8246A858;
	sub_8246E2C8(ctx, base);
	pc = 0x8246A858; continue 'dispatch;
            }
            0x8246A858 => {
    //   block [0x8246A858..0x8246A87C)
	// 8246A858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A85C: 38BD0001  addi r5, r29, 1
	ctx.r[5].s64 = ctx.r[29].s64 + 1;
	// 8246A860: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8246A864: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8246A868: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8246A86C: 480CA2E5  bl 0x82534b50
	ctx.lr = 0x8246A870;
	sub_82534B50(ctx, base);
	// 8246A870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246A874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8246A878: 480CA88C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A880 size=84
    let mut pc: u32 = 0x8246A880;
    'dispatch: loop {
        match pc {
            0x8246A880 => {
    //   block [0x8246A880..0x8246A890)
	// 8246A880: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A888: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246A88C: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
	pc = 0x8246A890; continue 'dispatch;
            }
            0x8246A890 => {
    //   block [0x8246A890..0x8246A8B8)
	// 8246A890: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A894: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246A898: 89280000  lbz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A89C: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 8246A8A0: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 8246A8A4: 41980014  blt cr6, 0x8246a8b8
	if ctx.cr[6].lt {
	pc = 0x8246A8B8; continue 'dispatch;
	}
	// 8246A8A8: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 8246A8AC: 4199000C  bgt cr6, 0x8246a8b8
	if ctx.cr[6].gt {
	pc = 0x8246A8B8; continue 'dispatch;
	}
	// 8246A8B0: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8246A8B4: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	pc = 0x8246A8B8; continue 'dispatch;
            }
            0x8246A8B8 => {
    //   block [0x8246A8B8..0x8246A8D4)
	// 8246A8B8: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246A8BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A8C0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A8C4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246A8C8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246A8CC: 4198FFC4  blt cr6, 0x8246a890
	if ctx.cr[6].lt {
	pc = 0x8246A890; continue 'dispatch;
	}
	// 8246A8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A8D8 size=84
    let mut pc: u32 = 0x8246A8D8;
    'dispatch: loop {
        match pc {
            0x8246A8D8 => {
    //   block [0x8246A8D8..0x8246A8E8)
	// 8246A8D8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A8DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A8E0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246A8E4: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
	pc = 0x8246A8E8; continue 'dispatch;
            }
            0x8246A8E8 => {
    //   block [0x8246A8E8..0x8246A910)
	// 8246A8E8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A8EC: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8246A8F0: 89280000  lbz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A8F4: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 8246A8F8: 2F0A0041  cmpwi cr6, r10, 0x41
	ctx.cr[6].compare_i32(ctx.r[10].s32, 65, &mut ctx.xer);
	// 8246A8FC: 41980014  blt cr6, 0x8246a910
	if ctx.cr[6].lt {
	pc = 0x8246A910; continue 'dispatch;
	}
	// 8246A900: 2F0A005A  cmpwi cr6, r10, 0x5a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 90, &mut ctx.xer);
	// 8246A904: 4199000C  bgt cr6, 0x8246a910
	if ctx.cr[6].gt {
	pc = 0x8246A910; continue 'dispatch;
	}
	// 8246A908: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 8246A90C: 7D490774  extsb r9, r10
	ctx.r[9].s64 = ctx.r[10].s8 as i64;
	pc = 0x8246A910; continue 'dispatch;
            }
            0x8246A910 => {
    //   block [0x8246A910..0x8246A92C)
	// 8246A910: 99280000  stb r9, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246A914: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A918: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A91C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246A920: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246A924: 4198FFC4  blt cr6, 0x8246a8e8
	if ctx.cr[6].lt {
	pc = 0x8246A8E8; continue 'dispatch;
	}
	// 8246A928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A930 size=80
    let mut pc: u32 = 0x8246A930;
    'dispatch: loop {
        match pc {
            0x8246A930 => {
    //   block [0x8246A930..0x8246A948)
	// 8246A930: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A934: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A938: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8246A93C: 419A0038  beq cr6, 0x8246a974
	if ctx.cr[6].eq {
	pc = 0x8246A974; continue 'dispatch;
	}
	// 8246A940: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A944: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	pc = 0x8246A948; continue 'dispatch;
            }
            0x8246A948 => {
    //   block [0x8246A948..0x8246A974)
	// 8246A948: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246A94C: 40980034  bge cr6, 0x8246a980
	if !ctx.cr[6].lt {
		sub_8246A980(ctx, base);
		return;
	}
	// 8246A950: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A954: 7D0B28AE  lbzx r8, r11, r5
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8246A958: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246A95C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8246A960: 409A0020  bne cr6, 0x8246a980
	if !ctx.cr[6].eq {
		sub_8246A980(ctx, base);
		return;
	}
	// 8246A964: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A968: 7D2B28AE  lbzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8246A96C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246A970: 409AFFD8  bne cr6, 0x8246a948
	if !ctx.cr[6].eq {
	pc = 0x8246A948; continue 'dispatch;
	}
	pc = 0x8246A974; continue 'dispatch;
            }
            0x8246A974 => {
    //   block [0x8246A974..0x8246A980)
	// 8246A974: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246A978: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246A97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A980 size=12
    let mut pc: u32 = 0x8246A980;
    'dispatch: loop {
        match pc {
            0x8246A980 => {
    //   block [0x8246A980..0x8246A98C)
	// 8246A980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A984: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246A988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A990 size=36
    let mut pc: u32 = 0x8246A990;
    'dispatch: loop {
        match pc {
            0x8246A990 => {
    //   block [0x8246A990..0x8246A9B4)
	// 8246A990: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A994: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246A998: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 8246A99C: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 8246A9A0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8246A9A4: 40980010  bge cr6, 0x8246a9b4
	if !ctx.cr[6].lt {
		sub_8246A9B4(ctx, base);
		return;
	}
	// 8246A9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A9AC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246A9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A9B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A9B4 size=68
    let mut pc: u32 = 0x8246A9B4;
    'dispatch: loop {
        match pc {
            0x8246A9B4 => {
    //   block [0x8246A9B4..0x8246A9D0)
	// 8246A9B4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246A9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246A9BC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8246A9C0: 4099002C  ble cr6, 0x8246a9ec
	if !ctx.cr[6].gt {
	pc = 0x8246A9EC; continue 'dispatch;
	}
	// 8246A9C4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A9C8: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246A9CC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	pc = 0x8246A9D0; continue 'dispatch;
            }
            0x8246A9D0 => {
    //   block [0x8246A9D0..0x8246A9EC)
	// 8246A9D0: 7D2758AE  lbzx r9, r7, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246A9D4: 7CCA58AE  lbzx r6, r10, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246A9D8: 7F064840  cmplw cr6, r6, r9
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8246A9DC: 409AFFCC  bne cr6, 0x8246a9a8
	if !ctx.cr[6].eq {
		sub_8246A990(ctx, base);
		return;
	}
	// 8246A9E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246A9E4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8246A9E8: 4198FFE8  blt cr6, 0x8246a9d0
	if ctx.cr[6].lt {
	pc = 0x8246A9D0; continue 'dispatch;
	}
	pc = 0x8246A9EC; continue 'dispatch;
            }
            0x8246A9EC => {
    //   block [0x8246A9EC..0x8246A9F8)
	// 8246A9EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246A9F0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246A9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246A9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246A9F8 size=72
    let mut pc: u32 = 0x8246A9F8;
    'dispatch: loop {
        match pc {
            0x8246A9F8 => {
    //   block [0x8246A9F8..0x8246AA04)
	// 8246A9F8: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 8246A9FC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8246AA00: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x8246AA04; continue 'dispatch;
            }
            0x8246AA04 => {
    //   block [0x8246AA04..0x8246AA40)
	// 8246AA04: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AA08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246AA0C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8246AA10: 409AFFF4  bne cr6, 0x8246aa04
	if !ctx.cr[6].eq {
	pc = 0x8246AA04; continue 'dispatch;
	}
	// 8246AA14: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246AA18: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AA1C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246AA20: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246AA24: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246AA28: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246AA2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8246AA30: 40980010  bge cr6, 0x8246aa40
	if !ctx.cr[6].lt {
		sub_8246AA40(ctx, base);
		return;
	}
	// 8246AA34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246AA38: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246AA40 size=76
    let mut pc: u32 = 0x8246AA40;
    'dispatch: loop {
        match pc {
            0x8246AA40 => {
    //   block [0x8246AA40..0x8246AA58)
	// 8246AA40: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AA44: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246AA48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246AA4C: 419A0034  beq cr6, 0x8246aa80
	if ctx.cr[6].eq {
	pc = 0x8246AA80; continue 'dispatch;
	}
	// 8246AA50: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AA54: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	pc = 0x8246AA58; continue 'dispatch;
            }
            0x8246AA58 => {
    //   block [0x8246AA58..0x8246AA80)
	// 8246AA58: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AA5C: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 8246AA60: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246AA64: 409AFFD0  bne cr6, 0x8246aa34
	if !ctx.cr[6].eq {
		sub_8246A9F8(ctx, base);
		return;
	}
	// 8246AA68: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8246AA6C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246AA70: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AA74: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246AA78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246AA7C: 409AFFDC  bne cr6, 0x8246aa58
	if !ctx.cr[6].eq {
	pc = 0x8246AA58; continue 'dispatch;
	}
	pc = 0x8246AA80; continue 'dispatch;
            }
            0x8246AA80 => {
    //   block [0x8246AA80..0x8246AA8C)
	// 8246AA80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8246AA84: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246AA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8246AA90 size=88
    let mut pc: u32 = 0x8246AA90;
    'dispatch: loop {
        match pc {
            0x8246AA90 => {
    //   block [0x8246AA90..0x8246AAAC)
	// 8246AA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246AA94: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AA98: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246AA9C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8246AAA0: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
	// 8246AAA4: 7CA80774  extsb r8, r5
	ctx.r[8].s64 = ctx.r[5].s8 as i64;
	// 8246AAA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	pc = 0x8246AAAC; continue 'dispatch;
            }
            0x8246AAAC => {
    //   block [0x8246AAAC..0x8246AAD0)
	// 8246AAAC: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AAB0: 7CAA58AE  lbzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246AAB4: 7CA50774  extsb r5, r5
	ctx.r[5].s64 = ctx.r[5].s8 as i64;
	// 8246AAB8: 7F054000  cmpw cr6, r5, r8
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8246AABC: 409A0014  bne cr6, 0x8246aad0
	if !ctx.cr[6].eq {
	pc = 0x8246AAD0; continue 'dispatch;
	}
	// 8246AAC0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8246AAC4: 7CCA59AE  stbx r6, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u8) };
	// 8246AAC8: 99230000  stb r9, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8246AACC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	pc = 0x8246AAD0; continue 'dispatch;
            }
            0x8246AAD0 => {
    //   block [0x8246AAD0..0x8246AAE8)
	// 8246AAD0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AAD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246AAD8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246AADC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246AAE0: 4198FFCC  blt cr6, 0x8246aaac
	if ctx.cr[6].lt {
	pc = 0x8246AAAC; continue 'dispatch;
	}
	// 8246AAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246AAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246AAE8 size=232
    let mut pc: u32 = 0x8246AAE8;
    'dispatch: loop {
        match pc {
            0x8246AAE8 => {
    //   block [0x8246AAE8..0x8246AB30)
	// 8246AAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246AAEC: 480CA5B1  bl 0x8253509c
	ctx.lr = 0x8246AAF0;
	sub_82535080(ctx, base);
	// 8246AAF0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246AAF4: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 8246AAF8: 81660004  lwz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AAFC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8246AB00: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8246AB04: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8246AB08: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 8246AB0C: 81570004  lwz r10, 4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AB10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8246AB14: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AB18: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8246AB1C: 3B4BFFFF  addi r26, r11, -1
	ctx.r[26].s64 = ctx.r[11].s64 + -1;
	// 8246AB20: 3B0AFFFF  addi r24, r10, -1
	ctx.r[24].s64 = ctx.r[10].s64 + -1;
	// 8246AB24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8246AB28: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8246AB2C: 40990074  ble cr6, 0x8246aba0
	if !ctx.cr[6].gt {
	pc = 0x8246ABA0; continue 'dispatch;
	}
	pc = 0x8246AB30; continue 'dispatch;
            }
            0x8246AB30 => {
    //   block [0x8246AB30..0x8246AB44)
	// 8246AB30: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8246AB34: 409A0010  bne cr6, 0x8246ab44
	if !ctx.cr[6].eq {
	pc = 0x8246AB44; continue 'dispatch;
	}
	// 8246AB38: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AB3C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AB40: 48000020  b 0x8246ab60
	pc = 0x8246AB60; continue 'dispatch;
            }
            0x8246AB44 => {
    //   block [0x8246AB44..0x8246AB60)
	// 8246AB44: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AB48: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246AB4C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246AB50: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AB54: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8246AB58: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8246AB5C: 7FDA5850  subf r30, r26, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	pc = 0x8246AB60; continue 'dispatch;
            }
            0x8246AB60 => {
    //   block [0x8246AB60..0x8246ABA0)
	// 8246AB60: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246AB64: 7C9CB214  add r4, r28, r22
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	// 8246AB68: 7C7BCA14  add r3, r27, r25
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 8246AB6C: 480C9FE5  bl 0x82534b50
	ctx.lr = 0x8246AB70;
	sub_82534B50(ctx, base);
	// 8246AB70: 7F7EDA14  add r27, r30, r27
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[27].u64;
	// 8246AB74: 7D7ED214  add r11, r30, r26
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[26].u64;
	// 8246AB78: 80970000  lwz r4, 0(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AB7C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8246AB80: 7C7BCA14  add r3, r27, r25
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 8246AB84: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8246AB88: 480C9FC9  bl 0x82534b50
	ctx.lr = 0x8246AB8C;
	sub_82534B50(ctx, base);
	// 8246AB8C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AB90: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8246AB94: 7F78DA14  add r27, r24, r27
	ctx.r[27].u64 = ctx.r[24].u64 + ctx.r[27].u64;
	// 8246AB98: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8246AB9C: 4198FF94  blt cr6, 0x8246ab30
	if ctx.cr[6].lt {
	pc = 0x8246AB30; continue 'dispatch;
	}
	pc = 0x8246ABA0; continue 'dispatch;
            }
            0x8246ABA0 => {
    //   block [0x8246ABA0..0x8246ABD0)
	// 8246ABA0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246ABA4: 7C9CB214  add r4, r28, r22
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	// 8246ABA8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ABAC: 7C7BCA14  add r3, r27, r25
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[25].u64;
	// 8246ABB0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8246ABB4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8246ABB8: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8246ABBC: 7D6BA850  subf r11, r11, r21
	ctx.r[11].s64 = ctx.r[21].s64 - ctx.r[11].s64;
	// 8246ABC0: 7CBA5850  subf r5, r26, r11
	ctx.r[5].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 8246ABC4: 480C9F8D  bl 0x82534b50
	ctx.lr = 0x8246ABC8;
	sub_82534B50(ctx, base);
	// 8246ABC8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8246ABCC: 480CA520  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246ABD0 size=128
    let mut pc: u32 = 0x8246ABD0;
    'dispatch: loop {
        match pc {
            0x8246ABD0 => {
    //   block [0x8246ABD0..0x8246AC50)
	// 8246ABD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246ABD4: 480CA4D9  bl 0x825350ac
	ctx.lr = 0x8246ABD8;
	sub_82535080(ctx, base);
	// 8246ABD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246ABDC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8246ABE0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8246ABE4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246ABE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246ABEC: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246ABF0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246ABF4: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246ABF8: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 8246ABFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AC00: 3B2AFFFF  addi r25, r10, -1
	ctx.r[25].s64 = ctx.r[10].s64 + -1;
	// 8246AC04: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246AC08: 7D79F214  add r11, r25, r30
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[30].u64;
	// 8246AC0C: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 8246AC10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8246AC14: 4BFF9425  bl 0x82464038
	ctx.lr = 0x8246AC18;
	sub_82464038(ctx, base);
	// 8246AC18: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8246AC1C: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AC20: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8246AC24: 480C9F2D  bl 0x82534b50
	ctx.lr = 0x8246AC28;
	sub_82534B50(ctx, base);
	// 8246AC28: 38B90001  addi r5, r25, 1
	ctx.r[5].s64 = ctx.r[25].s64 + 1;
	// 8246AC2C: 7C7CF214  add r3, r28, r30
	ctx.r[3].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 8246AC30: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AC34: 480C9F1D  bl 0x82534b50
	ctx.lr = 0x8246AC38;
	sub_82534B50(ctx, base);
	// 8246AC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246AC3C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8246AC40: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8246AC44: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8246AC48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246AC4C: 480CA4B0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246AC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246AC50 size=156
    let mut pc: u32 = 0x8246AC50;
    'dispatch: loop {
        match pc {
            0x8246AC50 => {
    //   block [0x8246AC50..0x8246AC78)
	// 8246AC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246AC54: 480CA459  bl 0x825350ac
	ctx.lr = 0x8246AC58;
	sub_82535080(ctx, base);
	// 8246AC58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246AC5C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8246AC60: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8246AC64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8246AC68: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 8246AC6C: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AC70: 3B8AFFFF  addi r28, r10, -1
	ctx.r[28].s64 = ctx.r[10].s64 + -1;
	// 8246AC74: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x8246AC78; continue 'dispatch;
            }
            0x8246AC78 => {
    //   block [0x8246AC78..0x8246ACEC)
	// 8246AC78: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AC7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246AC80: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8246AC84: 409AFFF4  bne cr6, 0x8246ac78
	if !ctx.cr[6].eq {
	pc = 0x8246AC78; continue 'dispatch;
	}
	// 8246AC88: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8246AC8C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AC90: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8246AC94: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8246AC98: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246AC9C: 557B003E  slwi r27, r11, 0
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 8246ACA0: 7D7BE214  add r11, r27, r28
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 8246ACA4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8246ACA8: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 8246ACAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8246ACB0: 4BFF9389  bl 0x82464038
	ctx.lr = 0x8246ACB4;
	sub_82464038(ctx, base);
	// 8246ACB4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8246ACB8: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ACBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246ACC0: 480C9E91  bl 0x82534b50
	ctx.lr = 0x8246ACC4;
	sub_82534B50(ctx, base);
	// 8246ACC4: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 8246ACC8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8246ACCC: 7C7DE214  add r3, r29, r28
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 8246ACD0: 480C9E81  bl 0x82534b50
	ctx.lr = 0x8246ACD4;
	sub_82534B50(ctx, base);
	// 8246ACD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8246ACD8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8246ACDC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8246ACE0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8246ACE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246ACE8: 480CA414  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246ACF0 size=200
    let mut pc: u32 = 0x8246ACF0;
    'dispatch: loop {
        match pc {
            0x8246ACF0 => {
    //   block [0x8246ACF0..0x8246AD38)
	// 8246ACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246ACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246ACF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246ACFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246AD00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246AD04: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AD08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246AD0C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246AD10: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246AD14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246AD18: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AD1C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246AD20: 4BFF9319  bl 0x82464038
	ctx.lr = 0x8246AD24;
	sub_82464038(ctx, base);
	// 8246AD24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AD28: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8246AD2C: 354BFFFF  addic. r10, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246AD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246AD34: 40810048  ble 0x8246ad7c
	if !ctx.cr[0].gt {
	pc = 0x8246AD7C; continue 'dispatch;
	}
	pc = 0x8246AD38; continue 'dispatch;
            }
            0x8246AD38 => {
    //   block [0x8246AD38..0x8246AD60)
	// 8246AD38: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AD3C: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246AD40: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 8246AD44: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 8246AD48: 41980018  blt cr6, 0x8246ad60
	if ctx.cr[6].lt {
	pc = 0x8246AD60; continue 'dispatch;
	}
	// 8246AD4C: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 8246AD50: 41990010  bgt cr6, 0x8246ad60
	if ctx.cr[6].gt {
	pc = 0x8246AD60; continue 'dispatch;
	}
	// 8246AD54: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8246AD58: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246AD5C: 48000008  b 0x8246ad64
	pc = 0x8246AD64; continue 'dispatch;
            }
            0x8246AD60 => {
    //   block [0x8246AD60..0x8246AD64)
	// 8246AD60: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	pc = 0x8246AD64; continue 'dispatch;
            }
            0x8246AD64 => {
    //   block [0x8246AD64..0x8246AD7C)
	// 8246AD64: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 8246AD68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246AD6C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AD70: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246AD74: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246AD78: 4198FFC0  blt cr6, 0x8246ad38
	if ctx.cr[6].lt {
	pc = 0x8246AD38; continue 'dispatch;
	}
	pc = 0x8246AD7C; continue 'dispatch;
            }
            0x8246AD7C => {
    //   block [0x8246AD7C..0x8246ADB8)
	// 8246AD7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246AD84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246AD88: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8246AD8C: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8246AD90: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 8246AD94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AD98: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246AD9C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246ADA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246ADA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246ADA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246ADAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246ADB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246ADB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246ADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246ADB8 size=200
    let mut pc: u32 = 0x8246ADB8;
    'dispatch: loop {
        match pc {
            0x8246ADB8 => {
    //   block [0x8246ADB8..0x8246AE00)
	// 8246ADB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246ADBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8246ADC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8246ADC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8246ADC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246ADCC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246ADD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246ADD4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246ADD8: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246ADDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246ADE0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246ADE4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246ADE8: 4BFF9251  bl 0x82464038
	ctx.lr = 0x8246ADEC;
	sub_82464038(ctx, base);
	// 8246ADEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246ADF0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8246ADF4: 354BFFFF  addic. r10, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246ADF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246ADFC: 40810048  ble 0x8246ae44
	if !ctx.cr[0].gt {
	pc = 0x8246AE44; continue 'dispatch;
	}
	pc = 0x8246AE00; continue 'dispatch;
            }
            0x8246AE00 => {
    //   block [0x8246AE00..0x8246AE28)
	// 8246AE00: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AE04: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246AE08: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 8246AE0C: 2F0A0041  cmpwi cr6, r10, 0x41
	ctx.cr[6].compare_i32(ctx.r[10].s32, 65, &mut ctx.xer);
	// 8246AE10: 41980018  blt cr6, 0x8246ae28
	if ctx.cr[6].lt {
	pc = 0x8246AE28; continue 'dispatch;
	}
	// 8246AE14: 2F0A005A  cmpwi cr6, r10, 0x5a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 90, &mut ctx.xer);
	// 8246AE18: 41990010  bgt cr6, 0x8246ae28
	if ctx.cr[6].gt {
	pc = 0x8246AE28; continue 'dispatch;
	}
	// 8246AE1C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 8246AE20: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246AE24: 48000008  b 0x8246ae2c
	pc = 0x8246AE2C; continue 'dispatch;
            }
            0x8246AE28 => {
    //   block [0x8246AE28..0x8246AE2C)
	// 8246AE28: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	pc = 0x8246AE2C; continue 'dispatch;
            }
            0x8246AE2C => {
    //   block [0x8246AE2C..0x8246AE44)
	// 8246AE2C: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 8246AE30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246AE34: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AE38: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246AE3C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246AE40: 4198FFC0  blt cr6, 0x8246ae00
	if ctx.cr[6].lt {
	pc = 0x8246AE00; continue 'dispatch;
	}
	pc = 0x8246AE44; continue 'dispatch;
            }
            0x8246AE44 => {
    //   block [0x8246AE44..0x8246AE80)
	// 8246AE44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246AE4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8246AE50: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8246AE54: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8246AE58: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 8246AE5C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AE60: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246AE64: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246AE68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8246AE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8246AE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8246AE74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8246AE78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8246AE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8246AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8246AE80 size=184
    let mut pc: u32 = 0x8246AE80;
    'dispatch: loop {
        match pc {
            0x8246AE80 => {
    //   block [0x8246AE80..0x8246AEDC)
	// 8246AE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8246AE84: 480CA22D  bl 0x825350b0
	ctx.lr = 0x8246AE88;
	sub_82535080(ctx, base);
	// 8246AE88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8246AE8C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AE90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8246AE94: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8246AE98: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8246AE9C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 8246AEA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8246AEA4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AEA8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8246AEAC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8246AEB0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8246AEB4: 4BFF9185  bl 0x82464038
	ctx.lr = 0x8246AEB8;
	sub_82464038(ctx, base);
	// 8246AEB8: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AEBC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8246AEC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8246AEC4: 480C9C8D  bl 0x82534b50
	ctx.lr = 0x8246AEC8;
	sub_82534B50(ctx, base);
	// 8246AEC8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AECC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8246AED0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8246AED4: 40810038  ble 0x8246af0c
	if !ctx.cr[0].gt {
	pc = 0x8246AF0C; continue 'dispatch;
	}
	// 8246AED8: 7F890774  extsb r9, r28
	ctx.r[9].s64 = ctx.r[28].s8 as i64;
	pc = 0x8246AEDC; continue 'dispatch;
            }
            0x8246AEDC => {
    //   block [0x8246AEDC..0x8246AEF8)
	// 8246AEDC: 7D4BF0AE  lbzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8246AEE0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8246AEE4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8246AEE8: 409A0010  bne cr6, 0x8246aef8
	if !ctx.cr[6].eq {
	pc = 0x8246AEF8; continue 'dispatch;
	}
	// 8246AEEC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8246AEF0: 7F6BF1AE  stbx r27, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u8) };
	// 8246AEF4: 419A0018  beq cr6, 0x8246af0c
	if ctx.cr[6].eq {
	pc = 0x8246AF0C; continue 'dispatch;
	}
	pc = 0x8246AEF8; continue 'dispatch;
            }
            0x8246AEF8 => {
    //   block [0x8246AEF8..0x8246AF0C)
	// 8246AEF8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AEFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8246AF00: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8246AF04: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8246AF08: 4198FFD4  blt cr6, 0x8246aedc
	if ctx.cr[6].lt {
	pc = 0x8246AEDC; continue 'dispatch;
	}
	pc = 0x8246AF0C; continue 'dispatch;
            }
            0x8246AF0C => {
    //   block [0x8246AF0C..0x8246AF38)
	// 8246AF0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8246AF14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8246AF18: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8246AF1C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8246AF20: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 8246AF24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8246AF28: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8246AF2C: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8246AF30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8246AF34: 480CA1CC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


