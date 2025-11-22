pub fn sub_82C53CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C53CC0 size=24
    let mut pc: u32 = 0x82C53CC0;
    'dispatch: loop {
        match pc {
            0x82C53CC0 => {
    //   block [0x82C53CC0..0x82C53CD8)
	// 82C53CC0: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C53CC4: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 82C53CC8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C53CCC: 13E350C7  vcmpequd (lvx128) v31, v3, v10
	tmp.u32 = ctx.r[3].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C53CD8 size=104
    let mut pc: u32 = 0x82C53CD8;
    'dispatch: loop {
        match pc {
            0x82C53CD8 => {
    //   block [0x82C53CD8..0x82C53D40)
	// 82C53CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C53CDC: 48055731  bl 0x82ca940c
	ctx.lr = 0x82C53CE0;
	sub_82CA93D0(ctx, base);
	// 82C53CE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C53CE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C53CE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C53CEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C53CF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C53CF4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82C53CF8: 409A0008  bne cr6, 0x82c53d00
	if !ctx.cr[6].eq {
	pc = 0x82C53D00; continue 'dispatch;
	}
	// 82C53CFC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C53D00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C53D04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C53D08: 419A0028  beq cr6, 0x82c53d30
	if ctx.cr[6].eq {
	pc = 0x82C53D30; continue 'dispatch;
	}
	// 82C53D0C: 93AB0028  stw r29, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82C53D10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C53D14: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C53D18: 38800032  li r4, 0x32
	ctx.r[4].s64 = 50;
	// 82C53D1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53D20: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C53D24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C53D28: 4E800421  bctrl
	ctx.lr = 0x82C53D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C53D2C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C53D30: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C53D34: 93FE0028  stw r31, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 82C53D38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C53D3C: 48055720  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C53D40 size=72
    let mut pc: u32 = 0x82C53D40;
    'dispatch: loop {
        match pc {
            0x82C53D40 => {
    //   block [0x82C53D40..0x82C53D88)
	// 82C53D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C53D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C53D48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C53D4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C53D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C53D54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C53D58: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C53D5C: 392BCD98  addi r9, r11, -0x3268
	ctx.r[9].s64 = ctx.r[11].s64 + -12904;
	// 82C53D60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C53D64: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C53D68: 419A000C  beq cr6, 0x82c53d74
	if ctx.cr[6].eq {
	pc = 0x82C53D74; continue 'dispatch;
	}
	// 82C53D6C: 4BBF1A45  bl 0x828457b0
	ctx.lr = 0x82C53D70;
	sub_828457B0(ctx, base);
	// 82C53D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C53D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C53D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C53D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C53D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C53D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C53D88 size=136
    let mut pc: u32 = 0x82C53D88;
    'dispatch: loop {
        match pc {
            0x82C53D88 => {
    //   block [0x82C53D88..0x82C53E10)
	// 82C53D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C53D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C53D90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C53D94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C53D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C53D9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C53DA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C53DA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C53DA8: 390BCD9C  addi r8, r11, -0x3264
	ctx.r[8].s64 = ctx.r[11].s64 + -12900;
	// 82C53DAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C53DB0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C53DB4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C53DB8: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82C53DBC: 409A0008  bne cr6, 0x82c53dc4
	if !ctx.cr[6].eq {
	pc = 0x82C53DC4; continue 'dispatch;
	}
	// 82C53DC0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C53DC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C53DC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C53DCC: 419A000C  beq cr6, 0x82c53dd8
	if ctx.cr[6].eq {
	pc = 0x82C53DD8; continue 'dispatch;
	}
	// 82C53DD0: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82C53DD4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C53DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C53DDC: 4BFB22AD  bl 0x82c06088
	ctx.lr = 0x82C53DE0;
	sub_82C06088(ctx, base);
	// 82C53DE0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C53DE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C53DE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C53DEC: 419A000C  beq cr6, 0x82c53df8
	if ctx.cr[6].eq {
	pc = 0x82C53DF8; continue 'dispatch;
	}
	// 82C53DF0: 4BBF19C1  bl 0x828457b0
	ctx.lr = 0x82C53DF4;
	sub_828457B0(ctx, base);
	// 82C53DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C53DF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C53DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C53E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C53E04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C53E08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C53E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C53E10 size=132
    let mut pc: u32 = 0x82C53E10;
    'dispatch: loop {
        match pc {
            0x82C53E10 => {
    //   block [0x82C53E10..0x82C53E94)
	// 82C53E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C53E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C53E18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C53E1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C53E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C53E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C53E28: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53E2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C53E30: 419A004C  beq cr6, 0x82c53e7c
	if ctx.cr[6].eq {
	pc = 0x82C53E7C; continue 'dispatch;
	}
	// 82C53E34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C53E38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C53E3C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82C53E40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53E44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53E48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C53E4C: 4E800421  bctrl
	ctx.lr = 0x82C53E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C53E50: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53E54: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82C53E58: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82C53E5C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53E60: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C53E64: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C53E68: 4E800421  bctrl
	ctx.lr = 0x82C53E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C53E6C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C53E70: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C53E74: 7CA639D6  mullw r5, r6, r7
	ctx.r[5].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82C53E78: 54A3083C  slwi r3, r5, 1
	ctx.r[3].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C53E7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C53E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C53E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C53E88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C53E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C53E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C53E98 size=240
    let mut pc: u32 = 0x82C53E98;
    'dispatch: loop {
        match pc {
            0x82C53E98 => {
    //   block [0x82C53E98..0x82C53F88)
	// 82C53E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C53E9C: 48055561  bl 0x82ca93fc
	ctx.lr = 0x82C53EA0;
	sub_82CA93D0(ctx, base);
	// 82C53EA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C53EA4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82C53EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C53EAC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C53EB0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82C53EB4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C53EB8: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C53EBC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82C53EC0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53EC4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82C53EC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C53ECC: 409A0010  bne cr6, 0x82c53edc
	if !ctx.cr[6].eq {
	pc = 0x82C53EDC; continue 'dispatch;
	}
	// 82C53ED0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C53ED4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C53ED8: 48055574  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C53EDC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C53EE0: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C53EE4: 419A006C  beq cr6, 0x82c53f50
	if ctx.cr[6].eq {
	pc = 0x82C53F50; continue 'dispatch;
	}
	// 82C53EE8: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C53EEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C53EF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53EF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53EF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C53EFC: 4E800421  bctrl
	ctx.lr = 0x82C53F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C53F00: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53F04: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82C53F08: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82C53F0C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C53F10: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C53F14: 553D083C  slwi r29, r9, 1
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82C53F18: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53F1C: 7F9CEB96  divwu r28, r28, r29
	ctx.r[28].u32 = ctx.r[28].u32 / ctx.r[29].u32;
	// 82C53F20: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C53F24: 80E8000C  lwz r7, 0xc(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C53F28: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C53F2C: 4E800421  bctrl
	ctx.lr = 0x82C53F30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C53F30: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C53F34: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C53F38: 7CDC5850  subf r6, r28, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82C53F3C: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82C53F40: 7C6621D6  mullw r3, r6, r4
	ctx.r[3].s64 = (ctx.r[6].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82C53F44: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C53F48: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C53F4C: 90BF0014  stw r5, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82C53F50: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53F54: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C53F58: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82C53F5C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C53F60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53F64: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53F68: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C53F6C: 4E800421  bctrl
	ctx.lr = 0x82C53F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C53F70: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53F74: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C53F78: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C53F7C: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82C53F80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C53F84: 480554C8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C53F88 size=12
    let mut pc: u32 = 0x82C53F88;
    'dispatch: loop {
        match pc {
            0x82C53F88 => {
    //   block [0x82C53F88..0x82C53F94)
	// 82C53F88: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53F8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C53F90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53F94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C53F94 size=16
    let mut pc: u32 = 0x82C53F94;
    'dispatch: loop {
        match pc {
            0x82C53F94 => {
    //   block [0x82C53F94..0x82C53FA4)
	// 82C53F94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53F98: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C53F9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C53FA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53FA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C53FA4 size=4
    let mut pc: u32 = 0x82C53FA4;
    'dispatch: loop {
        match pc {
            0x82C53FA4 => {
    //   block [0x82C53FA4..0x82C53FA8)
	// 82C53FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C53FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C53FA8 size=120
    let mut pc: u32 = 0x82C53FA8;
    'dispatch: loop {
        match pc {
            0x82C53FA8 => {
    //   block [0x82C53FA8..0x82C54020)
	// 82C53FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C53FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C53FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C53FB4: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C53FB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C53FBC: 419A0050  beq cr6, 0x82c5400c
	if ctx.cr[6].eq {
	pc = 0x82C5400C; continue 'dispatch;
	}
	// 82C53FC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C53FC4: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C53FC8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C53FCC: 4E800421  bctrl
	ctx.lr = 0x82C53FD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C53FD0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82C53FD4: 41980038  blt cr6, 0x82c5400c
	if ctx.cr[6].lt {
	pc = 0x82C5400C; continue 'dispatch;
	}
	// 82C53FD8: 419A0020  beq cr6, 0x82c53ff8
	if ctx.cr[6].eq {
	pc = 0x82C53FF8; continue 'dispatch;
	}
	// 82C53FDC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82C53FE0: 4098002C  bge cr6, 0x82c5400c
	if !ctx.cr[6].lt {
	pc = 0x82C5400C; continue 'dispatch;
	}
	// 82C53FE4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82C53FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C53FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C53FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C53FF4: 4E800020  blr
	return;
	// 82C53FF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C53FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C54008: 4E800020  blr
	return;
	// 82C5400C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C54010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5401C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54020 size=108
    let mut pc: u32 = 0x82C54020;
    'dispatch: loop {
        match pc {
            0x82C54020 => {
    //   block [0x82C54020..0x82C5408C)
	// 82C54020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C54028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5402C: 896300F8  lbz r11, 0xf8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(248 as u32) ) } as u64;
	// 82C54030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54034: 419A0018  beq cr6, 0x82c5404c
	if ctx.cr[6].eq {
	pc = 0x82C5404C; continue 'dispatch;
	}
	// 82C54038: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C5403C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C54048: 4E800020  blr
	return;
	// 82C5404C: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54050: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C54054: 419A0024  beq cr6, 0x82c54078
	if ctx.cr[6].eq {
	pc = 0x82C54078; continue 'dispatch;
	}
	// 82C54058: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5405C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C54060: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54064: 4E800421  bctrl
	ctx.lr = 0x82C54068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54068: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C5406C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C54070: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C54074: 419A0008  beq cr6, 0x82c5407c
	if ctx.cr[6].eq {
	pc = 0x82C5407C; continue 'dispatch;
	}
	// 82C54078: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C5407C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C54088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C54090 size=28
    let mut pc: u32 = 0x82C54090;
    'dispatch: loop {
        match pc {
            0x82C54090 => {
    //   block [0x82C54090..0x82C540AC)
	// 82C54090: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C54094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C54098: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82C5409C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C540A0: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 82C540A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C540A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C540AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C540AC size=20
    let mut pc: u32 = 0x82C540AC;
    'dispatch: loop {
        match pc {
            0x82C540AC => {
    //   block [0x82C540AC..0x82C540C0)
	// 82C540AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C540B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C540B4: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C540B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C540BC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C540C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C540C0 size=4
    let mut pc: u32 = 0x82C540C0;
    'dispatch: loop {
        match pc {
            0x82C540C0 => {
    //   block [0x82C540C0..0x82C540C4)
	// 82C540C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C540C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C540C8 size=28
    let mut pc: u32 = 0x82C540C8;
    'dispatch: loop {
        match pc {
            0x82C540C8 => {
    //   block [0x82C540C8..0x82C540E4)
	// 82C540C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C540CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C540D0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82C540D4: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C540D8: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 82C540DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C540E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C540E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C540E4 size=20
    let mut pc: u32 = 0x82C540E4;
    'dispatch: loop {
        match pc {
            0x82C540E4 => {
    //   block [0x82C540E4..0x82C540F8)
	// 82C540E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C540E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C540EC: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C540F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C540F4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C540F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C540F8 size=4
    let mut pc: u32 = 0x82C540F8;
    'dispatch: loop {
        match pc {
            0x82C540F8 => {
    //   block [0x82C540F8..0x82C540FC)
	// 82C540F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C54100 size=20
    let mut pc: u32 = 0x82C54100;
    'dispatch: loop {
        match pc {
            0x82C54100 => {
    //   block [0x82C54100..0x82C54114)
	// 82C54100: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82C54104: 409A0010  bne cr6, 0x82c54114
	if !ctx.cr[6].eq {
		sub_82C54114(ctx, base);
		return;
	}
	// 82C54108: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C5410C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C54110: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C54114 size=20
    let mut pc: u32 = 0x82C54114;
    'dispatch: loop {
        match pc {
            0x82C54114 => {
    //   block [0x82C54114..0x82C54128)
	// 82C54114: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54118: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5411C: 409A000C  bne cr6, 0x82c54128
	if !ctx.cr[6].eq {
		sub_82C54128(ctx, base);
		return;
	}
	// 82C54120: 996300F8  stb r11, 0xf8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), ctx.r[11].u8 ) };
	// 82C54124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C54128 size=20
    let mut pc: u32 = 0x82C54128;
    'dispatch: loop {
        match pc {
            0x82C54128 => {
    //   block [0x82C54128..0x82C5413C)
	// 82C54128: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5412C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C54130: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54134: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C54138: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54140 size=92
    let mut pc: u32 = 0x82C54140;
    'dispatch: loop {
        match pc {
            0x82C54140 => {
    //   block [0x82C54140..0x82C5419C)
	// 82C54140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C54148: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5414C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C54150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C54158: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C5415C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C54160: 48002269  bl 0x82c563c8
	ctx.lr = 0x82C54164;
	sub_82C563C8(ctx, base);
	// 82C54164: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54168: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5416C: 419A0018  beq cr6, 0x82c54184
	if ctx.cr[6].eq {
	pc = 0x82C54184; continue 'dispatch;
	}
	// 82C54170: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54174: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54178: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C5417C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54180: 4E800421  bctrl
	ctx.lr = 0x82C54184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C54188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5418C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C54190: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C54194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C54198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C541A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C541A0 size=92
    let mut pc: u32 = 0x82C541A0;
    'dispatch: loop {
        match pc {
            0x82C541A0 => {
    //   block [0x82C541A0..0x82C541FC)
	// 82C541A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C541A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C541A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C541AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C541B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C541B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C541B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C541BC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C541C0: 48002581  bl 0x82c56740
	ctx.lr = 0x82C541C4;
	sub_82C56740(ctx, base);
	// 82C541C4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C541C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C541CC: 419A0018  beq cr6, 0x82c541e4
	if ctx.cr[6].eq {
	pc = 0x82C541E4; continue 'dispatch;
	}
	// 82C541D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C541D4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C541D8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C541DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C541E0: 4E800421  bctrl
	ctx.lr = 0x82C541E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C541E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C541E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C541EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C541F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C541F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C541F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54200 size=1016
    let mut pc: u32 = 0x82C54200;
    'dispatch: loop {
        match pc {
            0x82C54200 => {
    //   block [0x82C54200..0x82C545F8)
	// 82C54200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54204: 48055201  bl 0x82ca9404
	ctx.lr = 0x82C54208;
	sub_82CA93D0(ctx, base);
	// 82C54208: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5420C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C54210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54214: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C54218: 4BFB1519  bl 0x82c05730
	ctx.lr = 0x82C5421C;
	sub_82C05730(ctx, base);
	// 82C5421C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54220: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C54224: 3BEBCEB0  addi r31, r11, -0x3150
	ctx.r[31].s64 = ctx.r[11].s64 + -12624;
	// 82C54228: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5422C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54230: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C54234: 4E800421  bctrl
	ctx.lr = 0x82C54238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54238: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C5423C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C54240: 3888CEA4  addi r4, r8, -0x315c
	ctx.r[4].s64 = ctx.r[8].s64 + -12636;
	// 82C54244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54248: 4BFB15B1  bl 0x82c057f8
	ctx.lr = 0x82C5424C;
	sub_82C057F8(ctx, base);
	// 82C5424C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54250: 4BFB15A9  bl 0x82c057f8
	ctx.lr = 0x82C54254;
	sub_82C057F8(ctx, base);
	// 82C54254: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C54258: 4BFB15A1  bl 0x82c057f8
	ctx.lr = 0x82C5425C;
	sub_82C057F8(ctx, base);
	// 82C5425C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C54260: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54264: 3B870C88  addi r28, r7, 0xc88
	ctx.r[28].s64 = ctx.r[7].s64 + 3208;
	// 82C54268: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C5426C: 4BFB158D  bl 0x82c057f8
	ctx.lr = 0x82C54270;
	sub_82C057F8(ctx, base);
	// 82C54270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54274: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54278: 4BFB1849  bl 0x82c05ac0
	ctx.lr = 0x82C5427C;
	sub_82C05AC0(ctx, base);
	// 82C5427C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C54280: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54284: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C54288: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C5428C: 4E800421  bctrl
	ctx.lr = 0x82C54290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54290: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54294: 4BFB14DD  bl 0x82c05770
	ctx.lr = 0x82C54298;
	sub_82C05770(ctx, base);
	// 82C54298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5429C: 4BFB1495  bl 0x82c05730
	ctx.lr = 0x82C542A0;
	sub_82C05730(ctx, base);
	// 82C542A0: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C542A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C542A8: 3885CE98  addi r4, r5, -0x3168
	ctx.r[4].s64 = ctx.r[5].s64 + -12648;
	// 82C542AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C542B0: 3BEBCE88  addi r31, r11, -0x3178
	ctx.r[31].s64 = ctx.r[11].s64 + -12664;
	// 82C542B4: 837E0030  lwz r27, 0x30(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C542B8: 4BFB1541  bl 0x82c057f8
	ctx.lr = 0x82C542BC;
	sub_82C057F8(ctx, base);
	// 82C542BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C542C0: 4BFB1679  bl 0x82c05938
	ctx.lr = 0x82C542C4;
	sub_82C05938(ctx, base);
	// 82C542C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C542C8: 4BFB1531  bl 0x82c057f8
	ctx.lr = 0x82C542CC;
	sub_82C057F8(ctx, base);
	// 82C542CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C542D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C542D4: 4BFB1525  bl 0x82c057f8
	ctx.lr = 0x82C542D8;
	sub_82C057F8(ctx, base);
	// 82C542D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C542DC: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C542E0: 4BFB17E1  bl 0x82c05ac0
	ctx.lr = 0x82C542E4;
	sub_82C05AC0(ctx, base);
	// 82C542E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C542E8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C542EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C542F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C542F4: 4E800421  bctrl
	ctx.lr = 0x82C542F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C542F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C542FC: 4BFB1475  bl 0x82c05770
	ctx.lr = 0x82C54300;
	sub_82C05770(ctx, base);
	// 82C54300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54304: 4BFB142D  bl 0x82c05730
	ctx.lr = 0x82C54308;
	sub_82C05730(ctx, base);
	// 82C54308: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C5430C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C54310: 3889CE7C  addi r4, r9, -0x3184
	ctx.r[4].s64 = ctx.r[9].s64 + -12676;
	// 82C54314: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54318: 3BE8CE6C  addi r31, r8, -0x3194
	ctx.r[31].s64 = ctx.r[8].s64 + -12692;
	// 82C5431C: 837E0034  lwz r27, 0x34(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C54320: 4BFB14D9  bl 0x82c057f8
	ctx.lr = 0x82C54324;
	sub_82C057F8(ctx, base);
	// 82C54324: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C54328: 4BFB1541  bl 0x82c05868
	ctx.lr = 0x82C5432C;
	sub_82C05868(ctx, base);
	// 82C5432C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C54330: 4BFB14C9  bl 0x82c057f8
	ctx.lr = 0x82C54334;
	sub_82C057F8(ctx, base);
	// 82C54334: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54338: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5433C: 4BFB14BD  bl 0x82c057f8
	ctx.lr = 0x82C54340;
	sub_82C057F8(ctx, base);
	// 82C54340: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54344: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54348: 4BFB1779  bl 0x82c05ac0
	ctx.lr = 0x82C5434C;
	sub_82C05AC0(ctx, base);
	// 82C5434C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C54350: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54354: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C54358: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C5435C: 4E800421  bctrl
	ctx.lr = 0x82C54360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54364: 4BFB140D  bl 0x82c05770
	ctx.lr = 0x82C54368;
	sub_82C05770(ctx, base);
	// 82C54368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5436C: 4BFB13C5  bl 0x82c05730
	ctx.lr = 0x82C54370;
	sub_82C05730(ctx, base);
	// 82C54370: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C54374: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C54378: 3886CE60  addi r4, r6, -0x31a0
	ctx.r[4].s64 = ctx.r[6].s64 + -12704;
	// 82C5437C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54380: 3BE5CE54  addi r31, r5, -0x31ac
	ctx.r[31].s64 = ctx.r[5].s64 + -12716;
	// 82C54384: 837E0038  lwz r27, 0x38(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C54388: 4BFB1471  bl 0x82c057f8
	ctx.lr = 0x82C5438C;
	sub_82C057F8(ctx, base);
	// 82C5438C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C54390: 4BFB14D9  bl 0x82c05868
	ctx.lr = 0x82C54394;
	sub_82C05868(ctx, base);
	// 82C54394: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C54398: 4BFB1461  bl 0x82c057f8
	ctx.lr = 0x82C5439C;
	sub_82C057F8(ctx, base);
	// 82C5439C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C543A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C543A4: 4BFB1455  bl 0x82c057f8
	ctx.lr = 0x82C543A8;
	sub_82C057F8(ctx, base);
	// 82C543A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C543AC: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C543B0: 4BFB1711  bl 0x82c05ac0
	ctx.lr = 0x82C543B4;
	sub_82C05AC0(ctx, base);
	// 82C543B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C543B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C543BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C543C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C543C4: 4E800421  bctrl
	ctx.lr = 0x82C543C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C543C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C543CC: 4BFB13A5  bl 0x82c05770
	ctx.lr = 0x82C543D0;
	sub_82C05770(ctx, base);
	// 82C543D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C543D4: 4BFB135D  bl 0x82c05730
	ctx.lr = 0x82C543D8;
	sub_82C05730(ctx, base);
	// 82C543D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C543DC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C543E0: 388ACE40  addi r4, r10, -0x31c0
	ctx.r[4].s64 = ctx.r[10].s64 + -12736;
	// 82C543E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C543E8: 3BE9CE2C  addi r31, r9, -0x31d4
	ctx.r[31].s64 = ctx.r[9].s64 + -12756;
	// 82C543EC: 837E003C  lwz r27, 0x3c(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C543F0: 4BFB1409  bl 0x82c057f8
	ctx.lr = 0x82C543F4;
	sub_82C057F8(ctx, base);
	// 82C543F4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C543F8: 4BFB1541  bl 0x82c05938
	ctx.lr = 0x82C543FC;
	sub_82C05938(ctx, base);
	// 82C543FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C54400: 4BFB13F9  bl 0x82c057f8
	ctx.lr = 0x82C54404;
	sub_82C057F8(ctx, base);
	// 82C54404: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54408: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5440C: 4BFB13ED  bl 0x82c057f8
	ctx.lr = 0x82C54410;
	sub_82C057F8(ctx, base);
	// 82C54410: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54414: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54418: 4BFB16A9  bl 0x82c05ac0
	ctx.lr = 0x82C5441C;
	sub_82C05AC0(ctx, base);
	// 82C5441C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C54420: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54424: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C54428: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C5442C: 4E800421  bctrl
	ctx.lr = 0x82C54430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54430: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54434: 4BFB133D  bl 0x82c05770
	ctx.lr = 0x82C54438;
	sub_82C05770(ctx, base);
	// 82C54438: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5443C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C54440: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54444: 80C70014  lwz r6, 0x14(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C54448: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C5444C: 4E800421  bctrl
	ctx.lr = 0x82C54450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54450: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54454: 4BFB12DD  bl 0x82c05730
	ctx.lr = 0x82C54458;
	sub_82C05730(ctx, base);
	// 82C54458: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C5445C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C54460: 3885CE18  addi r4, r5, -0x31e8
	ctx.r[4].s64 = ctx.r[5].s64 + -12776;
	// 82C54464: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54468: 3BCBCE04  addi r30, r11, -0x31fc
	ctx.r[30].s64 = ctx.r[11].s64 + -12796;
	// 82C5446C: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C54470: 4BFB1389  bl 0x82c057f8
	ctx.lr = 0x82C54474;
	sub_82C057F8(ctx, base);
	// 82C54474: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C54478: 4BFB13F1  bl 0x82c05868
	ctx.lr = 0x82C5447C;
	sub_82C05868(ctx, base);
	// 82C5447C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C54480: 4BFB1379  bl 0x82c057f8
	ctx.lr = 0x82C54484;
	sub_82C057F8(ctx, base);
	// 82C54484: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54488: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5448C: 4BFB136D  bl 0x82c057f8
	ctx.lr = 0x82C54490;
	sub_82C057F8(ctx, base);
	// 82C54490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54494: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54498: 4BFB1629  bl 0x82c05ac0
	ctx.lr = 0x82C5449C;
	sub_82C05AC0(ctx, base);
	// 82C5449C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C544A0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C544A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C544A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C544AC: 4E800421  bctrl
	ctx.lr = 0x82C544B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C544B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C544B4: 4BFB12BD  bl 0x82c05770
	ctx.lr = 0x82C544B8;
	sub_82C05770(ctx, base);
	// 82C544B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C544BC: 4BFB1275  bl 0x82c05730
	ctx.lr = 0x82C544C0;
	sub_82C05730(ctx, base);
	// 82C544C0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C544C4: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C544C8: 3889CDF4  addi r4, r9, -0x320c
	ctx.r[4].s64 = ctx.r[9].s64 + -12812;
	// 82C544CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C544D0: 3BE8CDE4  addi r31, r8, -0x321c
	ctx.r[31].s64 = ctx.r[8].s64 + -12828;
	// 82C544D4: EBC10068  ld r30, 0x68(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82C544D8: 4BFB1321  bl 0x82c057f8
	ctx.lr = 0x82C544DC;
	sub_82C057F8(ctx, base);
	// 82C544DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C544E0: 4BFB13F1  bl 0x82c058d0
	ctx.lr = 0x82C544E4;
	sub_82C058D0(ctx, base);
	// 82C544E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C544E8: 4BFB1311  bl 0x82c057f8
	ctx.lr = 0x82C544EC;
	sub_82C057F8(ctx, base);
	// 82C544EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C544F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C544F4: 4BFB1305  bl 0x82c057f8
	ctx.lr = 0x82C544F8;
	sub_82C057F8(ctx, base);
	// 82C544F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C544FC: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54500: 4BFB15C1  bl 0x82c05ac0
	ctx.lr = 0x82C54504;
	sub_82C05AC0(ctx, base);
	// 82C54504: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C54508: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5450C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C54510: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C54514: 4E800421  bctrl
	ctx.lr = 0x82C54518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54518: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5451C: 4BFB1255  bl 0x82c05770
	ctx.lr = 0x82C54520;
	sub_82C05770(ctx, base);
	// 82C54520: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54524: 4BFB120D  bl 0x82c05730
	ctx.lr = 0x82C54528;
	sub_82C05730(ctx, base);
	// 82C54528: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C5452C: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C54530: 3886CDD4  addi r4, r6, -0x322c
	ctx.r[4].s64 = ctx.r[6].s64 + -12844;
	// 82C54534: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54538: 3BE5CDC4  addi r31, r5, -0x323c
	ctx.r[31].s64 = ctx.r[5].s64 + -12860;
	// 82C5453C: EBC10070  ld r30, 0x70(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82C54540: 4BFB12B9  bl 0x82c057f8
	ctx.lr = 0x82C54544;
	sub_82C057F8(ctx, base);
	// 82C54544: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C54548: 4BFB1389  bl 0x82c058d0
	ctx.lr = 0x82C5454C;
	sub_82C058D0(ctx, base);
	// 82C5454C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C54550: 4BFB12A9  bl 0x82c057f8
	ctx.lr = 0x82C54554;
	sub_82C057F8(ctx, base);
	// 82C54554: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5455C: 4BFB129D  bl 0x82c057f8
	ctx.lr = 0x82C54560;
	sub_82C057F8(ctx, base);
	// 82C54560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54564: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54568: 4BFB1559  bl 0x82c05ac0
	ctx.lr = 0x82C5456C;
	sub_82C05AC0(ctx, base);
	// 82C5456C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C54570: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54574: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C54578: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C5457C: 4E800421  bctrl
	ctx.lr = 0x82C54580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54584: 4BFB11ED  bl 0x82c05770
	ctx.lr = 0x82C54588;
	sub_82C05770(ctx, base);
	// 82C54588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5458C: 4BFB11A5  bl 0x82c05730
	ctx.lr = 0x82C54590;
	sub_82C05730(ctx, base);
	// 82C54590: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C54594: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C54598: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 82C5459C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C545A0: 3BC9CDA4  addi r30, r9, -0x325c
	ctx.r[30].s64 = ctx.r[9].s64 + -12892;
	// 82C545A4: 8BE10064  lbz r31, 0x64(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C545A8: 4BFB1251  bl 0x82c057f8
	ctx.lr = 0x82C545AC;
	sub_82C057F8(ctx, base);
	// 82C545AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C545B0: 4BFB1441  bl 0x82c059f0
	ctx.lr = 0x82C545B4;
	sub_82C059F0(ctx, base);
	// 82C545B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C545B8: 4BFB1241  bl 0x82c057f8
	ctx.lr = 0x82C545BC;
	sub_82C057F8(ctx, base);
	// 82C545BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C545C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C545C4: 4BFB1235  bl 0x82c057f8
	ctx.lr = 0x82C545C8;
	sub_82C057F8(ctx, base);
	// 82C545C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C545CC: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C545D0: 4BFB14F1  bl 0x82c05ac0
	ctx.lr = 0x82C545D4;
	sub_82C05AC0(ctx, base);
	// 82C545D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C545D8: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C545DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C545E0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C545E4: 4E800421  bctrl
	ctx.lr = 0x82C545E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C545E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C545EC: 4BFB1185  bl 0x82c05770
	ctx.lr = 0x82C545F0;
	sub_82C05770(ctx, base);
	// 82C545F0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C545F4: 48054E60  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C545F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C545F8 size=204
    let mut pc: u32 = 0x82C545F8;
    'dispatch: loop {
        match pc {
            0x82C545F8 => {
    //   block [0x82C545F8..0x82C546C4)
	// 82C545F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C545FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C54600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C54604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5460C: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C54610: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C54614: 419A009C  beq cr6, 0x82c546b0
	if ctx.cr[6].eq {
	pc = 0x82C546B0; continue 'dispatch;
	}
	// 82C54618: 813F0044  lwz r9, 0x44(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C5461C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C54620: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C54624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54628: 409A0038  bne cr6, 0x82c54660
	if !ctx.cr[6].eq {
	pc = 0x82C54660; continue 'dispatch;
	}
	// 82C5462C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54630: 7CCA49D6  mullw r6, r10, r9
	ctx.r[6].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C54634: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54638: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5463C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C54640: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82C54644: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54648: 4E800421  bctrl
	ctx.lr = 0x82C5464C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5464C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C54658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5465C: 4E800020  blr
	return;
	// 82C54660: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C54664: 41980020  blt cr6, 0x82c54684
	if ctx.cr[6].lt {
	pc = 0x82C54684; continue 'dispatch;
	}
	// 82C54668: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C5466C: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82C54670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5467C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C54680: 4E800020  blr
	return;
	// 82C54684: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54688: 7CAB49D6  mullw r5, r11, r9
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C5468C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54690: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C54694: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54698: 7CC55050  subf r6, r5, r10
	ctx.r[6].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 82C5469C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82C546A0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C546A4: 4E800421  bctrl
	ctx.lr = 0x82C546A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C546A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C546AC: 911F0040  stw r8, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 82C546B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C546B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C546B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C546BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C546C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C546C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C546C8 size=288
    let mut pc: u32 = 0x82C546C8;
    'dispatch: loop {
        match pc {
            0x82C546C8 => {
    //   block [0x82C546C8..0x82C547E8)
	// 82C546C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C546CC: 48054D39  bl 0x82ca9404
	ctx.lr = 0x82C546D0;
	sub_82CA93D0(ctx, base);
	// 82C546D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C546D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C546D8: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C546DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C546E0: 409A0020  bne cr6, 0x82c54700
	if !ctx.cr[6].eq {
	pc = 0x82C54700; continue 'dispatch;
	}
	// 82C546E4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C546E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C546EC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C546F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C546F4: 4E800421  bctrl
	ctx.lr = 0x82C546F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C546F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C546FC: 48054D58  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C54700: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C54704: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82C54708: 813F0044  lwz r9, 0x44(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C5470C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C54710: 553E083C  slwi r30, r9, 1
	ctx.r[30].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82C54714: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C54718: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5471C: 7F88F1D6  mullw r28, r8, r30
	ctx.r[28].s64 = (ctx.r[8].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82C54720: 80C70004  lwz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54724: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C54728: 4E800421  bctrl
	ctx.lr = 0x82C5472C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5472C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C54730: 7CDC1850  subf r6, r28, r3
	ctx.r[6].s64 = ctx.r[3].s64 - ctx.r[28].s64;
	// 82C54734: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54738: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5473C: 409A0038  bne cr6, 0x82c54774
	if !ctx.cr[6].eq {
	pc = 0x82C54774; continue 'dispatch;
	}
	// 82C54740: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54744: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C54748: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C5474C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54750: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54754: 4E800421  bctrl
	ctx.lr = 0x82C54758;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54758: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5475C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54760: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C54764: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C54768: 4E800421  bctrl
	ctx.lr = 0x82C5476C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5476C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C54770: 48054CE4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C54774: 7F66F396  divwu r27, r6, r30
	ctx.r[27].u32 = ctx.r[6].u32 / ctx.r[30].u32;
	// 82C54778: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C5477C: 41980028  blt cr6, 0x82c547a4
	if ctx.cr[6].lt {
	pc = 0x82C547A4; continue 'dispatch;
	}
	// 82C54780: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54784: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C54788: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C5478C: 4E800421  bctrl
	ctx.lr = 0x82C54790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54790: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C54794: 7D1B4850  subf r8, r27, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[27].s64;
	// 82C54798: 911F0040  stw r8, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 82C5479C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C547A0: 48054CB4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C547A4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C547A8: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82C547AC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C547B0: 7CCB3050  subf r6, r11, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 82C547B4: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82C547B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C547BC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C547C0: 4E800421  bctrl
	ctx.lr = 0x82C547C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C547C4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C547C8: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C547CC: 80E8000C  lwz r7, 0xc(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C547D0: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C547D4: 4E800421  bctrl
	ctx.lr = 0x82C547D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C547D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82C547DC: 90DF0040  stw r6, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[6].u32 ) };
	// 82C547E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C547E4: 48054C70  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C547E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C547E8 size=92
    let mut pc: u32 = 0x82C547E8;
    'dispatch: loop {
        match pc {
            0x82C547E8 => {
    //   block [0x82C547E8..0x82C54844)
	// 82C547E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C547EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C547F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C547F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C547F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C547FC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54800: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C54804: 419A0014  beq cr6, 0x82c54818
	if ctx.cr[6].eq {
	pc = 0x82C54818; continue 'dispatch;
	}
	// 82C54808: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5480C: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C54810: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54814: 4E800421  bctrl
	ctx.lr = 0x82C54818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54818: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5481C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C54820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C54824: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54828: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C5482C: 4E800421  bctrl
	ctx.lr = 0x82C54830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5483C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C54840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C54848 size=164
    let mut pc: u32 = 0x82C54848;
    'dispatch: loop {
        match pc {
            0x82C54848 => {
    //   block [0x82C54848..0x82C548EC)
	// 82C54848: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5484C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54850: 55490528  rlwinm r9, r10, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82C54854: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C54858: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5485C: 81040038  lwz r8, 0x38(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C54860: 91030058  stw r8, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82C54864: 80E4003C  lwz r7, 0x3c(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C54868: 90E3005C  stw r7, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82C5486C: C004001C  lfs f0, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C54870: D0030060  stfs f0, 0x60(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82C54874: 80C40028  lwz r6, 0x28(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C54878: 90C30054  stw r6, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82C5487C: 81650020  lwz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C54880: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82C54884: 8924004A  lbz r9, 0x4a(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(74 as u32) ) } as u64;
	// 82C54888: 99230084  stb r9, 0x84(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[9].u8 ) };
	// 82C5488C: 419A0044  beq cr6, 0x82c548d0
	if ctx.cr[6].eq {
	pc = 0x82C548D0; continue 'dispatch;
	}
	// 82C54890: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C54894: 39650014  addi r11, r5, 0x14
	ctx.r[11].s64 = ctx.r[5].s64 + 20;
	// 82C54898: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82C5489C: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 82C548A0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82C548A4: C0090C18  lfs f0, 0xc18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C548A8: 39200070  li r9, 0x70
	ctx.r[9].s64 = 112;
	// 82C548AC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82C548B0: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C548B4: 13CB4407  vcmpneb. (lvlx128) v30, v11, v8
	tmp.u32 = ctx.r[11].u32 + ctx.r[8].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C548B8: 13AB3407  vcmpneb. (lvlx128) v29, v11, v6
	tmp.u32 = ctx.r[11].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C548EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C548EC size=52
    let mut pc: u32 = 0x82C548EC;
    'dispatch: loop {
        match pc {
            0x82C548EC => {
    //   block [0x82C548EC..0x82C54920)
	// 82C548EC: C0050024  lfs f0, 0x24(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C548F0: D003004C  stfs f0, 0x4c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82C548F4: C1A50028  lfs f13, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C548F8: D1A30050  stfs f13, 0x50(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C548FC: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54900: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82C54904: 554A06B4  rlwinm r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82C54908: 81240014  lwz r9, 0x14(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C5490C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C54910: 91230038  stw r9, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82C54914: 409A000C  bne cr6, 0x82c54920
	if !ctx.cr[6].eq {
		sub_82C54920(ctx, base);
		return;
	}
	// 82C54918: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5491C: 48000008  b 0x82c54924
	sub_82C54920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C54920 size=36
    let mut pc: u32 = 0x82C54920;
    'dispatch: loop {
        match pc {
            0x82C54920 => {
    //   block [0x82C54920..0x82C54944)
	// 82C54920: 8165002C  lwz r11, 0x2c(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C54924: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82C54928: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C5492C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C54930: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C54934: 40980008  bge cr6, 0x82c5493c
	if !ctx.cr[6].lt {
	pc = 0x82C5493C; continue 'dispatch;
	}
	// 82C54938: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82C5493C: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82C54940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54948 size=192
    let mut pc: u32 = 0x82C54948;
    'dispatch: loop {
        match pc {
            0x82C54948 => {
    //   block [0x82C54948..0x82C54A08)
	// 82C54948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5494C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C54950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54954: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C54958: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C5495C: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C54960: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C54964: 419A008C  beq cr6, 0x82c549f0
	if ctx.cr[6].eq {
	pc = 0x82C549F0; continue 'dispatch;
	}
	// 82C54968: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82C5496C: 4198000C  blt cr6, 0x82c54978
	if ctx.cr[6].lt {
	pc = 0x82C54978; continue 'dispatch;
	}
	// 82C54970: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82C54974: 4099007C  ble cr6, 0x82c549f0
	if !ctx.cr[6].gt {
	pc = 0x82C549F0; continue 'dispatch;
	}
	// 82C54978: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C5497C: 409A0030  bne cr6, 0x82c549ac
	if !ctx.cr[6].eq {
	pc = 0x82C549AC; continue 'dispatch;
	}
	// 82C54980: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54984: 39200070  li r9, 0x70
	ctx.r[9].s64 = 112;
	// 82C54988: 888A0084  lbz r4, 0x84(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(132 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54A08 size=276
    let mut pc: u32 = 0x82C54A08;
    'dispatch: loop {
        match pc {
            0x82C54A08 => {
    //   block [0x82C54A08..0x82C54B1C)
	// 82C54A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54A0C: 480549F5  bl 0x82ca9400
	ctx.lr = 0x82C54A10;
	sub_82CA93D0(ctx, base);
	// 82C54A10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54A14: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C54A18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C54A1C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C54A20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C54A24: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C54A28: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C54A2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54A30: 419900E0  bgt cr6, 0x82c54b10
	if ctx.cr[6].gt {
	pc = 0x82C54B10; continue 'dispatch;
	}
	// 82C54A34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C54A38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C54A3C: 3B8BCD84  addi r28, r11, -0x327c
	ctx.r[28].s64 = ctx.r[11].s64 + -12924;
	// 82C54A40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54A44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54A48: 4B5D8489  bl 0x8222ced0
	ctx.lr = 0x82C54A4C;
	sub_8222CED0(ctx, base);
	// 82C54A4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54A50: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54A54: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82C54A58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C54A5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C54A60: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54A64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54A68: 4E800421  bctrl
	ctx.lr = 0x82C54A6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C54A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54A74: 4B5C0365  bl 0x82214dd8
	ctx.lr = 0x82C54A78;
	sub_82214DD8(ctx, base);
	// 82C54A78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C54A7C: 419A0028  beq cr6, 0x82c54aa4
	if ctx.cr[6].eq {
	pc = 0x82C54AA4; continue 'dispatch;
	}
	// 82C54A80: 897D0048  lbz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C54A84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54A88: 409A0084  bne cr6, 0x82c54b0c
	if !ctx.cr[6].eq {
	pc = 0x82C54B0C; continue 'dispatch;
	}
	// 82C54A8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54A90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54A94: 419A0078  beq cr6, 0x82c54b0c
	if ctx.cr[6].eq {
	pc = 0x82C54B0C; continue 'dispatch;
	}
	// 82C54A98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C54A9C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C54AA0: 480549B0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C54AA4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82C54AA8: 4B5CA7B1  bl 0x8221f258
	ctx.lr = 0x82C54AAC;
	sub_8221F258(ctx, base);
	// 82C54AAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C54AB0: 419A001C  beq cr6, 0x82c54acc
	if ctx.cr[6].eq {
	pc = 0x82C54ACC; continue 'dispatch;
	}
	// 82C54AB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C54AB8: 93630004  stw r27, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82C54ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C54AC0: 394BCD9C  addi r10, r11, -0x3264
	ctx.r[10].s64 = ctx.r[11].s64 + -12900;
	// 82C54AC4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C54AC8: 48000008  b 0x82c54ad0
	pc = 0x82C54AD0; continue 'dispatch;
	// 82C54ACC: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82C54AD0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C54AD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54ADC: 4B5D83F5  bl 0x8222ced0
	ctx.lr = 0x82C54AE0;
	sub_8222CED0(ctx, base);
	// 82C54AE0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54AE4: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54AE8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82C54AEC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82C54AF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C54AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C54AF8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C54AFC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54B00: 4E800421  bctrl
	ctx.lr = 0x82C54B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54B04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54B08: 4B5C02D1  bl 0x82214dd8
	ctx.lr = 0x82C54B0C;
	sub_82214DD8(ctx, base);
	// 82C54B0C: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C54B10: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C54B14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C54B18: 48054938  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54B20 size=96
    let mut pc: u32 = 0x82C54B20;
    'dispatch: loop {
        match pc {
            0x82C54B20 => {
    //   block [0x82C54B20..0x82C54B80)
	// 82C54B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C54B28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C54B2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54B30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C54B34: 4BFBA0ED  bl 0x82c0ec20
	ctx.lr = 0x82C54B38;
	sub_82C0EC20(ctx, base);
	// 82C54B38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C54B3C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C54B40: 392BCEBC  addi r9, r11, -0x3144
	ctx.r[9].s64 = ctx.r[11].s64 + -12612;
	// 82C54B44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C54B48: 390AC304  addi r8, r10, -0x3cfc
	ctx.r[8].s64 = ctx.r[10].s64 + -15612;
	// 82C54B4C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C54B50: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C54B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C54B58: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C54B5C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C54B60: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C54B64: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C54B68: 93FF0008  stw r31, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C54B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C54B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C54B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C54B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C54B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54B80 size=236
    let mut pc: u32 = 0x82C54B80;
    'dispatch: loop {
        match pc {
            0x82C54B80 => {
    //   block [0x82C54B80..0x82C54C6C)
	// 82C54B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54B84: 48054885  bl 0x82ca9408
	ctx.lr = 0x82C54B88;
	sub_82CA93D0(ctx, base);
	// 82C54B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54B8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C54B90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C54B94: 394BCED0  addi r10, r11, -0x3130
	ctx.r[10].s64 = ctx.r[11].s64 + -12592;
	// 82C54B98: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C54B9C: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C54BA0: 4BFB5701  bl 0x82c0a2a0
	ctx.lr = 0x82C54BA4;
	sub_82C0A2A0(ctx, base);
	// 82C54BA4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C54BA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C54BAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54BB0: 419A000C  beq cr6, 0x82c54bbc
	if ctx.cr[6].eq {
	pc = 0x82C54BBC; continue 'dispatch;
	}
	// 82C54BB4: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C54BB8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82C54BBC: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 82C54BC0: 4B634E89  bl 0x82289a48
	ctx.lr = 0x82C54BC4;
	sub_82289A48(ctx, base);
	// 82C54BC4: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 82C54BC8: 4B5C0211  bl 0x82214dd8
	ctx.lr = 0x82C54BCC;
	sub_82214DD8(ctx, base);
	// 82C54BCC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82C54BD0: 4B5C0209  bl 0x82214dd8
	ctx.lr = 0x82C54BD4;
	sub_82214DD8(ctx, base);
	// 82C54BD4: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 82C54BD8: 4B5C0201  bl 0x82214dd8
	ctx.lr = 0x82C54BDC;
	sub_82214DD8(ctx, base);
	// 82C54BDC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C54BE0: 3B9F0020  addi r28, r31, 0x20
	ctx.r[28].s64 = ctx.r[31].s64 + 32;
	// 82C54BE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C54BE8: 419A0014  beq cr6, 0x82c54bfc
	if ctx.cr[6].eq {
	pc = 0x82C54BFC; continue 'dispatch;
	}
	// 82C54BEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54BF0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54BF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54BF8: 4E800421  bctrl
	ctx.lr = 0x82C54BFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54BFC: 93DC0004  stw r30, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C54C00: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 82C54C04: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C54C08: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C54C0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C54C10: 419A0014  beq cr6, 0x82c54c24
	if ctx.cr[6].eq {
	pc = 0x82C54C24; continue 'dispatch;
	}
	// 82C54C14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54C18: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54C1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54C20: 4E800421  bctrl
	ctx.lr = 0x82C54C24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54C24: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C54C28: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 82C54C2C: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C54C30: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C54C34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C54C38: 419A0014  beq cr6, 0x82c54c4c
	if ctx.cr[6].eq {
	pc = 0x82C54C4C; continue 'dispatch;
	}
	// 82C54C3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54C40: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54C44: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54C48: 4E800421  bctrl
	ctx.lr = 0x82C54C4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54C4C: 93DC0004  stw r30, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C54C50: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C54C54: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C54C58: 480017D9  bl 0x82c56430
	ctx.lr = 0x82C54C5C;
	sub_82C56430(ctx, base);
	// 82C54C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C54C60: 4BFB14B1  bl 0x82c06110
	ctx.lr = 0x82C54C64;
	sub_82C06110(ctx, base);
	// 82C54C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C54C68: 480547F0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C54C70 size=420
    let mut pc: u32 = 0x82C54C70;
    'dispatch: loop {
        match pc {
            0x82C54C70 => {
    //   block [0x82C54C70..0x82C54E14)
	// 82C54C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54C74: 4805478D  bl 0x82ca9400
	ctx.lr = 0x82C54C78;
	sub_82CA93D0(ctx, base);
	// 82C54C78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C54C80: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82C54C84: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82C54C88: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82C54C8C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C54C90: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C54C94: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82C54C98: 409A0048  bne cr6, 0x82c54ce0
	if !ctx.cr[6].eq {
	pc = 0x82C54CE0; continue 'dispatch;
	}
	// 82C54C9C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C54CA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54CA4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C54CA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54CAC: 4E800421  bctrl
	ctx.lr = 0x82C54CB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54CB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C54CB4: 419A002C  beq cr6, 0x82c54ce0
	if ctx.cr[6].eq {
	pc = 0x82C54CE0; continue 'dispatch;
	}
	// 82C54CB8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C54CBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54CC0: 40990020  ble cr6, 0x82c54ce0
	if !ctx.cr[6].gt {
	pc = 0x82C54CE0; continue 'dispatch;
	}
	// 82C54CC4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C54CC8: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C54CCC: 1D2B03E8  mulli r9, r11, 0x3e8
	ctx.r[9].s64 = ctx.r[11].s64 * 1000;
	// 82C54CD0: 7D095396  divwu r8, r9, r10
	ctx.r[8].u32 = ctx.r[9].u32 / ctx.r[10].u32;
	// 82C54CD4: 2B0803E8  cmplwi cr6, r8, 0x3e8
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1000 as u32, &mut ctx.xer);
	// 82C54CD8: 40980008  bge cr6, 0x82c54ce0
	if !ctx.cr[6].lt {
	pc = 0x82C54CE0; continue 'dispatch;
	}
	// 82C54CDC: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82C54CE0: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C54CE4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C54CE8: 555E083C  slwi r30, r10, 1
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82C54CEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54CF0: 7F6BF1D6  mullw r27, r11, r30
	ctx.r[27].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82C54CF4: 409A0008  bne cr6, 0x82c54cfc
	if !ctx.cr[6].eq {
	pc = 0x82C54CFC; continue 'dispatch;
	}
	// 82C54CF8: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82C54CFC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C54D00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54D04: 419A000C  beq cr6, 0x82c54d10
	if ctx.cr[6].eq {
	pc = 0x82C54D10; continue 'dispatch;
	}
	// 82C54D08: 7CCBF1D6  mullw r6, r11, r30
	ctx.r[6].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82C54D0C: 4800001C  b 0x82c54d28
	pc = 0x82C54D28; continue 'dispatch;
	// 82C54D10: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C54D14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54D18: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54D1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54D20: 4E800421  bctrl
	ctx.lr = 0x82C54D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54D24: 7CDB1850  subf r6, r27, r3
	ctx.r[6].s64 = ctx.r[3].s64 - ctx.r[27].s64;
	// 82C54D28: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C54D2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54D30: 409A0058  bne cr6, 0x82c54d88
	if !ctx.cr[6].eq {
	pc = 0x82C54D88; continue 'dispatch;
	}
	// 82C54D34: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82C54D38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54D3C: 419A000C  beq cr6, 0x82c54d48
	if ctx.cr[6].eq {
	pc = 0x82C54D48; continue 'dispatch;
	}
	// 82C54D40: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82C54D44: 48000014  b 0x82c54d58
	pc = 0x82C54D58; continue 'dispatch;
	// 82C54D48: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82C54D4C: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82C54D50: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 82C54D54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C54D58: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54D5C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C54D60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54D64: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54D68: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54D6C: 4E800421  bctrl
	ctx.lr = 0x82C54D70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54D70: 57A907FE  clrlwi r9, r29, 0x1f
	ctx.r[9].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82C54D74: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C54D78: 419A0080  beq cr6, 0x82c54df8
	if ctx.cr[6].eq {
	pc = 0x82C54DF8; continue 'dispatch;
	}
	// 82C54D7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C54D80: 4BFB1179  bl 0x82c05ef8
	ctx.lr = 0x82C54D84;
	sub_82C05EF8(ctx, base);
	// 82C54D84: 48000074  b 0x82c54df8
	pc = 0x82C54DF8; continue 'dispatch;
	// 82C54D88: 7D46F396  divwu r10, r6, r30
	ctx.r[10].u32 = ctx.r[6].u32 / ctx.r[30].u32;
	// 82C54D8C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C54D90: 4198003C  blt cr6, 0x82c54dcc
	if ctx.cr[6].lt {
	pc = 0x82C54DCC; continue 'dispatch;
	}
	// 82C54D94: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C54D98: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C54D9C: 7D0A4850  subf r8, r10, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82C54DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C54DA4: 911F0040  stw r8, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[8].u32 ) };
	// 82C54DA8: 4099000C  ble cr6, 0x82c54db4
	if !ctx.cr[6].gt {
	pc = 0x82C54DB4; continue 'dispatch;
	}
	// 82C54DAC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C54DB0: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82C54DB4: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C54DB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C54DBC: 419A0050  beq cr6, 0x82c54e0c
	if ctx.cr[6].eq {
	pc = 0x82C54E0C; continue 'dispatch;
	}
	// 82C54DC0: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C54DC4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C54DC8: 4098FFCC  bge cr6, 0x82c54d94
	if !ctx.cr[6].lt {
	pc = 0x82C54D94; continue 'dispatch;
	}
	// 82C54DCC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54DD0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82C54DD4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C54DD8: 7D6BF1D6  mullw r11, r11, r30
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82C54DDC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54DE0: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C54DE4: 7CCB3050  subf r6, r11, r6
	ctx.r[6].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 82C54DE8: 7CABDA14  add r5, r11, r27
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82C54DEC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C54DF0: 4E800421  bctrl
	ctx.lr = 0x82C54DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54DF4: 935F0040  stw r26, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[26].u32 ) };
	// 82C54DF8: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C54DFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C54E00: 4099000C  ble cr6, 0x82c54e0c
	if !ctx.cr[6].gt {
	pc = 0x82C54E0C; continue 'dispatch;
	}
	// 82C54E04: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C54E08: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82C54E0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C54E10: 48054640  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C54E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C54E18 size=748
    let mut pc: u32 = 0x82C54E18;
    'dispatch: loop {
        match pc {
            0x82C54E18 => {
    //   block [0x82C54E18..0x82C55104)
	// 82C54E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C54E1C: 480545DD  bl 0x82ca93f8
	ctx.lr = 0x82C54E20;
	sub_82CA93D0(ctx, base);
	// 82C54E20: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C54E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C54E28: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82C54E2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C54E30: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C54E34: 9B1F00F8  stb r24, 0xf8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[24].u8 ) };
	// 82C54E38: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54E3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C54E40: 409A0010  bne cr6, 0x82c54e50
	if !ctx.cr[6].eq {
	pc = 0x82C54E50; continue 'dispatch;
	}
	// 82C54E44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C54E48: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C54E4C: 480545FC  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82C54E50: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54E54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C54E58: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C54E5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54E60: 4E800421  bctrl
	ctx.lr = 0x82C54E64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54E64: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82C54E68: 409AFFDC  bne cr6, 0x82c54e44
	if !ctx.cr[6].eq {
	pc = 0x82C54E44; continue 'dispatch;
	}
	// 82C54E6C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C54E70: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C54E74: 4198000C  blt cr6, 0x82c54e80
	if ctx.cr[6].lt {
	pc = 0x82C54E80; continue 'dispatch;
	}
	// 82C54E78: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C54E7C: 40990014  ble cr6, 0x82c54e90
	if !ctx.cr[6].gt {
	pc = 0x82C54E90; continue 'dispatch;
	}
	// 82C54E80: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 82C54E84: 41980014  blt cr6, 0x82c54e98
	if ctx.cr[6].lt {
	pc = 0x82C54E98; continue 'dispatch;
	}
	// 82C54E88: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82C54E8C: 4199000C  bgt cr6, 0x82c54e98
	if ctx.cr[6].gt {
	pc = 0x82C54E98; continue 'dispatch;
	}
	// 82C54E90: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82C54E94: 48000008  b 0x82c54e9c
	pc = 0x82C54E9C; continue 'dispatch;
	// 82C54E98: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 82C54E9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54EA0: 3B9F0048  addi r28, r31, 0x48
	ctx.r[28].s64 = ctx.r[31].s64 + 72;
	// 82C54EA4: 837F009C  lwz r27, 0x9c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C54EA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C54EAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C54EB0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C54EB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54EB8: 4E800421  bctrl
	ctx.lr = 0x82C54EBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54EBC: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54EC0: 3B3F0044  addi r25, r31, 0x44
	ctx.r[25].s64 = ctx.r[31].s64 + 68;
	// 82C54EC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C54EC8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82C54ECC: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54ED0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C54ED4: 4E800421  bctrl
	ctx.lr = 0x82C54ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54ED8: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C54EDC: 80BF0048  lwz r5, 0x48(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C54EE0: 57A7063E  clrlwi r7, r29, 0x18
	ctx.r[7].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82C54EE4: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 82C54EE8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C54EEC: 90DF00E4  stw r6, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[6].u32 ) };
	// 82C54EF0: 90BF00E0  stw r5, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[5].u32 ) };
	// 82C54EF4: 409A001C  bne cr6, 0x82c54f10
	if !ctx.cr[6].eq {
	pc = 0x82C54F10; continue 'dispatch;
	}
	// 82C54EF8: 576B0420  rlwinm r11, r27, 0, 0x10, 0x10
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 82C54EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54F00: 419A0010  beq cr6, 0x82c54f10
	if ctx.cr[6].eq {
	pc = 0x82C54F10; continue 'dispatch;
	}
	// 82C54F04: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82C54F08: 616A0020  ori r10, r11, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 | 32;
	// 82C54F0C: 915F00E8  stw r10, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82C54F10: 897F0085  lbz r11, 0x85(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(133 as u32) ) } as u64;
	// 82C54F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54F18: 419A0010  beq cr6, 0x82c54f28
	if ctx.cr[6].eq {
	pc = 0x82C54F28; continue 'dispatch;
	}
	// 82C54F1C: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82C54F20: 616A0010  ori r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u64 | 16;
	// 82C54F24: 915F00E8  stw r10, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82C54F28: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C54F2C: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82C54F30: 4BFBC439  bl 0x82c11368
	ctx.lr = 0x82C54F34;
	sub_82C11368(ctx, base);
	// 82C54F34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C54F38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C54F3C: 4BFBCD8D  bl 0x82c11cc8
	ctx.lr = 0x82C54F40;
	sub_82C11CC8(ctx, base);
	// 82C54F40: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C54F44: 4BFB0FB5  bl 0x82c05ef8
	ctx.lr = 0x82C54F48;
	sub_82C05EF8(ctx, base);
	// 82C54F48: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C54F4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C54F50: 419AFEF4  beq cr6, 0x82c54e44
	if ctx.cr[6].eq {
	pc = 0x82C54E44; continue 'dispatch;
	}
	// 82C54F54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C54F58: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C54F5C: 48001395  bl 0x82c562f0
	ctx.lr = 0x82C54F60;
	sub_82C562F0(ctx, base);
	// 82C54F60: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54F64: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 82C54F68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C54F6C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54F70: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C54F74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54F78: 4E800421  bctrl
	ctx.lr = 0x82C54F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54F7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C54F80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C54F84: 4BFBCD45  bl 0x82c11cc8
	ctx.lr = 0x82C54F88;
	sub_82C11CC8(ctx, base);
	// 82C54F88: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C54F8C: 4BFB0F6D  bl 0x82c05ef8
	ctx.lr = 0x82C54F90;
	sub_82C05EF8(ctx, base);
	// 82C54F90: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C54F94: C03F0060  lfs f1, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C54F98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C54F9C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54FA0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54FA4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C54FA8: 4E800421  bctrl
	ctx.lr = 0x82C54FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54FAC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54FB0: C03F0064  lfs f1, 0x64(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C54FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C54FB8: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54FBC: 80C70018  lwz r6, 0x18(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C54FC0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C54FC4: 4E800421  bctrl
	ctx.lr = 0x82C54FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54FC8: 5745063E  clrlwi r5, r26, 0x18
	ctx.r[5].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	// 82C54FCC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C54FD0: 419A0080  beq cr6, 0x82c55050
	if ctx.cr[6].eq {
	pc = 0x82C55050; continue 'dispatch;
	}
	// 82C54FD4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54FD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C54FDC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54FE0: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C54FE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C54FE8: 4E800421  bctrl
	ctx.lr = 0x82C54FEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C54FEC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C54FF0: C05F0050  lfs f2, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C54FF4: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C54FF8: C03F004C  lfs f1, 0x4c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C54FFC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55000: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55004: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C55008: 4E800421  bctrl
	ctx.lr = 0x82C5500C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5500C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C55010: 4BFB0EE9  bl 0x82c05ef8
	ctx.lr = 0x82C55014;
	sub_82C05EF8(ctx, base);
	// 82C55014: 57670528  rlwinm r7, r27, 0, 0x14, 0x14
	ctx.r[7].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 82C55018: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C5501C: 419A0034  beq cr6, 0x82c55050
	if ctx.cr[6].eq {
	pc = 0x82C55050; continue 'dispatch;
	}
	// 82C55020: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55024: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 82C55028: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C5502C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C55030: 392BFFFE  addi r9, r11, -2
	ctx.r[9].s64 = ctx.r[11].s64 + -2;
	// 82C55034: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55038: 7D270034  cntlzw r7, r9
	ctx.r[7].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55108 size=144
    let mut pc: u32 = 0x82C55108;
    'dispatch: loop {
        match pc {
            0x82C55108 => {
    //   block [0x82C55108..0x82C55198)
	// 82C55108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5510C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C55110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C55114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C55118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5511C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C55120: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C55124: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C55128: 394BCEBC  addi r10, r11, -0x3144
	ctx.r[10].s64 = ctx.r[11].s64 + -12612;
	// 82C5512C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C55130: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55138: 419A001C  beq cr6, 0x82c55154
	if ctx.cr[6].eq {
	pc = 0x82C55154; continue 'dispatch;
	}
	// 82C5513C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55140: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C55144: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55148: 4E800421  bctrl
	ctx.lr = 0x82C5514C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5514C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C55150: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C55154: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C55158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5515C: 394BA37C  addi r10, r11, -0x5c84
	ctx.r[10].s64 = ctx.r[11].s64 + -23684;
	// 82C55160: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C55164: 4BFB9ABD  bl 0x82c0ec20
	ctx.lr = 0x82C55168;
	sub_82C0EC20(ctx, base);
	// 82C55168: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C5516C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55170: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C55174: 419A000C  beq cr6, 0x82c55180
	if ctx.cr[6].eq {
	pc = 0x82C55180; continue 'dispatch;
	}
	// 82C55178: 4BBF0639  bl 0x828457b0
	ctx.lr = 0x82C5517C;
	sub_828457B0(ctx, base);
	// 82C5517C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C55184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C55188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5518C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C55190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C55194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55198 size=80
    let mut pc: u32 = 0x82C55198;
    'dispatch: loop {
        match pc {
            0x82C55198 => {
    //   block [0x82C55198..0x82C551E8)
	// 82C55198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5519C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C551A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C551A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C551A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C551AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C551B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C551B4: 4BFFF9CD  bl 0x82c54b80
	ctx.lr = 0x82C551B8;
	sub_82C54B80(ctx, base);
	// 82C551B8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C551BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C551C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C551C4: 419A000C  beq cr6, 0x82c551d0
	if ctx.cr[6].eq {
	pc = 0x82C551D0; continue 'dispatch;
	}
	// 82C551C8: 4BBF05E9  bl 0x828457b0
	ctx.lr = 0x82C551CC;
	sub_828457B0(ctx, base);
	// 82C551CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C551D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C551D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C551D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C551DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C551E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C551E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C551E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C551E8 size=448
    let mut pc: u32 = 0x82C551E8;
    'dispatch: loop {
        match pc {
            0x82C551E8 => {
    //   block [0x82C551E8..0x82C553A8)
	// 82C551E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C551EC: 48054221  bl 0x82ca940c
	ctx.lr = 0x82C551F0;
	sub_82CA93D0(ctx, base);
	// 82C551F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C551F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C551F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C551FC: 897F00F8  lbz r11, 0xf8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 82C55200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55204: 419A0064  beq cr6, 0x82c55268
	if ctx.cr[6].eq {
	pc = 0x82C55268; continue 'dispatch;
	}
	// 82C55208: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C5520C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55214: 409A0014  bne cr6, 0x82c55228
	if !ctx.cr[6].eq {
	pc = 0x82C55228; continue 'dispatch;
	}
	// 82C55218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5521C: 997F00F8  stb r11, 0xf8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u8 ) };
	// 82C55220: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C55224: 48054238  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C55228: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5522C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C55230: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55234: 4E800421  bctrl
	ctx.lr = 0x82C55238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55238: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82C5523C: 41980164  blt cr6, 0x82c553a0
	if ctx.cr[6].lt {
	pc = 0x82C553A0; continue 'dispatch;
	}
	// 82C55240: 419A0018  beq cr6, 0x82c55258
	if ctx.cr[6].eq {
	pc = 0x82C55258; continue 'dispatch;
	}
	// 82C55244: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82C55248: 40980020  bge cr6, 0x82c55268
	if !ctx.cr[6].lt {
	pc = 0x82C55268; continue 'dispatch;
	}
	// 82C5524C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C55250: 997F00F8  stb r11, 0xf8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u8 ) };
	// 82C55254: 48000014  b 0x82c55268
	pc = 0x82C55268; continue 'dispatch;
	// 82C55258: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C5525C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C55260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55264: 4BFFFBB5  bl 0x82c54e18
	ctx.lr = 0x82C55268;
	sub_82C54E18(ctx, base);
	// 82C55268: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5526C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55270: 419A0130  beq cr6, 0x82c553a0
	if ctx.cr[6].eq {
	pc = 0x82C553A0; continue 'dispatch;
	}
	// 82C55274: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C55278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5527C: 4BFFF6CD  bl 0x82c54948
	ctx.lr = 0x82C55280;
	sub_82C54948(ctx, base);
	// 82C55280: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 82C55284: C05F0050  lfs f2, 0x50(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C55288: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C5528C: C03F004C  lfs f1, 0x4c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C55290: 48001871  bl 0x82c56b00
	ctx.lr = 0x82C55294;
	sub_82C56B00(ctx, base);
	// 82C55294: 3BDF0070  addi r30, r31, 0x70
	ctx.r[30].s64 = ctx.r[31].s64 + 112;
	// 82C55298: 80DF0088  lwz r6, 0x88(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C5529C: 393F0094  addi r9, r31, 0x94
	ctx.r[9].s64 = ctx.r[31].s64 + 148;
	// 82C552A0: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C552A4: 391F0090  addi r8, r31, 0x90
	ctx.r[8].s64 = ctx.r[31].s64 + 144;
	// 82C552A8: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C552AC: 38FF008C  addi r7, r31, 0x8c
	ctx.r[7].s64 = ctx.r[31].s64 + 140;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C553A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C553A8 size=1296
    let mut pc: u32 = 0x82C553A8;
    'dispatch: loop {
        match pc {
            0x82C553A8 => {
    //   block [0x82C553A8..0x82C55704)
	// 82C553A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C553AC: 48054041  bl 0x82ca93ec
	ctx.lr = 0x82C553B0;
	sub_82CA93D0(ctx, base);
	// 82C553B0: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82C553B4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C553B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C553BC: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82C553C0: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82C553C4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82C553C8: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 82C553CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C553D0: 4B57FF19  bl 0x821d52e8
	ctx.lr = 0x82C553D4;
	sub_821D52E8(ctx, base);
	// 82C553D4: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C553D8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82C553DC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82C553E0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C553E4: 82B80000  lwz r21, 0(r24)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C553E8: 4198000C  blt cr6, 0x82c553f4
	if ctx.cr[6].lt {
	pc = 0x82C553F4; continue 'dispatch;
	}
	// 82C553EC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C553F0: 40990014  ble cr6, 0x82c55404
	if !ctx.cr[6].gt {
	pc = 0x82C55404; continue 'dispatch;
	}
	// 82C553F4: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 82C553F8: 41980014  blt cr6, 0x82c5540c
	if ctx.cr[6].lt {
	pc = 0x82C5540C; continue 'dispatch;
	}
	// 82C553FC: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82C55400: 4199000C  bgt cr6, 0x82c5540c
	if ctx.cr[6].gt {
	pc = 0x82C5540C; continue 'dispatch;
	}
	// 82C55404: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82C55408: 48000008  b 0x82c55410
	pc = 0x82C55410; continue 'dispatch;
	// 82C5540C: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82C55410: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C55414: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55418: 419A00B4  beq cr6, 0x82c554cc
	if ctx.cr[6].eq {
	pc = 0x82C554CC; continue 'dispatch;
	}
	// 82C5541C: 56AB0528  rlwinm r11, r21, 0, 0x14, 0x14
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0xFFFFFFFFu64;
	// 82C55420: 933F00E8  stw r25, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[25].u32 ) };
	// 82C55424: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55428: 419A00A4  beq cr6, 0x82c554cc
	if ctx.cr[6].eq {
	pc = 0x82C554CC; continue 'dispatch;
	}
	// 82C5542C: C01F004C  lfs f0, 0x4c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C55430: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82C55434: C1BF0050  lfs f13, 0x50(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C55438: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82C5543C: 40990008  ble cr6, 0x82c55444
	if !ctx.cr[6].gt {
	pc = 0x82C55444; continue 'dispatch;
	}
	// 82C55440: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82C55444: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C55448: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82C5544C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55454: 419A001C  beq cr6, 0x82c55470
	if ctx.cr[6].eq {
	pc = 0x82C55470; continue 'dispatch;
	}
	// 82C55458: 4BFFF4F1  bl 0x82c54948
	ctx.lr = 0x82C5545C;
	sub_82C54948(ctx, base);
	// 82C5545C: C01F0050  lfs f0, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C55460: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C55464: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82C55468: 40980020  bge cr6, 0x82c55488
	if !ctx.cr[6].lt {
	pc = 0x82C55488; continue 'dispatch;
	}
	// 82C5546C: 48000018  b 0x82c55484
	pc = 0x82C55484; continue 'dispatch;
	// 82C55470: 4BFFF4D9  bl 0x82c54948
	ctx.lr = 0x82C55474;
	sub_82C54948(ctx, base);
	// 82C55474: C01F0050  lfs f0, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C55478: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C5547C: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82C55480: 40990008  ble cr6, 0x82c55488
	if !ctx.cr[6].gt {
	pc = 0x82C55488; continue 'dispatch;
	}
	// 82C55484: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82C55488: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C5548C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C55490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55494: 419A0038  beq cr6, 0x82c554cc
	if ctx.cr[6].eq {
	pc = 0x82C554CC; continue 'dispatch;
	}
	// 82C55498: 897B004C  lbz r11, 0x4c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C5549C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C554A0: 409A0020  bne cr6, 0x82c554c0
	if !ctx.cr[6].eq {
	pc = 0x82C554C0; continue 'dispatch;
	}
	// 82C554A4: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C554A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C554AC: 41980014  blt cr6, 0x82c554c0
	if ctx.cr[6].lt {
	pc = 0x82C554C0; continue 'dispatch;
	}
	// 82C554B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C554B4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82C554B8: CBE1FF98  lfd f31, -0x68(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 82C554BC: 48053F80  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82C554C0: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82C554C4: 616A0008  ori r10, r11, 8
	ctx.r[10].u64 = ctx.r[11].u64 | 8;
	// 82C554C8: 915F00E8  stw r10, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82C554CC: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C554D0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82C554D4: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C554D8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C554DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C554E0: 4E800421  bctrl
	ctx.lr = 0x82C554E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C554E4: 9B3F00F8  stb r25, 0xf8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[25].u8 ) };
	// 82C554E8: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C554EC: 4BFB565D  bl 0x82c0ab48
	ctx.lr = 0x82C554F0;
	sub_82C0AB48(ctx, base);
	// 82C554F0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C554F4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82C554F8: 419AFFB8  beq cr6, 0x82c554b0
	if ctx.cr[6].eq {
	pc = 0x82C554B0; continue 'dispatch;
	}
	// 82C554FC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C55500: 4B5C9D59  bl 0x8221f258
	ctx.lr = 0x82C55504;
	sub_8221F258(ctx, base);
	// 82C55504: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55508: 419A0010  beq cr6, 0x82c55518
	if ctx.cr[6].eq {
	pc = 0x82C55518; continue 'dispatch;
	}
	// 82C5550C: 4BFFF615  bl 0x82c54b20
	ctx.lr = 0x82C55510;
	sub_82C54B20(ctx, base);
	// 82C55510: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C55514: 48000008  b 0x82c5551c
	pc = 0x82C5551C; continue 'dispatch;
	// 82C55518: 7EFCBB78  mr r28, r23
	ctx.r[28].u64 = ctx.r[23].u64;
	// 82C5551C: 37DC0004  addic. r30, r28, 4
	ctx.xer.ca = (ctx.r[28].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C55520: 935C0010  stw r26, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 82C55524: 3BBF0020  addi r29, r31, 0x20
	ctx.r[29].s64 = ctx.r[31].s64 + 32;
	// 82C55528: 41820018  beq 0x82c55540
	if ctx.cr[0].eq {
	pc = 0x82C55540; continue 'dispatch;
	}
	// 82C5552C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55530: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C55534: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55538: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C5553C: 4E800421  bctrl
	ctx.lr = 0x82C55540;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55540: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55544: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55548: 419A0014  beq cr6, 0x82c5555c
	if ctx.cr[6].eq {
	pc = 0x82C5555C; continue 'dispatch;
	}
	// 82C5554C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55550: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55554: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55558: 4E800421  bctrl
	ctx.lr = 0x82C5555C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5555C: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C55560: 3BDF008C  addi r30, r31, 0x8c
	ctx.r[30].s64 = ctx.r[31].s64 + 140;
	// 82C55564: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C55568: 38980004  addi r4, r24, 4
	ctx.r[4].s64 = ctx.r[24].s64 + 4;
	// 82C5556C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C55574: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82C55578: 4B60FC29  bl 0x822651a0
	ctx.lr = 0x82C5557C;
	sub_822651A0(ctx, base);
	// 82C5557C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C55580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55584: 419A0010  beq cr6, 0x82c55594
	if ctx.cr[6].eq {
	pc = 0x82C55594; continue 'dispatch;
	}
	// 82C55588: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5558C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C55590: 409A0038  bne cr6, 0x82c555c8
	if !ctx.cr[6].eq {
	pc = 0x82C555C8; continue 'dispatch;
	}
	// 82C55594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C55598: 83B80008  lwz r29, 8(r24)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5559C: 4B5A2265  bl 0x821f7800
	ctx.lr = 0x82C555A0;
	sub_821F7800(ctx, base);
	// 82C555A0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82C555A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C555A8: 388B8864  addi r4, r11, -0x779c
	ctx.r[4].s64 = ctx.r[11].s64 + -30620;
	// 82C555AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C555B0: 4B58E8A1  bl 0x821e3e50
	ctx.lr = 0x82C555B4;
	sub_821E3E50(ctx, base);
	// 82C555B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C555B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C555BC: 4B60FBE5  bl 0x822651a0
	ctx.lr = 0x82C555C0;
	sub_822651A0(ctx, base);
	// 82C555C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C555C4: 4B5BF815  bl 0x82214dd8
	ctx.lr = 0x82C555C8;
	sub_82214DD8(ctx, base);
	// 82C555C8: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C555CC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82C555D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C555D4: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C555D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C555DC: 4E800421  bctrl
	ctx.lr = 0x82C555E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C555E0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C555E4: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C555E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C555EC: 419A0018  beq cr6, 0x82c55604
	if ctx.cr[6].eq {
	pc = 0x82C55604; continue 'dispatch;
	}
	// 82C555F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C555F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C555F8: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C555FC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C55600: 4E800421  bctrl
	ctx.lr = 0x82C55604;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55604: 92E1005C  stw r23, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[23].u32 ) };
	// 82C55608: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C5560C: 92E10058  stw r23, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[23].u32 ) };
	// 82C55610: 419A0068  beq cr6, 0x82c55678
	if ctx.cr[6].eq {
	pc = 0x82C55678; continue 'dispatch;
	}
	// 82C55614: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C55618: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C5561C: 4BFBC2A5  bl 0x82c118c0
	ctx.lr = 0x82C55620;
	sub_82C118C0(ctx, base);
	// 82C55620: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C55624: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82C55628: 4B60FB79  bl 0x822651a0
	ctx.lr = 0x82C5562C;
	sub_822651A0(ctx, base);
	// 82C5562C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C55630: 4B5BF7A9  bl 0x82214dd8
	ctx.lr = 0x82C55634;
	sub_82214DD8(ctx, base);
	// 82C55634: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C55638: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C5563C: 4BFBE31D  bl 0x82c13958
	ctx.lr = 0x82C55640;
	sub_82C13958(ctx, base);
	// 82C55640: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C55644: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C55648: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5564C: 4BFB08AD  bl 0x82c05ef8
	ctx.lr = 0x82C55650;
	sub_82C05EF8(ctx, base);
	// 82C55650: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C55654: 419A0024  beq cr6, 0x82c55678
	if ctx.cr[6].eq {
	pc = 0x82C55678; continue 'dispatch;
	}
	// 82C55658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C5565C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C55660: 4BFBC2A1  bl 0x82c11900
	ctx.lr = 0x82C55664;
	sub_82C11900(ctx, base);
	// 82C55664: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C55668: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 82C5566C: 4B60FB35  bl 0x822651a0
	ctx.lr = 0x82C55670;
	sub_822651A0(ctx, base);
	// 82C55670: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C55674: 4B5BF765  bl 0x82214dd8
	ctx.lr = 0x82C55678;
	sub_82214DD8(ctx, base);
	// 82C55678: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 82C5567C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82C55680: 811F0058  lwz r8, 0x58(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C55684: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82C55688: 2B0A000F  cmplwi cr6, r10, 0xf
	ctx.cr[6].compare_u32(ctx.r[10].u32, 15 as u32, &mut ctx.xer);
	// 82C5568C: 81696258  lwz r11, 0x6258(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(25176 as u32) ) } as u64;
	// 82C55690: 911F00EC  stw r8, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[8].u32 ) };
	// 82C55694: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82C55698: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C5569C: 80FB000C  lwz r7, 0xc(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C556A0: 91696258  stw r11, 0x6258(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(25176 as u32), ctx.r[11].u32 ) };
	// 82C556A4: 90FF00F4  stw r7, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[7].u32 ) };
	// 82C556A8: 419900D0  bgt cr6, 0x82c55778
	if ctx.cr[6].gt {
	pc = 0x82C55778; continue 'dispatch;
	}
	// 82C556AC: 3D8082C5  lis r12, -0x7d3b
	ctx.r[12].s64 = -2101018624;
	// 82C556B0: 398C56C4  addi r12, r12, 0x56c4
	ctx.r[12].s64 = ctx.r[12].s64 + 22212;
	// 82C556B4: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82C556B8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82C556BC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82C556C0: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x82C55704; continue 'dispatch;
		},
		1 => {
	pc = 0x82C55704; continue 'dispatch;
		},
		2 => {
	pc = 0x82C55778; continue 'dispatch;
		},
		3 => {
	pc = 0x82C5570C; continue 'dispatch;
		},
		4 => {
	pc = 0x82C55718; continue 'dispatch;
		},
		5 => {
	pc = 0x82C55724; continue 'dispatch;
		},
		6 => {
	pc = 0x82C55730; continue 'dispatch;
		},
		7 => {
	pc = 0x82C5573C; continue 'dispatch;
		},
		8 => {
	pc = 0x82C55748; continue 'dispatch;
		},
		9 => {
	pc = 0x82C55754; continue 'dispatch;
		},
		10 => {
	pc = 0x82C55760; continue 'dispatch;
		},
		11 => {
	pc = 0x82C5576C; continue 'dispatch;
		},
		12 => {
	pc = 0x82C55748; continue 'dispatch;
		},
		13 => {
	pc = 0x82C55754; continue 'dispatch;
		},
		14 => {
	pc = 0x82C55778; continue 'dispatch;
		},
		15 => {
	pc = 0x82C55724; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82C556C4: 82C55704  lwz r22, 0x5704(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22276 as u32) ) } as u64;
	// 82C556C8: 82C55704  lwz r22, 0x5704(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22276 as u32) ) } as u64;
	// 82C556CC: 82C55778  lwz r22, 0x5778(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22392 as u32) ) } as u64;
	// 82C556D0: 82C5570C  lwz r22, 0x570c(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22284 as u32) ) } as u64;
	// 82C556D4: 82C55718  lwz r22, 0x5718(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22296 as u32) ) } as u64;
	// 82C556D8: 82C55724  lwz r22, 0x5724(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22308 as u32) ) } as u64;
	// 82C556DC: 82C55730  lwz r22, 0x5730(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22320 as u32) ) } as u64;
	// 82C556E0: 82C5573C  lwz r22, 0x573c(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22332 as u32) ) } as u64;
	// 82C556E4: 82C55748  lwz r22, 0x5748(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22344 as u32) ) } as u64;
	// 82C556E8: 82C55754  lwz r22, 0x5754(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22356 as u32) ) } as u64;
	// 82C556EC: 82C55760  lwz r22, 0x5760(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22368 as u32) ) } as u64;
	// 82C556F0: 82C5576C  lwz r22, 0x576c(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22380 as u32) ) } as u64;
	// 82C556F4: 82C55748  lwz r22, 0x5748(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22344 as u32) ) } as u64;
	// 82C556F8: 82C55754  lwz r22, 0x5754(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22356 as u32) ) } as u64;
	// 82C556FC: 82C55778  lwz r22, 0x5778(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22392 as u32) ) } as u64;
	// 82C55700: 82C55724  lwz r22, 0x5724(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(22308 as u32) ) } as u64;
            }
            0x82C55704 => {
    //   block [0x82C55704..0x82C5570C)
	// 82C55704: 92FF00F0  stw r23, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[23].u32 ) };
	// 82C55708: 48000074  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C5570C => {
    //   block [0x82C5570C..0x82C55718)
	// 82C5570C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82C55710: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C55714: 48000068  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C55718 => {
    //   block [0x82C55718..0x82C55724)
	// 82C55718: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82C5571C: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C55720: 4800005C  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C55724 => {
    //   block [0x82C55724..0x82C55730)
	// 82C55724: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82C55728: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C5572C: 48000050  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C55730 => {
    //   block [0x82C55730..0x82C5573C)
	// 82C55730: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82C55734: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C55738: 48000044  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C5573C => {
    //   block [0x82C5573C..0x82C55748)
	// 82C5573C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82C55740: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C55744: 48000038  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C55748 => {
    //   block [0x82C55748..0x82C55754)
	// 82C55748: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 82C5574C: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C55750: 4800002C  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C55754 => {
    //   block [0x82C55754..0x82C55760)
	// 82C55754: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82C55758: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C5575C: 48000020  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C55760 => {
    //   block [0x82C55760..0x82C5576C)
	// 82C55760: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82C55764: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C55768: 48000014  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C5576C => {
    //   block [0x82C5576C..0x82C55778)
	// 82C5576C: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82C55770: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C55774: 48000008  b 0x82c5577c
	pc = 0x82C5577C; continue 'dispatch;
            }
            0x82C55778 => {
    //   block [0x82C55778..0x82C558B8)
	// 82C55778: 933F00F0  stw r25, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[25].u32 ) };
	// 82C5577C: 897F0084  lbz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82C55780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55784: 419A0010  beq cr6, 0x82c55794
	if ctx.cr[6].eq {
	pc = 0x82C55794; continue 'dispatch;
	}
	// 82C55788: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82C5578C: 616A0002  ori r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u64 | 2;
	// 82C55790: 915F00E8  stw r10, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82C55794: 897B004B  lbz r11, 0x4b(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(75 as u32) ) } as u64;
	// 82C55798: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5579C: 419A0010  beq cr6, 0x82c557ac
	if ctx.cr[6].eq {
	pc = 0x82C557AC; continue 'dispatch;
	}
	// 82C557A0: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82C557A4: 616A0004  ori r10, r11, 4
	ctx.r[10].u64 = ctx.r[11].u64 | 4;
	// 82C557A8: 915F00E8  stw r10, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82C557AC: 56AB0462  rlwinm r11, r21, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0xFFFFFFFFu64;
	// 82C557B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C557B4: 419A0010  beq cr6, 0x82c557c4
	if ctx.cr[6].eq {
	pc = 0x82C557C4; continue 'dispatch;
	}
	// 82C557B8: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82C557BC: 616A0010  ori r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u64 | 16;
	// 82C557C0: 915F00E8  stw r10, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82C557C4: 56AB05EE  rlwinm r11, r21, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[21].u32 as u64 & 0xFFFFFFFFu64;
	// 82C557C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C557CC: 419A001C  beq cr6, 0x82c557e8
	if ctx.cr[6].eq {
	pc = 0x82C557E8; continue 'dispatch;
	}
	// 82C557D0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C557D4: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82C557D8: 81380038  lwz r9, 0x38(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C557DC: 7D0B5396  divwu r8, r11, r10
	ctx.r[8].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82C557E0: 7CE849D6  mullw r7, r8, r9
	ctx.r[7].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C557E4: 90FF0040  stw r7, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[7].u32 ) };
	// 82C557E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C557EC: C01B002C  lfs f0, 0x2c(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C557F0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C557F4: D01F0064  stfs f0, 0x64(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82C557F8: C1BB0030  lfs f13, 0x30(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C557FC: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C55800: C3EA0BE8  lfs f31, 0xbe8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3048 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C55804: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C55808: 419A0054  beq cr6, 0x82c5585c
	if ctx.cr[6].eq {
	pc = 0x82C5585C; continue 'dispatch;
	}
	// 82C5580C: 480549C5  bl 0x82caa1d0
	ctx.lr = 0x82C55810;
	sub_82CAA1D0(ctx, base);
	// 82C55810: 3D6051EB  lis r11, 0x51eb
	ctx.r[11].s64 = 1374355456;
	// 82C55814: C01B0030  lfs f0, 0x30(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C55818: 616A851F  ori r10, r11, 0x851f
	ctx.r[10].u64 = ctx.r[11].u64 | 34079;
	// 82C5581C: C1BF0064  lfs f13, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C55820: 7D235096  mulhw r9, r3, r10
	ctx.r[9].s64 = ((ctx.r[3].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 82C55824: 7D2B3670  srawi r11, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 82C55828: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C5582C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C55830: 1CE800C8  mulli r7, r8, 0xc8
	ctx.r[7].s64 = ctx.r[8].s64 * 200;
	// 82C55834: 7D671850  subf r11, r7, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[7].s64;
	// 82C55838: 396BFF9C  addi r11, r11, -0x64
	ctx.r[11].s64 = ctx.r[11].s64 + -100;
	// 82C5583C: 7D6607B4  extsw r6, r11
	ctx.r[6].s64 = ctx.r[11].s32 as i64;
	// 82C55840: F8C10060  std r6, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u64 ) };
	// 82C55844: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C55848: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 82C5584C: FD405818  frsp f10, f11
	ctx.f[10].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82C55850: ED2A0032  fmuls f9, f10, f0
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C55854: ED096FFA  fmadds f8, f9, f31, f13
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 82C55858: D11F0064  stfs f8, 0x64(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82C5585C: C01F0064  lfs f0, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C55860: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82C55864: 40980008  bge cr6, 0x82c5586c
	if !ctx.cr[6].lt {
	pc = 0x82C5586C; continue 'dispatch;
	}
	// 82C55868: D3FF0064  stfs f31, 0x64(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82C5586C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55870: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C55874: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C55878: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C5587C: 4E800421  bctrl
	ctx.lr = 0x82C55880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55880: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82C55884: 41980024  blt cr6, 0x82c558a8
	if ctx.cr[6].lt {
	pc = 0x82C558A8; continue 'dispatch;
	}
	// 82C55888: 409AFC28  bne cr6, 0x82c554b0
	if !ctx.cr[6].eq {
	pc = 0x82C554B0; continue 'dispatch;
	}
	// 82C5588C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C55890: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82C55894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55898: 4BFFF581  bl 0x82c54e18
	ctx.lr = 0x82C5589C;
	sub_82C54E18(ctx, base);
	// 82C5589C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82C558A0: CBE1FF98  lfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 82C558A4: 48053B98  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82C558A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C558AC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82C558B0: CBE1FF98  lfd f31, -0x68(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 82C558B4: 48053B88  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C558B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C558B8 size=340
    let mut pc: u32 = 0x82C558B8;
    'dispatch: loop {
        match pc {
            0x82C558B8 => {
    //   block [0x82C558B8..0x82C55A0C)
	// 82C558B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C558BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C558C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C558C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C558C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C558CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C558D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C558D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C558D8: 392BCED0  addi r9, r11, -0x3130
	ctx.r[9].s64 = ctx.r[11].s64 + -12592;
	// 82C558DC: 390AB7CC  addi r8, r10, -0x4834
	ctx.r[8].s64 = ctx.r[10].s64 + -18484;
	// 82C558E0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C558E4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82C558E8: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C558EC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C558F0: 4BFF81B9  bl 0x82c4daa8
	ctx.lr = 0x82C558F4;
	sub_82C4DAA8(ctx, base);
	// 82C558F4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C558F8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C558FC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82C55900: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82C55904: 3941005C  addi r10, r1, 0x5c
	ctx.r[10].s64 = ctx.r[1].s64 + 92;
	// 82C55908: C0070C18  lfs f0, 0xc18(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5590C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C55910: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C55914: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C55918: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C5591C: 38E9B7CC  addi r7, r9, -0x4834
	ctx.r[7].s64 = ctx.r[9].s64 + -18484;
	// 82C55920: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82C55924: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C55928: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82C5592C: 39200070  li r9, 0x70
	ctx.r[9].s64 = 112;
	// 82C55930: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C55934: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82C55938: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C5593C: C1A80C14  lfs f13, 0xc14(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C55940: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C55944: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82C55948: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C5594C: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 82C55950: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82C55954: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82C55958: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82C5595C: D01F004C  stfs f0, 0x4c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82C55960: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C55964: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C55968: D1BF0060  stfs f13, 0x60(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82C5596C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C55970: D1BF0064  stfs f13, 0x64(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82C55974: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82C55978: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82C5597C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82C55980: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82C55984: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82C55988: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82C5598C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82C55990: 13E03407  vcmpneb. (lvlx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C55994: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C55998: 13C02C07  vcmpneb. (lvlx128) v30, v0, v5
	tmp.u32 = ctx.r[5].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C5599C: 90FF0058  stw r7, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82C559A0: 13A02407  vcmpneb. (lvlx128) v29, v0, v4
	tmp.u32 = ctx.r[4].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55A10 size=252
    let mut pc: u32 = 0x82C55A10;
    'dispatch: loop {
        match pc {
            0x82C55A10 => {
    //   block [0x82C55A10..0x82C55B0C)
	// 82C55A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C55A18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C55A1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C55A20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55A24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C55A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C55A2C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55A30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55A34: 419A00BC  beq cr6, 0x82c55af0
	if ctx.cr[6].eq {
	pc = 0x82C55AF0; continue 'dispatch;
	}
	// 82C55A38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C55A3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C55A40: 388BCF04  addi r4, r11, -0x30fc
	ctx.r[4].s64 = ctx.r[11].s64 + -12540;
	// 82C55A44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C55A48: 4B5D7489  bl 0x8222ced0
	ctx.lr = 0x82C55A4C;
	sub_8222CED0(ctx, base);
	// 82C55A4C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55A50: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55A54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C55A58: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55A60: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55A64: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C55A68: 4E800421  bctrl
	ctx.lr = 0x82C55A6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C55A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C55A74: 4B5BF365  bl 0x82214dd8
	ctx.lr = 0x82C55A78;
	sub_82214DD8(ctx, base);
	// 82C55A78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55A7C: 419A0074  beq cr6, 0x82c55af0
	if ctx.cr[6].eq {
	pc = 0x82C55AF0; continue 'dispatch;
	}
	// 82C55A80: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C55A84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55A88: 419A0068  beq cr6, 0x82c55af0
	if ctx.cr[6].eq {
	pc = 0x82C55AF0; continue 'dispatch;
	}
	// 82C55A8C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C55A90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55A94: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55A98: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55A9C: 4E800421  bctrl
	ctx.lr = 0x82C55AA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55AA0: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C55AA4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C55AA8: 409A0048  bne cr6, 0x82c55af0
	if !ctx.cr[6].eq {
	pc = 0x82C55AF0; continue 'dispatch;
	}
	// 82C55AAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55AB0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55AB4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C55AB8: 409A000C  bne cr6, 0x82c55ac4
	if !ctx.cr[6].eq {
	pc = 0x82C55AC4; continue 'dispatch;
	}
	// 82C55ABC: 887E0015  lbz r3, 0x15(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82C55AC0: 48000034  b 0x82c55af4
	pc = 0x82C55AF4; continue 'dispatch;
	// 82C55AC4: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55AC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55ACC: 419A0018  beq cr6, 0x82c55ae4
	if ctx.cr[6].eq {
	pc = 0x82C55AE4; continue 'dispatch;
	}
	// 82C55AD0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C55AD4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55AD8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C55ADC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C55AE0: 40980008  bge cr6, 0x82c55ae8
	if !ctx.cr[6].lt {
	pc = 0x82C55AE8; continue 'dispatch;
	}
	// 82C55AE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C55AE8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C55AEC: 48000008  b 0x82c55af4
	pc = 0x82C55AF4; continue 'dispatch;
	// 82C55AF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C55AF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C55AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C55AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C55B00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C55B04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C55B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55B10 size=112
    let mut pc: u32 = 0x82C55B10;
    'dispatch: loop {
        match pc {
            0x82C55B10 => {
    //   block [0x82C55B10..0x82C55B80)
	// 82C55B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C55B18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C55B1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55B20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C55B24: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C55B28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55B2C: 419A0040  beq cr6, 0x82c55b6c
	if ctx.cr[6].eq {
	pc = 0x82C55B6C; continue 'dispatch;
	}
	// 82C55B30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55B34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55B38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55B3C: 4E800421  bctrl
	ctx.lr = 0x82C55B40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55B40: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C55B44: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C55B48: 419A0024  beq cr6, 0x82c55b6c
	if ctx.cr[6].eq {
	pc = 0x82C55B6C; continue 'dispatch;
	}
	// 82C55B4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C55B50: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82C55B54: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82C55B58: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C55B5C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C55B60: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C55B64: 993F0010  stb r9, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 82C55B68: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C55B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C55B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C55B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C55B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C55B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55B80 size=92
    let mut pc: u32 = 0x82C55B80;
    'dispatch: loop {
        match pc {
            0x82C55B80 => {
    //   block [0x82C55B80..0x82C55BDC)
	// 82C55B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C55B88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C55B8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C55B90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55B94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C55B98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C55B9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C55BA0: 394BCF18  addi r10, r11, -0x30e8
	ctx.r[10].s64 = ctx.r[11].s64 + -12520;
	// 82C55BA4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C55BA8: 4BFB04E1  bl 0x82c06088
	ctx.lr = 0x82C55BAC;
	sub_82C06088(ctx, base);
	// 82C55BAC: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C55BB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55BB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C55BB8: 419A000C  beq cr6, 0x82c55bc4
	if ctx.cr[6].eq {
	pc = 0x82C55BC4; continue 'dispatch;
	}
	// 82C55BBC: 4BBEFBF5  bl 0x828457b0
	ctx.lr = 0x82C55BC0;
	sub_828457B0(ctx, base);
	// 82C55BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C55BC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C55BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C55BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C55BD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C55BD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C55BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55BE0 size=332
    let mut pc: u32 = 0x82C55BE0;
    'dispatch: loop {
        match pc {
            0x82C55BE0 => {
    //   block [0x82C55BE0..0x82C55D2C)
	// 82C55BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55BE4: 4805381D  bl 0x82ca9400
	ctx.lr = 0x82C55BE8;
	sub_82CA93D0(ctx, base);
	// 82C55BE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55BEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C55BF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C55BF4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C55BF8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55BFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55C00: 419A0120  beq cr6, 0x82c55d20
	if ctx.cr[6].eq {
	pc = 0x82C55D20; continue 'dispatch;
	}
	// 82C55C04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C55C08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C55C0C: 3B8BCF04  addi r28, r11, -0x30fc
	ctx.r[28].s64 = ctx.r[11].s64 + -12540;
	// 82C55C10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C55C14: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C55C18: 4B5D72B9  bl 0x8222ced0
	ctx.lr = 0x82C55C1C;
	sub_8222CED0(ctx, base);
	// 82C55C1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55C20: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55C24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C55C28: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55C2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C55C30: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55C34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55C38: 4E800421  bctrl
	ctx.lr = 0x82C55C3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55C3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C55C40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C55C44: 4B5BF195  bl 0x82214dd8
	ctx.lr = 0x82C55C48;
	sub_82214DD8(ctx, base);
	// 82C55C48: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C55C4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55C50: 409A0084  bne cr6, 0x82c55cd4
	if !ctx.cr[6].eq {
	pc = 0x82C55CD4; continue 'dispatch;
	}
	// 82C55C54: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C55C58: 4B5C9601  bl 0x8221f258
	ctx.lr = 0x82C55C5C;
	sub_8221F258(ctx, base);
	// 82C55C5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55C60: 419A0034  beq cr6, 0x82c55c94
	if ctx.cr[6].eq {
	pc = 0x82C55C94; continue 'dispatch;
	}
	// 82C55C64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C55C68: 93630004  stw r27, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82C55C6C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82C55C70: 9363000C  stw r27, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82C55C74: 392BCF18  addi r9, r11, -0x30e8
	ctx.r[9].s64 = ctx.r[11].s64 + -12520;
	// 82C55C78: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82C55C7C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82C55C80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C55C84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C55C88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C55C8C: 99030010  stb r8, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u8 ) };
	// 82C55C90: 48000008  b 0x82c55c98
	pc = 0x82C55C98; continue 'dispatch;
	// 82C55C94: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82C55C98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C55C9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82C55CA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C55CA4: 4B5D722D  bl 0x8222ced0
	ctx.lr = 0x82C55CA8;
	sub_8222CED0(ctx, base);
	// 82C55CA8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55CAC: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55CB0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82C55CB4: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55CB8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82C55CBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C55CC0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C55CC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55CC8: 4E800421  bctrl
	ctx.lr = 0x82C55CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55CCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82C55CD0: 4B5BF109  bl 0x82214dd8
	ctx.lr = 0x82C55CD4;
	sub_82214DD8(ctx, base);
	// 82C55CD4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C55CD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55CDC: 419A0020  beq cr6, 0x82c55cfc
	if ctx.cr[6].eq {
	pc = 0x82C55CFC; continue 'dispatch;
	}
	// 82C55CE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55CE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C55CE8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C55CEC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C55CF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55CF4: 4E800421  bctrl
	ctx.lr = 0x82C55CF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55CF8: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82C55CFC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55D00: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C55D04: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C55D08: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C55D0C: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55D10: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C55D14: 891E0014  lbz r8, 0x14(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C55D18: 991F0010  stb r8, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u8 ) };
	// 82C55D1C: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82C55D20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C55D24: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C55D28: 48053728  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C55D30 size=24
    let mut pc: u32 = 0x82C55D30;
    'dispatch: loop {
        match pc {
            0x82C55D30 => {
    //   block [0x82C55D30..0x82C55D48)
	// 82C55D30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55D34: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55D38: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C55D3C: 4098000C  bge cr6, 0x82c55d48
	if !ctx.cr[6].lt {
		sub_82C55D48(ctx, base);
		return;
	}
	// 82C55D40: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82C55D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C55D48 size=16
    let mut pc: u32 = 0x82C55D48;
    'dispatch: loop {
        match pc {
            0x82C55D48 => {
    //   block [0x82C55D48..0x82C55D58)
	// 82C55D48: 7D6B5010  subfc r11, r11, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C55D4C: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C55D50: 554307FE  clrlwi r3, r10, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82C55D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55D58 size=120
    let mut pc: u32 = 0x82C55D58;
    'dispatch: loop {
        match pc {
            0x82C55D58 => {
    //   block [0x82C55D58..0x82C55DD0)
	// 82C55D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55D5C: 480536B1  bl 0x82ca940c
	ctx.lr = 0x82C55D60;
	sub_82CA93D0(ctx, base);
	// 82C55D60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55D64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C55D68: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C55D6C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55D70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55D74: 419A0054  beq cr6, 0x82c55dc8
	if ctx.cr[6].eq {
	pc = 0x82C55DC8; continue 'dispatch;
	}
	// 82C55D78: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55D7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55D80: 419A0018  beq cr6, 0x82c55d98
	if ctx.cr[6].eq {
	pc = 0x82C55D98; continue 'dispatch;
	}
	// 82C55D84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55D88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C55D8C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C55D90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55D94: 4E800421  bctrl
	ctx.lr = 0x82C55D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55D98: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55D9C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55DA0: 409A0028  bne cr6, 0x82c55dc8
	if !ctx.cr[6].eq {
	pc = 0x82C55DC8; continue 'dispatch;
	}
	// 82C55DA4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55DA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55DAC: 419A0010  beq cr6, 0x82c55dbc
	if ctx.cr[6].eq {
	pc = 0x82C55DBC; continue 'dispatch;
	}
	// 82C55DB0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55DB4: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55DB8: 409A0010  bne cr6, 0x82c55dc8
	if !ctx.cr[6].eq {
	pc = 0x82C55DC8; continue 'dispatch;
	}
	// 82C55DBC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C55DC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55DC4: 409AFFB4  bne cr6, 0x82c55d78
	if !ctx.cr[6].eq {
	pc = 0x82C55D78; continue 'dispatch;
	}
	// 82C55DC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C55DCC: 48053690  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55DD0 size=120
    let mut pc: u32 = 0x82C55DD0;
    'dispatch: loop {
        match pc {
            0x82C55DD0 => {
    //   block [0x82C55DD0..0x82C55E48)
	// 82C55DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55DD4: 48053639  bl 0x82ca940c
	ctx.lr = 0x82C55DD8;
	sub_82CA93D0(ctx, base);
	// 82C55DD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55DDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C55DE0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C55DE4: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55DE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55DEC: 419A0054  beq cr6, 0x82c55e40
	if ctx.cr[6].eq {
	pc = 0x82C55E40; continue 'dispatch;
	}
	// 82C55DF0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55DF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55DF8: 419A0018  beq cr6, 0x82c55e10
	if ctx.cr[6].eq {
	pc = 0x82C55E10; continue 'dispatch;
	}
	// 82C55DFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55E00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C55E04: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C55E08: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55E0C: 4E800421  bctrl
	ctx.lr = 0x82C55E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55E10: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55E14: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55E18: 409A0028  bne cr6, 0x82c55e40
	if !ctx.cr[6].eq {
	pc = 0x82C55E40; continue 'dispatch;
	}
	// 82C55E1C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55E20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55E24: 419A0010  beq cr6, 0x82c55e34
	if ctx.cr[6].eq {
	pc = 0x82C55E34; continue 'dispatch;
	}
	// 82C55E28: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55E2C: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55E30: 409A0010  bne cr6, 0x82c55e40
	if !ctx.cr[6].eq {
	pc = 0x82C55E40; continue 'dispatch;
	}
	// 82C55E34: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C55E38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55E3C: 409AFFB4  bne cr6, 0x82c55df0
	if !ctx.cr[6].eq {
	pc = 0x82C55DF0; continue 'dispatch;
	}
	// 82C55E40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C55E44: 48053618  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55E48 size=136
    let mut pc: u32 = 0x82C55E48;
    'dispatch: loop {
        match pc {
            0x82C55E48 => {
    //   block [0x82C55E48..0x82C55ED0)
	// 82C55E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C55E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C55E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C55E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55E5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C55E60: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55E64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55E68: 419A0050  beq cr6, 0x82c55eb8
	if ctx.cr[6].eq {
	pc = 0x82C55EB8; continue 'dispatch;
	}
	// 82C55E6C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55E70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55E74: 419A0014  beq cr6, 0x82c55e88
	if ctx.cr[6].eq {
	pc = 0x82C55E88; continue 'dispatch;
	}
	// 82C55E78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55E7C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C55E80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55E84: 4E800421  bctrl
	ctx.lr = 0x82C55E88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55E88: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55E8C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55E90: 409A0028  bne cr6, 0x82c55eb8
	if !ctx.cr[6].eq {
	pc = 0x82C55EB8; continue 'dispatch;
	}
	// 82C55E94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55E9C: 419A0010  beq cr6, 0x82c55eac
	if ctx.cr[6].eq {
	pc = 0x82C55EAC; continue 'dispatch;
	}
	// 82C55EA0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55EA4: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55EA8: 409A0010  bne cr6, 0x82c55eb8
	if !ctx.cr[6].eq {
	pc = 0x82C55EB8; continue 'dispatch;
	}
	// 82C55EAC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82C55EB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C55EB4: 409AFFB8  bne cr6, 0x82c55e6c
	if !ctx.cr[6].eq {
	pc = 0x82C55E6C; continue 'dispatch;
	}
	// 82C55EB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C55EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C55EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C55EC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C55EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C55ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55ED0 size=164
    let mut pc: u32 = 0x82C55ED0;
    'dispatch: loop {
        match pc {
            0x82C55ED0 => {
    //   block [0x82C55ED0..0x82C55F74)
	// 82C55ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55ED4: 48053531  bl 0x82ca9404
	ctx.lr = 0x82C55ED8;
	sub_82CA93D0(ctx, base);
	// 82C55ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C55EE0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C55EE4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C55EE8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55EEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55EF0: 419A007C  beq cr6, 0x82c55f6c
	if ctx.cr[6].eq {
	pc = 0x82C55F6C; continue 'dispatch;
	}
	// 82C55EF4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C55EF8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55EFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55F00: 419A0040  beq cr6, 0x82c55f40
	if ctx.cr[6].eq {
	pc = 0x82C55F40; continue 'dispatch;
	}
	// 82C55F04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55F08: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C55F0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C55F10: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C55F14: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55F18: 4E800421  bctrl
	ctx.lr = 0x82C55F1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55F1C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C55F24: 419A0014  beq cr6, 0x82c55f38
	if ctx.cr[6].eq {
	pc = 0x82C55F38; continue 'dispatch;
	}
	// 82C55F28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55F2C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C55F30: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55F34: 4E800421  bctrl
	ctx.lr = 0x82C55F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55F38: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82C55F3C: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C55F40: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55F44: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55F48: 409A0024  bne cr6, 0x82c55f6c
	if !ctx.cr[6].eq {
	pc = 0x82C55F6C; continue 'dispatch;
	}
	// 82C55F4C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55F50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55F54: 419A0018  beq cr6, 0x82c55f6c
	if ctx.cr[6].eq {
	pc = 0x82C55F6C; continue 'dispatch;
	}
	// 82C55F58: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55F5C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55F60: 409A000C  bne cr6, 0x82c55f6c
	if !ctx.cr[6].eq {
	pc = 0x82C55F6C; continue 'dispatch;
	}
	// 82C55F64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55F68: 409AFF90  bne cr6, 0x82c55ef8
	if !ctx.cr[6].eq {
	pc = 0x82C55EF8; continue 'dispatch;
	}
	// 82C55F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C55F70: 480534E4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C55F78 size=108
    let mut pc: u32 = 0x82C55F78;
    'dispatch: loop {
        match pc {
            0x82C55F78 => {
    //   block [0x82C55F78..0x82C55FE4)
	// 82C55F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55F7C: 48053491  bl 0x82ca940c
	ctx.lr = 0x82C55F80;
	sub_82CA93D0(ctx, base);
	// 82C55F80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C55F84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C55F88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C55F8C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55F90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55F94: 419A0048  beq cr6, 0x82c55fdc
	if ctx.cr[6].eq {
	pc = 0x82C55FDC; continue 'dispatch;
	}
	// 82C55F98: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55F9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C55FA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C55FA4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C55FA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C55FAC: 4E800421  bctrl
	ctx.lr = 0x82C55FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C55FB0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55FB4: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55FB8: 409A0024  bne cr6, 0x82c55fdc
	if !ctx.cr[6].eq {
	pc = 0x82C55FDC; continue 'dispatch;
	}
	// 82C55FBC: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C55FC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55FC4: 419A0018  beq cr6, 0x82c55fdc
	if ctx.cr[6].eq {
	pc = 0x82C55FDC; continue 'dispatch;
	}
	// 82C55FC8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C55FCC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C55FD0: 409A000C  bne cr6, 0x82c55fdc
	if !ctx.cr[6].eq {
	pc = 0x82C55FDC; continue 'dispatch;
	}
	// 82C55FD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C55FD8: 409AFFC0  bne cr6, 0x82c55f98
	if !ctx.cr[6].eq {
	pc = 0x82C55F98; continue 'dispatch;
	}
	// 82C55FDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C55FE0: 4805347C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C55FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C55FE8 size=148
    let mut pc: u32 = 0x82C55FE8;
    'dispatch: loop {
        match pc {
            0x82C55FE8 => {
    //   block [0x82C55FE8..0x82C5607C)
	// 82C55FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C55FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C55FF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C55FF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C55FF8: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56080 size=20
    let mut pc: u32 = 0x82C56080;
    'dispatch: loop {
        match pc {
            0x82C56080 => {
    //   block [0x82C56080..0x82C56094)
	// 82C56080: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56084: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56088: 409A000C  bne cr6, 0x82c56094
	if !ctx.cr[6].eq {
		sub_82C56094(ctx, base);
		return;
	}
	// 82C5608C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C56090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56094(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56094 size=16
    let mut pc: u32 = 0x82C56094;
    'dispatch: loop {
        match pc {
            0x82C56094 => {
    //   block [0x82C56094..0x82C560A4)
	// 82C56094: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56098: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5609C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C560A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560A8 size=12
    let mut pc: u32 = 0x82C560A8;
    'dispatch: loop {
        match pc {
            0x82C560A8 => {
    //   block [0x82C560A8..0x82C560B4)
	// 82C560A8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C560AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C560B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560B4 size=16
    let mut pc: u32 = 0x82C560B4;
    'dispatch: loop {
        match pc {
            0x82C560B4 => {
    //   block [0x82C560B4..0x82C560C4)
	// 82C560B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C560B8: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C560BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C560C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560C4 size=4
    let mut pc: u32 = 0x82C560C4;
    'dispatch: loop {
        match pc {
            0x82C560C4 => {
    //   block [0x82C560C4..0x82C560C8)
	// 82C560C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560C8 size=12
    let mut pc: u32 = 0x82C560C8;
    'dispatch: loop {
        match pc {
            0x82C560C8 => {
    //   block [0x82C560C8..0x82C560D4)
	// 82C560C8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C560CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C560D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560D4 size=16
    let mut pc: u32 = 0x82C560D4;
    'dispatch: loop {
        match pc {
            0x82C560D4 => {
    //   block [0x82C560D4..0x82C560E4)
	// 82C560D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C560D8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C560DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C560E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560E4 size=4
    let mut pc: u32 = 0x82C560E4;
    'dispatch: loop {
        match pc {
            0x82C560E4 => {
    //   block [0x82C560E4..0x82C560E8)
	// 82C560E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560E8 size=12
    let mut pc: u32 = 0x82C560E8;
    'dispatch: loop {
        match pc {
            0x82C560E8 => {
    //   block [0x82C560E8..0x82C560F4)
	// 82C560E8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C560EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C560F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C560F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C560F4 size=16
    let mut pc: u32 = 0x82C560F4;
    'dispatch: loop {
        match pc {
            0x82C560F4 => {
    //   block [0x82C560F4..0x82C56104)
	// 82C560F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C560F8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C560FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56100: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56104 size=4
    let mut pc: u32 = 0x82C56104;
    'dispatch: loop {
        match pc {
            0x82C56104 => {
    //   block [0x82C56104..0x82C56108)
	// 82C56104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56108 size=12
    let mut pc: u32 = 0x82C56108;
    'dispatch: loop {
        match pc {
            0x82C56108 => {
    //   block [0x82C56108..0x82C56114)
	// 82C56108: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5610C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56110: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56114 size=16
    let mut pc: u32 = 0x82C56114;
    'dispatch: loop {
        match pc {
            0x82C56114 => {
    //   block [0x82C56114..0x82C56124)
	// 82C56114: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56118: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C5611C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56120: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56124 size=4
    let mut pc: u32 = 0x82C56124;
    'dispatch: loop {
        match pc {
            0x82C56124 => {
    //   block [0x82C56124..0x82C56128)
	// 82C56124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56128 size=12
    let mut pc: u32 = 0x82C56128;
    'dispatch: loop {
        match pc {
            0x82C56128 => {
    //   block [0x82C56128..0x82C56134)
	// 82C56128: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5612C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56130: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56134 size=16
    let mut pc: u32 = 0x82C56134;
    'dispatch: loop {
        match pc {
            0x82C56134 => {
    //   block [0x82C56134..0x82C56144)
	// 82C56134: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56138: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5613C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56140: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56144(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56144 size=4
    let mut pc: u32 = 0x82C56144;
    'dispatch: loop {
        match pc {
            0x82C56144 => {
    //   block [0x82C56144..0x82C56148)
	// 82C56144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56148 size=12
    let mut pc: u32 = 0x82C56148;
    'dispatch: loop {
        match pc {
            0x82C56148 => {
    //   block [0x82C56148..0x82C56154)
	// 82C56148: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5614C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56150: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56154(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56154 size=16
    let mut pc: u32 = 0x82C56154;
    'dispatch: loop {
        match pc {
            0x82C56154 => {
    //   block [0x82C56154..0x82C56164)
	// 82C56154: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56158: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5615C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56160: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56164 size=4
    let mut pc: u32 = 0x82C56164;
    'dispatch: loop {
        match pc {
            0x82C56164 => {
    //   block [0x82C56164..0x82C56168)
	// 82C56164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56168 size=220
    let mut pc: u32 = 0x82C56168;
    'dispatch: loop {
        match pc {
            0x82C56168 => {
    //   block [0x82C56168..0x82C56244)
	// 82C56168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5616C: 4805329D  bl 0x82ca9408
	ctx.lr = 0x82C56170;
	sub_82CA93D0(ctx, base);
	// 82C56170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56174: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C56178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5617C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C56180: 4BFAF5B1  bl 0x82c05730
	ctx.lr = 0x82C56184;
	sub_82C05730(ctx, base);
	// 82C56184: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C56188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5618C: 388BB5B4  addi r4, r11, -0x4a4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19020;
	// 82C56190: 4BFAF669  bl 0x82c057f8
	ctx.lr = 0x82C56194;
	sub_82C057F8(ctx, base);
	// 82C56194: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C56198: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5619C: 3BCB0C88  addi r30, r11, 0xc88
	ctx.r[30].s64 = ctx.r[11].s64 + 3208;
	// 82C561A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C561A4: 4BFAF655  bl 0x82c057f8
	ctx.lr = 0x82C561A8;
	sub_82C057F8(ctx, base);
	// 82C561A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C561AC: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C561B0: 4BFAF911  bl 0x82c05ac0
	ctx.lr = 0x82C561B4;
	sub_82C05AC0(ctx, base);
	// 82C561B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C561B8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C561BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C561C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C561C4: 4E800421  bctrl
	ctx.lr = 0x82C561C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C561C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C561CC: 4BFAF5A5  bl 0x82c05770
	ctx.lr = 0x82C561D0;
	sub_82C05770(ctx, base);
	// 82C561D0: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C561D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C561D8: 419A0018  beq cr6, 0x82c561f0
	if ctx.cr[6].eq {
	pc = 0x82C561F0; continue 'dispatch;
	}
	// 82C561DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C561E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C561E4: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C561E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C561EC: 4E800421  bctrl
	ctx.lr = 0x82C561F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C561F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C561F4: 4BFAF53D  bl 0x82c05730
	ctx.lr = 0x82C561F8;
	sub_82C05730(ctx, base);
	// 82C561F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C561FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C56200: 388BB5C0  addi r4, r11, -0x4a40
	ctx.r[4].s64 = ctx.r[11].s64 + -19008;
	// 82C56204: 4BFAF5F5  bl 0x82c057f8
	ctx.lr = 0x82C56208;
	sub_82C057F8(ctx, base);
	// 82C56208: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C5620C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C56210: 4BFAF5E9  bl 0x82c057f8
	ctx.lr = 0x82C56214;
	sub_82C057F8(ctx, base);
	// 82C56214: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C56218: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5621C: 4BFAF8A5  bl 0x82c05ac0
	ctx.lr = 0x82C56220;
	sub_82C05AC0(ctx, base);
	// 82C56220: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C56224: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5622C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56230: 4E800421  bctrl
	ctx.lr = 0x82C56234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C56234: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C56238: 4BFAF539  bl 0x82c05770
	ctx.lr = 0x82C5623C;
	sub_82C05770(ctx, base);
	// 82C5623C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C56240: 48053218  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56248 size=12
    let mut pc: u32 = 0x82C56248;
    'dispatch: loop {
        match pc {
            0x82C56248 => {
    //   block [0x82C56248..0x82C56254)
	// 82C56248: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5624C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56250: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56254 size=16
    let mut pc: u32 = 0x82C56254;
    'dispatch: loop {
        match pc {
            0x82C56254 => {
    //   block [0x82C56254..0x82C56264)
	// 82C56254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56258: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C5625C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56260: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56264 size=4
    let mut pc: u32 = 0x82C56264;
    'dispatch: loop {
        match pc {
            0x82C56264 => {
    //   block [0x82C56264..0x82C56268)
	// 82C56264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56268 size=12
    let mut pc: u32 = 0x82C56268;
    'dispatch: loop {
        match pc {
            0x82C56268 => {
    //   block [0x82C56268..0x82C56274)
	// 82C56268: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5626C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56270: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56274 size=16
    let mut pc: u32 = 0x82C56274;
    'dispatch: loop {
        match pc {
            0x82C56274 => {
    //   block [0x82C56274..0x82C56284)
	// 82C56274: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56278: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C5627C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56280: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56284 size=4
    let mut pc: u32 = 0x82C56284;
    'dispatch: loop {
        match pc {
            0x82C56284 => {
    //   block [0x82C56284..0x82C56288)
	// 82C56284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56288 size=104
    let mut pc: u32 = 0x82C56288;
    'dispatch: loop {
        match pc {
            0x82C56288 => {
    //   block [0x82C56288..0x82C562F0)
	// 82C56288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5628C: 4805317D  bl 0x82ca9408
	ctx.lr = 0x82C56290;
	sub_82CA93D0(ctx, base);
	// 82C56290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C56298: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C5629C: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82C562A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C562A4: 4BFF6D15  bl 0x82c4cfb8
	ctx.lr = 0x82C562A8;
	sub_82C4CFB8(ctx, base);
	// 82C562A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C562AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C562B0: 419A0038  beq cr6, 0x82c562e8
	if ctx.cr[6].eq {
	pc = 0x82C562E8; continue 'dispatch;
	}
	// 82C562B4: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C562B8: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82C562BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C562C0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C562C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C562C8: 4E800421  bctrl
	ctx.lr = 0x82C562CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C562CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C562D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C562D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C562D8: 4BFF7271  bl 0x82c4d548
	ctx.lr = 0x82C562DC;
	sub_82C4D548(ctx, base);
	// 82C562DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C562E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C562E4: 409AFFD0  bne cr6, 0x82c562b4
	if !ctx.cr[6].eq {
	pc = 0x82C562B4; continue 'dispatch;
	}
	// 82C562E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C562EC: 4805316C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C562F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C562F0 size=104
    let mut pc: u32 = 0x82C562F0;
    'dispatch: loop {
        match pc {
            0x82C562F0 => {
    //   block [0x82C562F0..0x82C56358)
	// 82C562F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C562F4: 48053115  bl 0x82ca9408
	ctx.lr = 0x82C562F8;
	sub_82CA93D0(ctx, base);
	// 82C562F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C562FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C56300: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C56304: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82C56308: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C5630C: 4BFF6CAD  bl 0x82c4cfb8
	ctx.lr = 0x82C56310;
	sub_82C4CFB8(ctx, base);
	// 82C56310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56314: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56318: 419A0038  beq cr6, 0x82c56350
	if ctx.cr[6].eq {
	pc = 0x82C56350; continue 'dispatch;
	}
	// 82C5631C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56320: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C56324: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56328: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C5632C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56330: 4E800421  bctrl
	ctx.lr = 0x82C56334;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C56334: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C56338: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C5633C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C56340: 4BFF7209  bl 0x82c4d548
	ctx.lr = 0x82C56344;
	sub_82C4D548(ctx, base);
	// 82C56344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56348: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5634C: 409AFFD0  bne cr6, 0x82c5631c
	if !ctx.cr[6].eq {
	pc = 0x82C5631C; continue 'dispatch;
	}
	// 82C56350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C56354: 48053104  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56358 size=112
    let mut pc: u32 = 0x82C56358;
    'dispatch: loop {
        match pc {
            0x82C56358 => {
    //   block [0x82C56358..0x82C563C8)
	// 82C56358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5635C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C56360: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C56364: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C56368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5636C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56370: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C56374: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82C56378: 394BB968  addi r10, r11, -0x4698
	ctx.r[10].s64 = ctx.r[11].s64 + -18072;
	// 82C5637C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C56380: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56384: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56388: 419A0014  beq cr6, 0x82c5639c
	if ctx.cr[6].eq {
	pc = 0x82C5639C; continue 'dispatch;
	}
	// 82C5638C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56390: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56394: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56398: 4E800421  bctrl
	ctx.lr = 0x82C5639C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5639C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C563A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C563A4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C563A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C563AC: 4BFAFD65  bl 0x82c06110
	ctx.lr = 0x82C563B0;
	sub_82C06110(ctx, base);
	// 82C563B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C563B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C563B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C563BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C563C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C563C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C563C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C563C8 size=100
    let mut pc: u32 = 0x82C563C8;
    'dispatch: loop {
        match pc {
            0x82C563C8 => {
    //   block [0x82C563C8..0x82C5642C)
	// 82C563C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C563CC: 48053041  bl 0x82ca940c
	ctx.lr = 0x82C563D0;
	sub_82CA93D0(ctx, base);
	// 82C563D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C563D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C563D8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82C563DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C563E0: 4B5C8E79  bl 0x8221f258
	ctx.lr = 0x82C563E4;
	sub_8221F258(ctx, base);
	// 82C563E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C563E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C563EC: 419A0018  beq cr6, 0x82c56404
	if ctx.cr[6].eq {
	pc = 0x82C56404; continue 'dispatch;
	}
	// 82C563F0: 4BFF76A1  bl 0x82c4da90
	ctx.lr = 0x82C563F4;
	sub_82C4DA90(ctx, base);
	// 82C563F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C563F8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C563FC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C56400: 48000008  b 0x82c56408
	pc = 0x82C56408; continue 'dispatch;
	// 82C56404: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C56408: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C5640C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82C56410: 4BFBB8B9  bl 0x82c11cc8
	ctx.lr = 0x82C56414;
	sub_82C11CC8(ctx, base);
	// 82C56414: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C56418: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C5641C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C56420: 4BFF6EA1  bl 0x82c4d2c0
	ctx.lr = 0x82C56424;
	sub_82C4D2C0(ctx, base);
	// 82C56424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C56428: 48053034  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56430 size=172
    let mut pc: u32 = 0x82C56430;
    'dispatch: loop {
        match pc {
            0x82C56430 => {
    //   block [0x82C56430..0x82C564DC)
	// 82C56430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56434: 48052FD1  bl 0x82ca9404
	ctx.lr = 0x82C56438;
	sub_82CA93D0(ctx, base);
	// 82C56438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5643C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C56440: 3BBC0004  addi r29, r28, 4
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	// 82C56444: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C56448: 4BFF6B71  bl 0x82c4cfb8
	ctx.lr = 0x82C5644C;
	sub_82C4CFB8(ctx, base);
	// 82C5644C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56450: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56454: 419A006C  beq cr6, 0x82c564c0
	if ctx.cr[6].eq {
	pc = 0x82C564C0; continue 'dispatch;
	}
	// 82C56458: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C5645C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C56460: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C56464: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C56468: 4BFF75D1  bl 0x82c4da38
	ctx.lr = 0x82C5646C;
	sub_82C4DA38(ctx, base);
	// 82C5646C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56470: 419A003C  beq cr6, 0x82c564ac
	if ctx.cr[6].eq {
	pc = 0x82C564AC; continue 'dispatch;
	}
	// 82C56474: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C56478: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82C5647C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56480: 419A0014  beq cr6, 0x82c56494
	if ctx.cr[6].eq {
	pc = 0x82C56494; continue 'dispatch;
	}
	// 82C56484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56488: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5648C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56490: 4E800421  bctrl
	ctx.lr = 0x82C56494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C56494: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82C56498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5649C: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C564A0: 4BF409D9  bl 0x82b96e78
	ctx.lr = 0x82C564A4;
	sub_82B96E78(ctx, base);
	// 82C564A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C564A8: 4BBEF309  bl 0x828457b0
	ctx.lr = 0x82C564AC;
	sub_828457B0(ctx, base);
	// 82C564AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C564B0: 4BFF6B09  bl 0x82c4cfb8
	ctx.lr = 0x82C564B4;
	sub_82C4CFB8(ctx, base);
	// 82C564B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C564B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C564BC: 409AFFA0  bne cr6, 0x82c5645c
	if !ctx.cr[6].eq {
	pc = 0x82C5645C; continue 'dispatch;
	}
	// 82C564C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C564C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C564C8: 394BB7CC  addi r10, r11, -0x4834
	ctx.r[10].s64 = ctx.r[11].s64 + -18484;
	// 82C564CC: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C564D0: 4BFF76D1  bl 0x82c4dba0
	ctx.lr = 0x82C564D4;
	sub_82C4DBA0(ctx, base);
	// 82C564D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C564D8: 48052F7C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C564E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C564E0 size=208
    let mut pc: u32 = 0x82C564E0;
    'dispatch: loop {
        match pc {
            0x82C564E0 => {
    //   block [0x82C564E0..0x82C565B0)
	// 82C564E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C564E4: 48052F25  bl 0x82ca9408
	ctx.lr = 0x82C564E8;
	sub_82CA93D0(ctx, base);
	// 82C564E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C564EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C564F0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82C564F4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C564F8: 4B5C8D61  bl 0x8221f258
	ctx.lr = 0x82C564FC;
	sub_8221F258(ctx, base);
	// 82C564FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56500: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C56504: 419A0020  beq cr6, 0x82c56524
	if ctx.cr[6].eq {
	pc = 0x82C56524; continue 'dispatch;
	}
	// 82C56508: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C5650C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56510: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C56514: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C56518: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82C5651C: 93A30010  stw r29, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82C56520: 48000008  b 0x82c56528
	pc = 0x82C56528; continue 'dispatch;
	// 82C56524: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82C56528: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C5652C: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82C56530: 4BFFFD59  bl 0x82c56288
	ctx.lr = 0x82C56534;
	sub_82C56288(ctx, base);
	// 82C56534: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C56538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5653C: 4BFBB78D  bl 0x82c11cc8
	ctx.lr = 0x82C56540;
	sub_82C11CC8(ctx, base);
	// 82C56540: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C56544: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82C56548: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5654C: 409A005C  bne cr6, 0x82c565a8
	if !ctx.cr[6].eq {
	pc = 0x82C565A8; continue 'dispatch;
	}
	// 82C56550: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56554: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C56558: 409A0050  bne cr6, 0x82c565a8
	if !ctx.cr[6].eq {
	pc = 0x82C565A8; continue 'dispatch;
	}
	// 82C5655C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56560: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C56564: 409A0044  bne cr6, 0x82c565a8
	if !ctx.cr[6].eq {
	pc = 0x82C565A8; continue 'dispatch;
	}
	// 82C56568: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C5656C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56570: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C56574: 409A001C  bne cr6, 0x82c56590
	if !ctx.cr[6].eq {
	pc = 0x82C56590; continue 'dispatch;
	}
	// 82C56578: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C5657C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C56580: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C56584: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C56588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5658C: 48052ECC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C56590: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56594: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C56598: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C5659C: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C565A0: 93E90008  stw r31, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C565A4: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C565A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C565AC: 48052EAC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C565B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C565B0 size=184
    let mut pc: u32 = 0x82C565B0;
    'dispatch: loop {
        match pc {
            0x82C565B0 => {
    //   block [0x82C565B0..0x82C56668)
	// 82C565B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C565B4: 48052E55  bl 0x82ca9408
	ctx.lr = 0x82C565B8;
	sub_82CA93D0(ctx, base);
	// 82C565B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C565BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C565C0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82C565C4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C565C8: 3BBE0008  addi r29, r30, 8
	ctx.r[29].s64 = ctx.r[30].s64 + 8;
	// 82C565CC: 4B5C8C8D  bl 0x8221f258
	ctx.lr = 0x82C565D0;
	sub_8221F258(ctx, base);
	// 82C565D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C565D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C565D8: 419A0018  beq cr6, 0x82c565f0
	if ctx.cr[6].eq {
	pc = 0x82C565F0; continue 'dispatch;
	}
	// 82C565DC: 4BFF74B5  bl 0x82c4da90
	ctx.lr = 0x82C565E0;
	sub_82C4DA90(ctx, base);
	// 82C565E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C565E4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C565E8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C565EC: 48000008  b 0x82c565f4
	pc = 0x82C565F4; continue 'dispatch;
	// 82C565F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C565F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C565F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82C565FC: 4BFBB6CD  bl 0x82c11cc8
	ctx.lr = 0x82C56600;
	sub_82C11CC8(ctx, base);
	// 82C56600: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C56604: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C56608: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82C5660C: 4BFF6CB5  bl 0x82c4d2c0
	ctx.lr = 0x82C56610;
	sub_82C4D2C0(ctx, base);
	// 82C56610: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56614: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56618: 419A0048  beq cr6, 0x82c56660
	if ctx.cr[6].eq {
	pc = 0x82C56660; continue 'dispatch;
	}
	// 82C5661C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56620: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C56624: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56628: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5662C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56630: 4E800421  bctrl
	ctx.lr = 0x82C56634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C56634: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C56638: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C5663C: 409A0024  bne cr6, 0x82c56660
	if !ctx.cr[6].eq {
	pc = 0x82C56660; continue 'dispatch;
	}
	// 82C56640: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56644: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56648: 419A0018  beq cr6, 0x82c56660
	if ctx.cr[6].eq {
	pc = 0x82C56660; continue 'dispatch;
	}
	// 82C5664C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C56650: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C56654: 409A000C  bne cr6, 0x82c56660
	if !ctx.cr[6].eq {
	pc = 0x82C56660; continue 'dispatch;
	}
	// 82C56658: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5665C: 409AFFC0  bne cr6, 0x82c5661c
	if !ctx.cr[6].eq {
	pc = 0x82C5661C; continue 'dispatch;
	}
	// 82C56660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C56664: 48052DF4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56668 size=216
    let mut pc: u32 = 0x82C56668;
    'dispatch: loop {
        match pc {
            0x82C56668 => {
    //   block [0x82C56668..0x82C56740)
	// 82C56668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5666C: 48052DA1  bl 0x82ca940c
	ctx.lr = 0x82C56670;
	sub_82CA93D0(ctx, base);
	// 82C56670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56674: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C56678: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5667C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C56680: 419A00B0  beq cr6, 0x82c56730
	if ctx.cr[6].eq {
	pc = 0x82C56730; continue 'dispatch;
	}
	// 82C56684: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C56688: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5668C: 393D0008  addi r9, r29, 8
	ctx.r[9].s64 = ctx.r[29].s64 + 8;
	// 82C56690: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C56694: 409A005C  bne cr6, 0x82c566f0
	if !ctx.cr[6].eq {
	pc = 0x82C566F0; continue 'dispatch;
	}
	// 82C56698: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5669C: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C566A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C566A4: 409A0024  bne cr6, 0x82c566c8
	if !ctx.cr[6].eq {
	pc = 0x82C566C8; continue 'dispatch;
	}
	// 82C566A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C566AC: 409A0010  bne cr6, 0x82c566bc
	if !ctx.cr[6].eq {
	pc = 0x82C566BC; continue 'dispatch;
	}
	// 82C566B0: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C566B4: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C566B8: 4800002C  b 0x82c566e4
	pc = 0x82C566E4; continue 'dispatch;
	// 82C566BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C566C0: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82C566C4: 48000020  b 0x82c566e4
	pc = 0x82C566E4; continue 'dispatch;
	// 82C566C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C566CC: 409A0010  bne cr6, 0x82c566dc
	if !ctx.cr[6].eq {
	pc = 0x82C566DC; continue 'dispatch;
	}
	// 82C566D0: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C566D4: 93EA0008  stw r31, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C566D8: 4800000C  b 0x82c566e4
	pc = 0x82C566E4; continue 'dispatch;
	// 82C566DC: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C566E0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C566E4: 93E90004  stw r31, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C566E8: 93E90000  stw r31, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C566EC: 93E90008  stw r31, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C566F0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C566F4: 419A0030  beq cr6, 0x82c56724
	if ctx.cr[6].eq {
	pc = 0x82C56724; continue 'dispatch;
	}
	// 82C566F8: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C566FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56700: 419A0014  beq cr6, 0x82c56714
	if ctx.cr[6].eq {
	pc = 0x82C56714; continue 'dispatch;
	}
	// 82C56704: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56708: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5670C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56710: 4E800421  bctrl
	ctx.lr = 0x82C56714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C56714: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C56718: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C5671C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C56720: 4BBEF091  bl 0x828457b0
	ctx.lr = 0x82C56724;
	sub_828457B0(ctx, base);
	// 82C56724: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56728: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C5672C: 409AFF5C  bne cr6, 0x82c56688
	if !ctx.cr[6].eq {
	pc = 0x82C56688; continue 'dispatch;
	}
	// 82C56730: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82C56734: 4BFFFCFD  bl 0x82c56430
	ctx.lr = 0x82C56738;
	sub_82C56430(ctx, base);
	// 82C56738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5673C: 48052D20  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56740 size=140
    let mut pc: u32 = 0x82C56740;
    'dispatch: loop {
        match pc {
            0x82C56740 => {
    //   block [0x82C56740..0x82C567CC)
	// 82C56740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56744: 48052CC9  bl 0x82ca940c
	ctx.lr = 0x82C56748;
	sub_82CA93D0(ctx, base);
	// 82C56748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5674C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C56750: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C56754: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82C56758: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82C5675C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C56760: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C56764: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82C56768: 4BFF6A71  bl 0x82c4d1d8
	ctx.lr = 0x82C5676C;
	sub_82C4D1D8(ctx, base);
	// 82C5676C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56770: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C56774: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C56778: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C5677C: 4BFF72BD  bl 0x82c4da38
	ctx.lr = 0x82C56780;
	sub_82C4DA38(ctx, base);
	// 82C56780: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56784: 419A0040  beq cr6, 0x82c567c4
	if ctx.cr[6].eq {
	pc = 0x82C567C4; continue 'dispatch;
	}
	// 82C56788: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5678C: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82C56790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56794: 419A0014  beq cr6, 0x82c567a8
	if ctx.cr[6].eq {
	pc = 0x82C567A8; continue 'dispatch;
	}
	// 82C56798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5679C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C567A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C567A4: 4E800421  bctrl
	ctx.lr = 0x82C567A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C567A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C567AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C567B0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C567B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C567B8: 4BF406C1  bl 0x82b96e78
	ctx.lr = 0x82C567BC;
	sub_82B96E78(ctx, base);
	// 82C567BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C567C0: 4BBEEFF1  bl 0x828457b0
	ctx.lr = 0x82C567C4;
	sub_828457B0(ctx, base);
	// 82C567C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C567C8: 48052C94  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C567D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C567D0 size=284
    let mut pc: u32 = 0x82C567D0;
    'dispatch: loop {
        match pc {
            0x82C567D0 => {
    //   block [0x82C567D0..0x82C568EC)
	// 82C567D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C567D4: 48052C31  bl 0x82ca9404
	ctx.lr = 0x82C567D8;
	sub_82CA93D0(ctx, base);
	// 82C567D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C567DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C567E0: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C567E4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C567E8: 419A00FC  beq cr6, 0x82c568e4
	if ctx.cr[6].eq {
	pc = 0x82C568E4; continue 'dispatch;
	}
	// 82C567EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C567F0: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C567F4: 3BFC0008  addi r31, r28, 8
	ctx.r[31].s64 = ctx.r[28].s64 + 8;
	// 82C567F8: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C567FC: 419A000C  beq cr6, 0x82c56808
	if ctx.cr[6].eq {
	pc = 0x82C56808; continue 'dispatch;
	}
	// 82C56800: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82C56804: 48000024  b 0x82c56828
	pc = 0x82C56828; continue 'dispatch;
	// 82C56808: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5680C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56810: 419A0014  beq cr6, 0x82c56824
	if ctx.cr[6].eq {
	pc = 0x82C56824; continue 'dispatch;
	}
	// 82C56814: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C56818: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82C5681C: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C56820: 409A0008  bne cr6, 0x82c56828
	if !ctx.cr[6].eq {
	pc = 0x82C56828; continue 'dispatch;
	}
	// 82C56824: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82C56828: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5682C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56830: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56834: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56838: 4E800421  bctrl
	ctx.lr = 0x82C5683C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5683C: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C56840: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C56844: 419A0094  beq cr6, 0x82c568d8
	if ctx.cr[6].eq {
	pc = 0x82C568D8; continue 'dispatch;
	}
	// 82C56848: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5684C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C56850: 409A005C  bne cr6, 0x82c568ac
	if !ctx.cr[6].eq {
	pc = 0x82C568AC; continue 'dispatch;
	}
	// 82C56854: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5685C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C56860: 409A0024  bne cr6, 0x82c56884
	if !ctx.cr[6].eq {
	pc = 0x82C56884; continue 'dispatch;
	}
	// 82C56864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56868: 409A0010  bne cr6, 0x82c56878
	if !ctx.cr[6].eq {
	pc = 0x82C56878; continue 'dispatch;
	}
	// 82C5686C: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C56870: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C56874: 4800002C  b 0x82c568a0
	pc = 0x82C568A0; continue 'dispatch;
	// 82C56878: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5687C: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C56880: 48000020  b 0x82c568a0
	pc = 0x82C568A0; continue 'dispatch;
	// 82C56884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56888: 409A0010  bne cr6, 0x82c56898
	if !ctx.cr[6].eq {
	pc = 0x82C56898; continue 'dispatch;
	}
	// 82C5688C: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C56890: 93CA0008  stw r30, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C56894: 4800000C  b 0x82c568a0
	pc = 0x82C568A0; continue 'dispatch;
	// 82C56898: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C5689C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C568A0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C568A4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C568A8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C568AC: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C568B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C568B4: 419A0014  beq cr6, 0x82c568c8
	if ctx.cr[6].eq {
	pc = 0x82C568C8; continue 'dispatch;
	}
	// 82C568B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C568BC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C568C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C568C4: 4E800421  bctrl
	ctx.lr = 0x82C568C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C568C8: 93DC0004  stw r30, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C568CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C568D0: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C568D4: 4BBEEEDD  bl 0x828457b0
	ctx.lr = 0x82C568D8;
	sub_828457B0(ctx, base);
	// 82C568D8: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82C568DC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C568E0: 409AFF10  bne cr6, 0x82c567f0
	if !ctx.cr[6].eq {
	pc = 0x82C567F0; continue 'dispatch;
	}
	// 82C568E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C568E8: 48052B6C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C568F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C568F0 size=116
    let mut pc: u32 = 0x82C568F0;
    'dispatch: loop {
        match pc {
            0x82C568F0 => {
    //   block [0x82C568F0..0x82C56964)
	// 82C568F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C568F4: 48052B19  bl 0x82ca940c
	ctx.lr = 0x82C568F8;
	sub_82CA93D0(ctx, base);
	// 82C568F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C568FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C56900: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C56904: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82C56908: 4BFFFE39  bl 0x82c56740
	ctx.lr = 0x82C5690C;
	sub_82C56740(ctx, base);
	// 82C5690C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56910: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56914: 419A0048  beq cr6, 0x82c5695c
	if ctx.cr[6].eq {
	pc = 0x82C5695C; continue 'dispatch;
	}
	// 82C56918: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5691C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C56920: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56924: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56928: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C5692C: 4E800421  bctrl
	ctx.lr = 0x82C56930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C56930: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C56934: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C56938: 409A0024  bne cr6, 0x82c5695c
	if !ctx.cr[6].eq {
	pc = 0x82C5695C; continue 'dispatch;
	}
	// 82C5693C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56940: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56944: 419A0018  beq cr6, 0x82c5695c
	if ctx.cr[6].eq {
	pc = 0x82C5695C; continue 'dispatch;
	}
	// 82C56948: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5694C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C56950: 409A000C  bne cr6, 0x82c5695c
	if !ctx.cr[6].eq {
	pc = 0x82C5695C; continue 'dispatch;
	}
	// 82C56954: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56958: 409AFFC0  bne cr6, 0x82c56918
	if !ctx.cr[6].eq {
	pc = 0x82C56918; continue 'dispatch;
	}
	// 82C5695C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C56960: 48052AFC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56968 size=76
    let mut pc: u32 = 0x82C56968;
    'dispatch: loop {
        match pc {
            0x82C56968 => {
    //   block [0x82C56968..0x82C569B4)
	// 82C56968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5696C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C56970: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C56974: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C56978: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5697C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56980: 48053851  bl 0x82caa1d0
	ctx.lr = 0x82C56984;
	sub_82CAA1D0(ctx, base);
	// 82C56984: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C56988: 48053849  bl 0x82caa1d0
	ctx.lr = 0x82C5698C;
	sub_82CAA1D0(ctx, base);
	// 82C5698C: 7D7E19D6  mullw r11, r30, r3
	ctx.r[11].s64 = (ctx.r[30].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82C56990: 7D4BFB96  divwu r10, r11, r31
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82C56994: 7D2AF9D6  mullw r9, r10, r31
	ctx.r[9].s64 = (ctx.r[10].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82C56998: 7C695850  subf r3, r9, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82C5699C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C569A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C569A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C569A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C569AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C569B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C569B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C569B8 size=92
    let mut pc: u32 = 0x82C569B8;
    'dispatch: loop {
        match pc {
            0x82C569B8 => {
    //   block [0x82C569B8..0x82C56A14)
	// 82C569B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C569BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C569C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C569C4: ED620828  fsubs f11, f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (((ctx.f[2].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C569C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C569CC: ED421828  fsubs f10, f2, f3
	ctx.f[10].f64 = (((ctx.f[2].f64 - ctx.f[3].f64) as f32) as f64);
	// 82C569D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C569D4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C569D8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C569DC: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C569E0: C1AACF20  lfs f13, -0x30e0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12512 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C569E4: C1880AB4  lfs f12, 0xab4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2740 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C569E8: C8290D40  lfd f1, 0xd40(r9)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(3392 as u32) ) };
	// 82C569EC: ED2A5824  fdivs f9, f10, f11
	ctx.f[9].f64 = ((ctx.f[10].f64 / ctx.f[11].f64) as f32) as f64;
	// 82C569F0: ED004828  fsubs f8, f0, f9
	ctx.f[8].f64 = (((ctx.f[0].f64 - ctx.f[9].f64) as f32) as f64);
	// 82C569F4: ECE80372  fmuls f7, f8, f13
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C569F8: EC470332  fmuls f2, f7, f12
	ctx.f[2].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 82C569FC: 4B5A7AAD  bl 0x821fe4a8
	ctx.lr = 0x82C56A00;
	sub_821FE4A8(ctx, base);
	// 82C56A00: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C56A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C56A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C56A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C56A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C56A18 size=116
    let mut pc: u32 = 0x82C56A18;
    'dispatch: loop {
        match pc {
            0x82C56A18 => {
    //   block [0x82C56A18..0x82C56A8C)
	// 82C56A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C56A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56A24: EC030828  fsubs f0, f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[3].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C56A28: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C56A2C: EDA20828  fsubs f13, f2, f1
	ctx.f[13].f64 = (((ctx.f[2].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C56A30: ED806824  fdivs f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82C56A34: C00B9048  lfs f0, -0x6fb8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28600 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C56A38: EC2C003A  fmadds f1, f12, f0, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C56A3C: 4B5E3475  bl 0x82239eb0
	ctx.lr = 0x82C56A40;
	sub_82239EB0(ctx, base);
	// 82C56A40: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C56A44: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C56A48: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C56A4C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C56A50: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C56A54: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C56A58: C1A9CF20  lfs f13, -0x30e0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12512 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C56A5C: C1870AB4  lfs f12, 0xab4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2740 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C56A60: C8280D40  lfd f1, 0xd40(r8)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(3392 as u32) ) };
	// 82C56A64: ED4B002A  fadds f10, f11, f0
	ctx.f[10].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 82C56A68: ED205028  fsubs f9, f0, f10
	ctx.f[9].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82C56A6C: ED090372  fmuls f8, f9, f13
	ctx.f[8].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C56A70: EC480332  fmuls f2, f8, f12
	ctx.f[2].f64 = (((ctx.f[8].f64 * ctx.f[12].f64) as f32) as f64);
	// 82C56A74: 4B5A7A35  bl 0x821fe4a8
	ctx.lr = 0x82C56A78;
	sub_821FE4A8(ctx, base);
	// 82C56A78: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C56A7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C56A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C56A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C56A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C56A90 size=112
    let mut pc: u32 = 0x82C56A90;
    'dispatch: loop {
        match pc {
            0x82C56A90 => {
    //   block [0x82C56A90..0x82C56B00)
	// 82C56A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C56A98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56A9C: EC030828  fsubs f0, f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[3].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C56AA0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82C56AA4: EDA20828  fsubs f13, f2, f1
	ctx.f[13].f64 = (((ctx.f[2].f64 - ctx.f[1].f64) as f32) as f64);
	// 82C56AA8: ED806824  fdivs f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82C56AAC: C00B9048  lfs f0, -0x6fb8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28600 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C56AB0: EC2C0032  fmuls f1, f12, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C56AB4: 4B5E33FD  bl 0x82239eb0
	ctx.lr = 0x82C56AB8;
	sub_82239EB0(ctx, base);
	// 82C56AB8: FD600818  frsp f11, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C56ABC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C56AC0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C56AC4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C56AC8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C56ACC: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C56AD0: C1A9CF20  lfs f13, -0x30e0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12512 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C56AD4: C1870AB4  lfs f12, 0xab4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2740 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C56AD8: C8280D40  lfd f1, 0xd40(r8)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(3392 as u32) ) };
	// 82C56ADC: ED405828  fsubs f10, f0, f11
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 82C56AE0: ED2A0372  fmuls f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C56AE4: EC490332  fmuls f2, f9, f12
	ctx.f[2].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82C56AE8: 4B5A79C1  bl 0x821fe4a8
	ctx.lr = 0x82C56AEC;
	sub_821FE4A8(ctx, base);
	// 82C56AEC: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C56AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C56AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C56AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C56AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C56B00 size=48
    let mut pc: u32 = 0x82C56B00;
    'dispatch: loop {
        match pc {
            0x82C56B00 => {
    //   block [0x82C56B00..0x82C56B30)
	// 82C56B00: FF011000  fcmpu cr6, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[2].f64);
	// 82C56B04: 40990018  ble cr6, 0x82c56b1c
	if !ctx.cr[6].gt {
	pc = 0x82C56B1C; continue 'dispatch;
	}
	// 82C56B08: EDA11828  fsubs f13, f1, f3
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[3].f64) as f32) as f64);
	// 82C56B0C: FC000890  fmr f0, f1
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82C56B10: FC201090  fmr f1, f2
	ctx.f[1].f64 = ctx.f[2].f64;
	// 82C56B14: EC6D102A  fadds f3, f13, f2
	ctx.f[3].f64 = ((ctx.f[13].f64 + ctx.f[2].f64) as f32) as f64;
	// 82C56B18: FC400090  fmr f2, f0
	ctx.f[2].f64 = ctx.f[0].f64;
	// 82C56B1C: FF030800  fcmpu cr6, f3, f1
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[1].f64);
	// 82C56B20: 41990010  bgt cr6, 0x82c56b30
	if ctx.cr[6].gt {
		sub_82C56B30(ctx, base);
		return;
	}
	// 82C56B24: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C56B28: C02B0C14  lfs f1, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C56B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C56B30 size=20
    let mut pc: u32 = 0x82C56B30;
    'dispatch: loop {
        match pc {
            0x82C56B30 => {
    //   block [0x82C56B30..0x82C56B44)
	// 82C56B30: FF031000  fcmpu cr6, f3, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[2].f64);
	// 82C56B34: 41980010  blt cr6, 0x82c56b44
	if ctx.cr[6].lt {
		sub_82C56B44(ctx, base);
		return;
	}
	// 82C56B38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C56B3C: C02B0C18  lfs f1, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C56B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56B44 size=24
    let mut pc: u32 = 0x82C56B44;
    'dispatch: loop {
        match pc {
            0x82C56B44 => {
    //   block [0x82C56B44..0x82C56B5C)
	// 82C56B44: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82C56B48: 41980018  blt cr6, 0x82c56b60
	if ctx.cr[6].lt {
		sub_82C56B60(ctx, base);
		return;
	}
	// 82C56B4C: 419A0010  beq cr6, 0x82c56b5c
	if ctx.cr[6].eq {
		sub_82C56B5C(ctx, base);
		return;
	}
	// 82C56B50: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82C56B54: 4098FFE4  bge cr6, 0x82c56b38
	if !ctx.cr[6].lt {
		sub_82C56B30(ctx, base);
		return;
	}
	// 82C56B58: 4BFFFF38  b 0x82c56a90
	sub_82C56A90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56B5C size=4
    let mut pc: u32 = 0x82C56B5C;
    'dispatch: loop {
        match pc {
            0x82C56B5C => {
    //   block [0x82C56B5C..0x82C56B60)
	// 82C56B5C: 4BFFFEBC  b 0x82c56a18
	sub_82C56A18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56B60 size=4
    let mut pc: u32 = 0x82C56B60;
    'dispatch: loop {
        match pc {
            0x82C56B60 => {
    //   block [0x82C56B60..0x82C56B64)
	// 82C56B60: 4BFFFE58  b 0x82c569b8
	sub_82C569B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56B68 size=16
    let mut pc: u32 = 0x82C56B68;
    'dispatch: loop {
        match pc {
            0x82C56B68 => {
    //   block [0x82C56B68..0x82C56B78)
	// 82C56B68: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56B6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56B70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56B74: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56B78 size=8
    let mut pc: u32 = 0x82C56B78;
    'dispatch: loop {
        match pc {
            0x82C56B78 => {
    //   block [0x82C56B78..0x82C56B80)
	// 82C56B78: 886B0001  lbz r3, 1(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82C56B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56B80 size=40
    let mut pc: u32 = 0x82C56B80;
    'dispatch: loop {
        match pc {
            0x82C56B80 => {
    //   block [0x82C56B80..0x82C56BA8)
	// 82C56B80: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56B84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56B88: 419A0020  beq cr6, 0x82c56ba8
	if ctx.cr[6].eq {
		sub_82C56BA8(ctx, base);
		return;
	}
	// 82C56B8C: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82C56B90: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C56B94: 40980014  bge cr6, 0x82c56ba8
	if !ctx.cr[6].lt {
		sub_82C56BA8(ctx, base);
		return;
	}
	// 82C56B98: 3944000A  addi r10, r4, 0xa
	ctx.r[10].s64 = ctx.r[4].s64 + 10;
	// 82C56B9C: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C56BA0: 7C6958AE  lbzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C56BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BA8 size=8
    let mut pc: u32 = 0x82C56BA8;
    'dispatch: loop {
        match pc {
            0x82C56BA8 => {
    //   block [0x82C56BA8..0x82C56BB0)
	// 82C56BA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BB0 size=16
    let mut pc: u32 = 0x82C56BB0;
    'dispatch: loop {
        match pc {
            0x82C56BB0 => {
    //   block [0x82C56BB0..0x82C56BC0)
	// 82C56BB0: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56BB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56BB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56BBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BC0 size=8
    let mut pc: u32 = 0x82C56BC0;
    'dispatch: loop {
        match pc {
            0x82C56BC0 => {
    //   block [0x82C56BC0..0x82C56BC8)
	// 82C56BC0: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C56BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BC8 size=16
    let mut pc: u32 = 0x82C56BC8;
    'dispatch: loop {
        match pc {
            0x82C56BC8 => {
    //   block [0x82C56BC8..0x82C56BD8)
	// 82C56BC8: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56BCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56BD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56BD4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BD8 size=8
    let mut pc: u32 = 0x82C56BD8;
    'dispatch: loop {
        match pc {
            0x82C56BD8 => {
    //   block [0x82C56BD8..0x82C56BE0)
	// 82C56BD8: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C56BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BE0 size=8
    let mut pc: u32 = 0x82C56BE0;
    'dispatch: loop {
        match pc {
            0x82C56BE0 => {
    //   block [0x82C56BE0..0x82C56BE8)
	// 82C56BE0: 80630068  lwz r3, 0x68(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C56BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BE8 size=16
    let mut pc: u32 = 0x82C56BE8;
    'dispatch: loop {
        match pc {
            0x82C56BE8 => {
    //   block [0x82C56BE8..0x82C56BF8)
	// 82C56BE8: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56BEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56BF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56BF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56BF8 size=8
    let mut pc: u32 = 0x82C56BF8;
    'dispatch: loop {
        match pc {
            0x82C56BF8 => {
    //   block [0x82C56BF8..0x82C56C00)
	// 82C56BF8: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C56BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56C00 size=20
    let mut pc: u32 = 0x82C56C00;
    'dispatch: loop {
        match pc {
            0x82C56C00 => {
    //   block [0x82C56C00..0x82C56C14)
	// 82C56C00: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56C04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56C08: 409A000C  bne cr6, 0x82c56c14
	if !ctx.cr[6].eq {
		sub_82C56C14(ctx, base);
		return;
	}
	// 82C56C0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56C14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56C14 size=32
    let mut pc: u32 = 0x82C56C14;
    'dispatch: loop {
        match pc {
            0x82C56C14 => {
    //   block [0x82C56C14..0x82C56C34)
	// 82C56C14: 81430064  lwz r10, 0x64(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C56C18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C56C1C: 419A0018  beq cr6, 0x82c56c34
	if ctx.cr[6].eq {
		sub_82C56C34(ctx, base);
		return;
	}
	// 82C56C20: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56C24: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56C28: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56C2C: 7C695050  subf r3, r9, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C56C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56C34 size=8
    let mut pc: u32 = 0x82C56C34;
    'dispatch: loop {
        match pc {
            0x82C56C34 => {
    //   block [0x82C56C34..0x82C56C3C)
	// 82C56C34: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C56C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56C40 size=16
    let mut pc: u32 = 0x82C56C40;
    'dispatch: loop {
        match pc {
            0x82C56C40 => {
    //   block [0x82C56C40..0x82C56C50)
	// 82C56C40: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C56C44: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82C56C48: 9163006C  stw r11, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82C56C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56C50 size=44
    let mut pc: u32 = 0x82C56C50;
    'dispatch: loop {
        match pc {
            0x82C56C50 => {
    //   block [0x82C56C50..0x82C56C7C)
	// 82C56C50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C56C54: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56C58: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C56C5C: 419A0108  beq cr6, 0x82c56d64
	if ctx.cr[6].eq {
		sub_82C56D18(ctx, base);
		return;
	}
	// 82C56C60: 806B0064  lwz r3, 0x64(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C56C64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56C68: 419A0014  beq cr6, 0x82c56c7c
	if ctx.cr[6].eq {
		sub_82C56C7C(ctx, base);
		return;
	}
	// 82C56C6C: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56C70: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56C74: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82C56C78: 48000008  b 0x82c56c80
	sub_82C56C7C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56C7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56C7C size=156
    let mut pc: u32 = 0x82C56C7C;
    'dispatch: loop {
        match pc {
            0x82C56C7C => {
    //   block [0x82C56C7C..0x82C56D18)
	// 82C56C7C: 8149001C  lwz r10, 0x1c(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C56C80: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C56C84: 409800E0  bge cr6, 0x82c56d64
	if !ctx.cr[6].lt {
		sub_82C56D18(ctx, base);
		return;
	}
	// 82C56C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C56C8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56C90: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82C56C94: 419A0084  beq cr6, 0x82c56d18
	if ctx.cr[6].eq {
		sub_82C56D18(ctx, base);
		return;
	}
	// 82C56C98: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56C9C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82C56CA0: 5549C9FA  rlwinm r9, r10, 0x19, 7, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	// 82C56CA4: 554ACFBE  rlwinm r10, r10, 0x19, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	// 82C56CA8: 7D29182E  lwzx r9, r9, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82C56CAC: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C56CB0: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C56CB4: 80EB005C  lwz r7, 0x5c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56CB8: 80660000  lwz r3, 0(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56CBC: 81470018  lwz r10, 0x18(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C56CC0: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C56CC4: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C56CC8: 41980020  blt cr6, 0x82c56ce8
	if ctx.cr[6].lt {
	pc = 0x82C56CE8; continue 'dispatch;
	}
	// 82C56CCC: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56CD0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82C56CD4: 7CEA4850  subf r7, r10, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82C56CD8: 54E3003E  slwi r3, r7, 0
	ctx.r[3].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C56CDC: 90E60000  stw r7, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C56CE0: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C56CE4: 4098FFE8  bge cr6, 0x82c56ccc
	if !ctx.cr[6].lt {
	pc = 0x82C56CCC; continue 'dispatch;
	}
	// 82C56CE8: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56CEC: 812A0024  lwz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C56CF0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C56CF4: 40980070  bge cr6, 0x82c56d64
	if !ctx.cr[6].lt {
		sub_82C56D18(ctx, base);
		return;
	}
	// 82C56CF8: 548A0030  rlwinm r10, r4, 0, 0, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82C56CFC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C56D00: 814B005C  lwz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56D04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C56D08: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C56D0C: 7D0941D6  mullw r8, r9, r8
	ctx.r[8].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82C56D10: 910B006C  stw r8, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82C56D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56D18 size=84
    let mut pc: u32 = 0x82C56D18;
    'dispatch: loop {
        match pc {
            0x82C56D18 => {
    //   block [0x82C56D18..0x82C56D6C)
	// 82C56D18: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82C56D1C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C56D20: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C56D24: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C56D28: 80EB005C  lwz r7, 0x5c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56D2C: 80C70024  lwz r6, 0x24(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C56D30: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C56D34: 40990030  ble cr6, 0x82c56d64
	if !ctx.cr[6].gt {
	pc = 0x82C56D64; continue 'dispatch;
	}
	// 82C56D38: 812B0060  lwz r9, 0x60(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C56D3C: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82C56D40: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C56D44: 4199FFBC  bgt cr6, 0x82c56d00
	if ctx.cr[6].gt {
		sub_82C56C7C(ctx, base);
		return;
	}
	// 82C56D48: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C56D4C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82C56D50: 812B005C  lwz r9, 0x5c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C56D54: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C56D58: 80E90024  lwz r7, 0x24(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C56D5C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C56D60: 4198FFD8  blt cr6, 0x82c56d38
	if ctx.cr[6].lt {
	pc = 0x82C56D38; continue 'dispatch;
	}
	// 82C56D64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56D70 size=60
    let mut pc: u32 = 0x82C56D70;
    'dispatch: loop {
        match pc {
            0x82C56D70 => {
    //   block [0x82C56D70..0x82C56DAC)
	// 82C56D70: 3D606672  lis r11, 0x6672
	ctx.r[11].s64 = 1718747136;
	// 82C56D74: 616B736B  ori r11, r11, 0x736b
	ctx.r[11].u64 = ctx.r[11].u64 | 29547;
	// 82C56D78: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C56D7C: 41990040  bgt cr6, 0x82c56dbc
	if ctx.cr[6].gt {
		sub_82C56DBC(ctx, base);
		return;
	}
	// 82C56D80: 419A0034  beq cr6, 0x82c56db4
	if ctx.cr[6].eq {
		sub_82C56DB4(ctx, base);
		return;
	}
	// 82C56D84: 3D60584D  lis r11, 0x584d
	ctx.r[11].s64 = 1481441280;
	// 82C56D88: 616A4132  ori r10, r11, 0x4132
	ctx.r[10].u64 = ctx.r[11].u64 | 16690;
	// 82C56D8C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C56D90: 419A001C  beq cr6, 0x82c56dac
	if ctx.cr[6].eq {
		sub_82C56DAC(ctx, base);
		return;
	}
	// 82C56D94: 3D606461  lis r11, 0x6461
	ctx.r[11].s64 = 1684078592;
	// 82C56D98: 616A7461  ori r10, r11, 0x7461
	ctx.r[10].u64 = ctx.r[11].u64 | 29793;
	// 82C56D9C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C56DA0: 409A002C  bne cr6, 0x82c56dcc
	if !ctx.cr[6].eq {
		sub_82C56DBC(ctx, base);
		return;
	}
	// 82C56DA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56DAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56DAC size=8
    let mut pc: u32 = 0x82C56DAC;
    'dispatch: loop {
        match pc {
            0x82C56DAC => {
    //   block [0x82C56DAC..0x82C56DB4)
	// 82C56DAC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82C56DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56DB4 size=8
    let mut pc: u32 = 0x82C56DB4;
    'dispatch: loop {
        match pc {
            0x82C56DB4 => {
    //   block [0x82C56DB4..0x82C56DBC)
	// 82C56DB4: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82C56DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56DBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56DBC size=24
    let mut pc: u32 = 0x82C56DBC;
    'dispatch: loop {
        match pc {
            0x82C56DBC => {
    //   block [0x82C56DBC..0x82C56DD4)
	// 82C56DBC: 3D607365  lis r11, 0x7365
	ctx.r[11].s64 = 1935998976;
	// 82C56DC0: 616A656B  ori r10, r11, 0x656b
	ctx.r[10].u64 = ctx.r[11].u64 | 25963;
	// 82C56DC4: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C56DC8: 419A000C  beq cr6, 0x82c56dd4
	if ctx.cr[6].eq {
		sub_82C56DD4(ctx, base);
		return;
	}
	// 82C56DCC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82C56DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56DD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56DD4 size=8
    let mut pc: u32 = 0x82C56DD4;
    'dispatch: loop {
        match pc {
            0x82C56DD4 => {
    //   block [0x82C56DD4..0x82C56DDC)
	// 82C56DD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C56DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56DE0 size=20
    let mut pc: u32 = 0x82C56DE0;
    'dispatch: loop {
        match pc {
            0x82C56DE0 => {
    //   block [0x82C56DE0..0x82C56DF4)
	// 82C56DE0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C56DE4: 419A0010  beq cr6, 0x82c56df4
	if ctx.cr[6].eq {
		sub_82C56DF4(ctx, base);
		return;
	}
	// 82C56DE8: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82C56DEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C56DF0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56DF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56DF4 size=8
    let mut pc: u32 = 0x82C56DF4;
    'dispatch: loop {
        match pc {
            0x82C56DF4 => {
    //   block [0x82C56DF4..0x82C56DFC)
	// 82C56DF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56E00 size=84
    let mut pc: u32 = 0x82C56E00;
    'dispatch: loop {
        match pc {
            0x82C56E00 => {
    //   block [0x82C56E00..0x82C56E54)
	// 82C56E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56E04: 48052609  bl 0x82ca940c
	ctx.lr = 0x82C56E08;
	sub_82CA93D0(ctx, base);
	// 82C56E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56E0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C56E10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C56E14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C56E18: 392BCF24  addi r9, r11, -0x30dc
	ctx.r[9].s64 = ctx.r[11].s64 + -12508;
	// 82C56E1C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82C56E20: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C56E24: 3BDD000C  addi r30, r29, 0xc
	ctx.r[30].s64 = ctx.r[29].s64 + 12;
	// 82C56E28: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C56E2C: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 82C56E30: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C56E34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C56E38: 480005B1  bl 0x82c573e8
	ctx.lr = 0x82C56E3C;
	sub_82C573E8(ctx, base);
	// 82C56E3C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82C56E40: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 82C56E44: 4080FFF0  bge 0x82c56e34
	if !ctx.cr[0].lt {
	pc = 0x82C56E34; continue 'dispatch;
	}
	// 82C56E48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C56E4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C56E50: 4805260C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56E58 size=92
    let mut pc: u32 = 0x82C56E58;
    'dispatch: loop {
        match pc {
            0x82C56E58 => {
    //   block [0x82C56E58..0x82C56EB4)
	// 82C56E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56E5C: 480525B1  bl 0x82ca940c
	ctx.lr = 0x82C56E60;
	sub_82CA93D0(ctx, base);
	// 82C56E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56E64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C56E68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C56E6C: 3BFD005C  addi r31, r29, 0x5c
	ctx.r[31].s64 = ctx.r[29].s64 + 92;
	// 82C56E70: 394BCF24  addi r10, r11, -0x30dc
	ctx.r[10].s64 = ctx.r[11].s64 + -12508;
	// 82C56E74: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 82C56E78: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C56E7C: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 82C56E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C56E84: 48000705  bl 0x82c57588
	ctx.lr = 0x82C56E88;
	sub_82C57588(ctx, base);
	// 82C56E88: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C56E8C: 4080FFF0  bge 0x82c56e7c
	if !ctx.cr[0].lt {
	pc = 0x82C56E7C; continue 'dispatch;
	}
	// 82C56E90: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56E94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C56E98: 419A0014  beq cr6, 0x82c56eac
	if ctx.cr[6].eq {
	pc = 0x82C56EAC; continue 'dispatch;
	}
	// 82C56E9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C56EA0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C56EA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C56EA8: 4E800421  bctrl
	ctx.lr = 0x82C56EAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C56EAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C56EB0: 480525AC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56EB8 size=80
    let mut pc: u32 = 0x82C56EB8;
    'dispatch: loop {
        match pc {
            0x82C56EB8 => {
    //   block [0x82C56EB8..0x82C56F08)
	// 82C56EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C56EC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C56EC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C56EC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56ECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56ED0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C56ED4: 4BFFFF85  bl 0x82c56e58
	ctx.lr = 0x82C56ED8;
	sub_82C56E58(ctx, base);
	// 82C56ED8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C56EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C56EE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C56EE4: 419A000C  beq cr6, 0x82c56ef0
	if ctx.cr[6].eq {
	pc = 0x82C56EF0; continue 'dispatch;
	}
	// 82C56EE8: 4BBEE8C9  bl 0x828457b0
	ctx.lr = 0x82C56EEC;
	sub_828457B0(ctx, base);
	// 82C56EEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C56EF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C56EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C56EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C56EFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C56F00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C56F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56F08 size=84
    let mut pc: u32 = 0x82C56F08;
    'dispatch: loop {
        match pc {
            0x82C56F08 => {
    //   block [0x82C56F08..0x82C56F5C)
	// 82C56F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C56F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C56F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C56F1C: 4BFFFEE5  bl 0x82c56e00
	ctx.lr = 0x82C56F20;
	sub_82C56E00(ctx, base);
	// 82C56F20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C56F24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C56F28: 392ACF34  addi r9, r10, -0x30cc
	ctx.r[9].s64 = ctx.r[10].s64 + -12492;
	// 82C56F2C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82C56F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C56F34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C56F38: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C56F3C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82C56F40: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82C56F44: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82C56F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C56F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C56F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C56F54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C56F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C56F60 size=32
    let mut pc: u32 = 0x82C56F60;
    'dispatch: loop {
        match pc {
            0x82C56F60 => {
    //   block [0x82C56F60..0x82C56F80)
	// 82C56F60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C56F64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C56F68: 392ACF34  addi r9, r10, -0x30cc
	ctx.r[9].s64 = ctx.r[10].s64 + -12492;
	// 82C56F6C: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82C56F70: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C56F74: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C56F78: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82C56F7C: 4BFFFEDC  b 0x82c56e58
	sub_82C56E58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C56F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C56F80 size=252
    let mut pc: u32 = 0x82C56F80;
    'dispatch: loop {
        match pc {
            0x82C56F80 => {
    //   block [0x82C56F80..0x82C5707C)
	// 82C56F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C56F84: 4805247D  bl 0x82ca9400
	ctx.lr = 0x82C56F88;
	sub_82CA93D0(ctx, base);
	// 82C56F88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C56F8C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82C56F90: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C56F94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C56F98: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C56F9C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C56FA0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C56FA4: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56FA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C56FAC: 419A00C4  beq cr6, 0x82c57070
	if ctx.cr[6].eq {
	pc = 0x82C57070; continue 'dispatch;
	}
	// 82C56FB0: 3BDC000C  addi r30, r28, 0xc
	ctx.r[30].s64 = ctx.r[28].s64 + 12;
	// 82C56FB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C56FB8: 4849C959  bl 0x830f3910
	ctx.lr = 0x82C56FBC;
	sub_830F3910(ctx, base);
	// 82C56FBC: 80BC006C  lwz r5, 0x6c(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C56FC0: 7F032840  cmplw cr6, r3, r5
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C56FC4: 41980010  blt cr6, 0x82c56fd4
	if ctx.cr[6].lt {
	pc = 0x82C56FD4; continue 'dispatch;
	}
	// 82C56FC8: 7FE51850  subf r31, r5, r3
	ctx.r[31].s64 = ctx.r[3].s64 - ctx.r[5].s64;
	// 82C56FCC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C56FD0: 409A0010  bne cr6, 0x82c56fe0
	if !ctx.cr[6].eq {
	pc = 0x82C56FE0; continue 'dispatch;
	}
	// 82C56FD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C56FD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C56FDC: 48052474  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C56FE0: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C56FE4: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C56FE8: 40980048  bge cr6, 0x82c57030
	if !ctx.cr[6].lt {
	pc = 0x82C57030; continue 'dispatch;
	}
	// 82C56FEC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C56FF0: 409A000C  bne cr6, 0x82c56ffc
	if !ctx.cr[6].eq {
	pc = 0x82C56FFC; continue 'dispatch;
	}
	// 82C56FF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C56FF8: 48000014  b 0x82c5700c
	pc = 0x82C5700C; continue 'dispatch;
	// 82C56FFC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82C57000: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C57004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C57008: 48000451  bl 0x82c57458
	ctx.lr = 0x82C5700C;
	sub_82C57458(ctx, base);
	// 82C5700C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C57010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57014: 419A005C  beq cr6, 0x82c57070
	if ctx.cr[6].eq {
	pc = 0x82C57070; continue 'dispatch;
	}
	// 82C57018: 7CBFD850  subf r5, r31, r27
	ctx.r[5].s64 = ctx.r[27].s64 - ctx.r[31].s64;
	// 82C5701C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C57020: 7C7FEA14  add r3, r31, r29
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82C57024: 4805298D  bl 0x82ca99b0
	ctx.lr = 0x82C57028;
	sub_82CA99B0(ctx, base);
	// 82C57028: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C5702C: 48000034  b 0x82c57060
	pc = 0x82C57060; continue 'dispatch;
	// 82C57030: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C57034: 409A000C  bne cr6, 0x82c57040
	if !ctx.cr[6].eq {
	pc = 0x82C57040; continue 'dispatch;
	}
	// 82C57038: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C5703C: 48000014  b 0x82c57050
	pc = 0x82C57050; continue 'dispatch;
	// 82C57040: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C57044: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82C57048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5704C: 4800040D  bl 0x82c57458
	ctx.lr = 0x82C57050;
	sub_82C57458(ctx, base);
	// 82C57050: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C57054: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57058: 419A0018  beq cr6, 0x82c57070
	if ctx.cr[6].eq {
	pc = 0x82C57070; continue 'dispatch;
	}
	// 82C5705C: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82C57060: 817C006C  lwz r11, 0x6c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C57064: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57068: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C5706C: 917C006C  stw r11, 0x6c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82C57070: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57074: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C57078: 480523D8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57080 size=188
    let mut pc: u32 = 0x82C57080;
    'dispatch: loop {
        match pc {
            0x82C57080 => {
    //   block [0x82C57080..0x82C5713C)
	// 82C57080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57084: 4805237D  bl 0x82ca9400
	ctx.lr = 0x82C57088;
	sub_82CA93D0(ctx, base);
	// 82C57088: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5708C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C57090: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C57094: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C57098: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C5709C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C570A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C570A4: 419A0064  beq cr6, 0x82c57108
	if ctx.cr[6].eq {
	pc = 0x82C57108; continue 'dispatch;
	}
	// 82C570A8: 3B7E000C  addi r27, r30, 0xc
	ctx.r[27].s64 = ctx.r[30].s64 + 12;
	// 82C570AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C570B0: 4849C861  bl 0x830f3910
	ctx.lr = 0x82C570B4;
	sub_830F3910(ctx, base);
	// 82C570B4: 80DE006C  lwz r6, 0x6c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C570B8: 7F033040  cmplw cr6, r3, r6
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C570BC: 4198004C  blt cr6, 0x82c57108
	if ctx.cr[6].lt {
	pc = 0x82C57108; continue 'dispatch;
	}
	// 82C570C0: 7FE61850  subf r31, r6, r3
	ctx.r[31].s64 = ctx.r[3].s64 - ctx.r[6].s64;
	// 82C570C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C570C8: 419A0040  beq cr6, 0x82c57108
	if ctx.cr[6].eq {
	pc = 0x82C57108; continue 'dispatch;
	}
	// 82C570CC: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C570D0: 4098002C  bge cr6, 0x82c570fc
	if !ctx.cr[6].lt {
	pc = 0x82C570FC; continue 'dispatch;
	}
	// 82C570D4: 7CBFE050  subf r5, r31, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82C570D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C570DC: 7C7FEA14  add r3, r31, r29
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82C570E0: 480528D1  bl 0x82ca99b0
	ctx.lr = 0x82C570E4;
	sub_82CA99B0(ctx, base);
	// 82C570E4: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C570E8: 80DE006C  lwz r6, 0x6c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C570EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C570F0: 419A0018  beq cr6, 0x82c57108
	if ctx.cr[6].eq {
	pc = 0x82C57108; continue 'dispatch;
	}
	// 82C570F4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82C570F8: 48000028  b 0x82c57120
	pc = 0x82C57120; continue 'dispatch;
	// 82C570FC: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57100: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C57104: 409A0018  bne cr6, 0x82c5711c
	if !ctx.cr[6].eq {
	pc = 0x82C5711C; continue 'dispatch;
	}
	// 82C57108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5710C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C57110: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C57114: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C57118: 48052338  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C5711C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82C57120: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82C57124: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C57128: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C5712C: 480003BD  bl 0x82c574e8
	ctx.lr = 0x82C57130;
	sub_82C574E8(ctx, base);
	// 82C57130: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82C57134: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C57138: 48052318  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C57140 size=84
    let mut pc: u32 = 0x82C57140;
    'dispatch: loop {
        match pc {
            0x82C57140 => {
    //   block [0x82C57140..0x82C57194)
	// 82C57140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5714C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57150: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57154: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57158: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C5715C: 409A000C  bne cr6, 0x82c57168
	if !ctx.cr[6].eq {
	pc = 0x82C57168; continue 'dispatch;
	}
	// 82C57160: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57164: 4800000C  b 0x82c57170
	pc = 0x82C57170; continue 'dispatch;
	// 82C57168: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C5716C: 4849C7A5  bl 0x830f3910
	ctx.lr = 0x82C57170;
	sub_830F3910(ctx, base);
	// 82C57170: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C57174: 7D435810  subfc r10, r3, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[3].u32;
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82C57178: 7D6A5110  subfe r11, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C5717C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82C57180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C57184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5718C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57198 size=132
    let mut pc: u32 = 0x82C57198;
    'dispatch: loop {
        match pc {
            0x82C57198 => {
    //   block [0x82C57198..0x82C5721C)
	// 82C57198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5719C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C571A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C571A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C571A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C571AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C571B0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C571B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C571B8: 419A0018  beq cr6, 0x82c571d0
	if ctx.cr[6].eq {
	pc = 0x82C571D0; continue 'dispatch;
	}
	// 82C571BC: 3BFE0034  addi r31, r30, 0x34
	ctx.r[31].s64 = ctx.r[30].s64 + 52;
	// 82C571C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C571C4: 4849C74D  bl 0x830f3910
	ctx.lr = 0x82C571C8;
	sub_830F3910(ctx, base);
	// 82C571C8: 2B03002C  cmplwi cr6, r3, 0x2c
	ctx.cr[6].compare_u32(ctx.r[3].u32, 44 as u32, &mut ctx.xer);
	// 82C571CC: 4098000C  bge cr6, 0x82c571d8
	if !ctx.cr[6].lt {
	pc = 0x82C571D8; continue 'dispatch;
	}
	// 82C571D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C571D4: 48000030  b 0x82c57204
	pc = 0x82C57204; continue 'dispatch;
	// 82C571D8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C571DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C571E0: 409A000C  bne cr6, 0x82c571ec
	if !ctx.cr[6].eq {
	pc = 0x82C571EC; continue 'dispatch;
	}
	// 82C571E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C571E8: 4800000C  b 0x82c571f4
	pc = 0x82C571F4; continue 'dispatch;
	// 82C571EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C571F0: 4825E3B9  bl 0x82eb55a8
	ctx.lr = 0x82C571F4;
	sub_82EB55A8(ctx, base);
	// 82C571F4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82C571F8: 907E005C  stw r3, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82C571FC: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82C57200: 69430001  xori r3, r10, 1
	ctx.r[3].u64 = ctx.r[10].u64 ^ 1;
	// 82C57204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5720C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57210: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C57214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57220 size=132
    let mut pc: u32 = 0x82C57220;
    'dispatch: loop {
        match pc {
            0x82C57220 => {
    //   block [0x82C57220..0x82C572A4)
	// 82C57220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5722C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57234: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C57238: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5723C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C57240: 419A0018  beq cr6, 0x82c57258
	if ctx.cr[6].eq {
	pc = 0x82C57258; continue 'dispatch;
	}
	// 82C57244: 3BFE0048  addi r31, r30, 0x48
	ctx.r[31].s64 = ctx.r[30].s64 + 72;
	// 82C57248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5724C: 4849C6C5  bl 0x830f3910
	ctx.lr = 0x82C57250;
	sub_830F3910(ctx, base);
	// 82C57250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C57254: 409A000C  bne cr6, 0x82c57260
	if !ctx.cr[6].eq {
	pc = 0x82C57260; continue 'dispatch;
	}
	// 82C57258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C5725C: 48000030  b 0x82c5728c
	pc = 0x82C5728C; continue 'dispatch;
	// 82C57260: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57264: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C57268: 409A000C  bne cr6, 0x82c57274
	if !ctx.cr[6].eq {
	pc = 0x82C57274; continue 'dispatch;
	}
	// 82C5726C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57270: 4800000C  b 0x82c5727c
	pc = 0x82C5727C; continue 'dispatch;
	// 82C57274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57278: 4825E331  bl 0x82eb55a8
	ctx.lr = 0x82C5727C;
	sub_82EB55A8(ctx, base);
	// 82C5727C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82C57280: 907E0064  stw r3, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 82C57284: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82C57288: 69430001  xori r3, r10, 1
	ctx.r[3].u64 = ctx.r[10].u64 ^ 1;
	// 82C5728C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5729C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C572A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C572A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C572A8 size=108
    let mut pc: u32 = 0x82C572A8;
    'dispatch: loop {
        match pc {
            0x82C572A8 => {
    //   block [0x82C572A8..0x82C57314)
	// 82C572A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C572AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C572B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C572B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C572B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C572BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C572C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C572C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C572C8: 392ACF34  addi r9, r10, -0x30cc
	ctx.r[9].s64 = ctx.r[10].s64 + -12492;
	// 82C572CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C572D0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82C572D4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C572D8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C572DC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82C572E0: 4BFFFB79  bl 0x82c56e58
	ctx.lr = 0x82C572E4;
	sub_82C56E58(ctx, base);
	// 82C572E4: 57C807FE  clrlwi r8, r30, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C572E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C572EC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C572F0: 419A000C  beq cr6, 0x82c572fc
	if ctx.cr[6].eq {
	pc = 0x82C572FC; continue 'dispatch;
	}
	// 82C572F4: 4BBEE4BD  bl 0x828457b0
	ctx.lr = 0x82C572F8;
	sub_828457B0(ctx, base);
	// 82C572F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C572FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57308: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5730C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57318 size=204
    let mut pc: u32 = 0x82C57318;
    'dispatch: loop {
        match pc {
            0x82C57318 => {
    //   block [0x82C57318..0x82C573E4)
	// 82C57318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5732C: 4BFFFE6D  bl 0x82c57198
	ctx.lr = 0x82C57330;
	sub_82C57198(ctx, base);
	// 82C57330: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C57334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57338: 409A001C  bne cr6, 0x82c57354
	if !ctx.cr[6].eq {
	pc = 0x82C57354; continue 'dispatch;
	}
	// 82C5733C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C57344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5734C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57350: 4E800020  blr
	return;
	// 82C57354: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57358: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C5735C: 409A000C  bne cr6, 0x82c57368
	if !ctx.cr[6].eq {
	pc = 0x82C57368; continue 'dispatch;
	}
	// 82C57360: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57364: 4800000C  b 0x82c57370
	pc = 0x82C57370; continue 'dispatch;
	// 82C57368: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82C5736C: 4825E23D  bl 0x82eb55a8
	ctx.lr = 0x82C57370;
	sub_82EB55A8(ctx, base);
	// 82C57370: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82C57374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C57378: 419AFFC4  beq cr6, 0x82c5733c
	if ctx.cr[6].eq {
	pc = 0x82C5733C; continue 'dispatch;
	}
	// 82C5737C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57380: 4BFFFEA1  bl 0x82c57220
	ctx.lr = 0x82C57384;
	sub_82C57220(ctx, base);
	// 82C57384: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C57388: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C5738C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C57390: 913F0068  stw r9, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82C57394: 890A0001  lbz r8, 1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82C57398: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C5739C: 419A0030  beq cr6, 0x82c573cc
	if ctx.cr[6].eq {
	pc = 0x82C573CC; continue 'dispatch;
	}
	// 82C573A0: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 82C573A4: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C573A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C573AC: 811F0068  lwz r8, 0x68(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C573B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82C573B4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82C573B8: 913F0068  stw r9, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82C573BC: 811F005C  lwz r8, 0x5c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C573C0: 88E80001  lbz r7, 1(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(1 as u32) ) } as u64;
	// 82C573C4: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C573C8: 4198FFDC  blt cr6, 0x82c573a4
	if ctx.cr[6].lt {
	pc = 0x82C573A4; continue 'dispatch;
	}
	// 82C573CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C573D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C573D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C573D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C573DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C573E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C573E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C573E8 size=36
    let mut pc: u32 = 0x82C573E8;
    'dispatch: loop {
        match pc {
            0x82C573E8 => {
    //   block [0x82C573E8..0x82C5740C)
	// 82C573E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C573EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C573F0: 392ACF44  addi r9, r10, -0x30bc
	ctx.r[9].s64 = ctx.r[10].s64 + -12476;
	// 82C573F4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C573F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C573FC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C57400: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C57404: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82C57408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57410 size=44
    let mut pc: u32 = 0x82C57410;
    'dispatch: loop {
        match pc {
            0x82C57410 => {
    //   block [0x82C57410..0x82C5743C)
	// 82C57410: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C57414: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57418: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5741C: 409A0020  bne cr6, 0x82c5743c
	if !ctx.cr[6].eq {
		sub_82C5743C(ctx, base);
		return;
	}
	// 82C57420: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C57424: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C57428: 409A0014  bne cr6, 0x82c5743c
	if !ctx.cr[6].eq {
		sub_82C5743C(ctx, base);
		return;
	}
	// 82C5742C: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C57430: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C57434: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82C57438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5743C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C5743C size=8
    let mut pc: u32 = 0x82C5743C;
    'dispatch: loop {
        match pc {
            0x82C5743C => {
    //   block [0x82C5743C..0x82C57444)
	// 82C5743C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57448 size=16
    let mut pc: u32 = 0x82C57448;
    'dispatch: loop {
        match pc {
            0x82C57448 => {
    //   block [0x82C57448..0x82C57458)
	// 82C57448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5744C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C57450: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C57454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57458 size=144
    let mut pc: u32 = 0x82C57458;
    'dispatch: loop {
        match pc {
            0x82C57458 => {
    //   block [0x82C57458..0x82C574E8)
	// 82C57458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57464: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82C57468: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5746C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C57470: 7D055214  add r8, r5, r10
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 82C57474: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C57478: 40990018  ble cr6, 0x82c57490
	if !ctx.cr[6].gt {
	pc = 0x82C57490; continue 'dispatch;
	}
	// 82C5747C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C57484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5748C: 4E800020  blr
	return;
	// 82C57490: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C57498: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5749C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82C574A0: 7D292A14  add r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[5].u64;
	// 82C574A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82C574A8: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82C574AC: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82C574B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C574B4: 79250020  clrldi r5, r9, 0x20
	ctx.r[5].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C574B8: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C574BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C574C0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C574C4: 4E800421  bctrl
	ctx.lr = 0x82C574C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C574C8: 7C670034  cntlzw r7, r3
	ctx.r[7].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82C574CC: 7CE60034  cntlzw r6, r7
	ctx.r[6].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 82C574D0: 54C5DFFE  rlwinm r5, r6, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 82C574D4: 68A30001  xori r3, r5, 1
	ctx.r[3].u64 = ctx.r[5].u64 ^ 1;
	// 82C574D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C574DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C574E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C574E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C574E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C574E8 size=156
    let mut pc: u32 = 0x82C574E8;
    'dispatch: loop {
        match pc {
            0x82C574E8 => {
    //   block [0x82C574E8..0x82C57584)
	// 82C574E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C574EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C574F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C574F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C574F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C574FC: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82C57500: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C57504: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82C57508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5750C: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 82C57510: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82C57514: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C57518: 40990010  ble cr6, 0x82c57528
	if !ctx.cr[6].gt {
	pc = 0x82C57528; continue 'dispatch;
	}
	// 82C5751C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C57520: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C57524: 48000044  b 0x82c57568
	pc = 0x82C57568; continue 'dispatch;
	// 82C57528: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C57530: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C57538: 7C683214  add r3, r8, r6
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82C5753C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82C57540: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82C57544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C57548: 78660020  clrldi r6, r3, 0x20
	ctx.r[6].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82C5754C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82C57550: 83C40010  lwz r30, 0x10(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C57554: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C57558: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82C5755C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57560: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 82C57564: 4E800421  bctrl
	ctx.lr = 0x82C57568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C57568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5756C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57578: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5757C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57588 size=92
    let mut pc: u32 = 0x82C57588;
    'dispatch: loop {
        match pc {
            0x82C57588 => {
    //   block [0x82C57588..0x82C575E4)
	// 82C57588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5758C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5759C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C575A0: 392BCF44  addi r9, r11, -0x30bc
	ctx.r[9].s64 = ctx.r[11].s64 + -12476;
	// 82C575A4: 895F0010  lbz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C575A8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C575AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C575B0: 419A0018  beq cr6, 0x82c575c8
	if ctx.cr[6].eq {
	pc = 0x82C575C8; continue 'dispatch;
	}
	// 82C575B4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C575B8: 4BBEE1F9  bl 0x828457b0
	ctx.lr = 0x82C575BC;
	sub_828457B0(ctx, base);
	// 82C575BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C575C0: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82C575C4: 48000008  b 0x82c575cc
	pc = 0x82C575CC; continue 'dispatch;
	// 82C575C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C575CC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C575D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C575D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C575D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C575DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C575E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C575E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C575E8 size=116
    let mut pc: u32 = 0x82C575E8;
    'dispatch: loop {
        match pc {
            0x82C575E8 => {
    //   block [0x82C575E8..0x82C5765C)
	// 82C575E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C575EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C575F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C575F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C575F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C575FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57600: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C57604: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C57608: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5760C: 419A0010  beq cr6, 0x82c5761c
	if ctx.cr[6].eq {
	pc = 0x82C5761C; continue 'dispatch;
	}
	// 82C57610: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C57614: 4BBEE19D  bl 0x828457b0
	ctx.lr = 0x82C57618;
	sub_828457B0(ctx, base);
	// 82C57618: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C5761C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C57620: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C57624: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C57628: 419A001C  beq cr6, 0x82c57644
	if ctx.cr[6].eq {
	pc = 0x82C57644; continue 'dispatch;
	}
	// 82C5762C: 4B5C7C2D  bl 0x8221f258
	ctx.lr = 0x82C57630;
	sub_8221F258(ctx, base);
	// 82C57630: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C57634: 419A0010  beq cr6, 0x82c57644
	if ctx.cr[6].eq {
	pc = 0x82C57644; continue 'dispatch;
	}
	// 82C57638: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C5763C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82C57640: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82C57644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5764C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57650: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C57654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57660 size=64
    let mut pc: u32 = 0x82C57660;
    'dispatch: loop {
        match pc {
            0x82C57660 => {
    //   block [0x82C57660..0x82C576A0)
	// 82C57660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57664: 48051DA9  bl 0x82ca940c
	ctx.lr = 0x82C57668;
	sub_82CA93D0(ctx, base);
	// 82C57668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5766C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57670: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C57674: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C57678: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5767C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57680: 419A0010  beq cr6, 0x82c57690
	if ctx.cr[6].eq {
	pc = 0x82C57690; continue 'dispatch;
	}
	// 82C57684: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C57688: 4BBEE129  bl 0x828457b0
	ctx.lr = 0x82C5768C;
	sub_828457B0(ctx, base);
	// 82C5768C: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C57690: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82C57694: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C57698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5769C: 48051DC0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C576A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C576A0 size=96
    let mut pc: u32 = 0x82C576A0;
    'dispatch: loop {
        match pc {
            0x82C576A0 => {
    //   block [0x82C576A0..0x82C57700)
	// 82C576A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C576A4: 48051D69  bl 0x82ca940c
	ctx.lr = 0x82C576A8;
	sub_82CA93D0(ctx, base);
	// 82C576A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C576AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C576B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C576B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C576B8: 392BCF44  addi r9, r11, -0x30bc
	ctx.r[9].s64 = ctx.r[11].s64 + -12476;
	// 82C576BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C576C0: 895F0010  lbz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C576C4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C576C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C576CC: 419A0010  beq cr6, 0x82c576dc
	if ctx.cr[6].eq {
	pc = 0x82C576DC; continue 'dispatch;
	}
	// 82C576D0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C576D4: 4BBEE0DD  bl 0x828457b0
	ctx.lr = 0x82C576D8;
	sub_828457B0(ctx, base);
	// 82C576D8: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 82C576DC: 57AB07FE  clrlwi r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82C576E0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C576E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C576E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C576EC: 419A000C  beq cr6, 0x82c576f8
	if ctx.cr[6].eq {
	pc = 0x82C576F8; continue 'dispatch;
	}
	// 82C576F0: 4BBEE0C1  bl 0x828457b0
	ctx.lr = 0x82C576F4;
	sub_828457B0(ctx, base);
	// 82C576F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C576F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C576FC: 48051D60  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57700 size=112
    let mut pc: u32 = 0x82C57700;
    'dispatch: loop {
        match pc {
            0x82C57700 => {
    //   block [0x82C57700..0x82C57770)
	// 82C57700: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C57704: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82C57708: 5569FF7E  rlwinm r9, r11, 0x1f, 0x1d, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C5770C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C57710: 419A002C  beq cr6, 0x82c5773c
	if ctx.cr[6].eq {
	pc = 0x82C5773C; continue 'dispatch;
	}
	// 82C57714: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82C57718: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C5771C: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57720: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C57724: 9901FFF0  stb r8, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u8 ) };
	// 82C57728: 98E1FFF1  stb r7, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[7].u8 ) };
	// 82C5772C: A0C1FFF0  lhz r6, -0x10(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82C57730: B0CB0000  sth r6, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u16 ) };
	// 82C57734: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82C57738: 4082FFDC  bne 0x82c57714
	if !ctx.cr[0].eq {
	pc = 0x82C57714; continue 'dispatch;
	}
	// 82C5773C: 5549E8FE  srwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C57740: 1008034C  vspltish v0, 8
	for i in 0..8 {
		ctx.v[0].u16[i] = 8 as u16;
	}
	// 82C57744: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C57748: 419A0020  beq cr6, 0x82c57768
	if ctx.cr[6].eq {
	pc = 0x82C57768; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57770 size=40
    let mut pc: u32 = 0x82C57770;
    'dispatch: loop {
        match pc {
            0x82C57770 => {
    //   block [0x82C57770..0x82C57798)
	// 82C57770: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82C57774: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C57778: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5777C: 9921FFF0  stb r9, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u8 ) };
	// 82C57780: 9901FFF1  stb r8, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[8].u8 ) };
	// 82C57784: A0E1FFF0  lhz r7, -0x10(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82C57788: B0EB0000  sth r7, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 82C5778C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82C57790: 4082FFE0  bne 0x82c57770
	if !ctx.cr[0].eq {
	pc = 0x82C57770; continue 'dispatch;
	}
	// 82C57794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57798 size=44
    let mut pc: u32 = 0x82C57798;
    'dispatch: loop {
        match pc {
            0x82C57798 => {
    //   block [0x82C57798..0x82C577C4)
	// 82C57798: 3D606461  lis r11, 0x6461
	ctx.r[11].s64 = 1684078592;
	// 82C5779C: 616A7461  ori r10, r11, 0x7461
	ctx.r[10].u64 = ctx.r[11].u64 | 29793;
	// 82C577A0: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C577A4: 419A0020  beq cr6, 0x82c577c4
	if ctx.cr[6].eq {
		sub_82C577C4(ctx, base);
		return;
	}
	// 82C577A8: 3D60666D  lis r11, 0x666d
	ctx.r[11].s64 = 1718419456;
	// 82C577AC: 616A7420  ori r10, r11, 0x7420
	ctx.r[10].u64 = ctx.r[11].u64 | 29728;
	// 82C577B0: 7D245050  subf r9, r4, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 82C577B4: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82C577B8: 550BDFFE  rlwinm r11, r8, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82C577BC: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 82C577C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C577C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C577C4 size=8
    let mut pc: u32 = 0x82C577C4;
    'dispatch: loop {
        match pc {
            0x82C577C4 => {
    //   block [0x82C577C4..0x82C577CC)
	// 82C577C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C577C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C577D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C577D0 size=80
    let mut pc: u32 = 0x82C577D0;
    'dispatch: loop {
        match pc {
            0x82C577D0 => {
    //   block [0x82C577D0..0x82C57820)
	// 82C577D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C577D4: 48051C39  bl 0x82ca940c
	ctx.lr = 0x82C577D8;
	sub_82CA93D0(ctx, base);
	// 82C577D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C577DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C577E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C577E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C577E8: 394BCF48  addi r10, r11, -0x30b8
	ctx.r[10].s64 = ctx.r[11].s64 + -12472;
	// 82C577EC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82C577F0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C577F4: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 82C577F8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C577FC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C57800: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C57804: 4BFFFBE5  bl 0x82c573e8
	ctx.lr = 0x82C57808;
	sub_82C573E8(ctx, base);
	// 82C57808: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C5780C: 3BBD0014  addi r29, r29, 0x14
	ctx.r[29].s64 = ctx.r[29].s64 + 20;
	// 82C57810: 4080FFF0  bge 0x82c57800
	if !ctx.cr[0].lt {
	pc = 0x82C57800; continue 'dispatch;
	}
	// 82C57814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57818: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5781C: 48051C40  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57820 size=92
    let mut pc: u32 = 0x82C57820;
    'dispatch: loop {
        match pc {
            0x82C57820 => {
    //   block [0x82C57820..0x82C5787C)
	// 82C57820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57824: 48051BE9  bl 0x82ca940c
	ctx.lr = 0x82C57828;
	sub_82CA93D0(ctx, base);
	// 82C57828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5782C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C57830: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C57834: 3BFD0034  addi r31, r29, 0x34
	ctx.r[31].s64 = ctx.r[29].s64 + 52;
	// 82C57838: 394BCF48  addi r10, r11, -0x30b8
	ctx.r[10].s64 = ctx.r[11].s64 + -12472;
	// 82C5783C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82C57840: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C57844: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 82C57848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5784C: 4BFFFD3D  bl 0x82c57588
	ctx.lr = 0x82C57850;
	sub_82C57588(ctx, base);
	// 82C57850: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C57854: 4080FFF0  bge 0x82c57844
	if !ctx.cr[0].lt {
	pc = 0x82C57844; continue 'dispatch;
	}
	// 82C57858: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5785C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C57860: 419A0014  beq cr6, 0x82c57874
	if ctx.cr[6].eq {
	pc = 0x82C57874; continue 'dispatch;
	}
	// 82C57864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57868: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5786C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C57870: 4E800421  bctrl
	ctx.lr = 0x82C57874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C57874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57878: 48051BE4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57880 size=80
    let mut pc: u32 = 0x82C57880;
    'dispatch: loop {
        match pc {
            0x82C57880 => {
    //   block [0x82C57880..0x82C578D0)
	// 82C57880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57888: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5788C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57890: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57898: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C5789C: 4BFFFF85  bl 0x82c57820
	ctx.lr = 0x82C578A0;
	sub_82C57820(ctx, base);
	// 82C578A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C578A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C578A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C578AC: 419A000C  beq cr6, 0x82c578b8
	if ctx.cr[6].eq {
	pc = 0x82C578B8; continue 'dispatch;
	}
	// 82C578B0: 4BBEDF01  bl 0x828457b0
	ctx.lr = 0x82C578B4;
	sub_828457B0(ctx, base);
	// 82C578B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C578B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C578BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C578C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C578C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C578C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C578CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C578D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C578D0 size=72
    let mut pc: u32 = 0x82C578D0;
    'dispatch: loop {
        match pc {
            0x82C578D0 => {
    //   block [0x82C578D0..0x82C57918)
	// 82C578D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C578D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C578D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C578DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C578E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C578E4: 4BFFFEED  bl 0x82c577d0
	ctx.lr = 0x82C578E8;
	sub_82C577D0(ctx, base);
	// 82C578E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C578EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C578F0: 392ACF58  addi r9, r10, -0x30a8
	ctx.r[9].s64 = ctx.r[10].s64 + -12456;
	// 82C578F4: 997F0046  stb r11, 0x46(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(70 as u32), ctx.r[11].u8 ) };
	// 82C578F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C578FC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C57900: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82C57904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C57908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5790C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57918 size=16
    let mut pc: u32 = 0x82C57918;
    'dispatch: loop {
        match pc {
            0x82C57918 => {
    //   block [0x82C57918..0x82C57928)
	// 82C57918: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5791C: 394BCF58  addi r10, r11, -0x30a8
	ctx.r[10].s64 = ctx.r[11].s64 + -12456;
	// 82C57920: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C57924: 4BFFFEFC  b 0x82c57820
	sub_82C57820(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57928 size=20
    let mut pc: u32 = 0x82C57928;
    'dispatch: loop {
        match pc {
            0x82C57928 => {
    //   block [0x82C57928..0x82C5793C)
	// 82C57928: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5792C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C57930: 409A000C  bne cr6, 0x82c5793c
	if !ctx.cr[6].eq {
		sub_82C5793C(ctx, base);
		return;
	}
	// 82C57934: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5793C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C5793C size=8
    let mut pc: u32 = 0x82C5793C;
    'dispatch: loop {
        match pc {
            0x82C5793C => {
    //   block [0x82C5793C..0x82C57944)
	// 82C5793C: A0630036  lhz r3, 0x36(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(54 as u32) ) } as u64;
	// 82C57940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57948 size=20
    let mut pc: u32 = 0x82C57948;
    'dispatch: loop {
        match pc {
            0x82C57948 => {
    //   block [0x82C57948..0x82C5795C)
	// 82C57948: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5794C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C57950: 409A000C  bne cr6, 0x82c5795c
	if !ctx.cr[6].eq {
		sub_82C5795C(ctx, base);
		return;
	}
	// 82C57954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5795C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C5795C size=8
    let mut pc: u32 = 0x82C5795C;
    'dispatch: loop {
        match pc {
            0x82C5795C => {
    //   block [0x82C5795C..0x82C57964)
	// 82C5795C: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C57960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57968 size=92
    let mut pc: u32 = 0x82C57968;
    'dispatch: loop {
        match pc {
            0x82C57968 => {
    //   block [0x82C57968..0x82C579C4)
	// 82C57968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5796C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57978: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5797C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C57980: 409A001C  bne cr6, 0x82c5799c
	if !ctx.cr[6].eq {
	pc = 0x82C5799C; continue 'dispatch;
	}
	// 82C57984: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C5798C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57998: 4E800020  blr
	return;
	// 82C5799C: A1630036  lhz r11, 0x36(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(54 as u32) ) } as u64;
	// 82C579A0: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82C579A4: 557F0C3C  rlwinm r31, r11, 1, 0x10, 0x1e
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82C579A8: 4849BF69  bl 0x830f3910
	ctx.lr = 0x82C579AC;
	sub_830F3910(ctx, base);
	// 82C579AC: 7C63FB96  divwu r3, r3, r31
	ctx.r[3].u32 = ctx.r[3].u32 / ctx.r[31].u32;
	// 82C579B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C579B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C579B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C579BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C579C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C579C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C579C8 size=240
    let mut pc: u32 = 0x82C579C8;
    'dispatch: loop {
        match pc {
            0x82C579C8 => {
    //   block [0x82C579C8..0x82C57AB8)
	// 82C579C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C579CC: 48051A35  bl 0x82ca9400
	ctx.lr = 0x82C579D0;
	sub_82CA93D0(ctx, base);
	// 82C579D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C579D4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82C579D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C579DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C579E0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C579E4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C579E8: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C579EC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C579F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C579F4: 419A00B8  beq cr6, 0x82c57aac
	if ctx.cr[6].eq {
	pc = 0x82C57AAC; continue 'dispatch;
	}
	// 82C579F8: 3BFD0020  addi r31, r29, 0x20
	ctx.r[31].s64 = ctx.r[29].s64 + 32;
	// 82C579FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57A00: 4849BF11  bl 0x830f3910
	ctx.lr = 0x82C57A04;
	sub_830F3910(ctx, base);
	// 82C57A04: 80BD0048  lwz r5, 0x48(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C57A08: 7FC51851  subf. r30, r5, r3
	ctx.r[30].s64 = ctx.r[3].s64 - ctx.r[5].s64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C57A0C: 418200A0  beq 0x82c57aac
	if ctx.cr[0].eq {
	pc = 0x82C57AAC; continue 'dispatch;
	}
	// 82C57A10: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57A14: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C57A18: 40980038  bge cr6, 0x82c57a50
	if !ctx.cr[6].lt {
	pc = 0x82C57A50; continue 'dispatch;
	}
	// 82C57A1C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C57A20: 409A000C  bne cr6, 0x82c57a2c
	if !ctx.cr[6].eq {
	pc = 0x82C57A2C; continue 'dispatch;
	}
	// 82C57A24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57A28: 48000014  b 0x82c57a3c
	pc = 0x82C57A3C; continue 'dispatch;
	// 82C57A2C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82C57A30: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82C57A34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57A38: 4BFFFA21  bl 0x82c57458
	ctx.lr = 0x82C57A3C;
	sub_82C57458(ctx, base);
	// 82C57A3C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C57A40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57A44: 419A0068  beq cr6, 0x82c57aac
	if ctx.cr[6].eq {
	pc = 0x82C57AAC; continue 'dispatch;
	}
	// 82C57A48: 93DA0000  stw r30, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C57A4C: 48000034  b 0x82c57a80
	pc = 0x82C57A80; continue 'dispatch;
	// 82C57A50: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C57A54: 409A000C  bne cr6, 0x82c57a60
	if !ctx.cr[6].eq {
	pc = 0x82C57A60; continue 'dispatch;
	}
	// 82C57A58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57A5C: 48000014  b 0x82c57a70
	pc = 0x82C57A70; continue 'dispatch;
	// 82C57A60: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82C57A64: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82C57A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57A6C: 4BFFF9ED  bl 0x82c57458
	ctx.lr = 0x82C57A70;
	sub_82C57458(ctx, base);
	// 82C57A70: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C57A74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57A78: 419A0034  beq cr6, 0x82c57aac
	if ctx.cr[6].eq {
	pc = 0x82C57AAC; continue 'dispatch;
	}
	// 82C57A7C: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C57A80: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C57A84: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57A88: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C57A8C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C57A90: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C57A94: 911D0048  stw r8, 0x48(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 82C57A98: 419A0014  beq cr6, 0x82c57aac
	if ctx.cr[6].eq {
	pc = 0x82C57AAC; continue 'dispatch;
	}
	// 82C57A9C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57AA0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C57AA4: 5564F87E  srwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82C57AA8: 4BFFFC59  bl 0x82c57700
	ctx.lr = 0x82C57AAC;
	sub_82C57700(ctx, base);
	// 82C57AAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C57AB0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C57AB4: 4805199C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57AB8 size=104
    let mut pc: u32 = 0x82C57AB8;
    'dispatch: loop {
        match pc {
            0x82C57AB8 => {
    //   block [0x82C57AB8..0x82C57B20)
	// 82C57AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C57AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57ACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57AD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57AD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C57AD8: 409A000C  bne cr6, 0x82c57ae4
	if !ctx.cr[6].eq {
	pc = 0x82C57AE4; continue 'dispatch;
	}
	// 82C57ADC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57AE0: 48000028  b 0x82c57b08
	pc = 0x82C57B08; continue 'dispatch;
	// 82C57AE4: A17F0036  lhz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82C57AE8: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82C57AEC: 7D4B21D6  mullw r10, r11, r4
	ctx.r[10].s64 = (ctx.r[11].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82C57AF0: 555E083C  slwi r30, r10, 1
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82C57AF4: 4849BE1D  bl 0x830f3910
	ctx.lr = 0x82C57AF8;
	sub_830F3910(ctx, base);
	// 82C57AF8: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C57AFC: 4098FFE0  bge cr6, 0x82c57adc
	if !ctx.cr[6].lt {
	pc = 0x82C57ADC; continue 'dispatch;
	}
	// 82C57B00: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82C57B04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C57B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57B14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C57B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57B20 size=460
    let mut pc: u32 = 0x82C57B20;
    'dispatch: loop {
        match pc {
            0x82C57B20 => {
    //   block [0x82C57B20..0x82C57CEC)
	// 82C57B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57B24: 480518E5  bl 0x82ca9408
	ctx.lr = 0x82C57B28;
	sub_82CA93D0(ctx, base);
	// 82C57B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57B30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57B34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C57B38: 419A017C  beq cr6, 0x82c57cb4
	if ctx.cr[6].eq {
	pc = 0x82C57CB4; continue 'dispatch;
	}
	// 82C57B3C: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82C57B40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C57B44: 4849BDCD  bl 0x830f3910
	ctx.lr = 0x82C57B48;
	sub_830F3910(ctx, base);
	// 82C57B48: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82C57B4C: 2B07000E  cmplwi cr6, r7, 0xe
	ctx.cr[6].compare_u32(ctx.r[7].u32, 14 as u32, &mut ctx.xer);
	// 82C57B50: 41980164  blt cr6, 0x82c57cb4
	if ctx.cr[6].lt {
	pc = 0x82C57CB4; continue 'dispatch;
	}
	// 82C57B54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C57B58: 3947FFEE  addi r10, r7, -0x12
	ctx.r[10].s64 = ctx.r[7].s64 + -18;
	// 82C57B5C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57B60: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C57B64: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82C57B68: 7D680034  cntlzw r8, r11
	ctx.r[8].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82C57B6C: 5526DFFE  rlwinm r6, r9, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82C57B70: 5505DFFE  rlwinm r5, r8, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82C57B74: 98DF0046  stb r6, 0x46(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(70 as u32), ctx.r[6].u8 ) };
	// 82C57B78: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82C57B7C: 68BD0001  xori r29, r5, 1
	ctx.r[29].u64 = ctx.r[5].u64 ^ 1;
	// 82C57B80: 409A000C  bne cr6, 0x82c57b8c
	if !ctx.cr[6].eq {
	pc = 0x82C57B8C; continue 'dispatch;
	}
	// 82C57B84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C57B88: 48000014  b 0x82c57b9c
	pc = 0x82C57B9C; continue 'dispatch;
	// 82C57B8C: 38DF0034  addi r6, r31, 0x34
	ctx.r[6].s64 = ctx.r[31].s64 + 52;
	// 82C57B90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C57B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C57B98: 4BFFF8C1  bl 0x82c57458
	ctx.lr = 0x82C57B9C;
	sub_82C57458(ctx, base);
	// 82C57B9C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C57BA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57BA4: 419A0110  beq cr6, 0x82c57cb4
	if ctx.cr[6].eq {
	pc = 0x82C57CB4; continue 'dispatch;
	}
	// 82C57BA8: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82C57BAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57BB0: 419A009C  beq cr6, 0x82c57c4c
	if ctx.cr[6].eq {
	pc = 0x82C57C4C; continue 'dispatch;
	}
	// 82C57BB4: 895F0035  lbz r10, 0x35(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82C57BB8: 893F0034  lbz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C57BBC: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82C57BC0: 99210051  stb r9, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[9].u8 ) };
	// 82C57BC4: A1010050  lhz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C57BC8: B11F0034  sth r8, 0x34(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[8].u16 ) };
	// 82C57BCC: 88DF0036  lbz r6, 0x36(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82C57BD0: 88FF0037  lbz r7, 0x37(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(55 as u32) ) } as u64;
	// 82C57BD4: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82C57BD8: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82C57BDC: A0A10050  lhz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C57BE0: B0BF0036  sth r5, 0x36(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(54 as u32), ctx.r[5].u16 ) };
	// 82C57BE4: 887F0038  lbz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C57BE8: 895F003B  lbz r10, 0x3b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(59 as u32) ) } as u64;
	// 82C57BEC: 893F003A  lbz r9, 0x3a(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(58 as u32) ) } as u64;
	// 82C57BF0: 889F0039  lbz r4, 0x39(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(57 as u32) ) } as u64;
	// 82C57BF4: 98810052  stb r4, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[4].u8 ) };
	// 82C57BF8: 99210051  stb r9, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[9].u8 ) };
	// 82C57BFC: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82C57C00: 98610053  stb r3, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[3].u8 ) };
	// 82C57C04: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C57C08: 911F0038  stw r8, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[8].u32 ) };
	// 82C57C0C: 88DF003C  lbz r6, 0x3c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C57C10: 88BF003F  lbz r5, 0x3f(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(63 as u32) ) } as u64;
	// 82C57C14: 889F003E  lbz r4, 0x3e(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(62 as u32) ) } as u64;
	// 82C57C18: 88FF003D  lbz r7, 0x3d(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(61 as u32) ) } as u64;
	// 82C57C1C: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 82C57C20: 98810051  stb r4, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[4].u8 ) };
	// 82C57C24: 98A10050  stb r5, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u8 ) };
	// 82C57C28: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82C57C2C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C57C30: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82C57C34: 893F0040  lbz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C57C38: 895F0041  lbz r10, 0x41(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 82C57C3C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82C57C40: 99210051  stb r9, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[9].u8 ) };
	// 82C57C44: A1010050  lhz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C57C48: B11F0040  sth r8, 0x40(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[8].u16 ) };
	// 82C57C4C: 895F0046  lbz r10, 0x46(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(70 as u32) ) } as u64;
	// 82C57C50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C57C54: 419A0040  beq cr6, 0x82c57c94
	if ctx.cr[6].eq {
	pc = 0x82C57C94; continue 'dispatch;
	}
	// 82C57C58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57C5C: 419A0040  beq cr6, 0x82c57c9c
	if ctx.cr[6].eq {
	pc = 0x82C57C9C; continue 'dispatch;
	}
	// 82C57C60: 897F0043  lbz r11, 0x43(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(67 as u32) ) } as u64;
	// 82C57C64: 895F0042  lbz r10, 0x42(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(66 as u32) ) } as u64;
	// 82C57C68: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82C57C6C: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 82C57C70: A1210050  lhz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C57C74: B13F0042  sth r9, 0x42(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(66 as u32), ctx.r[9].u16 ) };
	// 82C57C78: 891F0045  lbz r8, 0x45(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(69 as u32) ) } as u64;
	// 82C57C7C: 88FF0044  lbz r7, 0x44(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C57C80: 99010050  stb r8, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u8 ) };
	// 82C57C84: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 82C57C88: A0C10050  lhz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C57C8C: B0DF0044  sth r6, 0x44(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[6].u16 ) };
	// 82C57C90: 4800000C  b 0x82c57c9c
	pc = 0x82C57C9C; continue 'dispatch;
	// 82C57C94: B39F0042  sth r28, 0x42(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(66 as u32), ctx.r[28].u16 ) };
	// 82C57C98: B39F0044  sth r28, 0x44(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[28].u16 ) };
	// 82C57C9C: 897F0046  lbz r11, 0x46(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(70 as u32) ) } as u64;
	// 82C57CA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57CA4: 419A001C  beq cr6, 0x82c57cc0
	if ctx.cr[6].eq {
	pc = 0x82C57CC0; continue 'dispatch;
	}
	// 82C57CA8: A17F0042  lhz r11, 0x42(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(66 as u32) ) } as u64;
	// 82C57CAC: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82C57CB0: 419A0030  beq cr6, 0x82c57ce0
	if ctx.cr[6].eq {
	pc = 0x82C57CE0; continue 'dispatch;
	}
	// 82C57CB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C57CBC: 4805179C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C57CC0: A17F0036  lhz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82C57CC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C57CC8: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C57CCC: 813F003C  lwz r9, 0x3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C57CD0: 7D0B51D6  mullw r8, r11, r10
	ctx.r[8].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82C57CD4: 7CE94396  divwu r7, r9, r8
	ctx.r[7].u32 = ctx.r[9].u32 / ctx.r[8].u32;
	// 82C57CD8: 2B070002  cmplwi cr6, r7, 2
	ctx.cr[6].compare_u32(ctx.r[7].u32, 2 as u32, &mut ctx.xer);
	// 82C57CDC: 409A0008  bne cr6, 0x82c57ce4
	if !ctx.cr[6].eq {
	pc = 0x82C57CE4; continue 'dispatch;
	}
	// 82C57CE0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C57CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C57CE8: 48051770  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57CF0 size=92
    let mut pc: u32 = 0x82C57CF0;
    'dispatch: loop {
        match pc {
            0x82C57CF0 => {
    //   block [0x82C57CF0..0x82C57D4C)
	// 82C57CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C57CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57D04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57D08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C57D0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C57D10: 394BCF58  addi r10, r11, -0x30a8
	ctx.r[10].s64 = ctx.r[11].s64 + -12456;
	// 82C57D14: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C57D18: 4BFFFB09  bl 0x82c57820
	ctx.lr = 0x82C57D1C;
	sub_82C57820(ctx, base);
	// 82C57D1C: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C57D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57D24: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C57D28: 419A000C  beq cr6, 0x82c57d34
	if ctx.cr[6].eq {
	pc = 0x82C57D34; continue 'dispatch;
	}
	// 82C57D2C: 4BBEDA85  bl 0x828457b0
	ctx.lr = 0x82C57D30;
	sub_828457B0(ctx, base);
	// 82C57D30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57D34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57D40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C57D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57D50 size=12
    let mut pc: u32 = 0x82C57D50;
    'dispatch: loop {
        match pc {
            0x82C57D50 => {
    //   block [0x82C57D50..0x82C57D5C)
	// 82C57D50: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C57D54: 806B5164  lwz r3, 0x5164(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C57D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57D60 size=192
    let mut pc: u32 = 0x82C57D60;
    'dispatch: loop {
        match pc {
            0x82C57D60 => {
    //   block [0x82C57D60..0x82C57E20)
	// 82C57D60: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82C57D64: 39800030  li r12, 0x30
	ctx.r[12].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57E20 size=104
    let mut pc: u32 = 0x82C57E20;
    'dispatch: loop {
        match pc {
            0x82C57E20 => {
    //   block [0x82C57E20..0x82C57E88)
	// 82C57E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57E28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C57E2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57E30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57E34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C57E38: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82C57E3C: 889E004C  lbz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C57E40: 897F004C  lbz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C57E44: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82C57E48: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C57E4C: 419A000C  beq cr6, 0x82c57e58
	if ctx.cr[6].eq {
	pc = 0x82C57E58; continue 'dispatch;
	}
	// 82C57E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57E54: 48003D95  bl 0x82c5bbe8
	ctx.lr = 0x82C57E58;
	sub_82C5BBE8(ctx, base);
	// 82C57E58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C57E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C57E60: 48003D31  bl 0x82c5bb90
	ctx.lr = 0x82C57E64;
	sub_82C5BB90(ctx, base);
	// 82C57E64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C57E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57E6C: 48003D25  bl 0x82c5bb90
	ctx.lr = 0x82C57E70;
	sub_82C5BB90(ctx, base);
	// 82C57E70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57E7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C57E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57E88 size=76
    let mut pc: u32 = 0x82C57E88;
    'dispatch: loop {
        match pc {
            0x82C57E88 => {
    //   block [0x82C57E88..0x82C57ED4)
	// 82C57E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57E90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C57E94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57E98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57E9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C57EA0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C57EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C57EA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C57EAC: 48004025  bl 0x82c5bed0
	ctx.lr = 0x82C57EB0;
	sub_82C5BED0(ctx, base);
	// 82C57EB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C57EB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C57EB8: 48004019  bl 0x82c5bed0
	ctx.lr = 0x82C57EBC;
	sub_82C5BED0(ctx, base);
	// 82C57EBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C57EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C57EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C57EC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C57ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C57ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57ED8 size=12
    let mut pc: u32 = 0x82C57ED8;
    'dispatch: loop {
        match pc {
            0x82C57ED8 => {
    //   block [0x82C57ED8..0x82C57EE4)
	// 82C57ED8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C57EDC: 54A4063E  clrlwi r4, r5, 0x18
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82C57EE0: 48004780  b 0x82c5c660
	sub_82C5C660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57EE8 size=8
    let mut pc: u32 = 0x82C57EE8;
    'dispatch: loop {
        match pc {
            0x82C57EE8 => {
    //   block [0x82C57EE8..0x82C57EF0)
	// 82C57EE8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C57EEC: 48004F84  b 0x82c5ce70
	sub_82C5CE70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57EF0 size=8
    let mut pc: u32 = 0x82C57EF0;
    'dispatch: loop {
        match pc {
            0x82C57EF0 => {
    //   block [0x82C57EF0..0x82C57EF8)
	// 82C57EF0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C57EF4: 48004B9C  b 0x82c5ca90
	sub_82C5CA90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57EF8 size=8
    let mut pc: u32 = 0x82C57EF8;
    'dispatch: loop {
        match pc {
            0x82C57EF8 => {
    //   block [0x82C57EF8..0x82C57F00)
	// 82C57EF8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C57EFC: 48004A24  b 0x82c5c920
	sub_82C5C920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57F00 size=12
    let mut pc: u32 = 0x82C57F00;
    'dispatch: loop {
        match pc {
            0x82C57F00 => {
    //   block [0x82C57F00..0x82C57F0C)
	// 82C57F00: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 82C57F04: 6063BB80  ori r3, r3, 0xbb80
	ctx.r[3].u64 = ctx.r[3].u64 | 48000;
	// 82C57F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57F10 size=12
    let mut pc: u32 = 0x82C57F10;
    'dispatch: loop {
        match pc {
            0x82C57F10 => {
    //   block [0x82C57F10..0x82C57F1C)
	// 82C57F10: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C57F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57F18: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57F1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57F1C size=12
    let mut pc: u32 = 0x82C57F1C;
    'dispatch: loop {
        match pc {
            0x82C57F1C => {
    //   block [0x82C57F1C..0x82C57F28)
	// 82C57F1C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57F20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57F24: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57F28 size=12
    let mut pc: u32 = 0x82C57F28;
    'dispatch: loop {
        match pc {
            0x82C57F28 => {
    //   block [0x82C57F28..0x82C57F34)
	// 82C57F28: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57F30: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57F34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57F34 size=36
    let mut pc: u32 = 0x82C57F34;
    'dispatch: loop {
        match pc {
            0x82C57F34 => {
    //   block [0x82C57F34..0x82C57F58)
	// 82C57F34: 90640008  stw r3, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C57F38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57F3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C57F40: 409A0018  bne cr6, 0x82c57f58
	if !ctx.cr[6].eq {
		sub_82C57F58(ctx, base);
		return;
	}
	// 82C57F44: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C57F48: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C57F4C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C57F50: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C57F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C57F58 size=32
    let mut pc: u32 = 0x82C57F58;
    'dispatch: loop {
        match pc {
            0x82C57F58 => {
    //   block [0x82C57F58..0x82C57F78)
	// 82C57F58: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57F5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C57F60: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C57F64: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C57F68: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57F6C: 90890000  stw r4, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C57F70: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C57F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C57F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C57F78 size=148
    let mut pc: u32 = 0x82C57F78;
    'dispatch: loop {
        match pc {
            0x82C57F78 => {
    //   block [0x82C57F78..0x82C5800C)
	// 82C57F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C57F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C57F80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C57F84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C57F88: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57F8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C57F90: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 82C57F94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C57F98: 4099000C  ble cr6, 0x82c57fa4
	if !ctx.cr[6].gt {
	pc = 0x82C57FA4; continue 'dispatch;
	}
	// 82C57F9C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82C57FA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C57FA4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57FA8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C57FAC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57FB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C57FB4: 4E800421  bctrl
	ctx.lr = 0x82C57FB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C57FB8: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57FBC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C57FC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C57FC4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C57FC8: 40990030  ble cr6, 0x82c57ff8
	if !ctx.cr[6].gt {
	pc = 0x82C57FF8; continue 'dispatch;
	}
	// 82C57FCC: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 82C57FD0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C57FD4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82C57FD8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C57FDC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57FE0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82C57FE4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82C57FE8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82C57FEC: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C57FF0: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C57FF4: 4198FFDC  blt cr6, 0x82c57fd0
	if ctx.cr[6].lt {
	pc = 0x82C57FD0; continue 'dispatch;
	}
	// 82C57FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C57FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C58000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C58004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C58008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C58010 size=72
    let mut pc: u32 = 0x82C58010;
    'dispatch: loop {
        match pc {
            0x82C58010 => {
    //   block [0x82C58010..0x82C58058)
	// 82C58010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C58014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C58018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5801C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C58024: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C58028: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82C5802C: 392BCF84  addi r9, r11, -0x307c
	ctx.r[9].s64 = ctx.r[11].s64 + -12412;
	// 82C58030: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C58034: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C58038: 419A000C  beq cr6, 0x82c58044
	if ctx.cr[6].eq {
	pc = 0x82C58044; continue 'dispatch;
	}
	// 82C5803C: 4BBED775  bl 0x828457b0
	ctx.lr = 0x82C58040;
	sub_828457B0(ctx, base);
	// 82C58040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C58044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C58048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5804C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C58050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C58054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C58058 size=24
    let mut pc: u32 = 0x82C58058;
    'dispatch: loop {
        match pc {
            0x82C58058 => {
    //   block [0x82C58058..0x82C58070)
	// 82C58058: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5805C: 814B0150  lwz r10, 0x150(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C58060: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C58064: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82C58068: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C5806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C58070 size=16
    let mut pc: u32 = 0x82C58070;
    'dispatch: loop {
        match pc {
            0x82C58070 => {
    //   block [0x82C58070..0x82C58080)
	// 82C58070: 2B060006  cmplwi cr6, r6, 6
	ctx.cr[6].compare_u32(ctx.r[6].u32, 6 as u32, &mut ctx.xer);
	// 82C58074: 4098000C  bge cr6, 0x82c58080
	if !ctx.cr[6].lt {
		sub_82C58080(ctx, base);
		return;
	}
	// 82C58078: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C5807C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C58080 size=96
    let mut pc: u32 = 0x82C58080;
    'dispatch: loop {
        match pc {
            0x82C58080 => {
    //   block [0x82C58080..0x82C580E0)
	// 82C58080: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C58084: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C58088: 409A009C  bne cr6, 0x82c58124
	if !ctx.cr[6].eq {
		sub_82C58124(ctx, base);
		return;
	}
	// 82C5808C: 90C70000  stw r6, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82C58090: 8164027C  lwz r11, 0x27c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(636 as u32) ) } as u64;
	// 82C58094: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82C58098: 409A0048  bne cr6, 0x82c580e0
	if !ctx.cr[6].eq {
		sub_82C580E0(ctx, base);
		return;
	}
	// 82C5809C: 2B060006  cmplwi cr6, r6, 6
	ctx.cr[6].compare_u32(ctx.r[6].u32, 6 as u32, &mut ctx.xer);
	// 82C580A0: 409AFFD8  bne cr6, 0x82c58078
	if !ctx.cr[6].eq {
		sub_82C58070(ctx, base);
		return;
	}
	// 82C580A4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C580A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C580AC: 39650004  addi r11, r5, 4
	ctx.r[11].s64 = ctx.r[5].s64 + 4;
	// 82C580B0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82C580B4: C0080C18  lfs f0, 0xc18(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C580B8: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82C580BC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C580C0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82C580C4: 994BFFFC  stb r10, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u8 ) };
	// 82C580C8: 990BFFFD  stb r8, -3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-3 as u32), ctx.r[8].u8 ) };
	// 82C580CC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82C580D0: 2B090006  cmplwi cr6, r9, 6
	ctx.cr[6].compare_u32(ctx.r[9].u32, 6 as u32, &mut ctx.xer);
	// 82C580D4: 4198FFE4  blt cr6, 0x82c580b8
	if ctx.cr[6].lt {
	pc = 0x82C580B8; continue 'dispatch;
	}
	// 82C580D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C580DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C580E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C580E0 size=68
    let mut pc: u32 = 0x82C580E0;
    'dispatch: loop {
        match pc {
            0x82C580E0 => {
    //   block [0x82C580E0..0x82C58124)
	// 82C580E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C580E4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C580E8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82C580EC: 419A0230  beq cr6, 0x82c5831c
	if ctx.cr[6].eq {
		sub_82C58288(ctx, base);
		return;
	}
	// 82C580F0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C580F4: 39650004  addi r11, r5, 4
	ctx.r[11].s64 = ctx.r[5].s64 + 4;
	// 82C580F8: C0080C14  lfs f0, 0xc14(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C580FC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82C58100: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C58104: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82C58108: 994BFFFD  stb r10, -3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-3 as u32), ctx.r[10].u8 ) };
	// 82C5810C: 990BFFFC  stb r8, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u8 ) };
	// 82C58110: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82C58114: 7F093040  cmplw cr6, r9, r6
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C58118: 4198FFE4  blt cr6, 0x82c580fc
	if ctx.cr[6].lt {
	pc = 0x82C580FC; continue 'dispatch;
	}
	// 82C5811C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C58120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C58124 size=24
    let mut pc: u32 = 0x82C58124;
    'dispatch: loop {
        match pc {
            0x82C58124 => {
    //   block [0x82C58124..0x82C5813C)
	// 82C58124: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C58128: 409A0014  bne cr6, 0x82c5813c
	if !ctx.cr[6].eq {
		sub_82C5813C(ctx, base);
		return;
	}
	// 82C5812C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C58130: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C58134: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C58138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5813C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C5813C size=96
    let mut pc: u32 = 0x82C5813C;
    'dispatch: loop {
        match pc {
            0x82C5813C => {
    //   block [0x82C5813C..0x82C5819C)
	// 82C5813C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82C58140: 4198005C  blt cr6, 0x82c5819c
	if ctx.cr[6].lt {
		sub_82C5819C(ctx, base);
		return;
	}
	// 82C58144: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82C58148: 41990054  bgt cr6, 0x82c5819c
	if ctx.cr[6].gt {
		sub_82C5819C(ctx, base);
		return;
	}
	// 82C5814C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82C58150: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C58154: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C58158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C5815C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C58160: 3929FFFB  addi r9, r9, -5
	ctx.r[9].s64 = ctx.r[9].s64 + -5;
	// 82C58164: C0080C14  lfs f0, 0xc14(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58168: 39650004  addi r11, r5, 4
	ctx.r[11].s64 = ctx.r[5].s64 + 4;
	// 82C5816C: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82C58170: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82C58174: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C58178: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C5817C: 992BFFFD  stb r9, -3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-3 as u32), ctx.r[9].u8 ) };
	// 82C58180: 990BFFFC  stb r8, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u8 ) };
	// 82C58184: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82C58188: 80C70000  lwz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5818C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C58190: 4198FFE0  blt cr6, 0x82c58170
	if ctx.cr[6].lt {
	pc = 0x82C58170; continue 'dispatch;
	}
	// 82C58194: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C58198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5819C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C5819C size=184
    let mut pc: u32 = 0x82C5819C;
    'dispatch: loop {
        match pc {
            0x82C5819C => {
    //   block [0x82C5819C..0x82C58254)
	// 82C5819C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C581A0: 409A00B4  bne cr6, 0x82c58254
	if !ctx.cr[6].eq {
		sub_82C58254(ctx, base);
		return;
	}
	// 82C581A4: 8164027C  lwz r11, 0x27c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(636 as u32) ) } as u64;
	// 82C581A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82C581AC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82C581B0: 7D495810  subfc r10, r9, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[9].u32;
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82C581B4: 99250010  stb r9, 0x10(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 82C581B8: 3C608200  lis r3, -0x7e00
	ctx.r[3].s64 = -2113929216;
	// 82C581BC: 99250018  stb r9, 0x18(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[9].u8 ) };
	// 82C581C0: 7D0A5110  subfe r8, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C581C4: 7D645810  subfc r11, r4, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[4].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82C581C8: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82C581CC: 7CCB5910  subfe r6, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[6].u32 = res;
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C581D0: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82C581D4: C0030C14  lfs f0, 0xc14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C581D8: 39060001  addi r8, r6, 1
	ctx.r[8].s64 = ctx.r[6].s64 + 1;
	// 82C581DC: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C581E0: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82C581E4: D005000C  stfs f0, 0xc(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C581E8: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82C581EC: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C581F0: 5526DFFE  rlwinm r6, r9, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82C581F4: D005001C  stfs f0, 0x1c(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C581F8: 7D030034  cntlzw r3, r8
	ctx.r[3].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82C581FC: D0050024  stfs f0, 0x24(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C58200: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82C58204: D005002C  stfs f0, 0x2c(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82C58208: 5469DFFE  rlwinm r9, r3, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000001Fu64;
	// 82C5820C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82C58210: 99650001  stb r11, 1(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82C58214: 69280001  xori r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 ^ 1;
	// 82C58218: 99650011  stb r11, 0x11(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82C5821C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C58220: 99450009  stb r10, 9(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(9 as u32), ctx.r[10].u8 ) };
	// 82C58224: 68C90001  xori r9, r6, 1
	ctx.r[9].u64 = ctx.r[6].u64 ^ 1;
	// 82C58228: 99450019  stb r10, 0x19(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82C5822C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82C58230: 98650000  stb r3, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 82C58234: 99250008  stb r9, 8(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
	// 82C58238: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C5823C: 99650020  stb r11, 0x20(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82C58240: 99650021  stb r11, 0x21(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82C58244: 99050028  stb r8, 0x28(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[8].u8 ) };
	// 82C58248: 99450029  stb r10, 0x29(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 82C5824C: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C58250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C58254 size=52
    let mut pc: u32 = 0x82C58254;
    'dispatch: loop {
        match pc {
            0x82C58254 => {
    //   block [0x82C58254..0x82C58288)
	// 82C58254: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82C58258: 419A000C  beq cr6, 0x82c58264
	if ctx.cr[6].eq {
	pc = 0x82C58264; continue 'dispatch;
	}
	// 82C5825C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82C58260: 409AFE18  bne cr6, 0x82c58078
	if !ctx.cr[6].eq {
		sub_82C58070(ctx, base);
		return;
	}
	// 82C58264: 8124027C  lwz r9, 0x27c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(636 as u32) ) } as u64;
	// 82C58268: 2B090004  cmplwi cr6, r9, 4
	ctx.cr[6].compare_u32(ctx.r[9].u32, 4 as u32, &mut ctx.xer);
	// 82C5826C: 4199FEC0  bgt cr6, 0x82c5812c
	if ctx.cr[6].gt {
		sub_82C58124(ctx, base);
		return;
	}
	// 82C58270: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C58274: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82C58278: C1AA0C14  lfs f13, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C5827C: 409A000C  bne cr6, 0x82c58288
	if !ctx.cr[6].eq {
		sub_82C58288(ctx, base);
		return;
	}
	// 82C58280: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82C58284: 4800000C  b 0x82c58290
	sub_82C58288(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C58288 size=156
    let mut pc: u32 = 0x82C58288;
    'dispatch: loop {
        match pc {
            0x82C58288 => {
    //   block [0x82C58288..0x82C58324)
	// 82C58288: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5828C: C00BCFB4  lfs f0, -0x304c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12364 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58290: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82C58294: D1A50004  stfs f13, 4(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C58298: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82C5829C: D1A5000C  stfs f13, 0xc(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C582A0: 7C6B4810  subfc r3, r11, r9
	ctx.xer.ca = ctx.r[9].u32 >= ctx.r[11].u32;
	ctx.r[3].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82C582A4: D0050014  stfs f0, 0x14(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C582A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C582AC: 7D031910  subfe r8, r3, r3
	let x = (!ctx.r[3].u32);
	let y = ctx.r[3].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C582B0: 7D664810  subfc r11, r6, r9
	ctx.xer.ca = ctx.r[9].u32 >= ctx.r[6].u32;
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 82C582B4: 99450000  stb r10, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82C582B8: 39280001  addi r9, r8, 1
	ctx.r[9].s64 = ctx.r[8].s64 + 1;
	// 82C582BC: 99450001  stb r10, 1(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82C582C0: 7D0B5910  subfe r8, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C582C4: 552A063E  clrlwi r10, r9, 0x18
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82C582C8: 39280001  addi r9, r8, 1
	ctx.r[9].s64 = ctx.r[8].s64 + 1;
	// 82C582CC: 7D480034  cntlzw r8, r10
	ctx.r[8].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82C582D0: 5526063E  clrlwi r6, r9, 0x18
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82C582D4: 5503DFFE  rlwinm r3, r8, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82C582D8: 21460000  subfic r10, r6, 0
	ctx.xer.ca = ctx.r[6].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[6].s64;
	// 82C582DC: 68680001  xori r8, r3, 1
	ctx.r[8].u64 = ctx.r[3].u64 ^ 1;
	// 82C582E0: 7D2A5110  subfe r9, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C582E4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82C582E8: 99050008  stb r8, 8(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 82C582EC: 552307BC  rlwinm r3, r9, 0, 0x1e, 0x1e
	ctx.r[3].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82C582F0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C582F4: 99650011  stb r11, 0x11(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82C582F8: 98650010  stb r3, 0x10(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[3].u8 ) };
	// 82C582FC: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82C58300: 98C50009  stb r6, 9(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(9 as u32), ctx.r[6].u8 ) };
	// 82C58304: 8144027C  lwz r10, 0x27c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(636 as u32) ) } as u64;
	// 82C58308: 390A00FF  addi r8, r10, 0xff
	ctx.r[8].s64 = ctx.r[10].s64 + 255;
	// 82C5830C: D005001C  stfs f0, 0x1c(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82C58310: 99250019  stb r9, 0x19(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(25 as u32), ctx.r[9].u8 ) };
	// 82C58314: 99050018  stb r8, 0x18(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[8].u8 ) };
	// 82C58318: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5831C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C58320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C58328 size=220
    let mut pc: u32 = 0x82C58328;
    'dispatch: loop {
        match pc {
            0x82C58328 => {
    //   block [0x82C58328..0x82C58404)
	// 82C58328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5832C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C58330: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58334: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C58338: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C5833C: 409A00B4  bne cr6, 0x82c583f0
	if !ctx.cr[6].eq {
	pc = 0x82C583F0; continue 'dispatch;
	}
	// 82C58340: 810401F4  lwz r8, 0x1f4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C58344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C58348: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C5834C: 81640150  lwz r11, 0x150(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C58350: 99210070  stb r9, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u8 ) };
	// 82C58354: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82C58358: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82C5835C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82C58360: C0080000  lfs f0, 0(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58364: 99410078  stb r10, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u8 ) };
	// 82C58368: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82C5836C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82C58370: C1A80004  lfs f13, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C58374: 98E10080  stb r7, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[7].u8 ) };
	// 82C58378: D1A1007C  stfs f13, 0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82C5837C: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82C58380: C1880008  lfs f12, 8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C58384: 98C10088  stb r6, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[6].u8 ) };
	// 82C58388: D1810084  stfs f12, 0x84(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82C5838C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82C58390: C168000C  lfs f11, 0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C58394: 98A10090  stb r5, 0x90(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[5].u8 ) };
	// 82C58398: D161008C  stfs f11, 0x8c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82C5839C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82C583A0: C1480010  lfs f10, 0x10(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82C583A4: 98810098  stb r4, 0x98(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[4].u8 ) };
	// 82C583A8: D1410094  stfs f10, 0x94(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82C583AC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C583B0: C1280014  lfs f9, 0x14(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82C583B4: 98610058  stb r3, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u8 ) };
	// 82C583B8: D121009C  stfs f9, 0x9c(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82C583BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82C583C0: 99210060  stb r9, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u8 ) };
	// 82C583C4: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 82C583C8: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82C583CC: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82C583D0: 888B0014  lbz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C583D4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C583D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C583DC: 419A000C  beq cr6, 0x82c583e8
	if ctx.cr[6].eq {
	pc = 0x82C583E8; continue 'dispatch;
	}
	// 82C583E0: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C583E4: 48000008  b 0x82c583ec
	pc = 0x82C583EC; continue 'dispatch;
	// 82C583E8: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C583EC: 48080FF5  bl 0x82cd93e0
	ctx.lr = 0x82C583F0;
	sub_82CD93E0(ctx, base);
	// 82C583F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C583F4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82C583F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C583FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C58400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C58408 size=176
    let mut pc: u32 = 0x82C58408;
    'dispatch: loop {
        match pc {
            0x82C58408 => {
    //   block [0x82C58408..0x82C584B8)
	// 82C58408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5840C: 48051001  bl 0x82ca940c
	ctx.lr = 0x82C58410;
	sub_82CA93D0(ctx, base);
	// 82C58410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C58418: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82C5841C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C58420: 4B5C6E39  bl 0x8221f258
	ctx.lr = 0x82C58424;
	sub_8221F258(ctx, base);
	// 82C58424: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C58428: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5842C: 419A000C  beq cr6, 0x82c58438
	if ctx.cr[6].eq {
	pc = 0x82C58438; continue 'dispatch;
	}
	// 82C58430: 48003139  bl 0x82c5b568
	ctx.lr = 0x82C58434;
	sub_82C5B568(ctx, base);
	// 82C58434: 48000008  b 0x82c5843c
	pc = 0x82C5843C; continue 'dispatch;
	// 82C58438: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C5843C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C58440: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82C58444: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82C58448: 395E0010  addi r10, r30, 0x10
	ctx.r[10].s64 = ctx.r[30].s64 + 16;
	// 82C5844C: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C58450: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C58454: 409A005C  bne cr6, 0x82c584b0
	if !ctx.cr[6].eq {
	pc = 0x82C584B0; continue 'dispatch;
	}
	// 82C58458: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5845C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C58460: 409A0050  bne cr6, 0x82c584b0
	if !ctx.cr[6].eq {
	pc = 0x82C584B0; continue 'dispatch;
	}
	// 82C58464: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C58468: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5846C: 409A0044  bne cr6, 0x82c584b0
	if !ctx.cr[6].eq {
	pc = 0x82C584B0; continue 'dispatch;
	}
	// 82C58470: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C58474: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C58478: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5847C: 409A001C  bne cr6, 0x82c58498
	if !ctx.cr[6].eq {
	pc = 0x82C58498; continue 'dispatch;
	}
	// 82C58480: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C58484: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C58488: 906A0000  stw r3, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C5848C: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C58490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C58494: 48050FC8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C58498: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5849C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C584A0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C584A4: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C584A8: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C584AC: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C584B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C584B4: 48050FA8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C584B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C584B8 size=120
    let mut pc: u32 = 0x82C584B8;
    'dispatch: loop {
        match pc {
            0x82C584B8 => {
    //   block [0x82C584B8..0x82C58530)
	// 82C584B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C584BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C584C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C584C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C584C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C584CC: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82C584D0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C584D4: 4BFB9885  bl 0x82c11d58
	ctx.lr = 0x82C584D8;
	sub_82C11D58(ctx, base);
	// 82C584D8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C584DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C584E0: 419A0034  beq cr6, 0x82c58514
	if ctx.cr[6].eq {
	pc = 0x82C58514; continue 'dispatch;
	}
	// 82C584E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C584E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C584EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C584F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C584F4: 4E800421  bctrl
	ctx.lr = 0x82C584F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C584F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C584FC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C58500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C58504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C58508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5850C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C58510: 4E800020  blr
	return;
	// 82C58514: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C58518: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C5851C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C58520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C58524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C58528: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C58530 size=104
    let mut pc: u32 = 0x82C58530;
    'dispatch: loop {
        match pc {
            0x82C58530 => {
    //   block [0x82C58530..0x82C58598)
	// 82C58530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C58534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C58538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5853C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C58540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C58548: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C5854C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C58550: 4B5C6D09  bl 0x8221f258
	ctx.lr = 0x82C58554;
	sub_8221F258(ctx, base);
	// 82C58554: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C58558: 419A000C  beq cr6, 0x82c58564
	if ctx.cr[6].eq {
	pc = 0x82C58564; continue 'dispatch;
	}
	// 82C5855C: 48002DAD  bl 0x82c5b308
	ctx.lr = 0x82C58560;
	sub_82C5B308(ctx, base);
	// 82C58560: 48000008  b 0x82c58568
	pc = 0x82C58568; continue 'dispatch;
	// 82C58564: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C58568: 907F014C  stw r3, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[3].u32 ) };
	// 82C5856C: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C58570: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82C58574: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C58578: 815F014C  lwz r10, 0x14c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82C5857C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C58580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C58584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C58588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5858C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C58590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C58594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C58598 size=176
    let mut pc: u32 = 0x82C58598;
    'dispatch: loop {
        match pc {
            0x82C58598 => {
    //   block [0x82C58598..0x82C58648)
	// 82C58598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5859C: 48050E71  bl 0x82ca940c
	ctx.lr = 0x82C585A0;
	sub_82CA93D0(ctx, base);
	// 82C585A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C585A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C585A8: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82C585AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C585B0: 4B5C6CA9  bl 0x8221f258
	ctx.lr = 0x82C585B4;
	sub_8221F258(ctx, base);
	// 82C585B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C585B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C585BC: 419A000C  beq cr6, 0x82c585c8
	if ctx.cr[6].eq {
	pc = 0x82C585C8; continue 'dispatch;
	}
	// 82C585C0: 48003989  bl 0x82c5bf48
	ctx.lr = 0x82C585C4;
	sub_82C5BF48(ctx, base);
	// 82C585C4: 48000008  b 0x82c585cc
	pc = 0x82C585CC; continue 'dispatch;
	// 82C585C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C585CC: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82C585D0: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82C585D4: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82C585D8: 395E0124  addi r10, r30, 0x124
	ctx.r[10].s64 = ctx.r[30].s64 + 292;
	// 82C585DC: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C585E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C585E4: 409A005C  bne cr6, 0x82c58640
	if !ctx.cr[6].eq {
	pc = 0x82C58640; continue 'dispatch;
	}
	// 82C585E8: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C585EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C585F0: 409A0050  bne cr6, 0x82c58640
	if !ctx.cr[6].eq {
	pc = 0x82C58640; continue 'dispatch;
	}
	// 82C585F4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C585F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C585FC: 409A0044  bne cr6, 0x82c58640
	if !ctx.cr[6].eq {
	pc = 0x82C58640; continue 'dispatch;
	}
	// 82C58600: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C58604: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C58608: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5860C: 409A001C  bne cr6, 0x82c58628
	if !ctx.cr[6].eq {
	pc = 0x82C58628; continue 'dispatch;
	}
	// 82C58610: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C58614: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C58618: 906A0000  stw r3, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C5861C: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C58620: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C58624: 48050E38  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C58628: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5862C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C58630: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C58634: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C58638: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C5863C: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C58640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C58644: 48050E18  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C58648 size=120
    let mut pc: u32 = 0x82C58648;
    'dispatch: loop {
        match pc {
            0x82C58648 => {
    //   block [0x82C58648..0x82C586C0)
	// 82C58648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5864C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C58650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C58654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58658: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C5865C: 38630124  addi r3, r3, 0x124
	ctx.r[3].s64 = ctx.r[3].s64 + 292;
	// 82C58660: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C58664: 4BFB96F5  bl 0x82c11d58
	ctx.lr = 0x82C58668;
	sub_82C11D58(ctx, base);
	// 82C58668: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5866C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C58670: 419A0034  beq cr6, 0x82c586a4
	if ctx.cr[6].eq {
	pc = 0x82C586A4; continue 'dispatch;
	}
	// 82C58674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C58678: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C5867C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C58680: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C58684: 4E800421  bctrl
	ctx.lr = 0x82C58688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C58688: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C5868C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C58690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C58694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C58698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5869C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C586A0: 4E800020  blr
	return;
	// 82C586A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C586A8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C586AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C586B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C586B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C586B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C586BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C586C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C586C0 size=8
    let mut pc: u32 = 0x82C586C0;
    'dispatch: loop {
        match pc {
            0x82C586C0 => {
    //   block [0x82C586C0..0x82C586C8)
	// 82C586C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C586C4: 48002C34  b 0x82c5b2f8
	sub_82C5B2F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C586C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C586C8 size=36
    let mut pc: u32 = 0x82C586C8;
    'dispatch: loop {
        match pc {
            0x82C586C8 => {
    //   block [0x82C586C8..0x82C586EC)
	// 82C586C8: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C586CC: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C586D0: 7D4A2850  subf r10, r10, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82C586D4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C586D8: 41980014  blt cr6, 0x82c586ec
	if ctx.cr[6].lt {
		sub_82C586EC(ctx, base);
		return;
	}
	// 82C586DC: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82C586E0: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C586EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C586EC size=92
    let mut pc: u32 = 0x82C586EC;
    'dispatch: loop {
        match pc {
            0x82C586EC => {
    //   block [0x82C586EC..0x82C58748)
	// 82C586EC: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82C586F0: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82C586F4: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82C586F8: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C586FC: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82C58700: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C58704: FD60069C  fcfid f11, f0
	ctx.f[11].f64 = (ctx.f[0].s64 as f64);
	// 82C58708: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82C5870C: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82C58710: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82C58714: FD205818  frsp f9, f11
	ctx.f[9].f64 = (ctx.f[11].f64 as f32) as f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C58748 size=544
    let mut pc: u32 = 0x82C58748;
    'dispatch: loop {
        match pc {
            0x82C58748 => {
    //   block [0x82C58748..0x82C58968)
	// 82C58748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5874C: 48050CB1  bl 0x82ca93fc
	ctx.lr = 0x82C58750;
	sub_82CA93D0(ctx, base);
	// 82C58750: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82C58754: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58758: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C5875C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C58760: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82C58764: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C58768: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82C5876C: 817D0148  lwz r11, 0x148(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(328 as u32) ) } as u64;
	// 82C58770: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C58774: 409A0014  bne cr6, 0x82c58788
	if !ctx.cr[6].eq {
	pc = 0x82C58788; continue 'dispatch;
	}
	// 82C58778: 817D027C  lwz r11, 0x27c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(636 as u32) ) } as u64;
	// 82C5877C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82C58780: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82C58784: 41990008  bgt cr6, 0x82c5878c
	if ctx.cr[6].gt {
	pc = 0x82C5878C; continue 'dispatch;
	}
	// 82C58788: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C5878C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C58790: 997E0014  stb r11, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82C58794: 5569063E  clrlwi r9, r11, 0x18
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C58798: 3B400006  li r26, 6
	ctx.r[26].s64 = 6;
	// 82C5879C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C587A0: C3EA0C18  lfs f31, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C587A4: 419A00AC  beq cr6, 0x82c58850
	if ctx.cr[6].eq {
	pc = 0x82C58850; continue 'dispatch;
	}
	// 82C587A8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82C587AC: 39610101  addi r11, r1, 0x101
	ctx.r[11].s64 = ctx.r[1].s64 + 257;
	// 82C587B0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82C587B4: D3EB0003  stfs f31, 3(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), tmp.u32 ) };
	// 82C587B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C587BC: 9BEBFFFF  stb r31, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[31].u8 ) };
	// 82C587C0: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82C587C4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82C587C8: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 82C587CC: 4198FFE4  blt cr6, 0x82c587b0
	if ctx.cr[6].lt {
	pc = 0x82C587B0; continue 'dispatch;
	}
	// 82C587D0: 39610100  addi r11, r1, 0x100
	ctx.r[11].s64 = ctx.r[1].s64 + 256;
	// 82C587D4: 9B410050  stb r26, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u8 ) };
	// 82C587D8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C587DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C587E0: 48004009  bl 0x82c5c7e8
	ctx.lr = 0x82C587E4;
	sub_82C5C7E8(ctx, base);
	// 82C587E4: 90610090  stw r3, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 82C587E8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82C587EC: 9B810088  stb r28, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[28].u8 ) };
	// 82C587F0: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82C587F4: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 82C587F8: 91410094  stw r10, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82C587FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C58800: 9121008C  stw r9, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 82C58804: 38610061  addi r3, r1, 0x61
	ctx.r[3].s64 = ctx.r[1].s64 + 97;
	// 82C58808: 480511A9  bl 0x82ca99b0
	ctx.lr = 0x82C5880C;
	sub_82CA99B0(ctx, base);
	// 82C5880C: 3D000000  lis r8, 0
	ctx.r[8].s64 = 0;
	// 82C58810: 38E10088  addi r7, r1, 0x88
	ctx.r[7].s64 = ctx.r[1].s64 + 136;
	// 82C58814: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82C58818: 6106BB80  ori r6, r8, 0xbb80
	ctx.r[6].u64 = ctx.r[8].u64 | 48000;
	// 82C5881C: 9B810061  stb r28, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[28].u8 ) };
	// 82C58820: 9B410069  stb r26, 0x69(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(105 as u32), ctx.r[26].u8 ) };
	// 82C58824: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 82C58828: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 82C5882C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C58830: 9BE1006A  stb r31, 0x6a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(106 as u32), ctx.r[31].u8 ) };
	// 82C58834: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82C58838: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 82C5883C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82C58840: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82C58844: 4808131D  bl 0x82cd9b60
	ctx.lr = 0x82C58848;
	sub_82CD9B60(ctx, base);
	// 82C58848: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C5884C: 41980110  blt cr6, 0x82c5895c
	if ctx.cr[6].lt {
	pc = 0x82C5895C; continue 'dispatch;
	}
	// 82C58850: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82C58854: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C58858: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82C5885C: 38A10130  addi r5, r1, 0x130
	ctx.r[5].s64 = ctx.r[1].s64 + 304;
	// 82C58860: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C58864: 387D0144  addi r3, r29, 0x144
	ctx.r[3].s64 = ctx.r[29].s64 + 324;
	// 82C58868: 4BFFF809  bl 0x82c58070
	ctx.lr = 0x82C5886C;
	sub_82C58070(ctx, base);
	// 82C5886C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C58870: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58874: 419A00D4  beq cr6, 0x82c58948
	if ctx.cr[6].eq {
	pc = 0x82C58948; continue 'dispatch;
	}
	// 82C58878: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5887C: 39610130  addi r11, r1, 0x130
	ctx.r[11].s64 = ctx.r[1].s64 + 304;
	// 82C58880: 895E0014  lbz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C58884: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82C58888: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5888C: 9B610080  stb r27, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u8 ) };
	// 82C58890: 419A0010  beq cr6, 0x82c588a0
	if ctx.cr[6].eq {
	pc = 0x82C588A0; continue 'dispatch;
	}
	// 82C58894: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C58898: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82C5889C: 48000010  b 0x82c588ac
	pc = 0x82C588AC; continue 'dispatch;
	// 82C588A0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82C588A4: 48003F45  bl 0x82c5c7e8
	ctx.lr = 0x82C588A8;
	sub_82C5C7E8(ctx, base);
	// 82C588A8: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82C588AC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C588B0: 419A0010  beq cr6, 0x82c588c0
	if ctx.cr[6].eq {
	pc = 0x82C588C0; continue 'dispatch;
	}
	// 82C588B4: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82C588B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82C588BC: 48000008  b 0x82c588c4
	pc = 0x82C588C4; continue 'dispatch;
	// 82C588C0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82C588C4: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 82C588C8: 9B810098  stb r28, 0x98(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[28].u8 ) };
	// 82C588CC: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82C588D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C588D4: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82C588D8: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82C588DC: 480510D5  bl 0x82ca99b0
	ctx.lr = 0x82C588E0;
	sub_82CA99B0(ctx, base);
	// 82C588E0: 815D027C  lwz r10, 0x27c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(636 as u32) ) } as u64;
	// 82C588E4: 813D0278  lwz r9, 0x278(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(632 as u32) ) } as u64;
	// 82C588E8: 3D0082C6  lis r8, -0x7d3a
	ctx.r[8].s64 = -2100953088;
	// 82C588EC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82C588F0: D3E100DC  stfs f31, 0xdc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 82C588F4: 38A10098  addi r5, r1, 0x98
	ctx.r[5].s64 = ctx.r[1].s64 + 152;
	// 82C588F8: 93A100F8  stw r29, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[29].u32 ) };
	// 82C588FC: 39688058  addi r11, r8, -0x7fa8
	ctx.r[11].s64 = ctx.r[8].s64 + -32680;
	// 82C58900: 9B8100A0  stb r28, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[28].u8 ) };
	// 82C58904: 994100A4  stb r10, 0xa4(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[10].u8 ) };
	// 82C58908: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 82C5890C: 912100A8  stw r9, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[9].u32 ) };
	// 82C58910: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82C58914: 9BE100D8  stb r31, 0xd8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[31].u8 ) };
	// 82C58918: 9B8100D9  stb r28, 0xd9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(217 as u32), ctx.r[28].u8 ) };
	// 82C5891C: 9B4100DA  stb r26, 0xda(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(218 as u32), ctx.r[26].u8 ) };
	// 82C58920: 98C100DB  stb r6, 0xdb(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(219 as u32), ctx.r[6].u8 ) };
	// 82C58924: 9BE100E0  stb r31, 0xe0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[31].u8 ) };
	// 82C58928: 93E100E4  stw r31, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[31].u32 ) };
	// 82C5892C: 90A100E8  stw r5, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[5].u32 ) };
	// 82C58930: 93E100EC  stw r31, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[31].u32 ) };
	// 82C58934: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82C58938: 93E100F4  stw r31, 0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[31].u32 ) };
	// 82C5893C: 480811C5  bl 0x82cd9b00
	ctx.lr = 0x82C58940;
	sub_82CD9B00(ctx, base);
	// 82C58940: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C58944: 40980018  bge cr6, 0x82c5895c
	if !ctx.cr[6].lt {
	pc = 0x82C5895C; continue 'dispatch;
	}
	// 82C58948: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C5894C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C58950: 419A000C  beq cr6, 0x82c5895c
	if ctx.cr[6].eq {
	pc = 0x82C5895C; continue 'dispatch;
	}
	// 82C58954: 4808095D  bl 0x82cd92b0
	ctx.lr = 0x82C58958;
	sub_82CD92B0(ctx, base);
	// 82C58958: 93FE001C  stw r31, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 82C5895C: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 82C58960: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82C58964: 48050AE8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C58968 size=60
    let mut pc: u32 = 0x82C58968;
    'dispatch: loop {
        match pc {
            0x82C58968 => {
    //   block [0x82C58968..0x82C589A4)
	// 82C58968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5896C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C58970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58974: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82C58978: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C5897C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82C58980: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 82C58984: 38630158  addi r3, r3, 0x158
	ctx.r[3].s64 = ctx.r[3].s64 + 344;
	// 82C58988: 816B5164  lwz r11, 0x5164(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5898C: 808B005C  lwz r4, 0x5c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C58990: 4BFBA801  bl 0x82c13190
	ctx.lr = 0x82C58994;
	sub_82C13190(ctx, base);
	// 82C58994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C58998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5899C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C589A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C589A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C589A8 size=68
    let mut pc: u32 = 0x82C589A8;
    'dispatch: loop {
        match pc {
            0x82C589A8 => {
    //   block [0x82C589A8..0x82C589EC)
	// 82C589A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C589AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C589B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C589B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C589B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C589BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C589C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C589C4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C589C8: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 82C589CC: 4BFFF5AD  bl 0x82c57f78
	ctx.lr = 0x82C589D0;
	sub_82C57F78(ctx, base);
	// 82C589D0: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 82C589D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C589D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C589DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C589E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C589E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C589E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C589F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C589F0 size=68
    let mut pc: u32 = 0x82C589F0;
    'dispatch: loop {
        match pc {
            0x82C589F0 => {
    //   block [0x82C589F0..0x82C58A34)
	// 82C589F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C589F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C589F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C589FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C58A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C58A08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C58A0C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C58A10: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82C58A14: 4BFFF565  bl 0x82c57f78
	ctx.lr = 0x82C58A18;
	sub_82C57F78(ctx, base);
	// 82C58A18: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82C58A1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C58A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C58A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C58A28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C58A2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C58A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C58A38 size=112
    let mut pc: u32 = 0x82C58A38;
    'dispatch: loop {
        match pc {
            0x82C58A38 => {
    //   block [0x82C58A38..0x82C58AA8)
	// 82C58A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C58A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C58A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58A44: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82C58A48: 988302B5  stb r4, 0x2b5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(693 as u32), ctx.r[4].u8 ) };
	// 82C58A4C: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82C58A50: 90C30290  stw r6, 0x290(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(656 as u32), ctx.r[6].u32 ) };
	// 82C58A54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58A58: 419A0010  beq cr6, 0x82c58a68
	if ctx.cr[6].eq {
	pc = 0x82C58A68; continue 'dispatch;
	}
	// 82C58A5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C58A60: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58A64: 48000018  b 0x82c58a7c
	pc = 0x82C58A7C; continue 'dispatch;
	// 82C58A68: 896302B4  lbz r11, 0x2b4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(692 as u32) ) } as u64;
	// 82C58A6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58A70: 409A0028  bne cr6, 0x82c58a98
	if !ctx.cr[6].eq {
	pc = 0x82C58A98; continue 'dispatch;
	}
	// 82C58A74: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C58A78: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58A7C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C58A80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C58A84: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C58A88: 38630178  addi r3, r3, 0x178
	ctx.r[3].s64 = ctx.r[3].s64 + 376;
	// 82C58A8C: 816A5164  lwz r11, 0x5164(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C58A90: 808B005C  lwz r4, 0x5c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C58A94: 4BFBA6FD  bl 0x82c13190
	ctx.lr = 0x82C58A98;
	sub_82C13190(ctx, base);
	// 82C58A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C58A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C58AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C58AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C58AA8 size=44
    let mut pc: u32 = 0x82C58AA8;
    'dispatch: loop {
        match pc {
            0x82C58AA8 => {
    //   block [0x82C58AA8..0x82C58AD4)
	// 82C58AA8: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C58AAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C58AB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C58AB4: 409A0010  bne cr6, 0x82c58ac4
	if !ctx.cr[6].eq {
	pc = 0x82C58AC4; continue 'dispatch;
	}
	// 82C58AB8: 814B0284  lwz r10, 0x284(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C58ABC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C58AC0: 914B0284  stw r10, 0x284(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(644 as u32), ctx.r[10].u32 ) };
	// 82C58AC4: 806B0188  lwz r3, 0x188(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(392 as u32) ) } as u64;
	// 82C58AC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C58ACC: 419A0008  beq cr6, 0x82c58ad4
	if ctx.cr[6].eq {
		sub_82C58AD4(ctx, base);
		return;
	}
	// 82C58AD0: 4BFFF440  b 0x82c57f10
	sub_82C57F10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C58AD4 size=40
    let mut pc: u32 = 0x82C58AD4;
    'dispatch: loop {
        match pc {
            0x82C58AD4 => {
    //   block [0x82C58AD4..0x82C58AFC)
	// 82C58AD4: 814B0288  lwz r10, 0x288(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C58AD8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C58ADC: E92B0270  ld r9, 0x270(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(624 as u32) ) };
	// 82C58AE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C58AE4: 914B0288  stw r10, 0x288(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(648 as u32), ctx.r[10].u32 ) };
	// 82C58AE8: 81040010  lwz r8, 0x10(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C58AEC: 790AF842  rldicl r10, r8, 0x3f, 1
	ctx.r[10].u64 = ctx.r[8].u64 & 0x0000000000000001u64;
	// 82C58AF0: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C58AF4: F8EB0270  std r7, 0x270(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(624 as u32), ctx.r[7].u64 ) };
	// 82C58AF8: 4BBECCB8  b 0x828457b0
	sub_828457B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C58B00 size=1132
    let mut pc: u32 = 0x82C58B00;
    'dispatch: loop {
        match pc {
            0x82C58B00 => {
    //   block [0x82C58B00..0x82C58F6C)
	// 82C58B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C58B04: 48050901  bl 0x82ca9404
	ctx.lr = 0x82C58B08;
	sub_82CA93D0(ctx, base);
	// 82C58B08: DBA1FFB8  stfd f29, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[29].u64 ) };
	// 82C58B0C: DBC1FFC0  stfd f30, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82C58B10: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82C58B14: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58B18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C58B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C58B20: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C58B24: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82C58B28: C3EB0C14  lfs f31, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C58B2C: D3FF0294  stfs f31, 0x294(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), tmp.u32 ) };
	// 82C58B30: D3FF029C  stfs f31, 0x29c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 82C58B34: 4BFF4485  bl 0x82c4cfb8
	ctx.lr = 0x82C58B38;
	sub_82C4CFB8(ctx, base);
	// 82C58B38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C58B3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C58B40: 419A00A0  beq cr6, 0x82c58be0
	if ctx.cr[6].eq {
	pc = 0x82C58BE0; continue 'dispatch;
	}
	// 82C58B44: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 82C58B48: 3B9D0004  addi r28, r29, 4
	ctx.r[28].s64 = ctx.r[29].s64 + 4;
	// 82C58B4C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C58B50: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C58B54: 4800310D  bl 0x82c5bc60
	ctx.lr = 0x82C58B58;
	sub_82C5BC60(ctx, base);
	// 82C58B58: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C58B5C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C58B60: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82C58B64: 48003145  bl 0x82c5bca8
	ctx.lr = 0x82C58B68;
	sub_82C5BCA8(ctx, base);
	// 82C58B68: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C58B6C: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82C58B70: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82C58B74: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82C58B78: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C58B7C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82C58B80: FC406818  frsp f2, f13
	ctx.f[2].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82C58B84: 4B5A5925  bl 0x821fe4a8
	ctx.lr = 0x82C58B88;
	sub_821FE4A8(ctx, base);
	// 82C58B88: FD800818  frsp f12, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C58B8C: C17F0294  lfs f11, 0x294(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C58B90: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82C58B94: ED4C02F2  fmuls f10, f12, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 82C58B98: D15F0294  stfs f10, 0x294(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), tmp.u32 ) };
	// 82C58B9C: 811E0028  lwz r8, 0x28(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C58BA0: F9010060  std r8, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 82C58BA4: C9210060  lfd f9, 0x60(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C58BA8: FD004E9C  fcfid f8, f9
	ctx.f[8].f64 = (ctx.f[9].s64 as f64);
	// 82C58BAC: FC404018  frsp f2, f8
	ctx.f[2].f64 = (ctx.f[8].f64 as f32) as f64;
	// 82C58BB0: 4B5A58F9  bl 0x821fe4a8
	ctx.lr = 0x82C58BB4;
	sub_821FE4A8(ctx, base);
	// 82C58BB4: FCE00818  frsp f7, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[7].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C58BB8: C0DF029C  lfs f6, 0x29c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82C58BBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C58BC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C58BC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C58BC8: ECA601F2  fmuls f5, f6, f7
	ctx.f[5].f64 = (((ctx.f[6].f64 * ctx.f[7].f64) as f32) as f64);
	// 82C58BCC: D0BF029C  stfs f5, 0x29c(r31)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 82C58BD0: 4BFF4979  bl 0x82c4d548
	ctx.lr = 0x82C58BD4;
	sub_82C4D548(ctx, base);
	// 82C58BD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C58BD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C58BDC: 409AFF70  bne cr6, 0x82c58b4c
	if !ctx.cr[6].eq {
	pc = 0x82C58B4C; continue 'dispatch;
	}
	// 82C58BE0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C58BE4: 391F0010  addi r8, r31, 0x10
	ctx.r[8].s64 = ctx.r[31].s64 + 16;
	// 82C58BE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58BEC: 419A0094  beq cr6, 0x82c58c80
	if ctx.cr[6].eq {
	pc = 0x82C58C80; continue 'dispatch;
	}
	// 82C58BF0: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C58BF4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C58BF8: 7D29D850  subf r9, r9, r27
	ctx.r[9].s64 = ctx.r[27].s64 - ctx.r[9].s64;
	// 82C58BFC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C58C00: 4198000C  blt cr6, 0x82c58c0c
	if ctx.cr[6].lt {
	pc = 0x82C58C0C; continue 'dispatch;
	}
	// 82C58C04: C00B001C  lfs f0, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58C08: 48000040  b 0x82c58c48
	pc = 0x82C58C48; continue 'dispatch;
	// 82C58C0C: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C58C10: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58C14: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82C58C18: C1AB001C  lfs f13, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C58C1C: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82C58C20: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C58C24: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 82C58C28: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C58C2C: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 82C58C30: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C58C34: FCC04818  frsp f6, f9
	ctx.f[6].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C58C38: ECED0028  fsubs f7, f13, f0
	ctx.f[7].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C58C3C: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C58C40: ECA83024  fdivs f5, f8, f6
	ctx.f[5].f64 = ((ctx.f[8].f64 / ctx.f[6].f64) as f32) as f64;
	// 82C58C44: EC0501FA  fmadds f0, f5, f7, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[7].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C58C48: C1BF0294  lfs f13, 0x294(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C58C4C: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C58C50: D19F0294  stfs f12, 0x294(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), tmp.u32 ) };
	// 82C58C54: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C58C58: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C58C5C: 409A0024  bne cr6, 0x82c58c80
	if !ctx.cr[6].eq {
	pc = 0x82C58C80; continue 'dispatch;
	}
	// 82C58C60: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C58C64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58C68: 419A0018  beq cr6, 0x82c58c80
	if ctx.cr[6].eq {
	pc = 0x82C58C80; continue 'dispatch;
	}
	// 82C58C6C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C58C70: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C58C74: 409A000C  bne cr6, 0x82c58c80
	if !ctx.cr[6].eq {
	pc = 0x82C58C80; continue 'dispatch;
	}
	// 82C58C78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58C7C: 409AFF74  bne cr6, 0x82c58bf0
	if !ctx.cr[6].eq {
	pc = 0x82C58BF0; continue 'dispatch;
	}
	// 82C58C80: 815F0158  lwz r10, 0x158(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 82C58C84: 817F015C  lwz r11, 0x15c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 82C58C88: 7D4AD850  subf r10, r10, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[10].s64;
	// 82C58C8C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C58C90: 4198000C  blt cr6, 0x82c58c9c
	if ctx.cr[6].lt {
	pc = 0x82C58C9C; continue 'dispatch;
	}
	// 82C58C94: C01F0164  lfs f0, 0x164(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58C98: 48000040  b 0x82c58cd8
	pc = 0x82C58CD8; continue 'dispatch;
	// 82C58C9C: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82C58CA0: C01F0160  lfs f0, 0x160(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58CA4: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82C58CA8: C1BF0164  lfs f13, 0x164(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C58CAC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82C58CB0: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C58CB4: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82C58CB8: C9610060  lfd f11, 0x60(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C58CBC: FD205E9C  fcfid f9, f11
	ctx.f[9].f64 = (ctx.f[11].s64 as f64);
	// 82C58CC0: FD00569C  fcfid f8, f10
	ctx.f[8].f64 = (ctx.f[10].s64 as f64);
	// 82C58CC4: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C58CC8: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C58CCC: FCC04018  frsp f6, f8
	ctx.f[6].f64 = (ctx.f[8].f64 as f32) as f64;
	// 82C58CD0: ECA63824  fdivs f5, f6, f7
	ctx.f[5].f64 = ((ctx.f[6].f64 / ctx.f[7].f64) as f32) as f64;
	// 82C58CD4: EC05033A  fmadds f0, f5, f12, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C58CD8: C1BF029C  lfs f13, 0x29c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C58CDC: 897F02AF  lbz r11, 0x2af(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(687 as u32) ) } as u64;
	// 82C58CE0: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C58CE4: D1BF029C  stfs f13, 0x29c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 82C58CE8: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C58CEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58CF0: 409A0068  bne cr6, 0x82c58d58
	if !ctx.cr[6].eq {
	pc = 0x82C58D58; continue 'dispatch;
	}
	// 82C58CF4: 817E5164  lwz r11, 0x5164(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C58CF8: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C58CFC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C58D00: 7D29D850  subf r9, r9, r27
	ctx.r[9].s64 = ctx.r[27].s64 - ctx.r[9].s64;
	// 82C58D04: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C58D08: 4198000C  blt cr6, 0x82c58d14
	if ctx.cr[6].lt {
	pc = 0x82C58D14; continue 'dispatch;
	}
	// 82C58D0C: C00B0038  lfs f0, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58D10: 48000040  b 0x82c58d50
	pc = 0x82C58D50; continue 'dispatch;
	// 82C58D14: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C58D18: C00B0034  lfs f0, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58D1C: 79480020  clrldi r8, r10, 0x20
	ctx.r[8].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82C58D20: C18B0038  lfs f12, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C58D24: F9210060  std r9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 82C58D28: ECCC0028  fsubs f6, f12, f0
	ctx.f[6].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C58D2C: C9610060  lfd f11, 0x60(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C58D30: F9010060  std r8, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 82C58D34: FD005E9C  fcfid f8, f11
	ctx.f[8].f64 = (ctx.f[11].s64 as f64);
	// 82C58D38: FCA04018  frsp f5, f8
	ctx.f[5].f64 = (ctx.f[8].f64 as f32) as f64;
	// 82C58D3C: C9410060  lfd f10, 0x60(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C58D40: FD20569C  fcfid f9, f10
	ctx.f[9].f64 = (ctx.f[10].s64 as f64);
	// 82C58D44: FCE04818  frsp f7, f9
	ctx.f[7].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C58D48: EC853824  fdivs f4, f5, f7
	ctx.f[4].f64 = ((ctx.f[5].f64 / ctx.f[7].f64) as f32) as f64;
	// 82C58D4C: EC0401BA  fmadds f0, f4, f6, f0
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[6].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C58D50: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C58D54: D01F029C  stfs f0, 0x29c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 82C58D58: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82C58D5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C58D60: 419A00C4  beq cr6, 0x82c58e24
	if ctx.cr[6].eq {
	pc = 0x82C58E24; continue 'dispatch;
	}
	// 82C58D64: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82C58D68: 389F0230  addi r4, r31, 0x230
	ctx.r[4].s64 = ctx.r[31].s64 + 560;
	// 82C58D6C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C58D70: 4BFFF959  bl 0x82c586c8
	ctx.lr = 0x82C58D74;
	sub_82C586C8(ctx, base);
	// 82C58D74: 817F01F4  lwz r11, 0x1f4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C58D78: D3EB0000  stfs f31, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C58D7C: C0010068  lfs f0, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C58D80: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C58D84: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C58D88: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C58D8C: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C58D90: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C58D94: 815F01F4  lwz r10, 0x1f4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C58D98: C1810060  lfs f12, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C58D9C: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C58DA0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C58DA4: D3EA0004  stfs f31, 4(r10)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82C58DA8: 813F01F4  lwz r9, 0x1f4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C58DAC: D3E90008  stfs f31, 8(r9)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C58DB0: 80DF01F4  lwz r6, 0x1f4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C58DB4: D3E6000C  stfs f31, 0xc(r6)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C58DB8: 80BF01F4  lwz r5, 0x1f4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C58DBC: D3E50010  stfs f31, 0x10(r5)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C58DC0: 809F01F4  lwz r4, 0x1f4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C58DC4: D3E40014  stfs f31, 0x14(r4)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82C58DC8: 887F0260  lbz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 82C58DCC: 917F01AC  stw r11, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[11].u32 ) };
	// 82C58DD0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82C58DD4: 90FF01B0  stw r7, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[7].u32 ) };
	// 82C58DD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C58DDC: 911F01B4  stw r8, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[8].u32 ) };
	// 82C58DE0: 419A0020  beq cr6, 0x82c58e00
	if ctx.cr[6].eq {
	pc = 0x82C58E00; continue 'dispatch;
	}
	// 82C58DE4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C58DE8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C58F70 size=108
    let mut pc: u32 = 0x82C58F70;
    'dispatch: loop {
        match pc {
            0x82C58F70 => {
    //   block [0x82C58F70..0x82C58FDC)
	// 82C58F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C58F74: 48050499  bl 0x82ca940c
	ctx.lr = 0x82C58F78;
	sub_82CA93D0(ctx, base);
	// 82C58F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58F7C: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82C58F80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C58F84: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C58F88: 4B5C62D1  bl 0x8221f258
	ctx.lr = 0x82C58F8C;
	sub_8221F258(ctx, base);
	// 82C58F8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C58F90: 419A0010  beq cr6, 0x82c58fa0
	if ctx.cr[6].eq {
	pc = 0x82C58FA0; continue 'dispatch;
	}
	// 82C58F94: 480022E5  bl 0x82c5b278
	ctx.lr = 0x82C58F98;
	sub_82C5B278(ctx, base);
	// 82C58F98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C58F9C: 48000008  b 0x82c58fa4
	pc = 0x82C58FA4; continue 'dispatch;
	// 82C58FA0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C58FA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C58FA8: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82C58FAC: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82C58FB0: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 82C58FB4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C58FB8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C58FBC: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C58FC0: 4BFF4301  bl 0x82c4d2c0
	ctx.lr = 0x82C58FC4;
	sub_82C4D2C0(ctx, base);
	// 82C58FC4: 389D0010  addi r4, r29, 0x10
	ctx.r[4].s64 = ctx.r[29].s64 + 16;
	// 82C58FC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C58FCC: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C58FD0: 4BFF42F1  bl 0x82c4d2c0
	ctx.lr = 0x82C58FD4;
	sub_82C4D2C0(ctx, base);
	// 82C58FD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C58FD8: 48050484  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C58FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C58FE0 size=1016
    let mut pc: u32 = 0x82C58FE0;
    'dispatch: loop {
        match pc {
            0x82C58FE0 => {
    //   block [0x82C58FE0..0x82C593D8)
	// 82C58FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C58FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C58FE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C58FEC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C58FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C58FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C58FF8: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82C58FFC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C59000: 4B5A7771  bl 0x82200770
	ctx.lr = 0x82C59004;
	sub_82200770(ctx, base);
	// 82C59004: 80FF0124  lwz r7, 0x124(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 82C59008: 38BF0124  addi r5, r31, 0x124
	ctx.r[5].s64 = ctx.r[31].s64 + 292;
	// 82C5900C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C59010: 419A0148  beq cr6, 0x82c59158
	if ctx.cr[6].eq {
	pc = 0x82C59158; continue 'dispatch;
	}
	// 82C59014: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C59018: 39670028  addi r11, r7, 0x28
	ctx.r[11].s64 = ctx.r[7].s64 + 40;
	// 82C5901C: 81070028  lwz r8, 0x28(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C59020: 8127002C  lwz r9, 0x2c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C59024: 7D085050  subf r8, r8, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82C59028: 80C70048  lwz r6, 0x48(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C5902C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C59030: 4198000C  blt cr6, 0x82c5903c
	if ctx.cr[6].lt {
	pc = 0x82C5903C; continue 'dispatch;
	}
	// 82C59034: C00B000C  lfs f0, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59038: 48000040  b 0x82c59078
	pc = 0x82C59078; continue 'dispatch;
	// 82C5903C: 79080020  clrldi r8, r8, 0x20
	ctx.r[8].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82C59040: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59044: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C59048: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C5904C: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 82C59050: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C59054: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82C59058: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C5905C: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 82C59060: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C59064: FCC04818  frsp f6, f9
	ctx.f[6].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C59068: ECED0028  fsubs f7, f13, f0
	ctx.f[7].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C5906C: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C59070: ECA83024  fdivs f5, f8, f6
	ctx.f[5].f64 = ((ctx.f[8].f64 / ctx.f[6].f64) as f32) as f64;
	// 82C59074: EC0501FA  fmadds f0, f5, f7, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[7].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C59078: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C5907C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59080: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59084: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82C59088: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C5908C: 4199000C  bgt cr6, 0x82c59098
	if ctx.cr[6].gt {
	pc = 0x82C59098; continue 'dispatch;
	}
	// 82C59090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C59094: 48000008  b 0x82c5909c
	pc = 0x82C5909C; continue 'dispatch;
	// 82C59098: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C5909C: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C590A0: 39670038  addi r11, r7, 0x38
	ctx.r[11].s64 = ctx.r[7].s64 + 56;
	// 82C590A4: 8107003C  lwz r8, 0x3c(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C590A8: 81270038  lwz r9, 0x38(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C590AC: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C590B0: 80C70048  lwz r6, 0x48(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C590B4: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C590B8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C590BC: 4198000C  blt cr6, 0x82c590c8
	if ctx.cr[6].lt {
	pc = 0x82C590C8; continue 'dispatch;
	}
	// 82C590C0: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C590C4: 48000040  b 0x82c59104
	pc = 0x82C59104; continue 'dispatch;
	// 82C590C8: 79240020  clrldi r4, r9, 0x20
	ctx.r[4].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C590CC: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C590D0: 79080020  clrldi r8, r8, 0x20
	ctx.r[8].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82C590D4: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C590D8: F8810060  std r4, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u64 ) };
	// 82C590DC: C9610060  lfd f11, 0x60(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C590E0: F9010068  std r8, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u64 ) };
	// 82C590E4: C9210068  lfd f9, 0x68(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82C590E8: FCE04E9C  fcfid f7, f9
	ctx.f[7].f64 = (ctx.f[9].s64 as f64);
	// 82C590EC: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C590F0: FCC03818  frsp f6, f7
	ctx.f[6].f64 = (ctx.f[7].f64 as f32) as f64;
	// 82C590F4: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C590F8: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C590FC: ECA83024  fdivs f5, f8, f6
	ctx.f[5].f64 = ((ctx.f[8].f64 / ctx.f[6].f64) as f32) as f64;
	// 82C59100: EC05033A  fmadds f0, f5, f12, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C59104: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C59108: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5910C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59110: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82C59114: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C59118: 4199000C  bgt cr6, 0x82c59124
	if ctx.cr[6].gt {
	pc = 0x82C59124; continue 'dispatch;
	}
	// 82C5911C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C59120: 48000008  b 0x82c59128
	pc = 0x82C59128; continue 'dispatch;
	// 82C59124: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C59128: 9166000C  stw r11, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C5912C: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59130: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C59134: 409A0024  bne cr6, 0x82c59158
	if !ctx.cr[6].eq {
	pc = 0x82C59158; continue 'dispatch;
	}
	// 82C59138: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5913C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C59140: 419A0018  beq cr6, 0x82c59158
	if ctx.cr[6].eq {
	pc = 0x82C59158; continue 'dispatch;
	}
	// 82C59144: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59148: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C5914C: 409A000C  bne cr6, 0x82c59158
	if !ctx.cr[6].eq {
	pc = 0x82C59158; continue 'dispatch;
	}
	// 82C59150: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C59154: 409AFEC0  bne cr6, 0x82c59014
	if !ctx.cr[6].eq {
	pc = 0x82C59014; continue 'dispatch;
	}
	// 82C59158: 80FF011C  lwz r7, 0x11c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 82C5915C: 387F011C  addi r3, r31, 0x11c
	ctx.r[3].s64 = ctx.r[31].s64 + 284;
	// 82C59160: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C59164: 419A024C  beq cr6, 0x82c593b0
	if ctx.cr[6].eq {
	pc = 0x82C593B0; continue 'dispatch;
	}
	// 82C59168: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5916C: 39670158  addi r11, r7, 0x158
	ctx.r[11].s64 = ctx.r[7].s64 + 344;
	// 82C59170: 81070158  lwz r8, 0x158(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(344 as u32) ) } as u64;
	// 82C59174: 8127015C  lwz r9, 0x15c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(348 as u32) ) } as u64;
	// 82C59178: 7D085050  subf r8, r8, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82C5917C: 80C7018C  lwz r6, 0x18c(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C59180: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C59184: 4198000C  blt cr6, 0x82c59190
	if ctx.cr[6].lt {
	pc = 0x82C59190; continue 'dispatch;
	}
	// 82C59188: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5918C: 48000040  b 0x82c591cc
	pc = 0x82C591CC; continue 'dispatch;
	// 82C59190: 79080020  clrldi r8, r8, 0x20
	ctx.r[8].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82C59194: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59198: 79250020  clrldi r5, r9, 0x20
	ctx.r[5].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C5919C: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C591A0: F9010060  std r8, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 82C591A4: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C591A8: F8A10068  std r5, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[5].u64 ) };
	// 82C591AC: C9610068  lfd f11, 0x68(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82C591B0: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C591B4: C9210060  lfd f9, 0x60(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C591B8: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C591BC: FCE04E9C  fcfid f7, f9
	ctx.f[7].f64 = (ctx.f[9].s64 as f64);
	// 82C591C0: FCC03818  frsp f6, f7
	ctx.f[6].f64 = (ctx.f[7].f64 as f32) as f64;
	// 82C591C4: ECA64024  fdivs f5, f6, f8
	ctx.f[5].f64 = ((ctx.f[6].f64 / ctx.f[8].f64) as f32) as f64;
	// 82C591C8: EC05033A  fmadds f0, f5, f12, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C591CC: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C591D0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C591D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C591D8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82C591DC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C591E0: 4199000C  bgt cr6, 0x82c591ec
	if ctx.cr[6].gt {
	pc = 0x82C591EC; continue 'dispatch;
	}
	// 82C591E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C591E8: 48000008  b 0x82c591f0
	pc = 0x82C591F0; continue 'dispatch;
	// 82C591EC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C591F0: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C591F4: 38870010  addi r4, r7, 0x10
	ctx.r[4].s64 = ctx.r[7].s64 + 16;
	// 82C591F8: 80C70010  lwz r6, 0x10(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C591FC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C59200: 419A00BC  beq cr6, 0x82c592bc
	if ctx.cr[6].eq {
	pc = 0x82C592BC; continue 'dispatch;
	}
	// 82C59204: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C59208: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82C5920C: 81060010  lwz r8, 0x10(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C59210: 81260014  lwz r9, 0x14(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C59214: 7D085050  subf r8, r8, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82C59218: 80A60020  lwz r5, 0x20(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C5921C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C59220: 4198000C  blt cr6, 0x82c5922c
	if ctx.cr[6].lt {
	pc = 0x82C5922C; continue 'dispatch;
	}
	// 82C59224: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59228: 48000040  b 0x82c59268
	pc = 0x82C59268; continue 'dispatch;
	// 82C5922C: 79080020  clrldi r8, r8, 0x20
	ctx.r[8].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82C59230: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59234: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82C59238: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C5923C: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 82C59240: ECED0028  fsubs f7, f13, f0
	ctx.f[7].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C59244: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82C59248: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C5924C: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C59250: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 82C59254: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C59258: FCC04818  frsp f6, f9
	ctx.f[6].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C5925C: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C59260: ECA64024  fdivs f5, f6, f8
	ctx.f[5].f64 = ((ctx.f[6].f64 / ctx.f[8].f64) as f32) as f64;
	// 82C59264: EC0501FA  fmadds f0, f5, f7, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[7].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C59268: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82C5926C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59270: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59274: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82C59278: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C5927C: 4199000C  bgt cr6, 0x82c59288
	if ctx.cr[6].gt {
	pc = 0x82C59288; continue 'dispatch;
	}
	// 82C59280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C59284: 48000008  b 0x82c5928c
	pc = 0x82C5928C; continue 'dispatch;
	// 82C59288: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C5928C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C59290: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59294: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C59298: 409A0024  bne cr6, 0x82c592bc
	if !ctx.cr[6].eq {
	pc = 0x82C592BC; continue 'dispatch;
	}
	// 82C5929C: 80C60004  lwz r6, 4(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C592A0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C592A4: 419A0018  beq cr6, 0x82c592bc
	if ctx.cr[6].eq {
	pc = 0x82C592BC; continue 'dispatch;
	}
	// 82C592A8: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C592AC: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C592B0: 409A000C  bne cr6, 0x82c592bc
	if !ctx.cr[6].eq {
	pc = 0x82C592BC; continue 'dispatch;
	}
	// 82C592B4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C592B8: 409AFF4C  bne cr6, 0x82c59204
	if !ctx.cr[6].eq {
	pc = 0x82C59204; continue 'dispatch;
	}
	// 82C592BC: 8147018C  lwz r10, 0x18c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C592C0: C0070298  lfs f0, 0x298(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(664 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C592C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C592C8: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C592CC: 8127018C  lwz r9, 0x18c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C592D0: C1A70294  lfs f13, 0x294(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(660 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C592D4: D1A90008  stfs f13, 8(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C592D8: 8107018C  lwz r8, 0x18c(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C592DC: C187029C  lfs f12, 0x29c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(668 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C592E0: D1880010  stfs f12, 0x10(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C592E4: 80C7018C  lwz r6, 0x18c(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C592E8: 80A70280  lwz r5, 0x280(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(640 as u32) ) } as u64;
	// 82C592EC: 90A60014  stw r5, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82C592F0: 8087018C  lwz r4, 0x18c(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C592F4: 81470288  lwz r10, 0x288(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C592F8: 91440018  stw r10, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82C592FC: 8127018C  lwz r9, 0x18c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C59300: 890702B0  lbz r8, 0x2b0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(688 as u32) ) } as u64;
	// 82C59304: 9909001C  stb r8, 0x1c(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(28 as u32), ctx.r[8].u8 ) };
	// 82C59308: 80C7018C  lwz r6, 0x18c(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C5930C: 88A702B1  lbz r5, 0x2b1(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(689 as u32) ) } as u64;
	// 82C59310: 98A6001D  stb r5, 0x1d(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(29 as u32), ctx.r[5].u8 ) };
	// 82C59314: 8087018C  lwz r4, 0x18c(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C59318: E9470268  ld r10, 0x268(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(616 as u32) ) };
	// 82C5931C: F9440020  std r10, 0x20(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82C59320: 8127018C  lwz r9, 0x18c(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C59324: E9070270  ld r8, 0x270(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(624 as u32) ) };
	// 82C59328: F9090028  std r8, 0x28(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[8].u64 ) };
	// 82C5932C: 80C7018C  lwz r6, 0x18c(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C59330: 88A702B2  lbz r5, 0x2b2(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C59334: 98A6001E  stb r5, 0x1e(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(30 as u32), ctx.r[5].u8 ) };
	// 82C59338: 8087018C  lwz r4, 0x18c(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C5933C: 894702B4  lbz r10, 0x2b4(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(692 as u32) ) } as u64;
	// 82C59340: 9944001F  stb r10, 0x1f(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(31 as u32), ctx.r[10].u8 ) };
	// 82C59344: 892702B2  lbz r9, 0x2b2(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C59348: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5934C: 419A0018  beq cr6, 0x82c59364
	if ctx.cr[6].eq {
	pc = 0x82C59364; continue 'dispatch;
	}
	// 82C59350: 81470154  lwz r10, 0x154(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(340 as u32) ) } as u64;
	// 82C59354: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59358: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5935C: 419A0008  beq cr6, 0x82c59364
	if ctx.cr[6].eq {
	pc = 0x82C59364; continue 'dispatch;
	}
	// 82C59360: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59364: 8147018C  lwz r10, 0x18c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C59368: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C5936C: 912A0030  stw r9, 0x30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82C59370: 8107018C  lwz r8, 0x18c(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(396 as u32) ) } as u64;
	// 82C59374: E9480028  ld r10, 0x28(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(40 as u32) ) };
	// 82C59378: 81680030  lwz r11, 0x30(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C5937C: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C59380: F8C80028  std r6, 0x28(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82C59384: 80A7000C  lwz r5, 0xc(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59388: 7F051840  cmplw cr6, r5, r3
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C5938C: 409A0024  bne cr6, 0x82c593b0
	if !ctx.cr[6].eq {
	pc = 0x82C593B0; continue 'dispatch;
	}
	// 82C59390: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59394: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C59398: 419A0018  beq cr6, 0x82c593b0
	if ctx.cr[6].eq {
	pc = 0x82C593B0; continue 'dispatch;
	}
	// 82C5939C: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C593A0: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C593A4: 409A000C  bne cr6, 0x82c593b0
	if !ctx.cr[6].eq {
	pc = 0x82C593B0; continue 'dispatch;
	}
	// 82C593A8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C593AC: 409AFDBC  bne cr6, 0x82c59168
	if !ctx.cr[6].eq {
	pc = 0x82C59168; continue 'dispatch;
	}
	// 82C593B0: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C593B4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C593B8: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82C593BC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C593C0: 4B56A1F9  bl 0x821c35b8
	ctx.lr = 0x82C593C4;
	sub_821C35B8(ctx, base);
	// 82C593C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C593C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C593CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C593D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C593D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C593D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C593D8 size=100
    let mut pc: u32 = 0x82C593D8;
    'dispatch: loop {
        match pc {
            0x82C593D8 => {
    //   block [0x82C593D8..0x82C5943C)
	// 82C593D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C593DC: 48050031  bl 0x82ca940c
	ctx.lr = 0x82C593E0;
	sub_82CA93D0(ctx, base);
	// 82C593E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C593E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C593E8: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C593EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C593F0: 419A0044  beq cr6, 0x82c59434
	if ctx.cr[6].eq {
	pc = 0x82C59434; continue 'dispatch;
	}
	// 82C593F4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C593F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C593FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C59400: 419A0014  beq cr6, 0x82c59414
	if ctx.cr[6].eq {
	pc = 0x82C59414; continue 'dispatch;
	}
	// 82C59404: 480800ED  bl 0x82cd94f0
	ctx.lr = 0x82C59408;
	sub_82CD94F0(ctx, base);
	// 82C59408: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5940C: 4807FEA5  bl 0x82cd92b0
	ctx.lr = 0x82C59410;
	sub_82CD92B0(ctx, base);
	// 82C59410: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82C59414: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C59418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5941C: 419A000C  beq cr6, 0x82c59428
	if ctx.cr[6].eq {
	pc = 0x82C59428; continue 'dispatch;
	}
	// 82C59420: 4807FE91  bl 0x82cd92b0
	ctx.lr = 0x82C59424;
	sub_82CD92B0(ctx, base);
	// 82C59424: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C59428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5942C: 4BBEC385  bl 0x828457b0
	ctx.lr = 0x82C59430;
	sub_828457B0(ctx, base);
	// 82C59430: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C59434: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C59438: 48050024  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C59440 size=96
    let mut pc: u32 = 0x82C59440;
    'dispatch: loop {
        match pc {
            0x82C59440 => {
    //   block [0x82C59440..0x82C594A0)
	// 82C59440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C59448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5944C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C59450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C59458: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5945C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C59460: 394BCFB8  addi r10, r11, -0x3048
	ctx.r[10].s64 = ctx.r[11].s64 + -12360;
	// 82C59464: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C59468: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C5946C: 4BFF4735  bl 0x82c4dba0
	ctx.lr = 0x82C59470;
	sub_82C4DBA0(ctx, base);
	// 82C59470: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C59474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C59478: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5947C: 419A000C  beq cr6, 0x82c59488
	if ctx.cr[6].eq {
	pc = 0x82C59488; continue 'dispatch;
	}
	// 82C59480: 4BBEC331  bl 0x828457b0
	ctx.lr = 0x82C59484;
	sub_828457B0(ctx, base);
	// 82C59484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C59488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5948C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C59490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C59494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C59498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5949C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C594A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C594A0 size=120
    let mut pc: u32 = 0x82C594A0;
    'dispatch: loop {
        match pc {
            0x82C594A0 => {
    //   block [0x82C594A0..0x82C59518)
	// 82C594A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C594A4: 4804FF69  bl 0x82ca940c
	ctx.lr = 0x82C594A8;
	sub_82CA93D0(ctx, base);
	// 82C594A8: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C59518 size=696
    let mut pc: u32 = 0x82C59518;
    'dispatch: loop {
        match pc {
            0x82C59518 => {
    //   block [0x82C59518..0x82C597D0)
	// 82C59518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5951C: 4804FEF1  bl 0x82ca940c
	ctx.lr = 0x82C59520;
	sub_82CA93D0(ctx, base);
	// 82C59520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59524: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C59528: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C5952C: 897D02B2  lbz r11, 0x2b2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C59530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59534: 419A0024  beq cr6, 0x82c59558
	if ctx.cr[6].eq {
	pc = 0x82C59558; continue 'dispatch;
	}
	// 82C59538: 83FD0154  lwz r31, 0x154(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 82C5953C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59540: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C59544: 419A0158  beq cr6, 0x82c5969c
	if ctx.cr[6].eq {
	pc = 0x82C5969C; continue 'dispatch;
	}
	// 82C59548: 4BFFF561  bl 0x82c58aa8
	ctx.lr = 0x82C5954C;
	sub_82C58AA8(ctx, base);
	// 82C5954C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C59550: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C59554: 48000148  b 0x82c5969c
	pc = 0x82C5969C; continue 'dispatch;
	// 82C59558: 83FD0150  lwz r31, 0x150(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5955C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C59560: 4807FF91  bl 0x82cd94f0
	ctx.lr = 0x82C59564;
	sub_82CD94F0(ctx, base);
	// 82C59564: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5956C: 419A012C  beq cr6, 0x82c59698
	if ctx.cr[6].eq {
	pc = 0x82C59698; continue 'dispatch;
	}
	// 82C59570: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C59574: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C59578: 409A005C  bne cr6, 0x82c595d4
	if !ctx.cr[6].eq {
	pc = 0x82C595D4; continue 'dispatch;
	}
	// 82C5957C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59584: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59588: 409A0024  bne cr6, 0x82c595ac
	if !ctx.cr[6].eq {
	pc = 0x82C595AC; continue 'dispatch;
	}
	// 82C5958C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59590: 409A0010  bne cr6, 0x82c595a0
	if !ctx.cr[6].eq {
	pc = 0x82C595A0; continue 'dispatch;
	}
	// 82C59594: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C59598: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C5959C: 4800002C  b 0x82c595c8
	pc = 0x82C595C8; continue 'dispatch;
	// 82C595A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C595A4: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C595A8: 48000020  b 0x82c595c8
	pc = 0x82C595C8; continue 'dispatch;
	// 82C595AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C595B0: 409A0010  bne cr6, 0x82c595c0
	if !ctx.cr[6].eq {
	pc = 0x82C595C0; continue 'dispatch;
	}
	// 82C595B4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C595B8: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C595BC: 4800000C  b 0x82c595c8
	pc = 0x82C595C8; continue 'dispatch;
	// 82C595C0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C595C4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C595C8: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C595CC: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C595D0: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C595D4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C595D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C595DC: 409A0010  bne cr6, 0x82c595ec
	if !ctx.cr[6].eq {
	pc = 0x82C595EC; continue 'dispatch;
	}
	// 82C595E0: 817D0284  lwz r11, 0x284(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C595E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C595E8: 917D0284  stw r11, 0x284(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(644 as u32), ctx.r[11].u32 ) };
	// 82C595EC: 817D0188  lwz r11, 0x188(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(392 as u32) ) } as u64;
	// 82C595F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C595F4: 419A0068  beq cr6, 0x82c5965c
	if ctx.cr[6].eq {
	pc = 0x82C5965C; continue 'dispatch;
	}
	// 82C595F8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C595FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59600: 409A0080  bne cr6, 0x82c59680
	if !ctx.cr[6].eq {
	pc = 0x82C59680; continue 'dispatch;
	}
	// 82C59604: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59608: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5960C: 409A0074  bne cr6, 0x82c59680
	if !ctx.cr[6].eq {
	pc = 0x82C59680; continue 'dispatch;
	}
	// 82C59610: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59614: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59618: 409A0068  bne cr6, 0x82c59680
	if !ctx.cr[6].eq {
	pc = 0x82C59680; continue 'dispatch;
	}
	// 82C5961C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C59620: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59624: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59628: 409A0018  bne cr6, 0x82c59640
	if !ctx.cr[6].eq {
	pc = 0x82C59640; continue 'dispatch;
	}
	// 82C5962C: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C59630: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C59634: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C59638: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C5963C: 48000044  b 0x82c59680
	pc = 0x82C59680; continue 'dispatch;
	// 82C59640: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59644: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C59648: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C5964C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59650: 90690000  stw r3, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C59654: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C59658: 48000028  b 0x82c59680
	pc = 0x82C59680; continue 'dispatch;
	// 82C5965C: 817D0288  lwz r11, 0x288(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C59660: E95D0270  ld r10, 0x270(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(624 as u32) ) };
	// 82C59664: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C59668: 917D0288  stw r11, 0x288(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(648 as u32), ctx.r[11].u32 ) };
	// 82C5966C: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C59670: 792BF842  rldicl r11, r9, 0x3f, 1
	ctx.r[11].u64 = ctx.r[9].u64 & 0x0000000000000001u64;
	// 82C59674: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C59678: F91D0270  std r8, 0x270(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(624 as u32), ctx.r[8].u64 ) };
	// 82C5967C: 4BBEC135  bl 0x828457b0
	ctx.lr = 0x82C59680;
	sub_828457B0(ctx, base);
	// 82C59680: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C59684: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59688: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C5968C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C59690: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C59694: 409AFEDC  bne cr6, 0x82c59570
	if !ctx.cr[6].eq {
	pc = 0x82C59570; continue 'dispatch;
	}
	// 82C59698: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C5969C: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C596A0: 3BFD0024  addi r31, r29, 0x24
	ctx.r[31].s64 = ctx.r[29].s64 + 36;
	// 82C596A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C596A8: 419A0120  beq cr6, 0x82c597c8
	if ctx.cr[6].eq {
	pc = 0x82C597C8; continue 'dispatch;
	}
	// 82C596AC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C596B0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C596B4: 409A005C  bne cr6, 0x82c59710
	if !ctx.cr[6].eq {
	pc = 0x82C59710; continue 'dispatch;
	}
	// 82C596B8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C596BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C596C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C596C4: 409A0024  bne cr6, 0x82c596e8
	if !ctx.cr[6].eq {
	pc = 0x82C596E8; continue 'dispatch;
	}
	// 82C596C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C596CC: 409A0010  bne cr6, 0x82c596dc
	if !ctx.cr[6].eq {
	pc = 0x82C596DC; continue 'dispatch;
	}
	// 82C596D0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C596D4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C596D8: 4800002C  b 0x82c59704
	pc = 0x82C59704; continue 'dispatch;
	// 82C596DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C596E0: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C596E4: 48000020  b 0x82c59704
	pc = 0x82C59704; continue 'dispatch;
	// 82C596E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C596EC: 409A0010  bne cr6, 0x82c596fc
	if !ctx.cr[6].eq {
	pc = 0x82C596FC; continue 'dispatch;
	}
	// 82C596F0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C596F4: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C596F8: 4800000C  b 0x82c59704
	pc = 0x82C59704; continue 'dispatch;
	// 82C596FC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C59700: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C59704: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C59708: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C5970C: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C59710: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59718: 409A0010  bne cr6, 0x82c59728
	if !ctx.cr[6].eq {
	pc = 0x82C59728; continue 'dispatch;
	}
	// 82C5971C: 817D0284  lwz r11, 0x284(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C59720: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C59724: 917D0284  stw r11, 0x284(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(644 as u32), ctx.r[11].u32 ) };
	// 82C59728: 817D0188  lwz r11, 0x188(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(392 as u32) ) } as u64;
	// 82C5972C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59730: 419A0068  beq cr6, 0x82c59798
	if ctx.cr[6].eq {
	pc = 0x82C59798; continue 'dispatch;
	}
	// 82C59734: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C59738: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5973C: 409A0080  bne cr6, 0x82c597bc
	if !ctx.cr[6].eq {
	pc = 0x82C597BC; continue 'dispatch;
	}
	// 82C59740: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59744: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59748: 409A0074  bne cr6, 0x82c597bc
	if !ctx.cr[6].eq {
	pc = 0x82C597BC; continue 'dispatch;
	}
	// 82C5974C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59750: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59754: 409A0068  bne cr6, 0x82c597bc
	if !ctx.cr[6].eq {
	pc = 0x82C597BC; continue 'dispatch;
	}
	// 82C59758: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C5975C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59760: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59764: 409A0018  bne cr6, 0x82c5977c
	if !ctx.cr[6].eq {
	pc = 0x82C5977C; continue 'dispatch;
	}
	// 82C59768: 93C30004  stw r30, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C5976C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C59770: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C59774: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C59778: 48000044  b 0x82c597bc
	pc = 0x82C597BC; continue 'dispatch;
	// 82C5977C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59780: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C59784: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C59788: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5978C: 90690000  stw r3, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C59790: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C59794: 48000028  b 0x82c597bc
	pc = 0x82C597BC; continue 'dispatch;
	// 82C59798: 817D0288  lwz r11, 0x288(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C5979C: E95D0270  ld r10, 0x270(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(624 as u32) ) };
	// 82C597A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C597A4: 917D0288  stw r11, 0x288(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(648 as u32), ctx.r[11].u32 ) };
	// 82C597A8: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C597AC: 792BF842  rldicl r11, r9, 0x3f, 1
	ctx.r[11].u64 = ctx.r[9].u64 & 0x0000000000000001u64;
	// 82C597B0: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C597B4: F91D0270  std r8, 0x270(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(624 as u32), ctx.r[8].u64 ) };
	// 82C597B8: 4BBEBFF9  bl 0x828457b0
	ctx.lr = 0x82C597BC;
	sub_828457B0(ctx, base);
	// 82C597BC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C597C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C597C4: 409AFEE8  bne cr6, 0x82c596ac
	if !ctx.cr[6].eq {
	pc = 0x82C596AC; continue 'dispatch;
	}
	// 82C597C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C597CC: 4804FC90  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C597D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C597D0 size=684
    let mut pc: u32 = 0x82C597D0;
    'dispatch: loop {
        match pc {
            0x82C597D0 => {
    //   block [0x82C597D0..0x82C59A7C)
	// 82C597D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C597D4: 4804FC35  bl 0x82ca9408
	ctx.lr = 0x82C597D8;
	sub_82CA93D0(ctx, base);
	// 82C597D8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C597DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C597E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C597E4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C597E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C597EC: 83DF0150  lwz r30, 0x150(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C597F0: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C597F4: 4807FF65  bl 0x82cd9758
	ctx.lr = 0x82C597F8;
	sub_82CD9758(ctx, base);
	// 82C597F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C597FC: 4198000C  blt cr6, 0x82c59808
	if ctx.cr[6].lt {
	pc = 0x82C59808; continue 'dispatch;
	}
	// 82C59800: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C59804: F95F0268  std r10, 0x268(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[10].u64 ) };
	// 82C59808: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5980C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59810: 4099013C  ble cr6, 0x82c5994c
	if !ctx.cr[6].gt {
	pc = 0x82C5994C; continue 'dispatch;
	}
	// 82C59814: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59818: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5981C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C59820: 409A005C  bne cr6, 0x82c5987c
	if !ctx.cr[6].eq {
	pc = 0x82C5987C; continue 'dispatch;
	}
	// 82C59824: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59828: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5982C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C59830: 409A0024  bne cr6, 0x82c59854
	if !ctx.cr[6].eq {
	pc = 0x82C59854; continue 'dispatch;
	}
	// 82C59834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59838: 409A0010  bne cr6, 0x82c59848
	if !ctx.cr[6].eq {
	pc = 0x82C59848; continue 'dispatch;
	}
	// 82C5983C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C59840: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C59844: 4800002C  b 0x82c59870
	pc = 0x82C59870; continue 'dispatch;
	// 82C59848: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5984C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C59850: 48000020  b 0x82c59870
	pc = 0x82C59870; continue 'dispatch;
	// 82C59854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59858: 409A0010  bne cr6, 0x82c59868
	if !ctx.cr[6].eq {
	pc = 0x82C59868; continue 'dispatch;
	}
	// 82C5985C: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C59860: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C59864: 4800000C  b 0x82c59870
	pc = 0x82C59870; continue 'dispatch;
	// 82C59868: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5986C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C59870: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C59874: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C59878: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C5987C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59884: 409A0010  bne cr6, 0x82c59894
	if !ctx.cr[6].eq {
	pc = 0x82C59894; continue 'dispatch;
	}
	// 82C59888: 817F0284  lwz r11, 0x284(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C5988C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C59890: 917F0284  stw r11, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[11].u32 ) };
	// 82C59894: 817F0188  lwz r11, 0x188(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) } as u64;
	// 82C59898: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5989C: 419A0068  beq cr6, 0x82c59904
	if ctx.cr[6].eq {
	pc = 0x82C59904; continue 'dispatch;
	}
	// 82C598A0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C598A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C598A8: 409A0080  bne cr6, 0x82c59928
	if !ctx.cr[6].eq {
	pc = 0x82C59928; continue 'dispatch;
	}
	// 82C598AC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C598B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C598B4: 409A0074  bne cr6, 0x82c59928
	if !ctx.cr[6].eq {
	pc = 0x82C59928; continue 'dispatch;
	}
	// 82C598B8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C598BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C598C0: 409A0068  bne cr6, 0x82c59928
	if !ctx.cr[6].eq {
	pc = 0x82C59928; continue 'dispatch;
	}
	// 82C598C4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C598C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C598CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C598D0: 409A0018  bne cr6, 0x82c598e8
	if !ctx.cr[6].eq {
	pc = 0x82C598E8; continue 'dispatch;
	}
	// 82C598D4: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C598D8: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C598DC: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C598E0: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C598E4: 48000044  b 0x82c59928
	pc = 0x82C59928; continue 'dispatch;
	// 82C598E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C598EC: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C598F0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C598F4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C598F8: 90690000  stw r3, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82C598FC: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C59900: 48000028  b 0x82c59928
	pc = 0x82C59928; continue 'dispatch;
	// 82C59904: 817F0288  lwz r11, 0x288(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C59908: E95F0270  ld r10, 0x270(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) };
	// 82C5990C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C59910: 917F0288  stw r11, 0x288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), ctx.r[11].u32 ) };
	// 82C59914: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C59918: 792BF842  rldicl r11, r9, 0x3f, 1
	ctx.r[11].u64 = ctx.r[9].u64 & 0x0000000000000001u64;
	// 82C5991C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C59920: F91F0270  std r8, 0x270(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[8].u64 ) };
	// 82C59924: 4BBEBE8D  bl 0x828457b0
	ctx.lr = 0x82C59928;
	sub_828457B0(ctx, base);
	// 82C59928: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5992C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C59930: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C59934: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C59938: 5549003E  slwi r9, r10, 0
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C5993C: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C59940: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C59944: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C59948: 4199FECC  bgt cr6, 0x82c59814
	if ctx.cr[6].gt {
	pc = 0x82C59814; continue 'dispatch;
	}
	// 82C5994C: 3B9F0024  addi r28, r31, 0x24
	ctx.r[28].s64 = ctx.r[31].s64 + 36;
	// 82C59950: 83FF0024  lwz r31, 0x24(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C59954: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C59958: 419A011C  beq cr6, 0x82c59a74
	if ctx.cr[6].eq {
	pc = 0x82C59A74; continue 'dispatch;
	}
	// 82C5995C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C59960: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82C59964: 40980110  bge cr6, 0x82c59a74
	if !ctx.cr[6].lt {
	pc = 0x82C59A74; continue 'dispatch;
	}
	// 82C59968: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 82C5996C: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82C59970: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C59974: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82C59978: 48050039  bl 0x82ca99b0
	ctx.lr = 0x82C5997C;
	sub_82CA99B0(ctx, base);
	// 82C5997C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59980: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C59984: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C59988: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C5998C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C59990: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82C59994: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C59998: 4807FAF9  bl 0x82cd9490
	ctx.lr = 0x82C5999C;
	sub_82CD9490(ctx, base);
	// 82C5999C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C599A0: 7F09E040  cmplw cr6, r9, r28
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C599A4: 409A005C  bne cr6, 0x82c59a00
	if !ctx.cr[6].eq {
	pc = 0x82C59A00; continue 'dispatch;
	}
	// 82C599A8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C599AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C599B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C599B4: 409A0024  bne cr6, 0x82c599d8
	if !ctx.cr[6].eq {
	pc = 0x82C599D8; continue 'dispatch;
	}
	// 82C599B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C599BC: 409A0010  bne cr6, 0x82c599cc
	if !ctx.cr[6].eq {
	pc = 0x82C599CC; continue 'dispatch;
	}
	// 82C599C0: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C599C4: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C599C8: 4800002C  b 0x82c599f4
	pc = 0x82C599F4; continue 'dispatch;
	// 82C599CC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C599D0: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C599D4: 48000020  b 0x82c599f4
	pc = 0x82C599F4; continue 'dispatch;
	// 82C599D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C599DC: 409A0010  bne cr6, 0x82c599ec
	if !ctx.cr[6].eq {
	pc = 0x82C599EC; continue 'dispatch;
	}
	// 82C599E0: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C599E4: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C599E8: 4800000C  b 0x82c599f4
	pc = 0x82C599F4; continue 'dispatch;
	// 82C599EC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C599F0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C599F4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C599F8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C599FC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C59A00: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C59A04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59A08: 409A0054  bne cr6, 0x82c59a5c
	if !ctx.cr[6].eq {
	pc = 0x82C59A5C; continue 'dispatch;
	}
	// 82C59A0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59A10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59A14: 409A0048  bne cr6, 0x82c59a5c
	if !ctx.cr[6].eq {
	pc = 0x82C59A5C; continue 'dispatch;
	}
	// 82C59A18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59A1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59A20: 409A003C  bne cr6, 0x82c59a5c
	if !ctx.cr[6].eq {
	pc = 0x82C59A5C; continue 'dispatch;
	}
	// 82C59A24: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C59A28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59A2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59A30: 409A0014  bne cr6, 0x82c59a44
	if !ctx.cr[6].eq {
	pc = 0x82C59A44; continue 'dispatch;
	}
	// 82C59A34: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C59A38: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C59A3C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C59A40: 48000018  b 0x82c59a58
	pc = 0x82C59A58; continue 'dispatch;
	// 82C59A44: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59A48: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C59A4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C59A50: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59A54: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C59A58: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C59A5C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C59A60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C59A64: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C59A68: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59A6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C59A70: 409AFEEC  bne cr6, 0x82c5995c
	if !ctx.cr[6].eq {
	pc = 0x82C5995C; continue 'dispatch;
	}
	// 82C59A74: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82C59A78: 4804F9E0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C59A80 size=392
    let mut pc: u32 = 0x82C59A80;
    'dispatch: loop {
        match pc {
            0x82C59A80 => {
    //   block [0x82C59A80..0x82C59C08)
	// 82C59A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59A84: 4804F985  bl 0x82ca9408
	ctx.lr = 0x82C59A88;
	sub_82CA93D0(ctx, base);
	// 82C59A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C59A90: 897F02B4  lbz r11, 0x2b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 82C59A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59A98: 409A0168  bne cr6, 0x82c59c00
	if !ctx.cr[6].eq {
	pc = 0x82C59C00; continue 'dispatch;
	}
	// 82C59A9C: 811F0278  lwz r8, 0x278(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 82C59AA0: 788A0020  clrldi r10, r4, 0x20
	ctx.r[10].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82C59AA4: 80FF027C  lwz r7, 0x27c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(636 as u32) ) } as u64;
	// 82C59AA8: C19F02A0  lfs f12, 0x2a0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(672 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C59AAC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82C59AB0: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C59AB4: FD005E9C  fcfid f8, f11
	ctx.f[8].f64 = (ctx.f[11].s64 as f64);
	// 82C59AB8: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82C59ABC: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82C59AC0: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C59AC4: FCE0569C  fcfid f7, f10
	ctx.f[7].f64 = (ctx.f[10].s64 as f64);
	// 82C59AC8: F8E10050  std r7, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u64 ) };
	// 82C59ACC: C9210050  lfd f9, 0x50(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C59AD0: FCA04E9C  fcfid f5, f9
	ctx.f[5].f64 = (ctx.f[9].s64 as f64);
	// 82C59AD4: FC803818  frsp f4, f7
	ctx.f[4].f64 = (ctx.f[7].f64 as f32) as f64;
	// 82C59AD8: C0060C4C  lfs f0, 0xc4c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59ADC: FC602818  frsp f3, f5
	ctx.f[3].f64 = (ctx.f[5].f64 as f32) as f64;
	// 82C59AE0: 3CA08204  lis r5, -0x7dfc
	ctx.r[5].s64 = -2113667072;
	// 82C59AE4: FCC04018  frsp f6, f8
	ctx.f[6].f64 = (ctx.f[8].f64 as f32) as f64;
	// 82C59AE8: E97F0268  ld r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) };
	// 82C59AEC: 83DF0154  lwz r30, 0x154(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82C59AF0: C1A5BE14  lfs f13, -0x41ec(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-16876 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C59AF4: EC440332  fmuls f2, f4, f12
	ctx.f[2].f64 = (((ctx.f[4].f64 * ctx.f[12].f64) as f32) as f64);
	// 82C59AF8: EC230032  fmuls f1, f3, f0
	ctx.f[1].f64 = (((ctx.f[3].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C59AFC: EC0201B2  fmuls f0, f2, f6
	ctx.f[0].f64 = (((ctx.f[2].f64 * ctx.f[6].f64) as f32) as f64);
	// 82C59B00: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C59B04: ED810372  fmuls f12, f1, f13
	ctx.f[12].f64 = (((ctx.f[1].f64 * ctx.f[13].f64) as f32) as f64);
	// 82C59B08: FD606E5E  fctidz f11, f13
	ctx.f[11].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 82C59B0C: D9610050  stfd f11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[11].u64 ) };
	// 82C59B10: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C59B14: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C59B18: FD40665E  fctidz f10, f12
	ctx.f[10].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 82C59B1C: D9410050  stfd f10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[10].u64 ) };
	// 82C59B20: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C59B24: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C59B28: F89F0268  std r4, 0x268(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[4].u64 ) };
	// 82C59B2C: 419A00D4  beq cr6, 0x82c59c00
	if ctx.cr[6].eq {
	pc = 0x82C59C00; continue 'dispatch;
	}
	// 82C59B30: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C59B34: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59B38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59B3C: 419A00A4  beq cr6, 0x82c59be0
	if ctx.cr[6].eq {
	pc = 0x82C59BE0; continue 'dispatch;
	}
	// 82C59B40: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C59B44: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82C59B48: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59B4C: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82C59B50: 7F1D4840  cmplw cr6, r29, r9
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C59B54: 40990008  ble cr6, 0x82c59b5c
	if !ctx.cr[6].gt {
	pc = 0x82C59B5C; continue 'dispatch;
	}
	// 82C59B58: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82C59B5C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59B60: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C59B64: 7FABE850  subf r29, r11, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82C59B68: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C59B6C: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C59B70: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C59B74: 409A0084  bne cr6, 0x82c59bf8
	if !ctx.cr[6].eq {
	pc = 0x82C59BF8; continue 'dispatch;
	}
	// 82C59B78: 5524003E  slwi r4, r9, 0
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82C59B7C: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C59B80: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59B84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59B88: 409A0010  bne cr6, 0x82c59b98
	if !ctx.cr[6].eq {
	pc = 0x82C59B98; continue 'dispatch;
	}
	// 82C59B8C: 817F0284  lwz r11, 0x284(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C59B90: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C59B94: 917F0284  stw r11, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[11].u32 ) };
	// 82C59B98: 807F0188  lwz r3, 0x188(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) } as u64;
	// 82C59B9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C59BA0: 419A0010  beq cr6, 0x82c59bb0
	if ctx.cr[6].eq {
	pc = 0x82C59BB0; continue 'dispatch;
	}
	// 82C59BA4: 4BFFE36D  bl 0x82c57f10
	ctx.lr = 0x82C59BA8;
	sub_82C57F10(ctx, base);
	// 82C59BA8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C59BAC: 4800004C  b 0x82c59bf8
	pc = 0x82C59BF8; continue 'dispatch;
	// 82C59BB0: 817F0288  lwz r11, 0x288(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C59BB4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C59BB8: E95F0270  ld r10, 0x270(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) };
	// 82C59BBC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C59BC0: 917F0288  stw r11, 0x288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), ctx.r[11].u32 ) };
	// 82C59BC4: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C59BC8: 792BF842  rldicl r11, r9, 0x3f, 1
	ctx.r[11].u64 = ctx.r[9].u64 & 0x0000000000000001u64;
	// 82C59BCC: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C59BD0: F91F0270  std r8, 0x270(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[8].u64 ) };
	// 82C59BD4: 4BBEBBDD  bl 0x828457b0
	ctx.lr = 0x82C59BD8;
	sub_828457B0(ctx, base);
	// 82C59BD8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C59BDC: 4800001C  b 0x82c59bf8
	pc = 0x82C59BF8; continue 'dispatch;
	// 82C59BE0: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C59BE4: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82C59BE8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C59BEC: 909E0000  stw r4, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C59BF0: 419A0010  beq cr6, 0x82c59c00
	if ctx.cr[6].eq {
	pc = 0x82C59C00; continue 'dispatch;
	}
	// 82C59BF4: 4BFB514D  bl 0x82c0ed40
	ctx.lr = 0x82C59BF8;
	sub_82C0ED40(ctx, base);
	// 82C59BF8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C59BFC: 409AFF38  bne cr6, 0x82c59b34
	if !ctx.cr[6].eq {
	pc = 0x82C59B34; continue 'dispatch;
	}
	// 82C59C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C59C04: 4804F854  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C59C08 size=64
    let mut pc: u32 = 0x82C59C08;
    'dispatch: loop {
        match pc {
            0x82C59C08 => {
    //   block [0x82C59C08..0x82C59C48)
	// 82C59C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C59C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59C14: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82C59C18: 38840018  addi r4, r4, 0x18
	ctx.r[4].s64 = ctx.r[4].s64 + 24;
	// 82C59C1C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C59C20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C59C24: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C59C28: 4BFF35B1  bl 0x82c4d1d8
	ctx.lr = 0x82C59C2C;
	sub_82C4D1D8(ctx, base);
	// 82C59C2C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C59C30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C59C34: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C59C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C59C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C59C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C59C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C59C48 size=64
    let mut pc: u32 = 0x82C59C48;
    'dispatch: loop {
        match pc {
            0x82C59C48 => {
    //   block [0x82C59C48..0x82C59C88)
	// 82C59C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C59C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59C54: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82C59C58: 38840018  addi r4, r4, 0x18
	ctx.r[4].s64 = ctx.r[4].s64 + 24;
	// 82C59C5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C59C60: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C59C64: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C59C68: 4BFF3571  bl 0x82c4d1d8
	ctx.lr = 0x82C59C6C;
	sub_82C4D1D8(ctx, base);
	// 82C59C6C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C59C70: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C59C74: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C59C78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C59C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C59C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C59C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C59C88 size=116
    let mut pc: u32 = 0x82C59C88;
    'dispatch: loop {
        match pc {
            0x82C59C88 => {
    //   block [0x82C59C88..0x82C59CFC)
	// 82C59C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59C8C: 4804F781  bl 0x82ca940c
	ctx.lr = 0x82C59C90;
	sub_82CA93D0(ctx, base);
	// 82C59C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59C94: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C59C98: 3BE40018  addi r31, r4, 0x18
	ctx.r[31].s64 = ctx.r[4].s64 + 24;
	// 82C59C9C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82C59CA0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C59CA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C59CA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C59CAC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C59CB0: 4BFF3529  bl 0x82c4d1d8
	ctx.lr = 0x82C59CB4;
	sub_82C4D1D8(ctx, base);
	// 82C59CB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C59CB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C59CBC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C59CC0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C59CC4: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C59CC8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C59CCC: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C59CD0: 4BFF3D69  bl 0x82c4da38
	ctx.lr = 0x82C59CD4;
	sub_82C4DA38(ctx, base);
	// 82C59CD4: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82C59CD8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C59CDC: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C59CE0: 4BFF3D59  bl 0x82c4da38
	ctx.lr = 0x82C59CE4;
	sub_82C4DA38(ctx, base);
	// 82C59CE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C59CE8: 480015D9  bl 0x82c5b2c0
	ctx.lr = 0x82C59CEC;
	sub_82C5B2C0(ctx, base);
	// 82C59CEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C59CF0: 4BBEBAC1  bl 0x828457b0
	ctx.lr = 0x82C59CF4;
	sub_828457B0(ctx, base);
	// 82C59CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C59CF8: 4804F764  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C59D00 size=144
    let mut pc: u32 = 0x82C59D00;
    'dispatch: loop {
        match pc {
            0x82C59D00 => {
    //   block [0x82C59D00..0x82C59D90)
	// 82C59D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59D04: 4804F709  bl 0x82ca940c
	ctx.lr = 0x82C59D08;
	sub_82CA93D0(ctx, base);
	// 82C59D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59D0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C59D10: 3863011C  addi r3, r3, 0x11c
	ctx.r[3].s64 = ctx.r[3].s64 + 284;
	// 82C59D14: 83FE0034  lwz r31, 0x34(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C59D18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C59D1C: 4BFB803D  bl 0x82c11d58
	ctx.lr = 0x82C59D20;
	sub_82C11D58(ctx, base);
	// 82C59D20: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C59D24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59D28: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C59D2C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C59D30: 4E800421  bctrl
	ctx.lr = 0x82C59D34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C59D34: 893F02B2  lbz r9, 0x2b2(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C59D38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C59D3C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C59D40: 419A001C  beq cr6, 0x82c59d5c
	if ctx.cr[6].eq {
	pc = 0x82C59D5C; continue 'dispatch;
	}
	// 82C59D44: 807F0154  lwz r3, 0x154(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82C59D48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C59D4C: 419A0018  beq cr6, 0x82c59d64
	if ctx.cr[6].eq {
	pc = 0x82C59D64; continue 'dispatch;
	}
	// 82C59D50: 4BBEBA61  bl 0x828457b0
	ctx.lr = 0x82C59D54;
	sub_828457B0(ctx, base);
	// 82C59D54: 93BF0154  stw r29, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[29].u32 ) };
	// 82C59D58: 4800000C  b 0x82c59d64
	pc = 0x82C59D64; continue 'dispatch;
	// 82C59D5C: 387F0150  addi r3, r31, 0x150
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	// 82C59D60: 4BFFF679  bl 0x82c593d8
	ctx.lr = 0x82C59D64;
	sub_82C593D8(ctx, base);
	// 82C59D64: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C59D68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C59D6C: 419A0018  beq cr6, 0x82c59d84
	if ctx.cr[6].eq {
	pc = 0x82C59D84; continue 'dispatch;
	}
	// 82C59D70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59D74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C59D78: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C59D7C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C59D80: 4E800421  bctrl
	ctx.lr = 0x82C59D84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C59D84: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82C59D88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C59D8C: 4804F6D0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C59D90 size=124
    let mut pc: u32 = 0x82C59D90;
    'dispatch: loop {
        match pc {
            0x82C59D90 => {
    //   block [0x82C59D90..0x82C59E0C)
	// 82C59D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59D94: 4804F679  bl 0x82ca940c
	ctx.lr = 0x82C59D98;
	sub_82CA93D0(ctx, base);
	// 82C59D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59D9C: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C59DA0: 7FA45851  subf. r29, r4, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C59DA4: 41820060  beq 0x82c59e04
	if ctx.cr[0].eq {
	pc = 0x82C59E04; continue 'dispatch;
	}
	// 82C59DA8: 83E3011C  lwz r31, 0x11c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(284 as u32) ) } as u64;
	// 82C59DAC: 3BC3011C  addi r30, r3, 0x11c
	ctx.r[30].s64 = ctx.r[3].s64 + 284;
	// 82C59DB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C59DB4: 419A0050  beq cr6, 0x82c59e04
	if ctx.cr[6].eq {
	pc = 0x82C59E04; continue 'dispatch;
	}
	// 82C59DB8: 897F02B2  lbz r11, 0x2b2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C59DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C59DC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C59DC4: 419A0010  beq cr6, 0x82c59dd4
	if ctx.cr[6].eq {
	pc = 0x82C59DD4; continue 'dispatch;
	}
	// 82C59DC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C59DCC: 4BFFFCB5  bl 0x82c59a80
	ctx.lr = 0x82C59DD0;
	sub_82C59A80(ctx, base);
	// 82C59DD0: 48000008  b 0x82c59dd8
	pc = 0x82C59DD8; continue 'dispatch;
	// 82C59DD4: 4BFFF9FD  bl 0x82c597d0
	ctx.lr = 0x82C59DD8;
	sub_82C597D0(ctx, base);
	// 82C59DD8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59DDC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C59DE0: 409A0024  bne cr6, 0x82c59e04
	if !ctx.cr[6].eq {
	pc = 0x82C59E04; continue 'dispatch;
	}
	// 82C59DE4: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C59DE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C59DEC: 419A0018  beq cr6, 0x82c59e04
	if ctx.cr[6].eq {
	pc = 0x82C59E04; continue 'dispatch;
	}
	// 82C59DF0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C59DF4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C59DF8: 409A000C  bne cr6, 0x82c59e04
	if !ctx.cr[6].eq {
	pc = 0x82C59E04; continue 'dispatch;
	}
	// 82C59DFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C59E00: 409AFFB8  bne cr6, 0x82c59db8
	if !ctx.cr[6].eq {
	pc = 0x82C59DB8; continue 'dispatch;
	}
	// 82C59E04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C59E08: 4804F654  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C59E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C59E10 size=588
    let mut pc: u32 = 0x82C59E10;
    'dispatch: loop {
        match pc {
            0x82C59E10 => {
    //   block [0x82C59E10..0x82C5A05C)
	// 82C59E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C59E14: 4804F5F5  bl 0x82ca9408
	ctx.lr = 0x82C59E18;
	sub_82CA93D0(ctx, base);
	// 82C59E18: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82C59E1C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C59E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C59E24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C59E28: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C59E2C: 392BCFE0  addi r9, r11, -0x3020
	ctx.r[9].s64 = ctx.r[11].s64 + -12320;
	// 82C59E30: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C59E34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C59E38: 390ACFB8  addi r8, r10, -0x3048
	ctx.r[8].s64 = ctx.r[10].s64 + -12360;
	// 82C59E3C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C59E40: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82C59E44: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82C59E48: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C59E4C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C59E50: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C59E54: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82C59E58: 911F0018  stw r8, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82C59E5C: 4BFF3C4D  bl 0x82c4daa8
	ctx.lr = 0x82C59E60;
	sub_82C4DAA8(ctx, base);
	// 82C59E60: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C59E64: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82C59E68: 38C7CFB8  addi r6, r7, -0x3048
	ctx.r[6].s64 = ctx.r[7].s64 + -12360;
	// 82C59E6C: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82C59E70: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82C59E74: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82C59E78: 48001B59  bl 0x82c5b9d0
	ctx.lr = 0x82C59E7C;
	sub_82C5B9D0(ctx, base);
	// 82C59E7C: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 82C59E80: 48001B51  bl 0x82c5b9d0
	ctx.lr = 0x82C59E84;
	sub_82C5B9D0(ctx, base);
	// 82C59E84: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C59E88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C59E8C: 3885CD98  addi r4, r5, -0x3268
	ctx.r[4].s64 = ctx.r[5].s64 + -12904;
	// 82C59E90: 3C608200  lis r3, -0x7e00
	ctx.r[3].s64 = -2113929216;
	// 82C59E94: 917F0148  stw r11, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 82C59E98: 909F0144  stw r4, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[4].u32 ) };
	// 82C59E9C: 3B9F0230  addi r28, r31, 0x230
	ctx.r[28].s64 = ctx.r[31].s64 + 560;
	// 82C59EA0: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 82C59EA4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C59EA8: 93DF0150  stw r30, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[30].u32 ) };
	// 82C59EAC: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 82C59EB0: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 82C59EB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C59EB8: C3E30C14  lfs f31, 0xc14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C59EBC: 93DF0158  stw r30, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[30].u32 ) };
	// 82C59EC0: D3FF0160  stfs f31, 0x160(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 82C59EC4: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 82C59EC8: D3FF0164  stfs f31, 0x164(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 82C59ECC: 93DF0168  stw r30, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[30].u32 ) };
	// 82C59ED0: D3FF0170  stfs f31, 0x170(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), tmp.u32 ) };
	// 82C59ED4: 93DF016C  stw r30, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[30].u32 ) };
	// 82C59ED8: D3FF0174  stfs f31, 0x174(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 82C59EDC: 93DF0178  stw r30, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[30].u32 ) };
	// 82C59EE0: D3FF0180  stfs f31, 0x180(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 82C59EE4: 93DF017C  stw r30, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[30].u32 ) };
	// 82C59EE8: D3FF0184  stfs f31, 0x184(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 82C59EEC: 93DF0188  stw r30, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[30].u32 ) };
	// 82C59EF0: 93DF018C  stw r30, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[30].u32 ) };
	// 82C59EF4: 93DF0230  stw r30, 0x230(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), ctx.r[30].u32 ) };
	// 82C59EF8: 93DF0234  stw r30, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[30].u32 ) };
	// 82C59EFC: D3FF0294  stfs f31, 0x294(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), tmp.u32 ) };
	// 82C59F00: D3FF0298  stfs f31, 0x298(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), tmp.u32 ) };
	// 82C59F04: 9BDF0260  stb r30, 0x260(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[30].u8 ) };
	// 82C59F08: D3FF029C  stfs f31, 0x29c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), tmp.u32 ) };
	// 82C59F0C: FBDF0268  std r30, 0x268(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u64 ) };
	// 82C59F10: D3FF02A0  stfs f31, 0x2a0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(672 as u32), tmp.u32 ) };
	// 82C59F14: FBDF0270  std r30, 0x270(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[30].u64 ) };
	// 82C59F18: 93DF0278  stw r30, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[30].u32 ) };
	// 82C59F1C: 93DF027C  stw r30, 0x27c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(636 as u32), ctx.r[30].u32 ) };
	// 82C59F20: 93DF0280  stw r30, 0x280(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(640 as u32), ctx.r[30].u32 ) };
	// 82C59F24: 93DF0284  stw r30, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[30].u32 ) };
	// 82C59F28: 93DF0288  stw r30, 0x288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), ctx.r[30].u32 ) };
	// 82C59F2C: 93DF028C  stw r30, 0x28c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), ctx.r[30].u32 ) };
	// 82C59F30: 93DF0290  stw r30, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[30].u32 ) };
	// 82C59F34: 93DF02A4  stw r30, 0x2a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), ctx.r[30].u32 ) };
	// 82C59F38: 93DF02A8  stw r30, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[30].u32 ) };
	// 82C59F3C: 9BDF02AC  stb r30, 0x2ac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[30].u8 ) };
	// 82C59F40: 997F02AD  stb r11, 0x2ad(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(685 as u32), ctx.r[11].u8 ) };
	// 82C59F44: 9BDF02AE  stb r30, 0x2ae(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(686 as u32), ctx.r[30].u8 ) };
	// 82C59F48: 9BDF02AF  stb r30, 0x2af(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(687 as u32), ctx.r[30].u8 ) };
	// 82C59F4C: 9BDF02B0  stb r30, 0x2b0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(688 as u32), ctx.r[30].u8 ) };
	// 82C59F50: 9BDF02B1  stb r30, 0x2b1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(689 as u32), ctx.r[30].u8 ) };
	// 82C59F54: 9BDF02B2  stb r30, 0x2b2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(690 as u32), ctx.r[30].u8 ) };
	// 82C59F58: 9BDF02B3  stb r30, 0x2b3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(691 as u32), ctx.r[30].u8 ) };
	// 82C59F5C: 9BDF02B4  stb r30, 0x2b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[30].u8 ) };
	// 82C59F60: 9BDF02B5  stb r30, 0x2b5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(693 as u32), ctx.r[30].u8 ) };
	// 82C59F64: 387F0190  addi r3, r31, 0x190
	ctx.r[3].s64 = ctx.r[31].s64 + 400;
	// 82C59F68: 83AA5164  lwz r29, 0x5164(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C59F6C: 4B66066D  bl 0x822ba5d8
	ctx.lr = 0x82C59F70;
	sub_822BA5D8(ctx, base);
	// 82C59F70: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C59F74: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C59F78: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C59F7C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C59F80: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C59F84: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C59F88: 38BD00EC  addi r5, r29, 0xec
	ctx.r[5].s64 = ctx.r[29].s64 + 236;
	// 82C59F8C: 389D0104  addi r4, r29, 0x104
	ctx.r[4].s64 = ctx.r[29].s64 + 260;
	// 82C59F90: 90BF01D4  stw r5, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[5].u32 ) };
	// 82C59F94: 909F01D8  stw r4, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[4].u32 ) };
	// 82C59F98: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 82C59F9C: 90FF019C  stw r7, 0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(412 as u32), ctx.r[7].u32 ) };
	// 82C59FA0: 387F01F4  addi r3, r31, 0x1f4
	ctx.r[3].s64 = ctx.r[31].s64 + 500;
	// 82C59FA4: C00615D0  lfs f0, 0x15d0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(5584 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59FA8: 913F01A4  stw r9, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[9].u32 ) };
	// 82C59FAC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C59FB0: 911F01EC  stw r8, 0x1ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(492 as u32), ctx.r[8].u32 ) };
	// 82C59FB4: C01D013C  lfs f0, 0x13c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(316 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C59FB8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C59FBC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C59FC0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C59FC4: 917F01C4  stw r11, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[11].u32 ) };
	// 82C59FC8: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82C59FCC: 915F01C8  stw r10, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[10].u32 ) };
	// 82C59FD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C59FD4: 4B660605  bl 0x822ba5d8
	ctx.lr = 0x82C59FD8;
	sub_822BA5D8(ctx, base);
	// 82C59FD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82C59FDC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C59FE0: 913F0200  stw r9, 0x200(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(512 as u32), ctx.r[9].u32 ) };
	// 82C59FE4: 4B5C5275  bl 0x8221f258
	ctx.lr = 0x82C59FE8;
	sub_8221F258(ctx, base);
	// 82C59FE8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82C59FEC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82C59FF0: 907F01F4  stw r3, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[3].u32 ) };
	// 82C59FF4: 80BD005C  lwz r5, 0x5c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C59FF8: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82C59FFC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82C5A000: 93DF0234  stw r30, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[30].u32 ) };
	// 82C5A004: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82C5A008: C0080C18  lfs f0, 0xc18(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A00C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82C5A010: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5A014: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82C5A018: 13C03407  vcmpneb. (lvlx128) v30, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C5A01C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5A020: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C5A024: 13E03C07  vcmpneb. (lvlx128) v31, v0, v7
	tmp.u32 = ctx.r[7].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C5A028: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C5A02C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5A060 size=212
    let mut pc: u32 = 0x82C5A060;
    'dispatch: loop {
        match pc {
            0x82C5A060 => {
    //   block [0x82C5A060..0x82C5A134)
	// 82C5A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A064: 4804F3A9  bl 0x82ca940c
	ctx.lr = 0x82C5A068;
	sub_82CA93D0(ctx, base);
	// 82C5A068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A06C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C5A070: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C5A074: 897E02B2  lbz r11, 0x2b2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5A078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A07C: 419A0054  beq cr6, 0x82c5a0d0
	if ctx.cr[6].eq {
	pc = 0x82C5A0D0; continue 'dispatch;
	}
	// 82C5A080: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82C5A084: 4B5C51D5  bl 0x8221f258
	ctx.lr = 0x82C5A088;
	sub_8221F258(ctx, base);
	// 82C5A088: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5A08C: 419A0018  beq cr6, 0x82c5a0a4
	if ctx.cr[6].eq {
	pc = 0x82C5A0A4; continue 'dispatch;
	}
	// 82C5A090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5A094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A098: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5A09C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C5A0A0: 48000008  b 0x82c5a0a8
	pc = 0x82C5A0A8; continue 'dispatch;
	// 82C5A0A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C5A0A8: 93FE0154  stw r31, 0x154(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(340 as u32), ctx.r[31].u32 ) };
	// 82C5A0AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C5A0B0: 419A007C  beq cr6, 0x82c5a12c
	if ctx.cr[6].eq {
	pc = 0x82C5A12C; continue 'dispatch;
	}
	// 82C5A0B4: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C5A0B8: 387E0024  addi r3, r30, 0x24
	ctx.r[3].s64 = ctx.r[30].s64 + 36;
	// 82C5A0BC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C5A0C0: 4BFB4C81  bl 0x82c0ed40
	ctx.lr = 0x82C5A0C4;
	sub_82C0ED40(ctx, base);
	// 82C5A0C4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C5A0C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5A0CC: 4804F390  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C5A0D0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82C5A0D4: 4B5C5185  bl 0x8221f258
	ctx.lr = 0x82C5A0D8;
	sub_8221F258(ctx, base);
	// 82C5A0D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5A0DC: 419A0034  beq cr6, 0x82c5a110
	if ctx.cr[6].eq {
	pc = 0x82C5A110; continue 'dispatch;
	}
	// 82C5A0E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C5A0E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5A0E8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5A0EC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C5A0F0: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A0F4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C5A0F8: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C5A0FC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C5A100: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82C5A104: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C5A108: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82C5A10C: 48000008  b 0x82c5a114
	pc = 0x82C5A114; continue 'dispatch;
	// 82C5A110: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C5A114: 907E0150  stw r3, 0x150(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[3].u32 ) };
	// 82C5A118: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C5A11C: 80BE02A4  lwz r5, 0x2a4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(676 as u32) ) } as u64;
	// 82C5A120: 4BFFE629  bl 0x82c58748
	ctx.lr = 0x82C5A124;
	sub_82C58748(ctx, base);
	// 82C5A124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5A128: 4BFFF6A9  bl 0x82c597d0
	ctx.lr = 0x82C5A12C;
	sub_82C597D0(ctx, base);
	// 82C5A12C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5A130: 4804F32C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5A138 size=244
    let mut pc: u32 = 0x82C5A138;
    'dispatch: loop {
        match pc {
            0x82C5A138 => {
    //   block [0x82C5A138..0x82C5A22C)
	// 82C5A138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A13C: 4804F2C9  bl 0x82ca9404
	ctx.lr = 0x82C5A140;
	sub_82CA93D0(ctx, base);
	// 82C5A140: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A148: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C5A14C: 936100AC  stw r27, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[27].u32 ) };
	// 82C5A150: 817F0138  lwz r11, 0x138(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 82C5A154: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C5A158: 419A00CC  beq cr6, 0x82c5a224
	if ctx.cr[6].eq {
	pc = 0x82C5A224; continue 'dispatch;
	}
	// 82C5A15C: 3B9F012C  addi r28, r31, 0x12c
	ctx.r[28].s64 = ctx.r[31].s64 + 300;
	// 82C5A160: 38A100AC  addi r5, r1, 0xac
	ctx.r[5].s64 = ctx.r[1].s64 + 172;
	// 82C5A164: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C5A168: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5A16C: 4B71B315  bl 0x82375480
	ctx.lr = 0x82C5A170;
	sub_82375480(ctx, base);
	// 82C5A170: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5A174: 815F0130  lwz r10, 0x130(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82C5A178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A17C: 419A000C  beq cr6, 0x82c5a188
	if ctx.cr[6].eq {
	pc = 0x82C5A188; continue 'dispatch;
	}
	// 82C5A180: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C5A184: 419A0008  beq cr6, 0x82c5a18c
	if ctx.cr[6].eq {
	pc = 0x82C5A18C; continue 'dispatch;
	}
	// 82C5A188: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C5A18C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C5A190: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C5A194: 419A0090  beq cr6, 0x82c5a224
	if ctx.cr[6].eq {
	pc = 0x82C5A224; continue 'dispatch;
	}
	// 82C5A198: 3BDF011C  addi r30, r31, 0x11c
	ctx.r[30].s64 = ctx.r[31].s64 + 284;
	// 82C5A19C: 83FF011C  lwz r31, 0x11c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 82C5A1A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5A1A4: 419A006C  beq cr6, 0x82c5a210
	if ctx.cr[6].eq {
	pc = 0x82C5A210; continue 'dispatch;
	}
	// 82C5A1A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C5A1AC: 817F02A8  lwz r11, 0x2a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(680 as u32) ) } as u64;
	// 82C5A1B0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C5A1B4: 409A0030  bne cr6, 0x82c5a1e4
	if !ctx.cr[6].eq {
	pc = 0x82C5A1E4; continue 'dispatch;
	}
	// 82C5A1B8: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A1BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A1C0: 419A0024  beq cr6, 0x82c5a1e4
	if ctx.cr[6].eq {
	pc = 0x82C5A1E4; continue 'dispatch;
	}
	// 82C5A1C4: C03F0298  lfs f1, 0x298(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5A1C8: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A1CC: 4807F5E5  bl 0x82cd97b0
	ctx.lr = 0x82C5A1D0;
	sub_82CD97B0(ctx, base);
	// 82C5A1D0: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A1D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C5A1D8: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A1DC: 4807F36D  bl 0x82cd9548
	ctx.lr = 0x82C5A1E0;
	sub_82CD9548(ctx, base);
	// 82C5A1E0: 93BF02A8  stw r29, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[29].u32 ) };
	// 82C5A1E4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5A1E8: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C5A1EC: 409A0024  bne cr6, 0x82c5a210
	if !ctx.cr[6].eq {
	pc = 0x82C5A210; continue 'dispatch;
	}
	// 82C5A1F0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A1F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5A1F8: 419A0018  beq cr6, 0x82c5a210
	if ctx.cr[6].eq {
	pc = 0x82C5A210; continue 'dispatch;
	}
	// 82C5A1FC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5A200: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C5A204: 409A000C  bne cr6, 0x82c5a210
	if !ctx.cr[6].eq {
	pc = 0x82C5A210; continue 'dispatch;
	}
	// 82C5A208: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5A20C: 409AFFA0  bne cr6, 0x82c5a1ac
	if !ctx.cr[6].eq {
	pc = 0x82C5A1AC; continue 'dispatch;
	}
	// 82C5A210: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C5A214: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C5A218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5A21C: 4B71226D  bl 0x8236c488
	ctx.lr = 0x82C5A220;
	sub_8236C488(ctx, base);
	// 82C5A220: 4807F861  bl 0x82cd9a80
	ctx.lr = 0x82C5A224;
	sub_82CD9A80(ctx, base);
	// 82C5A224: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C5A228: 4804F22C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5A230 size=156
    let mut pc: u32 = 0x82C5A230;
    'dispatch: loop {
        match pc {
            0x82C5A230 => {
    //   block [0x82C5A230..0x82C5A2CC)
	// 82C5A230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A234: 4804F1D9  bl 0x82ca940c
	ctx.lr = 0x82C5A238;
	sub_82CA93D0(ctx, base);
	// 82C5A238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A23C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C5A240: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C5A244: 93A1009C  stw r29, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 82C5A248: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 82C5A24C: 3BFE012C  addi r31, r30, 0x12c
	ctx.r[31].s64 = ctx.r[30].s64 + 300;
	// 82C5A250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5A254: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C5A258: 4B71B229  bl 0x82375480
	ctx.lr = 0x82C5A25C;
	sub_82375480(ctx, base);
	// 82C5A25C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5A260: 813E0130  lwz r9, 0x130(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) } as u64;
	// 82C5A264: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A268: 419A000C  beq cr6, 0x82c5a274
	if ctx.cr[6].eq {
	pc = 0x82C5A274; continue 'dispatch;
	}
	// 82C5A26C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C5A270: 419A0008  beq cr6, 0x82c5a278
	if ctx.cr[6].eq {
	pc = 0x82C5A278; continue 'dispatch;
	}
	// 82C5A274: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C5A278: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C5A27C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C5A280: 419A0044  beq cr6, 0x82c5a2c4
	if ctx.cr[6].eq {
	pc = 0x82C5A2C4; continue 'dispatch;
	}
	// 82C5A284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A288: 409A0008  bne cr6, 0x82c5a290
	if !ctx.cr[6].eq {
	pc = 0x82C5A290; continue 'dispatch;
	}
	// 82C5A28C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C5A290: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A294: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C5A298: 409A0008  bne cr6, 0x82c5a2a0
	if !ctx.cr[6].eq {
	pc = 0x82C5A2A0; continue 'dispatch;
	}
	// 82C5A29C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C5A2A0: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5A2A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A2A8: 419A001C  beq cr6, 0x82c5a2c4
	if ctx.cr[6].eq {
	pc = 0x82C5A2C4; continue 'dispatch;
	}
	// 82C5A2AC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C5A2B0: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C5A2B4: 40820010  bne 0x82c5a2c4
	if !ctx.cr[0].eq {
	pc = 0x82C5A2C4; continue 'dispatch;
	}
	// 82C5A2B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C5A2BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5A2C0: 4BFFFE79  bl 0x82c5a138
	ctx.lr = 0x82C5A2C4;
	sub_82C5A138(ctx, base);
	// 82C5A2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5A2C8: 4804F194  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5A2D0 size=76
    let mut pc: u32 = 0x82C5A2D0;
    'dispatch: loop {
        match pc {
            0x82C5A2D0 => {
    //   block [0x82C5A2D0..0x82C5A31C)
	// 82C5A2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5A2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5A2DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A2E4: 897F02AD  lbz r11, 0x2ad(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(685 as u32) ) } as u64;
	// 82C5A2E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A2EC: 409A001C  bne cr6, 0x82c5a308
	if !ctx.cr[6].eq {
	pc = 0x82C5A308; continue 'dispatch;
	}
	// 82C5A2F0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C5A2F4: 809F02A8  lwz r4, 0x2a8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(680 as u32) ) } as u64;
	// 82C5A2F8: 806B5164  lwz r3, 0x5164(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5A2FC: 4BFFFF35  bl 0x82c5a230
	ctx.lr = 0x82C5A300;
	sub_82C5A230(ctx, base);
	// 82C5A300: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C5A304: 995F02AD  stb r10, 0x2ad(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(685 as u32), ctx.r[10].u8 ) };
	// 82C5A308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C5A30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5A310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5A314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5A318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5A320 size=128
    let mut pc: u32 = 0x82C5A320;
    'dispatch: loop {
        match pc {
            0x82C5A320 => {
    //   block [0x82C5A320..0x82C5A3A0)
	// 82C5A320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5A328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5A32C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5A330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A338: 897F02AC  lbz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 82C5A33C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A340: 419A0040  beq cr6, 0x82c5a380
	if ctx.cr[6].eq {
	pc = 0x82C5A380; continue 'dispatch;
	}
	// 82C5A344: 897F02AD  lbz r11, 0x2ad(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(685 as u32) ) } as u64;
	// 82C5A348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A34C: 409A0034  bne cr6, 0x82c5a380
	if !ctx.cr[6].eq {
	pc = 0x82C5A380; continue 'dispatch;
	}
	// 82C5A350: 809F02A8  lwz r4, 0x2a8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(680 as u32) ) } as u64;
	// 82C5A354: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C5A358: 419A0028  beq cr6, 0x82c5a380
	if ctx.cr[6].eq {
	pc = 0x82C5A380; continue 'dispatch;
	}
	// 82C5A35C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C5A360: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C5A364: 997F02AD  stb r11, 0x2ad(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(685 as u32), ctx.r[11].u8 ) };
	// 82C5A368: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C5A36C: 93DF02A8  stw r30, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[30].u32 ) };
	// 82C5A370: 806A5164  lwz r3, 0x5164(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5A374: 4BFFFEBD  bl 0x82c5a230
	ctx.lr = 0x82C5A378;
	sub_82C5A230(ctx, base);
	// 82C5A378: 9BDF02AC  stb r30, 0x2ac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[30].u8 ) };
	// 82C5A37C: 4800000C  b 0x82c5a388
	pc = 0x82C5A388; continue 'dispatch;
	// 82C5A380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5A384: 997F02AC  stb r11, 0x2ac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[11].u8 ) };
	// 82C5A388: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5A38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5A390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5A394: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5A398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5A39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5A3A0 size=212
    let mut pc: u32 = 0x82C5A3A0;
    'dispatch: loop {
        match pc {
            0x82C5A3A0 => {
    //   block [0x82C5A3A0..0x82C5A474)
	// 82C5A3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5A3A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5A3AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5A3B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A3B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A3B8: 897F02B2  lbz r11, 0x2b2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5A3BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A3C0: 409A009C  bne cr6, 0x82c5a45c
	if !ctx.cr[6].eq {
	pc = 0x82C5A45C; continue 'dispatch;
	}
	// 82C5A3C4: 4BFFF40D  bl 0x82c597d0
	ctx.lr = 0x82C5A3C8;
	sub_82C597D0(ctx, base);
	// 82C5A3C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C5A3CC: C1BF029C  lfs f13, 0x29c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C5A3D0: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A3D4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C5A3D8: 83DF0150  lwz r30, 0x150(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A3DC: 419A0080  beq cr6, 0x82c5a45c
	if ctx.cr[6].eq {
	pc = 0x82C5A45C; continue 'dispatch;
	}
	// 82C5A3E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C5A3E4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A3E8: 4807F051  bl 0x82cd9438
	ctx.lr = 0x82C5A3EC;
	sub_82CD9438(ctx, base);
	// 82C5A3EC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5A3F0: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C5A3F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5A3F8: 409A0064  bne cr6, 0x82c5a45c
	if !ctx.cr[6].eq {
	pc = 0x82C5A45C; continue 'dispatch;
	}
	// 82C5A3FC: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C5A400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A404: 409A0058  bne cr6, 0x82c5a45c
	if !ctx.cr[6].eq {
	pc = 0x82C5A45C; continue 'dispatch;
	}
	// 82C5A408: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A40C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C5A410: 419A004C  beq cr6, 0x82c5a45c
	if ctx.cr[6].eq {
	pc = 0x82C5A45C; continue 'dispatch;
	}
	// 82C5A414: 897F02B4  lbz r11, 0x2b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 82C5A418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A41C: 409A0040  bne cr6, 0x82c5a45c
	if !ctx.cr[6].eq {
	pc = 0x82C5A45C; continue 'dispatch;
	}
	// 82C5A420: 897F02AC  lbz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 82C5A424: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A428: 419A001C  beq cr6, 0x82c5a444
	if ctx.cr[6].eq {
	pc = 0x82C5A444; continue 'dispatch;
	}
	// 82C5A42C: E97F0268  ld r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) };
	// 82C5A430: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 82C5A434: 409A0010  bne cr6, 0x82c5a444
	if !ctx.cr[6].eq {
	pc = 0x82C5A444; continue 'dispatch;
	}
	// 82C5A438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5A43C: 4BFFFE95  bl 0x82c5a2d0
	ctx.lr = 0x82C5A440;
	sub_82C5A2D0(ctx, base);
	// 82C5A440: 4800001C  b 0x82c5a45c
	pc = 0x82C5A45C; continue 'dispatch;
	// 82C5A444: C03F0298  lfs f1, 0x298(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5A448: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A44C: 4807F365  bl 0x82cd97b0
	ctx.lr = 0x82C5A450;
	sub_82CD97B0(ctx, base);
	// 82C5A450: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C5A454: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A458: 4807F0F1  bl 0x82cd9548
	ctx.lr = 0x82C5A45C;
	sub_82CD9548(ctx, base);
	// 82C5A45C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5A460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5A464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5A468: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5A46C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5A470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5A478 size=216
    let mut pc: u32 = 0x82C5A478;
    'dispatch: loop {
        match pc {
            0x82C5A478 => {
    //   block [0x82C5A478..0x82C5A550)
	// 82C5A478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5A480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5A484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A48C: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A490: C03F02A0  lfs f1, 0x2a0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(672 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5A494: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C5A498: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82C5A49C: 419A0084  beq cr6, 0x82c5a520
	if ctx.cr[6].eq {
	pc = 0x82C5A520; continue 'dispatch;
	}
	// 82C5A4A0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C5A4A4: C00A0C18  lfs f0, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A4A8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C5A4AC: 409A0030  bne cr6, 0x82c5a4dc
	if !ctx.cr[6].eq {
	pc = 0x82C5A4DC; continue 'dispatch;
	}
	// 82C5A4B0: 4B556EB9  bl 0x821b1368
	ctx.lr = 0x82C5A4B4;
	sub_821B1368(ctx, base);
	// 82C5A4B4: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C5A4B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5A4BC: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A4C0: C00BD010  lfs f0, -0x2ff0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A4C4: 806A0018  lwz r3, 0x18(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A4C8: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C5A4CC: 4807F35D  bl 0x82cd9828
	ctx.lr = 0x82C5A4D0;
	sub_82CD9828(ctx, base);
	// 82C5A4D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5A4D4: 4BFFFECD  bl 0x82c5a3a0
	ctx.lr = 0x82C5A4D8;
	sub_82C5A3A0(ctx, base);
	// 82C5A4D8: 4800003C  b 0x82c5a514
	pc = 0x82C5A514; continue 'dispatch;
	// 82C5A4DC: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C5A4E0: 409A0014  bne cr6, 0x82c5a4f4
	if !ctx.cr[6].eq {
	pc = 0x82C5A4F4; continue 'dispatch;
	}
	// 82C5A4E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C5A4E8: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A4EC: 4807F0A5  bl 0x82cd9590
	ctx.lr = 0x82C5A4F0;
	sub_82CD9590(ctx, base);
	// 82C5A4F0: 48000024  b 0x82c5a514
	pc = 0x82C5A514; continue 'dispatch;
	// 82C5A4F4: 4B556E75  bl 0x821b1368
	ctx.lr = 0x82C5A4F8;
	sub_821B1368(ctx, base);
	// 82C5A4F8: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82C5A4FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5A500: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A504: C00BD010  lfs f0, -0x2ff0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A508: 806A0018  lwz r3, 0x18(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A50C: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82C5A510: 4807F319  bl 0x82cd9828
	ctx.lr = 0x82C5A514;
	sub_82CD9828(ctx, base);
	// 82C5A514: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A518: C01F02A0  lfs f0, 0x2a0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(672 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A51C: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C5A520: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A524: C03F0298  lfs f1, 0x298(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5A528: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A52C: 4807F285  bl 0x82cd97b0
	ctx.lr = 0x82C5A530;
	sub_82CD97B0(ctx, base);
	// 82C5A530: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C5A534: 387F0144  addi r3, r31, 0x144
	ctx.r[3].s64 = ctx.r[31].s64 + 324;
	// 82C5A538: 4BFFDDF1  bl 0x82c58328
	ctx.lr = 0x82C5A53C;
	sub_82C58328(ctx, base);
	// 82C5A53C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C5A540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5A544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5A548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5A54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5A550 size=472
    let mut pc: u32 = 0x82C5A550;
    'dispatch: loop {
        match pc {
            0x82C5A550 => {
    //   block [0x82C5A550..0x82C5A728)
	// 82C5A550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A554: 4804EEAD  bl 0x82ca9400
	ctx.lr = 0x82C5A558;
	sub_82CA93D0(ctx, base);
	// 82C5A558: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A55C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C5A560: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82C5A564: 897E02B3  lbz r11, 0x2b3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(691 as u32) ) } as u64;
	// 82C5A568: 895E02B2  lbz r10, 0x2b2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5A56C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C5A570: 419A01B0  beq cr6, 0x82c5a720
	if ctx.cr[6].eq {
	pc = 0x82C5A720; continue 'dispatch;
	}
	// 82C5A574: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A578: 419A001C  beq cr6, 0x82c5a594
	if ctx.cr[6].eq {
	pc = 0x82C5A594; continue 'dispatch;
	}
	// 82C5A57C: 817E016C  lwz r11, 0x16c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(364 as u32) ) } as u64;
	// 82C5A580: 815E0168  lwz r10, 0x168(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(360 as u32) ) } as u64;
	// 82C5A584: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C5A588: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C5A58C: 41980194  blt cr6, 0x82c5a720
	if ctx.cr[6].lt {
	pc = 0x82C5A720; continue 'dispatch;
	}
	// 82C5A590: 48000030  b 0x82c5a5c0
	pc = 0x82C5A5C0; continue 'dispatch;
	// 82C5A594: 817E0284  lwz r11, 0x284(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C5A598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A59C: 41990184  bgt cr6, 0x82c5a720
	if ctx.cr[6].gt {
	pc = 0x82C5A720; continue 'dispatch;
	}
	// 82C5A5A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C5A5A4: 80DE028C  lwz r6, 0x28c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(652 as u32) ) } as u64;
	// 82C5A5A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C5A5AC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C5A5B0: 387E0168  addi r3, r30, 0x168
	ctx.r[3].s64 = ctx.r[30].s64 + 360;
	// 82C5A5B4: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5A5B8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5A5BC: 4BFB8BD5  bl 0x82c13190
	ctx.lr = 0x82C5A5C0;
	sub_82C13190(ctx, base);
	// 82C5A5C0: 897E02B2  lbz r11, 0x2b2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5A5C4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C5A5C8: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82C5A5CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A5D0: 419A0018  beq cr6, 0x82c5a5e8
	if ctx.cr[6].eq {
	pc = 0x82C5A5E8; continue 'dispatch;
	}
	// 82C5A5D4: 817E0154  lwz r11, 0x154(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82C5A5D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A5DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5A5E0: 419A0008  beq cr6, 0x82c5a5e8
	if ctx.cr[6].eq {
	pc = 0x82C5A5E8; continue 'dispatch;
	}
	// 82C5A5E4: 836B0004  lwz r27, 4(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A5E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A5EC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82C5A5F0: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82C5A5F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5A5F8: 915E0188  stw r10, 0x188(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 82C5A5FC: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82C5A600: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C5A604: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C5A608: 4E800421  bctrl
	ctx.lr = 0x82C5A60C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5A60C: 891E02B2  lbz r8, 0x2b2(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5A610: 939E0188  stw r28, 0x188(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(392 as u32), ctx.r[28].u32 ) };
	// 82C5A614: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C5A618: 419A001C  beq cr6, 0x82c5a634
	if ctx.cr[6].eq {
	pc = 0x82C5A634; continue 'dispatch;
	}
	// 82C5A61C: 807E0154  lwz r3, 0x154(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82C5A620: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5A624: 419A0018  beq cr6, 0x82c5a63c
	if ctx.cr[6].eq {
	pc = 0x82C5A63C; continue 'dispatch;
	}
	// 82C5A628: 4BBEB189  bl 0x828457b0
	ctx.lr = 0x82C5A62C;
	sub_828457B0(ctx, base);
	// 82C5A62C: 939E0154  stw r28, 0x154(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(340 as u32), ctx.r[28].u32 ) };
	// 82C5A630: 4800000C  b 0x82c5a63c
	pc = 0x82C5A63C; continue 'dispatch;
	// 82C5A634: 387E0150  addi r3, r30, 0x150
	ctx.r[3].s64 = ctx.r[30].s64 + 336;
	// 82C5A638: 4BFFEDA1  bl 0x82c593d8
	ctx.lr = 0x82C5A63C;
	sub_82C593D8(ctx, base);
	// 82C5A63C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5A640: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C5A644: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C5A648: 419A00A4  beq cr6, 0x82c5a6ec
	if ctx.cr[6].eq {
	pc = 0x82C5A6EC; continue 'dispatch;
	}
	// 82C5A64C: 3BBE0024  addi r29, r30, 0x24
	ctx.r[29].s64 = ctx.r[30].s64 + 36;
	// 82C5A650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5A654: 4BFB46ED  bl 0x82c0ed40
	ctx.lr = 0x82C5A658;
	sub_82C0ED40(ctx, base);
	// 82C5A658: 897E02B3  lbz r11, 0x2b3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(691 as u32) ) } as u64;
	// 82C5A65C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A660: 419A0020  beq cr6, 0x82c5a680
	if ctx.cr[6].eq {
	pc = 0x82C5A680; continue 'dispatch;
	}
	// 82C5A664: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5A668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A66C: 419A0014  beq cr6, 0x82c5a680
	if ctx.cr[6].eq {
	pc = 0x82C5A680; continue 'dispatch;
	}
	// 82C5A670: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82C5A674: 817E0284  lwz r11, 0x284(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C5A678: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C5A67C: 917E0284  stw r11, 0x284(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(644 as u32), ctx.r[11].u32 ) };
	// 82C5A680: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5A684: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A688: 409A0054  bne cr6, 0x82c5a6dc
	if !ctx.cr[6].eq {
	pc = 0x82C5A6DC; continue 'dispatch;
	}
	// 82C5A68C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A694: 409A0048  bne cr6, 0x82c5a6dc
	if !ctx.cr[6].eq {
	pc = 0x82C5A6DC; continue 'dispatch;
	}
	// 82C5A698: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A69C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A6A0: 409A003C  bne cr6, 0x82c5a6dc
	if !ctx.cr[6].eq {
	pc = 0x82C5A6DC; continue 'dispatch;
	}
	// 82C5A6A4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C5A6A8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A6AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A6B0: 409A0014  bne cr6, 0x82c5a6c4
	if !ctx.cr[6].eq {
	pc = 0x82C5A6C4; continue 'dispatch;
	}
	// 82C5A6B4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C5A6B8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C5A6BC: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C5A6C0: 48000018  b 0x82c5a6d8
	pc = 0x82C5A6D8; continue 'dispatch;
	// 82C5A6C4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A6C8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C5A6CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C5A6D0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A6D4: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C5A6D8: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C5A6DC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5A6E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C5A6E4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C5A6E8: 409AFF68  bne cr6, 0x82c5a650
	if !ctx.cr[6].eq {
	pc = 0x82C5A650; continue 'dispatch;
	}
	// 82C5A6EC: 897E02B3  lbz r11, 0x2b3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(691 as u32) ) } as u64;
	// 82C5A6F0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C5A6F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5A6F8: 997E02B2  stb r11, 0x2b2(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(690 as u32), ctx.r[11].u8 ) };
	// 82C5A6FC: 4BFFF965  bl 0x82c5a060
	ctx.lr = 0x82C5A700;
	sub_82C5A060(ctx, base);
	// 82C5A700: 895E02B2  lbz r10, 0x2b2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5A704: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5A708: 409A0010  bne cr6, 0x82c5a718
	if !ctx.cr[6].eq {
	pc = 0x82C5A718; continue 'dispatch;
	}
	// 82C5A70C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C5A710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5A714: 4BFFFD65  bl 0x82c5a478
	ctx.lr = 0x82C5A718;
	sub_82C5A478(ctx, base);
	// 82C5A718: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5A71C: 4BFFFC85  bl 0x82c5a3a0
	ctx.lr = 0x82C5A720;
	sub_82C5A3A0(ctx, base);
	// 82C5A720: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C5A724: 4804ED2C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5A728 size=168
    let mut pc: u32 = 0x82C5A728;
    'dispatch: loop {
        match pc {
            0x82C5A728 => {
    //   block [0x82C5A728..0x82C5A7D0)
	// 82C5A728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A72C: 4804ECE1  bl 0x82ca940c
	ctx.lr = 0x82C5A730;
	sub_82CA93D0(ctx, base);
	// 82C5A730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A734: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A738: 815F0138  lwz r10, 0x138(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 82C5A73C: 356A0001  addic. r11, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C5A740: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82C5A744: 917F0138  stw r11, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 82C5A748: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C5A74C: 4082000C  bne 0x82c5a758
	if !ctx.cr[0].eq {
	pc = 0x82C5A758; continue 'dispatch;
	}
	// 82C5A750: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C5A754: 917F0138  stw r11, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 82C5A758: 3BDF012C  addi r30, r31, 0x12c
	ctx.r[30].s64 = ctx.r[31].s64 + 300;
	// 82C5A75C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C5A760: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C5A764: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C5A768: 4B71AD19  bl 0x82375480
	ctx.lr = 0x82C5A76C;
	sub_82375480(ctx, base);
	// 82C5A76C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C5A770: 813F0130  lwz r9, 0x130(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82C5A774: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A778: 419A000C  beq cr6, 0x82c5a784
	if ctx.cr[6].eq {
	pc = 0x82C5A784; continue 'dispatch;
	}
	// 82C5A77C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C5A780: 419A0008  beq cr6, 0x82c5a788
	if ctx.cr[6].eq {
	pc = 0x82C5A788; continue 'dispatch;
	}
	// 82C5A784: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C5A788: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5A78C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C5A790: 419A0038  beq cr6, 0x82c5a7c8
	if ctx.cr[6].eq {
	pc = 0x82C5A7C8; continue 'dispatch;
	}
	// 82C5A794: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A798: 409A0008  bne cr6, 0x82c5a7a0
	if !ctx.cr[6].eq {
	pc = 0x82C5A7A0; continue 'dispatch;
	}
	// 82C5A79C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C5A7A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A7A4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C5A7A8: 409A0008  bne cr6, 0x82c5a7b0
	if !ctx.cr[6].eq {
	pc = 0x82C5A7B0; continue 'dispatch;
	}
	// 82C5A7AC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C5A7B0: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C5A7B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A7B8: 409A0010  bne cr6, 0x82c5a7c8
	if !ctx.cr[6].eq {
	pc = 0x82C5A7C8; continue 'dispatch;
	}
	// 82C5A7BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C5A7C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5A7C4: 4BFFF975  bl 0x82c5a138
	ctx.lr = 0x82C5A7C8;
	sub_82C5A138(ctx, base);
	// 82C5A7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5A7CC: 4804EC90  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5A7D0 size=304
    let mut pc: u32 = 0x82C5A7D0;
    'dispatch: loop {
        match pc {
            0x82C5A7D0 => {
    //   block [0x82C5A7D0..0x82C5A900)
	// 82C5A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A7D4: 4804EC39  bl 0x82ca940c
	ctx.lr = 0x82C5A7D8;
	sub_82CA93D0(ctx, base);
	// 82C5A7D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A7DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5A7E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5A7E4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C5A7E8: 392BCFE0  addi r9, r11, -0x3020
	ctx.r[9].s64 = ctx.r[11].s64 + -12320;
	// 82C5A7EC: 895F02AC  lbz r10, 0x2ac(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 82C5A7F0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C5A7F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C5A7F8: 419A0034  beq cr6, 0x82c5a82c
	if ctx.cr[6].eq {
	pc = 0x82C5A82C; continue 'dispatch;
	}
	// 82C5A7FC: 897F02AD  lbz r11, 0x2ad(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(685 as u32) ) } as u64;
	// 82C5A800: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A804: 409A0028  bne cr6, 0x82c5a82c
	if !ctx.cr[6].eq {
	pc = 0x82C5A82C; continue 'dispatch;
	}
	// 82C5A808: 809F02A8  lwz r4, 0x2a8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(680 as u32) ) } as u64;
	// 82C5A80C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C5A810: 419A001C  beq cr6, 0x82c5a82c
	if ctx.cr[6].eq {
	pc = 0x82C5A82C; continue 'dispatch;
	}
	// 82C5A814: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C5A818: 93BF02A8  stw r29, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[29].u32 ) };
	// 82C5A81C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C5A820: 997F02AD  stb r11, 0x2ad(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(685 as u32), ctx.r[11].u8 ) };
	// 82C5A824: 806A5164  lwz r3, 0x5164(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5A828: 4BFFFA09  bl 0x82c5a230
	ctx.lr = 0x82C5A82C;
	sub_82C5A230(ctx, base);
	// 82C5A82C: 807F01F4  lwz r3, 0x1f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(500 as u32) ) } as u64;
	// 82C5A830: 4BBEAF81  bl 0x828457b0
	ctx.lr = 0x82C5A834;
	sub_828457B0(ctx, base);
	// 82C5A834: 93BF01F4  stw r29, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[29].u32 ) };
	// 82C5A838: 807F0154  lwz r3, 0x154(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82C5A83C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5A840: 419A000C  beq cr6, 0x82c5a84c
	if ctx.cr[6].eq {
	pc = 0x82C5A84C; continue 'dispatch;
	}
	// 82C5A844: 4BBEAF6D  bl 0x828457b0
	ctx.lr = 0x82C5A848;
	sub_828457B0(ctx, base);
	// 82C5A848: 93BF0154  stw r29, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[29].u32 ) };
	// 82C5A84C: 83DF0150  lwz r30, 0x150(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5A850: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C5A854: 419A0040  beq cr6, 0x82c5a894
	if ctx.cr[6].eq {
	pc = 0x82C5A894; continue 'dispatch;
	}
	// 82C5A858: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A85C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5A860: 419A0014  beq cr6, 0x82c5a874
	if ctx.cr[6].eq {
	pc = 0x82C5A874; continue 'dispatch;
	}
	// 82C5A864: 4807EC8D  bl 0x82cd94f0
	ctx.lr = 0x82C5A868;
	sub_82CD94F0(ctx, base);
	// 82C5A868: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5A86C: 4807EA45  bl 0x82cd92b0
	ctx.lr = 0x82C5A870;
	sub_82CD92B0(ctx, base);
	// 82C5A870: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82C5A874: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C5A878: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5A87C: 419A000C  beq cr6, 0x82c5a888
	if ctx.cr[6].eq {
	pc = 0x82C5A888; continue 'dispatch;
	}
	// 82C5A880: 4807EA31  bl 0x82cd92b0
	ctx.lr = 0x82C5A884;
	sub_82CD92B0(ctx, base);
	// 82C5A884: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82C5A888: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C5A88C: 4BBEAF25  bl 0x828457b0
	ctx.lr = 0x82C5A890;
	sub_828457B0(ctx, base);
	// 82C5A890: 93BF0150  stw r29, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[29].u32 ) };
	// 82C5A894: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82C5A898: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5A89C: 419A001C  beq cr6, 0x82c5a8b8
	if ctx.cr[6].eq {
	pc = 0x82C5A8B8; continue 'dispatch;
	}
	// 82C5A8A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A8A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C5A8A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A8AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C5A8B0: 4E800421  bctrl
	ctx.lr = 0x82C5A8B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5A8B4: 93BF014C  stw r29, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[29].u32 ) };
	// 82C5A8B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5A8BC: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 82C5A8C0: 394BCD98  addi r10, r11, -0x3268
	ctx.r[10].s64 = ctx.r[11].s64 + -12904;
	// 82C5A8C4: 915F0144  stw r10, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 82C5A8C8: 48000A39  bl 0x82c5b300
	ctx.lr = 0x82C5A8CC;
	sub_82C5B300(ctx, base);
	// 82C5A8CC: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82C5A8D0: 48000A31  bl 0x82c5b300
	ctx.lr = 0x82C5A8D4;
	sub_82C5B300(ctx, base);
	// 82C5A8D4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C5A8D8: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82C5A8DC: 3909CFB8  addi r8, r9, -0x3048
	ctx.r[8].s64 = ctx.r[9].s64 + -12360;
	// 82C5A8E0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C5A8E4: 911F0018  stw r8, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82C5A8E8: 4BFF32B9  bl 0x82c4dba0
	ctx.lr = 0x82C5A8EC;
	sub_82C4DBA0(ctx, base);
	// 82C5A8EC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C5A8F0: 38C7CF84  addi r6, r7, -0x307c
	ctx.r[6].s64 = ctx.r[7].s64 + -12412;
	// 82C5A8F4: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82C5A8F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5A8FC: 4804EB60  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5A900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5A900 size=256
    let mut pc: u32 = 0x82C5A900;
    'dispatch: loop {
        match pc {
            0x82C5A900 => {
    //   block [0x82C5A900..0x82C5AA00)
	// 82C5A900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5A904: 4804EB05  bl 0x82ca9408
	ctx.lr = 0x82C5A908;
	sub_82CA93D0(ctx, base);
	// 82C5A908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5A90C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C5A910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C5A914: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C5A918: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C5A91C: 897D02B2  lbz r11, 0x2b2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5A920: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A924: 419A0014  beq cr6, 0x82c5a938
	if ctx.cr[6].eq {
	pc = 0x82C5A938; continue 'dispatch;
	}
	// 82C5A928: 897D02B3  lbz r11, 0x2b3(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(691 as u32) ) } as u64;
	// 82C5A92C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A930: 419A0008  beq cr6, 0x82c5a938
	if ctx.cr[6].eq {
	pc = 0x82C5A938; continue 'dispatch;
	}
	// 82C5A934: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82C5A938: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82C5A93C: 4B5C491D  bl 0x8221f258
	ctx.lr = 0x82C5A940;
	sub_8221F258(ctx, base);
	// 82C5A940: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C5A944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5A948: 419A001C  beq cr6, 0x82c5a964
	if ctx.cr[6].eq {
	pc = 0x82C5A964; continue 'dispatch;
	}
	// 82C5A94C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C5A950: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C5A954: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C5A958: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82C5A95C: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82C5A960: 48000008  b 0x82c5a968
	pc = 0x82C5A968; continue 'dispatch;
	// 82C5A964: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C5A968: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82C5A96C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C5A970: 938B0010  stw r28, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82C5A974: 409A0010  bne cr6, 0x82c5a984
	if !ctx.cr[6].eq {
	pc = 0x82C5A984; continue 'dispatch;
	}
	// 82C5A978: 815D0284  lwz r10, 0x284(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C5A97C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82C5A980: 915D0284  stw r10, 0x284(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(644 as u32), ctx.r[10].u32 ) };
	// 82C5A984: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5A988: 395D0024  addi r10, r29, 0x24
	ctx.r[10].s64 = ctx.r[29].s64 + 36;
	// 82C5A98C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5A990: 409A0054  bne cr6, 0x82c5a9e4
	if !ctx.cr[6].eq {
	pc = 0x82C5A9E4; continue 'dispatch;
	}
	// 82C5A994: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A998: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5A99C: 409A0048  bne cr6, 0x82c5a9e4
	if !ctx.cr[6].eq {
	pc = 0x82C5A9E4; continue 'dispatch;
	}
	// 82C5A9A0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A9A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5A9A8: 409A003C  bne cr6, 0x82c5a9e4
	if !ctx.cr[6].eq {
	pc = 0x82C5A9E4; continue 'dispatch;
	}
	// 82C5A9AC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C5A9B0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5A9B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5A9B8: 409A0014  bne cr6, 0x82c5a9cc
	if !ctx.cr[6].eq {
	pc = 0x82C5A9CC; continue 'dispatch;
	}
	// 82C5A9BC: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C5A9C0: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C5A9C4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5A9C8: 48000018  b 0x82c5a9e0
	pc = 0x82C5A9E0; continue 'dispatch;
	// 82C5A9CC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A9D0: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C5A9D4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C5A9D8: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5A9DC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C5A9E0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C5A9E4: 817D0280  lwz r11, 0x280(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(640 as u32) ) } as u64;
	// 82C5A9E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C5A9EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C5A9F0: 917D0280  stw r11, 0x280(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(640 as u32), ctx.r[11].u32 ) };
	// 82C5A9F4: 4BFFF9AD  bl 0x82c5a3a0
	ctx.lr = 0x82C5A9F8;
	sub_82C5A3A0(ctx, base);
	// 82C5A9F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5A9FC: 4804EA5C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5AA00 size=176
    let mut pc: u32 = 0x82C5AA00;
    'dispatch: loop {
        match pc {
            0x82C5AA00 => {
    //   block [0x82C5AA00..0x82C5AAB0)
	// 82C5AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5AA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5AA0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AA10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5AA14: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82C5AA18: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82C5AA1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AA20: 989F02B3  stb r4, 0x2b3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(691 as u32), ctx.r[4].u8 ) };
	// 82C5AA24: 90DF028C  stw r6, 0x28c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), ctx.r[6].u32 ) };
	// 82C5AA28: 419A0044  beq cr6, 0x82c5aa6c
	if ctx.cr[6].eq {
	pc = 0x82C5AA6C; continue 'dispatch;
	}
	// 82C5AA2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C5AA30: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C5AA34: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C5AA38: 387F0168  addi r3, r31, 0x168
	ctx.r[3].s64 = ctx.r[31].s64 + 360;
	// 82C5AA3C: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5AA40: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AA44: 816A5164  lwz r11, 0x5164(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5AA48: 808B005C  lwz r4, 0x5c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5AA4C: 4BFB8745  bl 0x82c13190
	ctx.lr = 0x82C5AA50;
	sub_82C13190(ctx, base);
	// 82C5AA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AA54: 4BFFF8CD  bl 0x82c5a320
	ctx.lr = 0x82C5AA58;
	sub_82C5A320(ctx, base);
	// 82C5AA58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5AA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5AA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5AA64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5AA68: 4E800020  blr
	return;
	// 82C5AA6C: 897F02B2  lbz r11, 0x2b2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5AA70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AA74: 409A0028  bne cr6, 0x82c5aa9c
	if !ctx.cr[6].eq {
	pc = 0x82C5AA9C; continue 'dispatch;
	}
	// 82C5AA78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C5AA7C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C5AA80: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C5AA84: 387F0168  addi r3, r31, 0x168
	ctx.r[3].s64 = ctx.r[31].s64 + 360;
	// 82C5AA88: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5AA8C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AA90: 816A5164  lwz r11, 0x5164(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5AA94: 808B005C  lwz r4, 0x5c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5AA98: 4BFB86F9  bl 0x82c13190
	ctx.lr = 0x82C5AA9C;
	sub_82C13190(ctx, base);
	// 82C5AA9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5AAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5AAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5AAA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5AAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5AAB0 size=124
    let mut pc: u32 = 0x82C5AAB0;
    'dispatch: loop {
        match pc {
            0x82C5AAB0 => {
    //   block [0x82C5AAB0..0x82C5AB2C)
	// 82C5AAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5AAB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5AABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AAC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5AAC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5AAC8: 389F012C  addi r4, r31, 0x12c
	ctx.r[4].s64 = ctx.r[31].s64 + 300;
	// 82C5AACC: 394BD018  addi r10, r11, -0x2fe8
	ctx.r[10].s64 = ctx.r[11].s64 + -12264;
	// 82C5AAD0: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82C5AAD4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C5AAD8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C5AADC: 813F0130  lwz r9, 0x130(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82C5AAE0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5AAE4: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82C5AAE8: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82C5AAEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82C5AAF0: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C5AAF4: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C5AAF8: 4B711729  bl 0x8236c220
	ctx.lr = 0x82C5AAFC;
	sub_8236C220(ctx, base);
	// 82C5AAFC: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82C5AB00: 4BBEACB1  bl 0x828457b0
	ctx.lr = 0x82C5AB04;
	sub_828457B0(ctx, base);
	// 82C5AB04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5AB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AB0C: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 82C5AB10: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82C5AB14: 48000BFD  bl 0x82c5b710
	ctx.lr = 0x82C5AB18;
	sub_82C5B710(ctx, base);
	// 82C5AB18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5AB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5AB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5AB24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5AB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5AB30 size=80
    let mut pc: u32 = 0x82C5AB30;
    'dispatch: loop {
        match pc {
            0x82C5AB30 => {
    //   block [0x82C5AB30..0x82C5AB80)
	// 82C5AB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5AB38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5AB3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5AB40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AB44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5AB48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C5AB4C: 4BFFFC85  bl 0x82c5a7d0
	ctx.lr = 0x82C5AB50;
	sub_82C5A7D0(ctx, base);
	// 82C5AB50: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C5AB54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AB58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AB5C: 419A000C  beq cr6, 0x82c5ab68
	if ctx.cr[6].eq {
	pc = 0x82C5AB68; continue 'dispatch;
	}
	// 82C5AB60: 4BBEAC51  bl 0x828457b0
	ctx.lr = 0x82C5AB64;
	sub_828457B0(ctx, base);
	// 82C5AB64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AB68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5AB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5AB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5AB74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5AB78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5AB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5AB80 size=344
    let mut pc: u32 = 0x82C5AB80;
    'dispatch: loop {
        match pc {
            0x82C5AB80 => {
    //   block [0x82C5AB80..0x82C5ACD8)
	// 82C5AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AB84: 4804E889  bl 0x82ca940c
	ctx.lr = 0x82C5AB88;
	sub_82CA93D0(ctx, base);
	// 82C5AB88: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82C5AB8C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82C5AB90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AB94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5AB98: 48000A71  bl 0x82c5b608
	ctx.lr = 0x82C5AB9C;
	sub_82C5B608(ctx, base);
	// 82C5AB9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C5ABA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C5ABA4: 394BD018  addi r10, r11, -0x2fe8
	ctx.r[10].s64 = ctx.r[11].s64 + -12264;
	// 82C5ABA8: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 82C5ABAC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C5ABB0: 93DF011C  stw r30, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[30].u32 ) };
	// 82C5ABB4: 93DF0120  stw r30, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[30].u32 ) };
	// 82C5ABB8: 93DF0124  stw r30, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[30].u32 ) };
	// 82C5ABBC: 93DF0128  stw r30, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[30].u32 ) };
	// 82C5ABC0: 4B6C73E9  bl 0x82321fa8
	ctx.lr = 0x82C5ABC4;
	sub_82321FA8(ctx, base);
	// 82C5ABC4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C5ABC8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82C5ABCC: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 82C5ABD0: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 82C5ABD4: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C5ABD8: 93BF0138  stw r29, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[29].u32 ) };
	// 82C5ABDC: 38BF0070  addi r5, r31, 0x70
	ctx.r[5].s64 = ctx.r[31].s64 + 112;
	// 82C5ABE0: 3860003F  li r3, 0x3f
	ctx.r[3].s64 = 63;
	// 82C5ABE4: C3E90C14  lfs f31, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82C5ABE8: D3FF013C  stfs f31, 0x13c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 82C5ABEC: C028CF80  lfs f1, -0x3080(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12416 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5ABF0: 480A2A41  bl 0x82cfd630
	ctx.lr = 0x82C5ABF4;
	sub_82CFD630(ctx, base);
	// 82C5ABF4: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 82C5ABF8: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82C5ABFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C5AC00: 4B65F9D9  bl 0x822ba5d8
	ctx.lr = 0x82C5AC04;
	sub_822BA5D8(ctx, base);
	// 82C5AC04: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 82C5AC08: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82C5AC0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C5AC10: 4B65F9C9  bl 0x822ba5d8
	ctx.lr = 0x82C5AC14;
	sub_822BA5D8(ctx, base);
	// 82C5AC14: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82C5AC18: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC1C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC20: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC24: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC28: 90BF00C8  stw r5, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[5].u32 ) };
	// 82C5AC2C: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 82C5AC30: C3C70C18  lfs f30, 0xc18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82C5AC34: 90DF00C0  stw r6, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[6].u32 ) };
	// 82C5AC38: D3C10050  stfs f30, 0x50(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC3C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC40: D3C10050  stfs f30, 0x50(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC44: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC48: D3C10050  stfs f30, 0x50(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC4C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC50: D3C10050  stfs f30, 0x50(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC54: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC58: 911F00C4  stw r8, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[8].u32 ) };
	// 82C5AC5C: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82C5AC60: 913F00BC  stw r9, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[9].u32 ) };
	// 82C5AC64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C5AC68: 915F00B8  stw r10, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 82C5AC6C: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82C5AC70: 4B65F969  bl 0x822ba5d8
	ctx.lr = 0x82C5AC74;
	sub_822BA5D8(ctx, base);
	// 82C5AC74: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC78: D3C10054  stfs f30, 0x54(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C5AC7C: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC80: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C5AC84: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AC88: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82C5AC8C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C5AC90: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C5AC94: 397F010C  addi r11, r31, 0x10c
	ctx.r[11].s64 = ctx.r[31].s64 + 268;
	// 82C5AC98: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82C5AC9C: 90FF0118  stw r7, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[7].u32 ) };
	// 82C5ACA0: 90DF010C  stw r6, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[6].u32 ) };
	// 82C5ACA4: 90BF0110  stw r5, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[5].u32 ) };
	// 82C5ACA8: 907F0114  stw r3, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[3].u32 ) };
	// 82C5ACAC: 909F0108  stw r4, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[4].u32 ) };
	// 82C5ACB0: 917F0104  stw r11, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 82C5ACB4: 4B52DA4D  bl 0x82188700
	ctx.lr = 0x82C5ACB8;
	sub_82188700(ctx, base);
	// 82C5ACB8: 907F0144  stw r3, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[3].u32 ) };
	// 82C5ACBC: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82C5ACC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5ACC4: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82C5ACC8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C5ACCC: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82C5ACD0: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82C5ACD4: 4804E788  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5ACD8 size=80
    let mut pc: u32 = 0x82C5ACD8;
    'dispatch: loop {
        match pc {
            0x82C5ACD8 => {
    //   block [0x82C5ACD8..0x82C5AD28)
	// 82C5ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5ACE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5ACE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5ACE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5ACEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5ACF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C5ACF4: 4BFFFDBD  bl 0x82c5aab0
	ctx.lr = 0x82C5ACF8;
	sub_82C5AAB0(ctx, base);
	// 82C5ACF8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C5ACFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AD00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AD04: 419A000C  beq cr6, 0x82c5ad10
	if ctx.cr[6].eq {
	pc = 0x82C5AD10; continue 'dispatch;
	}
	// 82C5AD08: 4BBEAAA9  bl 0x828457b0
	ctx.lr = 0x82C5AD0C;
	sub_828457B0(ctx, base);
	// 82C5AD0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AD10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5AD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5AD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5AD1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5AD20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5AD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5AD28 size=260
    let mut pc: u32 = 0x82C5AD28;
    'dispatch: loop {
        match pc {
            0x82C5AD28 => {
    //   block [0x82C5AD28..0x82C5AE2C)
	// 82C5AD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AD2C: 4804E6E1  bl 0x82ca940c
	ctx.lr = 0x82C5AD30;
	sub_82CA93D0(ctx, base);
	// 82C5AD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AD34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5AD38: 4BFA7EF9  bl 0x82c02c30
	ctx.lr = 0x82C5AD3C;
	sub_82C02C30(ctx, base);
	// 82C5AD3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C5AD40: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82C5AD44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C5AD48: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82C5AD4C: 4804EC65  bl 0x82ca99b0
	ctx.lr = 0x82C5AD50;
	sub_82CA99B0(ctx, base);
	// 82C5AD50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C5AD54: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82C5AD58: 392A3D14  addi r9, r10, 0x3d14
	ctx.r[9].s64 = ctx.r[10].s64 + 15636;
	// 82C5AD5C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82C5AD60: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82C5AD64: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82C5AD68: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C5AD6C: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 82C5AD70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C5AD74: 80FD0008  lwz r7, 8(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C5AD78: 7D063830  slw r6, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[6].u64 = 0;
	} else {
		ctx.r[6].u64 = ((ctx.r[8].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82C5AD7C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82C5AD80: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82C5AD84: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82C5AD88: 4807ED71  bl 0x82cd9af8
	ctx.lr = 0x82C5AD8C;
	sub_82CD9AF8(ctx, base);
	// 82C5AD8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C5AD90: 41980068  blt cr6, 0x82c5adf8
	if ctx.cr[6].lt {
	pc = 0x82C5ADF8; continue 'dispatch;
	}
	// 82C5AD94: 38600150  li r3, 0x150
	ctx.r[3].s64 = 336;
	// 82C5AD98: 4B5C44C1  bl 0x8221f258
	ctx.lr = 0x82C5AD9C;
	sub_8221F258(ctx, base);
	// 82C5AD9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5ADA0: 419A0064  beq cr6, 0x82c5ae04
	if ctx.cr[6].eq {
	pc = 0x82C5AE04; continue 'dispatch;
	}
	// 82C5ADA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C5ADA8: 4BFFFDD9  bl 0x82c5ab80
	ctx.lr = 0x82C5ADAC;
	sub_82C5AB80(ctx, base);
	// 82C5ADAC: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82C5ADB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5ADB4: 907F5164  stw r3, 0x5164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20836 as u32), ctx.r[3].u32 ) };
	// 82C5ADB8: 419A003C  beq cr6, 0x82c5adf4
	if ctx.cr[6].eq {
	pc = 0x82C5ADF4; continue 'dispatch;
	}
	// 82C5ADBC: 480016F5  bl 0x82c5c4b0
	ctx.lr = 0x82C5ADC0;
	sub_82C5C4B0(ctx, base);
	// 82C5ADC0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C5ADC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5ADC8: 409A0058  bne cr6, 0x82c5ae20
	if !ctx.cr[6].eq {
	pc = 0x82C5AE20; continue 'dispatch;
	}
	// 82C5ADCC: 807F5164  lwz r3, 0x5164(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5ADD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5ADD4: 419A0018  beq cr6, 0x82c5adec
	if ctx.cr[6].eq {
	pc = 0x82C5ADEC; continue 'dispatch;
	}
	// 82C5ADD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5ADDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C5ADE0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5ADE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C5ADE8: 4E800421  bctrl
	ctx.lr = 0x82C5ADEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C5ADEC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82C5ADF0: 917F5164  stw r11, 0x5164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20836 as u32), ctx.r[11].u32 ) };
	// 82C5ADF4: 4807EB85  bl 0x82cd9978
	ctx.lr = 0x82C5ADF8;
	sub_82CD9978(ctx, base);
	// 82C5ADF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C5ADFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5AE00: 4804E65C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C5AE04: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C5AE08: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82C5AE0C: 916A5164  stw r11, 0x5164(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20836 as u32), ctx.r[11].u32 ) };
	// 82C5AE10: 4807EB69  bl 0x82cd9978
	ctx.lr = 0x82C5AE14;
	sub_82CD9978(ctx, base);
	// 82C5AE14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C5AE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5AE1C: 4804E640  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C5AE20: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C5AE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5AE28: 4804E634  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5AE30 size=88
    let mut pc: u32 = 0x82C5AE30;
    'dispatch: loop {
        match pc {
            0x82C5AE30 => {
    //   block [0x82C5AE30..0x82C5AE88)
	// 82C5AE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5AE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AE3C: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 82C5AE40: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C5AE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C5AE48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C5AE4C: 81695164  lwz r11, 0x5164(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20836 as u32) ) } as u64;
	// 82C5AE50: 386B012C  addi r3, r11, 0x12c
	ctx.r[3].s64 = ctx.r[11].s64 + 300;
	// 82C5AE54: 80EB0138  lwz r7, 0x138(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(312 as u32) ) } as u64;
	// 82C5AE58: 990A02AD  stb r8, 0x2ad(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(685 as u32), ctx.r[8].u8 ) };
	// 82C5AE5C: 54E6003E  slwi r6, r7, 0
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82C5AE60: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82C5AE64: 90EA02A8  stw r7, 0x2a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(680 as u32), ctx.r[7].u32 ) };
	// 82C5AE68: 4B70FB49  bl 0x8236a9b0
	ctx.lr = 0x82C5AE6C;
	sub_8236A9B0(ctx, base);
	// 82C5AE6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5AE70: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82C5AE74: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82C5AE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C5AE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5AE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5AE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5AE88 size=272
    let mut pc: u32 = 0x82C5AE88;
    'dispatch: loop {
        match pc {
            0x82C5AE88 => {
    //   block [0x82C5AE88..0x82C5AF98)
	// 82C5AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5AE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5AE94: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AE98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5AE9C: 897F02B5  lbz r11, 0x2b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(693 as u32) ) } as u64;
	// 82C5AEA0: 895F02B4  lbz r10, 0x2b4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 82C5AEA4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C5AEA8: 419A00DC  beq cr6, 0x82c5af84
	if ctx.cr[6].eq {
	pc = 0x82C5AF84; continue 'dispatch;
	}
	// 82C5AEAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AEB0: 419A0068  beq cr6, 0x82c5af18
	if ctx.cr[6].eq {
	pc = 0x82C5AF18; continue 'dispatch;
	}
	// 82C5AEB4: 817F017C  lwz r11, 0x17c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(380 as u32) ) } as u64;
	// 82C5AEB8: 815F0178  lwz r10, 0x178(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(376 as u32) ) } as u64;
	// 82C5AEBC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C5AEC0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C5AEC4: 419800C0  blt cr6, 0x82c5af84
	if ctx.cr[6].lt {
	pc = 0x82C5AF84; continue 'dispatch;
	}
	// 82C5AEC8: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5AECC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AED0: 419A00AC  beq cr6, 0x82c5af7c
	if ctx.cr[6].eq {
	pc = 0x82C5AF7C; continue 'dispatch;
	}
	// 82C5AED4: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5AED8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5AEDC: 419A00A0  beq cr6, 0x82c5af7c
	if ctx.cr[6].eq {
	pc = 0x82C5AF7C; continue 'dispatch;
	}
	// 82C5AEE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C5AEE4: 4807E6AD  bl 0x82cd9590
	ctx.lr = 0x82C5AEE8;
	sub_82CD9590(ctx, base);
	// 82C5AEE8: 897F02AC  lbz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 82C5AEEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AEF0: 419A008C  beq cr6, 0x82c5af7c
	if ctx.cr[6].eq {
	pc = 0x82C5AF7C; continue 'dispatch;
	}
	// 82C5AEF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5AEF8: 815F0150  lwz r10, 0x150(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5AEFC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C5AF00: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C5AF04: 806A0018  lwz r3, 0x18(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5AF08: 4807E7A1  bl 0x82cd96a8
	ctx.lr = 0x82C5AF0C;
	sub_82CD96A8(ctx, base);
	// 82C5AF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AF10: 4BFFFF21  bl 0x82c5ae30
	ctx.lr = 0x82C5AF14;
	sub_82C5AE30(ctx, base);
	// 82C5AF14: 48000068  b 0x82c5af7c
	pc = 0x82C5AF7C; continue 'dispatch;
	// 82C5AF18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C5AF1C: 80DF0290  lwz r6, 0x290(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(656 as u32) ) } as u64;
	// 82C5AF20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C5AF24: 387F0178  addi r3, r31, 0x178
	ctx.r[3].s64 = ctx.r[31].s64 + 376;
	// 82C5AF28: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5AF2C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82C5AF30: 4BFB8261  bl 0x82c13190
	ctx.lr = 0x82C5AF34;
	sub_82C13190(ctx, base);
	// 82C5AF34: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5AF38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AF3C: 419A0040  beq cr6, 0x82c5af7c
	if ctx.cr[6].eq {
	pc = 0x82C5AF7C; continue 'dispatch;
	}
	// 82C5AF40: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5AF44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5AF48: 419A0034  beq cr6, 0x82c5af7c
	if ctx.cr[6].eq {
	pc = 0x82C5AF7C; continue 'dispatch;
	}
	// 82C5AF4C: 897F02AC  lbz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 82C5AF50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5AF54: 419A0010  beq cr6, 0x82c5af64
	if ctx.cr[6].eq {
	pc = 0x82C5AF64; continue 'dispatch;
	}
	// 82C5AF58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5AF5C: 4BFFF375  bl 0x82c5a2d0
	ctx.lr = 0x82C5AF60;
	sub_82C5A2D0(ctx, base);
	// 82C5AF60: 4800001C  b 0x82c5af7c
	pc = 0x82C5AF7C; continue 'dispatch;
	// 82C5AF64: C03F0298  lfs f1, 0x298(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5AF68: 4807E849  bl 0x82cd97b0
	ctx.lr = 0x82C5AF6C;
	sub_82CD97B0(ctx, base);
	// 82C5AF6C: 817F0150  lwz r11, 0x150(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 82C5AF70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C5AF74: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C5AF78: 4807E5D1  bl 0x82cd9548
	ctx.lr = 0x82C5AF7C;
	sub_82CD9548(ctx, base);
	// 82C5AF7C: 897F02B5  lbz r11, 0x2b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(693 as u32) ) } as u64;
	// 82C5AF80: 997F02B4  stb r11, 0x2b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[11].u8 ) };
	// 82C5AF84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C5AF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5AF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5AF90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5AF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5AF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C5AF98 size=424
    let mut pc: u32 = 0x82C5AF98;
    'dispatch: loop {
        match pc {
            0x82C5AF98 => {
    //   block [0x82C5AF98..0x82C5B140)
	// 82C5AF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5AF9C: 4804E459  bl 0x82ca93f4
	ctx.lr = 0x82C5AFA0;
	sub_82CA93D0(ctx, base);
	// 82C5AFA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5AFA4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82C5AFA8: 386002C0  li r3, 0x2c0
	ctx.r[3].s64 = 704;
	// 82C5AFAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C5AFB0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C5AFB4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C5AFB8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82C5AFBC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82C5AFC0: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82C5AFC4: 4B5C4295  bl 0x8221f258
	ctx.lr = 0x82C5AFC8;
	sub_8221F258(ctx, base);
	// 82C5AFC8: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82C5AFCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C5AFD0: 419A0010  beq cr6, 0x82c5afe0
	if ctx.cr[6].eq {
	pc = 0x82C5AFE0; continue 'dispatch;
	}
	// 82C5AFD4: 4BFFEE3D  bl 0x82c59e10
	ctx.lr = 0x82C5AFD8;
	sub_82C59E10(ctx, base);
	// 82C5AFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5AFDC: 48000008  b 0x82c5afe4
	pc = 0x82C5AFE4; continue 'dispatch;
	// 82C5AFE0: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82C5AFE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C5AFE8: 93FD0034  stw r31, 0x34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 82C5AFEC: 57CAEFFE  rlwinm r10, r30, 0x1d, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	// 82C5AFF0: 93BF018C  stw r29, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[29].u32 ) };
	// 82C5AFF4: 57C9E7FE  rlwinm r9, r30, 0x1c, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x0000000Fu64;
	// 82C5AFF8: 939F0278  stw r28, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[28].u32 ) };
	// 82C5AFFC: 57C8FFFE  rlwinm r8, r30, 0x1f, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C5B000: 937F027C  stw r27, 0x27c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(636 as u32), ctx.r[27].u32 ) };
	// 82C5B004: 57C7F7FE  rlwinm r7, r30, 0x1e, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x00000003u64;
	// 82C5B008: 933F02A4  stw r25, 0x2a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), ctx.r[25].u32 ) };
	// 82C5B00C: 995F02B3  stb r10, 0x2b3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(691 as u32), ctx.r[10].u8 ) };
	// 82C5B010: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C5B014: 995F02B2  stb r10, 0x2b2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(690 as u32), ctx.r[10].u8 ) };
	// 82C5B018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B01C: 993F02B5  stb r9, 0x2b5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(693 as u32), ctx.r[9].u8 ) };
	// 82C5B020: 993F02B4  stb r9, 0x2b4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[9].u8 ) };
	// 82C5B024: 991F02AE  stb r8, 0x2ae(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(686 as u32), ctx.r[8].u8 ) };
	// 82C5B028: 98FF02AF  stb r7, 0x2af(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(687 as u32), ctx.r[7].u8 ) };
	// 82C5B02C: 917F01CC  stw r11, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 82C5B030: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	// 82C5B034: 935F0148  stw r26, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[26].u32 ) };
	// 82C5B038: 4BFFF029  bl 0x82c5a060
	ctx.lr = 0x82C5B03C;
	sub_82C5A060(ctx, base);
	// 82C5B03C: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82C5B040: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82C5B044: 889F02B3  lbz r4, 0x2b3(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(691 as u32) ) } as u64;
	// 82C5B048: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C5B04C: C1860C18  lfs f12, 0xc18(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C5B050: C0050C14  lfs f0, 0xc14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5B054: 419A000C  beq cr6, 0x82c5b060
	if ctx.cr[6].eq {
	pc = 0x82C5B060; continue 'dispatch;
	}
	// 82C5B058: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	// 82C5B05C: 48000008  b 0x82c5b064
	pc = 0x82C5B064; continue 'dispatch;
	// 82C5B060: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82C5B064: D1BF0170  stfs f13, 0x170(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), tmp.u32 ) };
	// 82C5B068: 931F0168  stw r24, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[24].u32 ) };
	// 82C5B06C: D1BF0174  stfs f13, 0x174(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 82C5B070: 931F016C  stw r24, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[24].u32 ) };
	// 82C5B074: 897F02B5  lbz r11, 0x2b5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(693 as u32) ) } as u64;
	// 82C5B078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5B07C: 419A0008  beq cr6, 0x82c5b084
	if ctx.cr[6].eq {
	pc = 0x82C5B084; continue 'dispatch;
	}
	// 82C5B080: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 82C5B084: D01F0180  stfs f0, 0x180(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 82C5B088: 931F0178  stw r24, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[24].u32 ) };
	// 82C5B08C: D01F0184  stfs f0, 0x184(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 82C5B090: 931F017C  stw r24, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[24].u32 ) };
	// 82C5B094: 897F02B2  lbz r11, 0x2b2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5B098: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5B09C: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82C5B0A0: 409A0008  bne cr6, 0x82c5b0a8
	if !ctx.cr[6].eq {
	pc = 0x82C5B0A8; continue 'dispatch;
	}
	// 82C5B0A4: 57CBDFFE  rlwinm r11, r30, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000001Fu64;
	// 82C5B0A8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C5B0AC: 997F02AC  stb r11, 0x2ac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[11].u8 ) };
	// 82C5B0B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5B0B4: 419A0018  beq cr6, 0x82c5b0cc
	if ctx.cr[6].eq {
	pc = 0x82C5B0CC; continue 'dispatch;
	}
	// 82C5B0B8: 897F02B4  lbz r11, 0x2b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 82C5B0BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5B0C0: 409A000C  bne cr6, 0x82c5b0cc
	if !ctx.cr[6].eq {
	pc = 0x82C5B0CC; continue 'dispatch;
	}
	// 82C5B0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B0C8: 4BFFFD69  bl 0x82c5ae30
	ctx.lr = 0x82C5B0CC;
	sub_82C5AE30(ctx, base);
	// 82C5B0CC: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5B0D0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82C5B0D4: 3957011C  addi r10, r23, 0x11c
	ctx.r[10].s64 = ctx.r[23].s64 + 284;
	// 82C5B0D8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5B0DC: 409A005C  bne cr6, 0x82c5b138
	if !ctx.cr[6].eq {
	pc = 0x82C5B138; continue 'dispatch;
	}
	// 82C5B0E0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5B0E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5B0E8: 409A0050  bne cr6, 0x82c5b138
	if !ctx.cr[6].eq {
	pc = 0x82C5B138; continue 'dispatch;
	}
	// 82C5B0EC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5B0F0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5B0F4: 409A0044  bne cr6, 0x82c5b138
	if !ctx.cr[6].eq {
	pc = 0x82C5B138; continue 'dispatch;
	}
	// 82C5B0F8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C5B0FC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C5B100: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C5B104: 409A001C  bne cr6, 0x82c5b120
	if !ctx.cr[6].eq {
	pc = 0x82C5B120; continue 'dispatch;
	}
	// 82C5B108: 930B0004  stw r24, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82C5B10C: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82C5B110: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C5B114: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C5B118: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C5B11C: 4804E328  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
	// 82C5B120: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5B124: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 82C5B128: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C5B12C: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5B130: 93E80004  stw r31, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C5B134: 93EA0004  stw r31, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C5B138: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C5B13C: 4804E308  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5B140 size=156
    let mut pc: u32 = 0x82C5B140;
    'dispatch: loop {
        match pc {
            0x82C5B140 => {
    //   block [0x82C5B140..0x82C5B1DC)
	// 82C5B140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5B144: 4804E2C5  bl 0x82ca9408
	ctx.lr = 0x82C5B148;
	sub_82CA93D0(ctx, base);
	// 82C5B148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5B14C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C5B150: 3BBE011C  addi r29, r30, 0x11c
	ctx.r[29].s64 = ctx.r[30].s64 + 284;
	// 82C5B154: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5B158: 83FE011C  lwz r31, 0x11c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) } as u64;
	// 82C5B15C: 7F845850  subf r28, r4, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82C5B160: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5B164: 419A0070  beq cr6, 0x82c5b1d4
	if ctx.cr[6].eq {
	pc = 0x82C5B1D4; continue 'dispatch;
	}
	// 82C5B168: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C5B16C: 80BE005C  lwz r5, 0x5c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5B170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B174: 4BFFD98D  bl 0x82c58b00
	ctx.lr = 0x82C5B178;
	sub_82C58B00(ctx, base);
	// 82C5B178: 897F02B2  lbz r11, 0x2b2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(690 as u32) ) } as u64;
	// 82C5B17C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5B180: 409A0010  bne cr6, 0x82c5b190
	if !ctx.cr[6].eq {
	pc = 0x82C5B190; continue 'dispatch;
	}
	// 82C5B184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B188: 809E005C  lwz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5B18C: 4BFFF2ED  bl 0x82c5a478
	ctx.lr = 0x82C5B190;
	sub_82C5A478(ctx, base);
	// 82C5B190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B194: 809E005C  lwz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5B198: 4BFFF3B9  bl 0x82c5a550
	ctx.lr = 0x82C5B19C;
	sub_82C5A550(ctx, base);
	// 82C5B19C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B1A0: 809E005C  lwz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5B1A4: 4BFFFCE5  bl 0x82c5ae88
	ctx.lr = 0x82C5B1A8;
	sub_82C5AE88(ctx, base);
	// 82C5B1A8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5B1AC: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C5B1B0: 409A0024  bne cr6, 0x82c5b1d4
	if !ctx.cr[6].eq {
	pc = 0x82C5B1D4; continue 'dispatch;
	}
	// 82C5B1B4: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C5B1B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5B1BC: 419A0018  beq cr6, 0x82c5b1d4
	if ctx.cr[6].eq {
	pc = 0x82C5B1D4; continue 'dispatch;
	}
	// 82C5B1C0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C5B1C4: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82C5B1C8: 409A000C  bne cr6, 0x82c5b1d4
	if !ctx.cr[6].eq {
	pc = 0x82C5B1D4; continue 'dispatch;
	}
	// 82C5B1CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C5B1D0: 409AFF98  bne cr6, 0x82c5b168
	if !ctx.cr[6].eq {
	pc = 0x82C5B168; continue 'dispatch;
	}
	// 82C5B1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C5B1D8: 4804E280  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5B1E0 size=152
    let mut pc: u32 = 0x82C5B1E0;
    'dispatch: loop {
        match pc {
            0x82C5B1E0 => {
    //   block [0x82C5B1E0..0x82C5B278)
	// 82C5B1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5B1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5B1E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C5B1EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5B1F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5B1F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5B1F8: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C5B1FC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C5B200: 409A001C  bne cr6, 0x82c5b21c
	if !ctx.cr[6].eq {
	pc = 0x82C5B21C; continue 'dispatch;
	}
	// 82C5B204: 4BFF6F25  bl 0x82c52128
	ctx.lr = 0x82C5B208;
	sub_82C52128(ctx, base);
	// 82C5B208: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C5B20C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C5B210: 419A000C  beq cr6, 0x82c5b21c
	if ctx.cr[6].eq {
	pc = 0x82C5B21C; continue 'dispatch;
	}
	// 82C5B214: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82C5B218: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82C5B21C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82C5B220: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82C5B224: 419A003C  beq cr6, 0x82c5b260
	if ctx.cr[6].eq {
	pc = 0x82C5B260; continue 'dispatch;
	}
	// 82C5B228: 4BFF6E59  bl 0x82c52080
	ctx.lr = 0x82C5B22C;
	sub_82C52080(ctx, base);
	// 82C5B22C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B230: 4BFFDDB1  bl 0x82c58fe0
	ctx.lr = 0x82C5B234;
	sub_82C58FE0(ctx, base);
	// 82C5B234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B238: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5B23C: 4BFFFF05  bl 0x82c5b140
	ctx.lr = 0x82C5B240;
	sub_82C5B140(ctx, base);
	// 82C5B240: 83DF005C  lwz r30, 0x5c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C5B244: 4B52D4BD  bl 0x82188700
	ctx.lr = 0x82C5B248;
	sub_82188700(ctx, base);
	// 82C5B248: 817F0144  lwz r11, 0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 82C5B24C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C5B250: 7D4B1850  subf r10, r11, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82C5B254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B258: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82C5B25C: 4BFFEB35  bl 0x82c59d90
	ctx.lr = 0x82C5B260;
	sub_82C59D90(ctx, base);
	// 82C5B260: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C5B264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5B268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5B26C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C5B270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5B274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5B278 size=72
    let mut pc: u32 = 0x82C5B278;
    'dispatch: loop {
        match pc {
            0x82C5B278 => {
    //   block [0x82C5B278..0x82C5B2C0)
	// 82C5B278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5B27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5B280: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5B284: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5B288: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5B28C: 4BFF2805  bl 0x82c4da90
	ctx.lr = 0x82C5B290;
	sub_82C4DA90(ctx, base);
	// 82C5B290: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82C5B294: 4BFF27FD  bl 0x82c4da90
	ctx.lr = 0x82C5B298;
	sub_82C4DA90(ctx, base);
	// 82C5B298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5B29C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B2A0: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82C5B2A4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C5B2A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82C5B2AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C5B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5B2B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5B2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C5B2C0 size=56
    let mut pc: u32 = 0x82C5B2C0;
    'dispatch: loop {
        match pc {
            0x82C5B2C0 => {
    //   block [0x82C5B2C0..0x82C5B2F8)
	// 82C5B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C5B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C5B2C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C5B2CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C5B2D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C5B2D4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82C5B2D8: 4BF3BBA1  bl 0x82b96e78
	ctx.lr = 0x82C5B2DC;
	sub_82B96E78(ctx, base);
	// 82C5B2DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C5B2E0: 4BF3BB99  bl 0x82b96e78
	ctx.lr = 0x82C5B2E4;
	sub_82B96E78(ctx, base);
	// 82C5B2E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C5B2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C5B2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C5B2F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C5B2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C5B2F8 size=8
    let mut pc: u32 = 0x82C5B2F8;
    'dispatch: loop {
        match pc {
            0x82C5B2F8 => {
    //   block [0x82C5B2F8..0x82C5B300)
	// 82C5B2F8: 3863002C  addi r3, r3, 0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + 44;
	// 82C5B2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C5B300 size=4
    let mut pc: u32 = 0x82C5B300;
    'dispatch: loop {
        match pc {
            0x82C5B300 => {
    //   block [0x82C5B300..0x82C5B304)
	// 82C5B300: 48001C20  b 0x82c5cf20
	sub_82C5CF20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C5B308 size=60
    let mut pc: u32 = 0x82C5B308;
    'dispatch: loop {
        match pc {
            0x82C5B308 => {
    //   block [0x82C5B308..0x82C5B344)
	// 82C5B308: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C5B30C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C5B310: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C5B314: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C5B318: 38E8D100  addi r7, r8, -0x2f00
	ctx.r[7].s64 = ctx.r[8].s64 + -12032;
	// 82C5B31C: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82C5B320: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C5B324: C1A90DB4  lfs f13, 0xdb4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3508 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C5B328: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C5B32C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C5B330: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82C5B334: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C5B338: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C5B33C: 90C30014  stw r6, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82C5B340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C5B348 size=16
    let mut pc: u32 = 0x82C5B348;
    'dispatch: loop {
        match pc {
            0x82C5B348 => {
    //   block [0x82C5B348..0x82C5B358)
	// 82C5B348: D023000C  stfs f1, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C5B34C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82C5B350: D0430010  stfs f2, 0x10(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82C5B354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C5B358 size=32
    let mut pc: u32 = 0x82C5B358;
    'dispatch: loop {
        match pc {
            0x82C5B358 => {
    //   block [0x82C5B358..0x82C5B378)
	// 82C5B358: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C5B35C: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 82C5B360: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C5B364: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82C5B368: 409A0010  bne cr6, 0x82c5b378
	if !ctx.cr[6].eq {
		sub_82C5B378(ctx, base);
		return;
	}
	// 82C5B36C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C5B370: C02B0C14  lfs f1, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5B374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C5B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C5B378 size=12
    let mut pc: u32 = 0x82C5B378;
    'dispatch: loop {
        match pc {
            0x82C5B378 => {
    //   block [0x82C5B378..0x82C5B384)
	// 82C5B378: C04B0010  lfs f2, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82C5B37C: C02B000C  lfs f1, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C5B380: 4BFFB780  b 0x82c56b00
	sub_82C56B00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


