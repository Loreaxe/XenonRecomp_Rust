pub fn sub_82446BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446BA8 size=100
    let mut pc: u32 = 0x82446BA8;
    'dispatch: loop {
        match pc {
            0x82446BA8 => {
    //   block [0x82446BA8..0x82446C0C)
	// 82446BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446BAC: 480EE511  bl 0x825350bc
	ctx.lr = 0x82446BB0;
	sub_82535080(ctx, base);
	// 82446BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82446BB8: 83DF2098  lwz r30, 0x2098(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8344 as u32) ) } as u64;
	// 82446BBC: 83BF2094  lwz r29, 0x2094(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8340 as u32) ) } as u64;
	// 82446BC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82446BC4: 48001805  bl 0x824483c8
	ctx.lr = 0x82446BC8;
	sub_824483C8(ctx, base);
	// 82446BC8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82446BCC: 419A0038  beq cr6, 0x82446c04
	if ctx.cr[6].eq {
	pc = 0x82446C04; continue 'dispatch;
	}
	// 82446BD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82446BD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446BD8: 480017F1  bl 0x824483c8
	ctx.lr = 0x82446BDC;
	sub_824483C8(ctx, base);
	// 82446BDC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82446BE0: 409A0024  bne cr6, 0x82446c04
	if !ctx.cr[6].eq {
	pc = 0x82446C04; continue 'dispatch;
	}
	// 82446BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446BE8: 4BFFF939  bl 0x82446520
	ctx.lr = 0x82446BEC;
	sub_82446520(ctx, base);
	// 82446BEC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446BF0: 419A0014  beq cr6, 0x82446c04
	if ctx.cr[6].eq {
	pc = 0x82446C04; continue 'dispatch;
	}
	// 82446BF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82446BF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82446BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446C00: 480017B1  bl 0x824483b0
	ctx.lr = 0x82446C04;
	sub_824483B0(ctx, base);
	// 82446C04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446C08: 480EE504  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446C10 size=284
    let mut pc: u32 = 0x82446C10;
    'dispatch: loop {
        match pc {
            0x82446C10 => {
    //   block [0x82446C10..0x82446D2C)
	// 82446C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446C14: 480EE499  bl 0x825350ac
	ctx.lr = 0x82446C18;
	sub_82535080(ctx, base);
	// 82446C18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82446C20: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82446C24: 3B5F1088  addi r26, r31, 0x1088
	ctx.r[26].s64 = ctx.r[31].s64 + 4232;
	// 82446C28: 837F208C  lwz r27, 0x208c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446C2C: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446C30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446C34: 4BFD3BD5  bl 0x8241a808
	ctx.lr = 0x82446C38;
	sub_8241A808(ctx, base);
	// 82446C38: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82446C3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446C40: 4BFD40C1  bl 0x8241ad00
	ctx.lr = 0x82446C44;
	sub_8241AD00(ctx, base);
	// 82446C44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82446C48: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82446C4C: 419A000C  beq cr6, 0x82446c58
	if ctx.cr[6].eq {
	pc = 0x82446C58; continue 'dispatch;
	}
	// 82446C50: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82446C54: 93CBC8C0  stw r30, -0x3740(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-14144 as u32), ctx.r[30].u32 ) };
	// 82446C58: 3880001A  li r4, 0x1a
	ctx.r[4].s64 = 26;
	// 82446C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446C60: 4BFF5AA9  bl 0x8243c708
	ctx.lr = 0x82446C64;
	sub_8243C708(ctx, base);
	// 82446C64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446C68: 409A000C  bne cr6, 0x82446c74
	if !ctx.cr[6].eq {
	pc = 0x82446C74; continue 'dispatch;
	}
	// 82446C6C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82446C70: 48000044  b 0x82446cb4
	pc = 0x82446CB4; continue 'dispatch;
	// 82446C74: 2F1EFFFE  cmpwi cr6, r30, -2
	ctx.cr[6].compare_i32(ctx.r[30].s32, -2, &mut ctx.xer);
	// 82446C78: 419A002C  beq cr6, 0x82446ca4
	if ctx.cr[6].eq {
	pc = 0x82446CA4; continue 'dispatch;
	}
	// 82446C7C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82446C80: 419A0018  beq cr6, 0x82446c98
	if ctx.cr[6].eq {
	pc = 0x82446C98; continue 'dispatch;
	}
	// 82446C84: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82446C88: 419A002C  beq cr6, 0x82446cb4
	if ctx.cr[6].eq {
	pc = 0x82446CB4; continue 'dispatch;
	}
	// 82446C8C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82446C90: 60840C07  ori r4, r4, 0xc07
	ctx.r[4].u64 = ctx.r[4].u64 | 3079;
	// 82446C94: 48000018  b 0x82446cac
	pc = 0x82446CAC; continue 'dispatch;
	// 82446C98: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82446C9C: 60840C08  ori r4, r4, 0xc08
	ctx.r[4].u64 = ctx.r[4].u64 | 3080;
	// 82446CA0: 4800000C  b 0x82446cac
	pc = 0x82446CAC; continue 'dispatch;
	// 82446CA4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82446CA8: 60840C09  ori r4, r4, 0xc09
	ctx.r[4].u64 = ctx.r[4].u64 | 3081;
	// 82446CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446CB0: 48000C59  bl 0x82447908
	ctx.lr = 0x82446CB4;
	sub_82447908(ctx, base);
	// 82446CB4: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82446CB8: 419A000C  beq cr6, 0x82446cc4
	if ctx.cr[6].eq {
	pc = 0x82446CC4; continue 'dispatch;
	}
	// 82446CBC: 2F1C0005  cmpwi cr6, r28, 5
	ctx.cr[6].compare_i32(ctx.r[28].s32, 5, &mut ctx.xer);
	// 82446CC0: 409A0018  bne cr6, 0x82446cd8
	if !ctx.cr[6].eq {
	pc = 0x82446CD8; continue 'dispatch;
	}
	// 82446CC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82446CC8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82446CCC: 4800B005  bl 0x82451cd0
	ctx.lr = 0x82446CD0;
	sub_82451CD0(ctx, base);
	// 82446CD0: 2F1C0005  cmpwi cr6, r28, 5
	ctx.cr[6].compare_i32(ctx.r[28].s32, 5, &mut ctx.xer);
	// 82446CD4: 419A000C  beq cr6, 0x82446ce0
	if ctx.cr[6].eq {
	pc = 0x82446CE0; continue 'dispatch;
	}
	// 82446CD8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82446CDC: 419A0010  beq cr6, 0x82446cec
	if ctx.cr[6].eq {
	pc = 0x82446CEC; continue 'dispatch;
	}
	// 82446CE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82446CE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446CE8: 4BFFF991  bl 0x82446678
	ctx.lr = 0x82446CEC;
	sub_82446678(ctx, base);
	// 82446CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446CF0: 4BFFF979  bl 0x82446668
	ctx.lr = 0x82446CF4;
	sub_82446668(ctx, base);
	// 82446CF4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82446CF8: 409A002C  bne cr6, 0x82446d24
	if !ctx.cr[6].eq {
	pc = 0x82446D24; continue 'dispatch;
	}
	// 82446CFC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82446D00: 409A0024  bne cr6, 0x82446d24
	if !ctx.cr[6].eq {
	pc = 0x82446D24; continue 'dispatch;
	}
	// 82446D04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446D08: 4BFD4289  bl 0x8241af90
	ctx.lr = 0x82446D0C;
	sub_8241AF90(ctx, base);
	// 82446D0C: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82446D10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82446D14: 409A0010  bne cr6, 0x82446d24
	if !ctx.cr[6].eq {
	pc = 0x82446D24; continue 'dispatch;
	}
	// 82446D18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82446D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446D20: 4BFFF959  bl 0x82446678
	ctx.lr = 0x82446D24;
	sub_82446678(ctx, base);
	// 82446D24: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82446D28: 480EE3D4  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82446D30 size=184
    let mut pc: u32 = 0x82446D30;
    'dispatch: loop {
        match pc {
            0x82446D30 => {
    //   block [0x82446D30..0x82446DE8)
	// 82446D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82446D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82446D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446D44: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82446D48: 4BFFFD59  bl 0x82446aa0
	ctx.lr = 0x82446D4C;
	sub_82446AA0(ctx, base);
	// 82446D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82446D50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82446D54: 419A007C  beq cr6, 0x82446dd0
	if ctx.cr[6].eq {
	pc = 0x82446DD0; continue 'dispatch;
	}
	// 82446D58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446D5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82446D60: 409A0070  bne cr6, 0x82446dd0
	if !ctx.cr[6].eq {
	pc = 0x82446DD0; continue 'dispatch;
	}
	// 82446D64: 8169208C  lwz r11, 0x208c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446D68: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446D6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446D70: 4BFFF919  bl 0x82446688
	ctx.lr = 0x82446D74;
	sub_82446688(ctx, base);
	// 82446D74: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446D78: 419A0058  beq cr6, 0x82446dd0
	if ctx.cr[6].eq {
	pc = 0x82446DD0; continue 'dispatch;
	}
	// 82446D7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446D80: 4BFD3B91  bl 0x8241a910
	ctx.lr = 0x82446D84;
	sub_8241A910(ctx, base);
	// 82446D84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82446D88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446D8C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82446D90: 4BFD3B09  bl 0x8241a898
	ctx.lr = 0x82446D94;
	sub_8241A898(ctx, base);
	// 82446D94: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82446D98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446D9C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82446DA0: 4BFD3BE9  bl 0x8241a988
	ctx.lr = 0x82446DA4;
	sub_8241A988(ctx, base);
	// 82446DA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82446DA8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82446DAC: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82446DB0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82446DB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82446DB8: 7D6A19D6  mullw r11, r10, r3
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82446DBC: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82446DC0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82446DC4: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82446DC8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82446DCC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82446DD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446DDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82446DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446DE8 size=124
    let mut pc: u32 = 0x82446DE8;
    'dispatch: loop {
        match pc {
            0x82446DE8 => {
    //   block [0x82446DE8..0x82446E64)
	// 82446DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82446DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82446DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82446DFC: 4BFF2EED  bl 0x82439ce8
	ctx.lr = 0x82446E00;
	sub_82439CE8(ctx, base);
	// 82446E00: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82446E04: 419A0018  beq cr6, 0x82446e1c
	if ctx.cr[6].eq {
	pc = 0x82446E1C; continue 'dispatch;
	}
	// 82446E08: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82446E0C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82446E10: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82446E14: 4BFD4905  bl 0x8241b718
	ctx.lr = 0x82446E18;
	sub_8241B718(ctx, base);
	// 82446E18: 48000008  b 0x82446e20
	pc = 0x82446E20; continue 'dispatch;
	// 82446E1C: 4BFFFAC5  bl 0x824468e0
	ctx.lr = 0x82446E20;
	sub_824468E0(ctx, base);
	// 82446E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82446E24: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82446E28: 409A0018  bne cr6, 0x82446e40
	if !ctx.cr[6].eq {
	pc = 0x82446E40; continue 'dispatch;
	}
	// 82446E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446E3C: 4E800020  blr
	return;
	// 82446E40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82446E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446E48: 4BFD3E51  bl 0x8241ac98
	ctx.lr = 0x82446E4C;
	sub_8241AC98(ctx, base);
	// 82446E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82446E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82446E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82446E5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82446E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446E68 size=132
    let mut pc: u32 = 0x82446E68;
    'dispatch: loop {
        match pc {
            0x82446E68 => {
    //   block [0x82446E68..0x82446EEC)
	// 82446E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446E6C: 480EE249  bl 0x825350b4
	ctx.lr = 0x82446E70;
	sub_82535080(ctx, base);
	// 82446E70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446E74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82446E78: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82446E7C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82446E80: 83FE208C  lwz r31, 0x208c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446E84: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446E88: 4BFF7A29  bl 0x8243e8b0
	ctx.lr = 0x82446E8C;
	sub_8243E8B0(ctx, base);
	// 82446E8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82446E90: 419A0050  beq cr6, 0x82446ee0
	if ctx.cr[6].eq {
	pc = 0x82446EE0; continue 'dispatch;
	}
	// 82446E94: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82446E98: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82446E9C: 409A0034  bne cr6, 0x82446ed0
	if !ctx.cr[6].eq {
	pc = 0x82446ED0; continue 'dispatch;
	}
	// 82446EA0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82446EA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82446EA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82446EAC: 387E1088  addi r3, r30, 0x1088
	ctx.r[3].s64 = ctx.r[30].s64 + 4232;
	// 82446EB0: 4BFFFA41  bl 0x824468f0
	ctx.lr = 0x82446EB4;
	sub_824468F0(ctx, base);
	// 82446EB4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82446EB8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446EBC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82446EC0: 40980010  bge cr6, 0x82446ed0
	if !ctx.cr[6].lt {
	pc = 0x82446ED0; continue 'dispatch;
	}
	// 82446EC4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82446EC8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82446ECC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82446ED0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82446ED4: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82446ED8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82446EDC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82446EE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446EE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82446EE8: 480EE21C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82446EF0 size=168
    let mut pc: u32 = 0x82446EF0;
    'dispatch: loop {
        match pc {
            0x82446EF0 => {
    //   block [0x82446EF0..0x82446F98)
	// 82446EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82446EF4: 480EE1C9  bl 0x825350bc
	ctx.lr = 0x82446EF8;
	sub_82535080(ctx, base);
	// 82446EF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82446EFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82446F00: 817E208C  lwz r11, 0x208c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446F04: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446F08: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82446F0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82446F10: 409A0010  bne cr6, 0x82446f20
	if !ctx.cr[6].eq {
	pc = 0x82446F20; continue 'dispatch;
	}
	// 82446F14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82446F18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446F1C: 480EE1F0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82446F20: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 82446F24: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82446F28: 394AC8A4  addi r10, r10, -0x375c
	ctx.r[10].s64 = ctx.r[10].s64 + -14172;
	// 82446F2C: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82446F30: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82446F34: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446F38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82446F3C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82446F40: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82446F44: 4200FFF0  bdnz 0x82446f34
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82446F34; continue 'dispatch;
	}
	// 82446F48: 4BFF2DA1  bl 0x82439ce8
	ctx.lr = 0x82446F4C;
	sub_82439CE8(ctx, base);
	// 82446F4C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82446F50: 419A0010  beq cr6, 0x82446f60
	if ctx.cr[6].eq {
	pc = 0x82446F60; continue 'dispatch;
	}
	// 82446F54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446F58: 4BFFFA29  bl 0x82446980
	ctx.lr = 0x82446F5C;
	sub_82446980(ctx, base);
	// 82446F5C: 48000014  b 0x82446f70
	pc = 0x82446F70; continue 'dispatch;
	// 82446F60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82446F64: 4BFFFA5D  bl 0x824469c0
	ctx.lr = 0x82446F68;
	sub_824469C0(ctx, base);
	// 82446F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446F6C: 4BFFFA3D  bl 0x824469a8
	ctx.lr = 0x82446F70;
	sub_824469A8(ctx, base);
	// 82446F70: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446F74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82446F78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82446F7C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82446F80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82446F84: 4E800421  bctrl
	ctx.lr = 0x82446F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82446F88: 48002B01  bl 0x82449a88
	ctx.lr = 0x82446F8C;
	sub_82449A88(ctx, base);
	// 82446F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82446F90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82446F94: 480EE178  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446F98 size=44
    let mut pc: u32 = 0x82446F98;
    'dispatch: loop {
        match pc {
            0x82446F98 => {
    //   block [0x82446F98..0x82446FC4)
	// 82446F98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82446F9C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82446FA0: 388B1088  addi r4, r11, 0x1088
	ctx.r[4].s64 = ctx.r[11].s64 + 4232;
	// 82446FA4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82446FA8: 814B208C  lwz r10, 0x208c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446FAC: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446FB0: 90AA002C  stw r5, 0x2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 82446FB4: 409A0010  bne cr6, 0x82446fc4
	if !ctx.cr[6].eq {
		sub_82446FC4(ctx, base);
		return;
	}
	// 82446FB8: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82446FBC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82446FC0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446FC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446FC4 size=8
    let mut pc: u32 = 0x82446FC4;
    'dispatch: loop {
        match pc {
            0x82446FC4 => {
    //   block [0x82446FC4..0x82446FCC)
	// 82446FC4: 4BFFFA2C  b 0x824469f0
	sub_824469F0(ctx, base);
	return;
	// 82446FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446FD0 size=36
    let mut pc: u32 = 0x82446FD0;
    'dispatch: loop {
        match pc {
            0x82446FD0 => {
    //   block [0x82446FD0..0x82446FF4)
	// 82446FD0: 8163208C  lwz r11, 0x208c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82446FD4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82446FD8: 38831088  addi r4, r3, 0x1088
	ctx.r[4].s64 = ctx.r[3].s64 + 4232;
	// 82446FDC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82446FE0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82446FE4: 409A0010  bne cr6, 0x82446ff4
	if !ctx.cr[6].eq {
		sub_82446FF4(ctx, base);
		return;
	}
	// 82446FE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82446FEC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82446FF0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82446FF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82446FF4 size=8
    let mut pc: u32 = 0x82446FF4;
    'dispatch: loop {
        match pc {
            0x82446FF4 => {
    //   block [0x82446FF4..0x82446FFC)
	// 82446FF4: 4BFFF9FC  b 0x824469f0
	sub_824469F0(ctx, base);
	return;
	// 82446FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447000 size=144
    let mut pc: u32 = 0x82447000;
    'dispatch: loop {
        match pc {
            0x82447000 => {
    //   block [0x82447000..0x82447090)
	// 82447000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447008: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244700C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447010: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447014: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82447018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244701C: 4BFF56ED  bl 0x8243c708
	ctx.lr = 0x82447020;
	sub_8243C708(ctx, base);
	// 82447020: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82447024: 409A000C  bne cr6, 0x82447030
	if !ctx.cr[6].eq {
	pc = 0x82447030; continue 'dispatch;
	}
	// 82447028: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244702C: 4800004C  b 0x82447078
	pc = 0x82447078; continue 'dispatch;
	// 82447030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447034: 4BFFF63D  bl 0x82446670
	ctx.lr = 0x82447038;
	sub_82446670(ctx, base);
	// 82447038: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8244703C: 419AFFEC  beq cr6, 0x82447028
	if ctx.cr[6].eq {
	pc = 0x82447028; continue 'dispatch;
	}
	// 82447040: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82447044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447048: 4BFFFA89  bl 0x82446ad0
	ctx.lr = 0x8244704C;
	sub_82446AD0(ctx, base);
	// 8244704C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82447050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447054: 4BFFFB55  bl 0x82446ba8
	ctx.lr = 0x82447058;
	sub_82446BA8(ctx, base);
	// 82447058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244705C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82447060: 4BFFFBB1  bl 0x82446c10
	ctx.lr = 0x82447064;
	sub_82446C10(ctx, base);
	// 82447064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447068: 4BFFFCC9  bl 0x82446d30
	ctx.lr = 0x8244706C;
	sub_82446D30(ctx, base);
	// 8244706C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447070: 4BFFF651  bl 0x824466c0
	ctx.lr = 0x82447074;
	sub_824466C0(ctx, base);
	// 82447074: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82447078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244707C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447084: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82447088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244708C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82447090 size=536
    let mut pc: u32 = 0x82447090;
    'dispatch: loop {
        match pc {
            0x82447090 => {
    //   block [0x82447090..0x824472A8)
	// 82447090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447094: 480EE00D  bl 0x825350a0
	ctx.lr = 0x82447098;
	sub_82535080(ctx, base);
	// 82447098: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244709C: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 824470A0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824470A4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824470A8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 824470AC: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 824470B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824470B4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 824470B8: 93560000  stw r26, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 824470BC: 83D9208C  lwz r30, 0x208c(r25)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824470C0: 3BF90D88  addi r31, r25, 0xd88
	ctx.r[31].s64 = ctx.r[25].s64 + 3464;
	// 824470C4: 4BFFFA8D  bl 0x82446b50
	ctx.lr = 0x824470C8;
	sub_82446B50(ctx, base);
	// 824470C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824470CC: 409A0044  bne cr6, 0x82447110
	if !ctx.cr[6].eq {
	pc = 0x82447110; continue 'dispatch;
	}
	// 824470D0: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824470D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824470D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824470DC: 4BFF76BD  bl 0x8243e798
	ctx.lr = 0x824470E0;
	sub_8243E798(ctx, base);
	// 824470E0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824470E4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824470E8: 419801B8  blt cr6, 0x824472a0
	if ctx.cr[6].lt {
	pc = 0x824472A0; continue 'dispatch;
	}
	// 824470EC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 824470F0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824470F4: 4BFF5615  bl 0x8243c708
	ctx.lr = 0x824470F8;
	sub_8243C708(ctx, base);
	// 824470F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824470FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447100: 409A0024  bne cr6, 0x82447124
	if !ctx.cr[6].eq {
	pc = 0x82447124; continue 'dispatch;
	}
	// 82447104: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82447108: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8244710C: 4BFF7775  bl 0x8243e880
	ctx.lr = 0x82447110;
	sub_8243E880(ctx, base);
	// 82447110: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 82447114: 396B6348  addi r11, r11, 0x6348
	ctx.r[11].s64 = ctx.r[11].s64 + 25416;
	// 82447118: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8244711C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82447120: 480EDFD0  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 82447124: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82447128: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8244712C: 4BFF76D5  bl 0x8243e800
	ctx.lr = 0x82447130;
	sub_8243E800(ctx, base);
	// 82447130: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82447134: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82447138: 41980168  blt cr6, 0x824472a0
	if ctx.cr[6].lt {
	pc = 0x824472A0; continue 'dispatch;
	}
	// 8244713C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82447140: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82447144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447148: 4BFF7739  bl 0x8243e880
	ctx.lr = 0x8244714C;
	sub_8243E880(ctx, base);
	// 8244714C: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82447150: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82447154: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82447158: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244715C: 419800D8  blt cr6, 0x82447234
	if ctx.cr[6].lt {
	pc = 0x82447234; continue 'dispatch;
	}
	// 82447160: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82447164: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82447168: 7D4B0194  addze r10, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8244716C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82447170: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82447174: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82447178: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8244717C: 555F083C  slwi r31, r10, 1
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82447180: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82447184: 40990064  ble cr6, 0x824471e8
	if !ctx.cr[6].gt {
	pc = 0x824471E8; continue 'dispatch;
	}
	// 82447188: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244718C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82447190: 555C083C  slwi r28, r10, 1
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82447194: 7D57E3D6  divw r10, r23, r28
	ctx.r[10].s32 = ctx.r[23].s32 / ctx.r[28].s32;
	// 82447198: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8244719C: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824471A0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824471A4: 557D083C  slwi r29, r11, 1
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824471A8: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824471AC: 41980008  blt cr6, 0x824471b4
	if ctx.cr[6].lt {
	pc = 0x824471B4; continue 'dispatch;
	}
	// 824471B0: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 824471B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824471B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824471BC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824471C0: 4BFFF219  bl 0x824463d8
	ctx.lr = 0x824471C4;
	sub_824463D8(ctx, base);
	// 824471C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824471C8: 7D7DF850  subf r11, r29, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 824471CC: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 824471D0: 7D3AE3D6  divw r9, r26, r28
	ctx.r[9].s32 = ctx.r[26].s32 / ctx.r[28].s32;
	// 824471D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824471D8: 552B2834  slwi r11, r9, 5
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824471DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824471E0: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824471E4: 41990030  bgt cr6, 0x82447214
	if ctx.cr[6].gt {
	pc = 0x82447214; continue 'dispatch;
	}
	// 824471E8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824471EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824471F0: 419A0024  beq cr6, 0x82447214
	if ctx.cr[6].eq {
	pc = 0x82447214; continue 'dispatch;
	}
	// 824471F4: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824471F8: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 824471FC: 396B6348  addi r11, r11, 0x6348
	ctx.r[11].s64 = ctx.r[11].s64 + 25416;
	// 82447200: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82447204: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82447208: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8244720C: 4BFD329D  bl 0x8241a4a8
	ctx.lr = 0x82447210;
	sub_8241A4A8(ctx, base);
	// 82447210: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82447214: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82447218: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244721C: 419A0070  beq cr6, 0x8244728c
	if ctx.cr[6].eq {
	pc = 0x8244728C; continue 'dispatch;
	}
	// 82447220: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82447224: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82447228: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8244722C: 4BFF5525  bl 0x8243c750
	ctx.lr = 0x82447230;
	sub_8243C750(ctx, base);
	// 82447230: 4800005C  b 0x8244728c
	pc = 0x8244728C; continue 'dispatch;
	// 82447234: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82447238: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244723C: 419A0050  beq cr6, 0x8244728c
	if ctx.cr[6].eq {
	pc = 0x8244728C; continue 'dispatch;
	}
	// 82447240: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82447244: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82447248: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8244724C: 557F2834  slwi r31, r11, 5
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82447250: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82447254: 4099002C  ble cr6, 0x82447280
	if !ctx.cr[6].gt {
	pc = 0x82447280; continue 'dispatch;
	}
	// 82447258: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8244725C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82447260: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82447264: 4BFFF1E5  bl 0x82446448
	ctx.lr = 0x82447268;
	sub_82446448(ctx, base);
	// 82447268: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8244726C: 7D63F850  subf r11, r3, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82447270: 7D435050  subf r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 82447274: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82447278: 915E0038  stw r10, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8244727C: 41990010  bgt cr6, 0x8244728c
	if ctx.cr[6].gt {
	pc = 0x8244728C; continue 'dispatch;
	}
	// 82447280: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 82447284: 396B6348  addi r11, r11, 0x6348
	ctx.r[11].s64 = ctx.r[11].s64 + 25416;
	// 82447288: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8244728C: 93560000  stw r26, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82447290: 7F4B07B4  extsw r11, r26
	ctx.r[11].s64 = ctx.r[26].s32 as i64;
	// 82447294: E95909C8  ld r10, 0x9c8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[25].u32.wrapping_add(2504 as u32) ) };
	// 82447298: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244729C: F97909C8  std r11, 0x9c8(r25)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[25].u32.wrapping_add(2504 as u32), ctx.r[11].u64 ) };
	// 824472A0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824472A4: 480EDE4C  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824472A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824472A8 size=164
    let mut pc: u32 = 0x824472A8;
    'dispatch: loop {
        match pc {
            0x824472A8 => {
    //   block [0x824472A8..0x8244734C)
	// 824472A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824472AC: 480EDE09  bl 0x825350b4
	ctx.lr = 0x824472B0;
	sub_82535080(ctx, base);
	// 824472B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824472B4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824472B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824472BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824472C0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824472C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824472C8: 2F1E0120  cmpwi cr6, r30, 0x120
	ctx.cr[6].compare_i32(ctx.r[30].s32, 288, &mut ctx.xer);
	// 824472CC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824472D0: 837F208C  lwz r27, 0x208c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824472D4: 41980070  blt cr6, 0x82447344
	if ctx.cr[6].lt {
	pc = 0x82447344; continue 'dispatch;
	}
	// 824472D8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824472DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824472E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824472E4: 4BFD313D  bl 0x8241a420
	ctx.lr = 0x824472E8;
	sub_8241A420(ctx, base);
	// 824472E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824472EC: 419A000C  beq cr6, 0x824472f8
	if ctx.cr[6].eq {
	pc = 0x824472F8; continue 'dispatch;
	}
	// 824472F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824472F4: 48000030  b 0x82447324
	pc = 0x82447324; continue 'dispatch;
	// 824472F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824472FC: 48002DA5  bl 0x8244a0a0
	ctx.lr = 0x82447300;
	sub_8244A0A0(ctx, base);
	// 82447300: 2F03006C  cmpwi cr6, r3, 0x6c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 108, &mut ctx.xer);
	// 82447304: 4198000C  blt cr6, 0x82447310
	if ctx.cr[6].lt {
	pc = 0x82447310; continue 'dispatch;
	}
	// 82447308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244730C: 48000018  b 0x82447324
	pc = 0x82447324; continue 'dispatch;
	// 82447310: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82447314: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82447318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244731C: 4BFFF13D  bl 0x82446458
	ctx.lr = 0x82447320;
	sub_82446458(ctx, base);
	// 82447320: 7D7D1850  subf r11, r29, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[29].s64;
	// 82447324: 3D408244  lis r10, -0x7dbc
	ctx.r[10].s64 = -2109472768;
	// 82447328: 392A7090  addi r9, r10, 0x7090
	ctx.r[9].s64 = ctx.r[10].s64 + 28816;
	// 8244732C: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 82447330: 913B003C  stw r9, 0x3c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82447334: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82447338: E97F09C8  ld r11, 0x9c8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2504 as u32) ) };
	// 8244733C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82447340: F97F09C8  std r11, 0x9c8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2504 as u32), ctx.r[11].u64 ) };
	// 82447344: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82447348: 480EDDBC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447350 size=264
    let mut pc: u32 = 0x82447350;
    'dispatch: loop {
        match pc {
            0x82447350 => {
    //   block [0x82447350..0x82447458)
	// 82447350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447354: 480EDD61  bl 0x825350b4
	ctx.lr = 0x82447358;
	sub_82535080(ctx, base);
	// 82447358: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244735C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82447360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82447368: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8244736C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82447370: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82447374: 839E208C  lwz r28, 0x208c(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8332 as u32) ) } as u64;
	// 82447378: 48002D29  bl 0x8244a0a0
	ctx.lr = 0x8244737C;
	sub_8244A0A0(ctx, base);
	// 8244737C: 2F03006C  cmpwi cr6, r3, 0x6c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 108, &mut ctx.xer);
	// 82447380: 41980018  blt cr6, 0x82447398
	if ctx.cr[6].lt {
	pc = 0x82447398; continue 'dispatch;
	}
	// 82447384: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 82447388: 396B72A8  addi r11, r11, 0x72a8
	ctx.r[11].s64 = ctx.r[11].s64 + 29352;
	// 8244738C: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82447390: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82447394: 480EDD70  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82447398: 38BFFFEE  addi r5, r31, -0x12
	ctx.r[5].s64 = ctx.r[31].s64 + -18;
	// 8244739C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824473A0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824473A4: 4099005C  ble cr6, 0x82447400
	if !ctx.cr[6].gt {
	pc = 0x82447400; continue 'dispatch;
	}
	// 824473A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824473AC: 38CB627C  addi r6, r11, 0x627c
	ctx.r[6].s64 = ctx.r[11].s64 + 25212;
	// 824473B0: 7D69EA14  add r11, r9, r29
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 824473B4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 824473B8: 38EB0012  addi r7, r11, 0x12
	ctx.r[7].s64 = ctx.r[11].s64 + 18;
	// 824473BC: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824473C0: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824473C4: 7D044051  subf. r8, r4, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824473C8: 40820014  bne 0x824473dc
	if !ctx.cr[0].eq {
	pc = 0x824473DC; continue 'dispatch;
	}
	// 824473CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824473D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824473D4: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 824473D8: 409AFFE4  bne cr6, 0x824473bc
	if !ctx.cr[6].eq {
	pc = 0x824473BC; continue 'dispatch;
	}
	// 824473DC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824473E0: 409A0014  bne cr6, 0x824473f4
	if !ctx.cr[6].eq {
	pc = 0x824473F4; continue 'dispatch;
	}
	// 824473E4: 39290012  addi r9, r9, 0x12
	ctx.r[9].s64 = ctx.r[9].s64 + 18;
	// 824473E8: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 824473EC: 4198FFC4  blt cr6, 0x824473b0
	if ctx.cr[6].lt {
	pc = 0x824473B0; continue 'dispatch;
	}
	// 824473F0: 48000010  b 0x82447400
	pc = 0x82447400; continue 'dispatch;
	// 824473F4: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824473F8: 396B72A8  addi r11, r11, 0x72a8
	ctx.r[11].s64 = ctx.r[11].s64 + 29352;
	// 824473FC: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82447400: 913B0000  stw r9, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82447404: 7D2B07B4  extsw r11, r9
	ctx.r[11].s64 = ctx.r[9].s32 as i64;
	// 82447408: E95E09C8  ld r10, 0x9c8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(2504 as u32) ) };
	// 8244740C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82447410: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82447414: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82447418: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244741C: F97E09C8  std r11, 0x9c8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(2504 as u32), ctx.r[11].u64 ) };
	// 82447420: 4BFFF731  bl 0x82446b50
	ctx.lr = 0x82447424;
	sub_82446B50(ctx, base);
	// 82447424: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82447428: 409A0028  bne cr6, 0x82447450
	if !ctx.cr[6].eq {
	pc = 0x82447450; continue 'dispatch;
	}
	// 8244742C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82447430: 815C0038  lwz r10, 0x38(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82447434: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82447438: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8244743C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82447440: 7D695BD6  divw r11, r9, r11
	ctx.r[11].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 82447444: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82447448: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244744C: 917C0038  stw r11, 0x38(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82447450: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82447454: 480EDCB0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447458 size=72
    let mut pc: u32 = 0x82447458;
    'dispatch: loop {
        match pc {
            0x82447458 => {
    //   block [0x82447458..0x824474A0)
	// 82447458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447460: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447464: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447468: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244746C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82447470: 388B6E68  addi r4, r11, 0x6e68
	ctx.r[4].s64 = ctx.r[11].s64 + 28264;
	// 82447474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82447478: 4BFF75A1  bl 0x8243ea18
	ctx.lr = 0x8244747C;
	sub_8243EA18(ctx, base);
	// 8244747C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82447480: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82447484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447488: 4BFF52C9  bl 0x8243c750
	ctx.lr = 0x8244748C;
	sub_8243C750(ctx, base);
	// 8244748C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244749C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824474A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824474A0 size=40
    let mut pc: u32 = 0x824474A0;
    'dispatch: loop {
        match pc {
            0x824474A0 => {
    //   block [0x824474A0..0x824474C8)
	// 824474A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824474A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824474A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824474AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824474B0: 4BFFFAE9  bl 0x82446f98
	ctx.lr = 0x824474B4;
	sub_82446F98(ctx, base);
	// 824474B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824474B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824474BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824474C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824474C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824474C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824474C8 size=220
    let mut pc: u32 = 0x824474C8;
    'dispatch: loop {
        match pc {
            0x824474C8 => {
    //   block [0x824474C8..0x824475A4)
	// 824474C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824474CC: 480EDBE9  bl 0x825350b4
	ctx.lr = 0x824474D0;
	sub_82535080(ctx, base);
	// 824474D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824474D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824474D8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824474DC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824474E0: 83DF208C  lwz r30, 0x208c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824474E4: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824474E8: 4198009C  blt cr6, 0x82447584
	if ctx.cr[6].lt {
	pc = 0x82447584; continue 'dispatch;
	}
	// 824474EC: 419A0080  beq cr6, 0x8244756c
	if ctx.cr[6].eq {
	pc = 0x8244756C; continue 'dispatch;
	}
	// 824474F0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824474F4: 409800A4  bge cr6, 0x82447598
	if !ctx.cr[6].lt {
	pc = 0x82447598; continue 'dispatch;
	}
	// 824474F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824474FC: 4BFFF18D  bl 0x82446688
	ctx.lr = 0x82447500;
	sub_82446688(ctx, base);
	// 82447500: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82447504: 419A0094  beq cr6, 0x82447598
	if ctx.cr[6].eq {
	pc = 0x82447598; continue 'dispatch;
	}
	// 82447508: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244750C: 4BFD3405  bl 0x8241a910
	ctx.lr = 0x82447510;
	sub_8241A910(ctx, base);
	// 82447510: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82447514: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82447518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244751C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82447520: 4BFFF519  bl 0x82446a38
	ctx.lr = 0x82447524;
	sub_82446A38(ctx, base);
	// 82447524: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82447528: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244752C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82447530: 7F9B5A14  add r28, r27, r11
	ctx.r[28].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82447534: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82447538: 4BFD4101  bl 0x8241b638
	ctx.lr = 0x8244753C;
	sub_8241B638(ctx, base);
	// 8244753C: 7D63E050  subf r11, r3, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[3].s64;
	// 82447540: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82447544: 387F1088  addi r3, r31, 0x1088
	ctx.r[3].s64 = ctx.r[31].s64 + 4232;
	// 82447548: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8244754C: 7F6B07B4  extsw r11, r27
	ctx.r[11].s64 = ctx.r[27].s32 as i64;
	// 82447550: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82447554: 7FAB07B4  extsw r11, r29
	ctx.r[11].s64 = ctx.r[29].s32 as i64;
	// 82447558: F9610068  std r11, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 8244755C: 4800A77D  bl 0x82451cd8
	ctx.lr = 0x82447560;
	sub_82451CD8(ctx, base);
	// 82447560: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447564: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82447568: 480EDB9C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8244756C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82447570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447574: 4BFFFA5D  bl 0x82446fd0
	ctx.lr = 0x82447578;
	sub_82446FD0(ctx, base);
	// 82447578: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244757C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82447580: 480EDB84  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82447584: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8244758C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447590: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82447594: 4BFFFA3D  bl 0x82446fd0
	ctx.lr = 0x82447598;
	sub_82446FD0(ctx, base);
	// 82447598: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244759C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824475A0: 480EDB64  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824475A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824475A8 size=164
    let mut pc: u32 = 0x824475A8;
    'dispatch: loop {
        match pc {
            0x824475A8 => {
    //   block [0x824475A8..0x8244764C)
	// 824475A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824475AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824475B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824475B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824475B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824475BC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824475C0: 4BFFF4E1  bl 0x82446aa0
	ctx.lr = 0x824475C4;
	sub_82446AA0(ctx, base);
	// 824475C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824475C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824475CC: 419A0064  beq cr6, 0x82447630
	if ctx.cr[6].eq {
	pc = 0x82447630; continue 'dispatch;
	}
	// 824475D0: 83E9208C  lwz r31, 0x208c(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8332 as u32) ) } as u64;
	// 824475D4: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 824475D8: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824475DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824475E0: 409A0050  bne cr6, 0x82447630
	if !ctx.cr[6].eq {
	pc = 0x82447630; continue 'dispatch;
	}
	// 824475E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824475E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824475EC: 419A0044  beq cr6, 0x82447630
	if ctx.cr[6].eq {
	pc = 0x82447630; continue 'dispatch;
	}
	// 824475F0: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 824475F4: 80CB0014  lwz r6, 0x14(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824475F8: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824475FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82447600: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82447604: 816A06D4  lwz r11, 0x6d4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1748 as u32) ) } as u64;
	// 82447608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244760C: 4E800421  bctrl
	ctx.lr = 0x82447610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82447610: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82447614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82447618: 4BFD39B1  bl 0x8241afc8
	ctx.lr = 0x8244761C;
	sub_8241AFC8(ctx, base);
	// 8244761C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 82447620: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82447624: 396B7350  addi r11, r11, 0x7350
	ctx.r[11].s64 = ctx.r[11].s64 + 29520;
	// 82447628: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8244762C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82447630: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447634: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82447638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244763C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447640: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82447644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447650 size=4
    let mut pc: u32 = 0x82447650;
    'dispatch: loop {
        match pc {
            0x82447650 => {
    //   block [0x82447650..0x82447654)
	// 82447650: 4BFFF9B0  b 0x82447000
	sub_82447000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447658 size=296
    let mut pc: u32 = 0x82447658;
    'dispatch: loop {
        match pc {
            0x82447658 => {
    //   block [0x82447658..0x82447780)
	// 82447658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244765C: 480EDA5D  bl 0x825350b8
	ctx.lr = 0x82447660;
	sub_82535080(ctx, base);
	// 82447660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447664: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82447668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244766C: 4BFF509D  bl 0x8243c708
	ctx.lr = 0x82447670;
	sub_8243C708(ctx, base);
	// 82447670: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82447674: 419A0100  beq cr6, 0x82447774
	if ctx.cr[6].eq {
	pc = 0x82447774; continue 'dispatch;
	}
	// 82447678: 3BBF2550  addi r29, r31, 0x2550
	ctx.r[29].s64 = ctx.r[31].s64 + 9552;
	// 8244767C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447680: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82447684: 3B9D0008  addi r28, r29, 8
	ctx.r[28].s64 = ctx.r[29].s64 + 8;
	// 82447688: 93BF208C  stw r29, 0x208c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8332 as u32), ctx.r[29].u32 ) };
	// 8244768C: 4BFFF08D  bl 0x82446718
	ctx.lr = 0x82447690;
	sub_82446718(ctx, base);
	// 82447690: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82447694: 409A00E4  bne cr6, 0x82447778
	if !ctx.cr[6].eq {
	pc = 0x82447778; continue 'dispatch;
	}
	// 82447698: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244769C: 4BFFF74D  bl 0x82446de8
	ctx.lr = 0x824476A0;
	sub_82446DE8(ctx, base);
	// 824476A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824476A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824476A8: 409A0018  bne cr6, 0x824476c0
	if !ctx.cr[6].eq {
	pc = 0x824476C0; continue 'dispatch;
	}
	// 824476AC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824476B0: 60840C04  ori r4, r4, 0xc04
	ctx.r[4].u64 = ctx.r[4].u64 | 3076;
	// 824476B4: 48000255  bl 0x82447908
	ctx.lr = 0x824476B8;
	sub_82447908(ctx, base);
	// 824476B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824476BC: 480EDA4C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 824476C0: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824476C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824476C8: 388B6298  addi r4, r11, 0x6298
	ctx.r[4].s64 = ctx.r[11].s64 + 25240;
	// 824476CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824476D0: 4BFD3991  bl 0x8241b060
	ctx.lr = 0x824476D4;
	sub_8241B060(ctx, base);
	// 824476D4: 80BC0004  lwz r5, 4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824476D8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824476DC: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824476E0: 4BFDBD69  bl 0x82423448
	ctx.lr = 0x824476E4;
	sub_82423448(ctx, base);
	// 824476E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824476E8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824476EC: 409A0018  bne cr6, 0x82447704
	if !ctx.cr[6].eq {
	pc = 0x82447704; continue 'dispatch;
	}
	// 824476F0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824476F4: 60840C05  ori r4, r4, 0xc05
	ctx.r[4].u64 = ctx.r[4].u64 | 3077;
	// 824476F8: 48000211  bl 0x82447908
	ctx.lr = 0x824476FC;
	sub_82447908(ctx, base);
	// 824476FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82447700: 480EDA08  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82447704: 397F2604  addi r11, r31, 0x2604
	ctx.r[11].s64 = ctx.r[31].s64 + 9732;
	// 82447708: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8244770C: 3CC08244  lis r6, -0x7dbc
	ctx.r[6].s64 = -2109472768;
	// 82447710: 909D0004  stw r4, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82447714: 3CE08244  lis r7, -0x7dbc
	ctx.r[7].s64 = -2109472768;
	// 82447718: 3D008244  lis r8, -0x7dbc
	ctx.r[8].s64 = -2109472768;
	// 8244771C: 3D208244  lis r9, -0x7dbc
	ctx.r[9].s64 = -2109472768;
	// 82447720: 3D408244  lis r10, -0x7dbc
	ctx.r[10].s64 = -2109472768;
	// 82447724: 917F219C  stw r11, 0x219c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8604 as u32), ctx.r[11].u32 ) };
	// 82447728: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244772C: 38C66048  addi r6, r6, 0x6048
	ctx.r[6].s64 = ctx.r[6].s64 + 24648;
	// 82447730: 38E76058  addi r7, r7, 0x6058
	ctx.r[7].s64 = ctx.r[7].s64 + 24664;
	// 82447734: 39086068  addi r8, r8, 0x6068
	ctx.r[8].s64 = ctx.r[8].s64 + 24680;
	// 82447738: 39296078  addi r9, r9, 0x6078
	ctx.r[9].s64 = ctx.r[9].s64 + 24696;
	// 8244773C: 394A6088  addi r10, r10, 0x6088
	ctx.r[10].s64 = ctx.r[10].s64 + 24712;
	// 82447740: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82447744: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82447748: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8244774C: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82447750: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82447754: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82447758: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8244775C: 4BFD4055  bl 0x8241b7b0
	ctx.lr = 0x82447760;
	sub_8241B7B0(ctx, base);
	// 82447760: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82447764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447768: 4BFFF831  bl 0x82446f98
	ctx.lr = 0x8244776C;
	sub_82446F98(ctx, base);
	// 8244776C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447770: 4BFFFCE9  bl 0x82447458
	ctx.lr = 0x82447774;
	sub_82447458(ctx, base);
	// 82447774: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244777C: 480ED98C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447780 size=44
    let mut pc: u32 = 0x82447780;
    'dispatch: loop {
        match pc {
            0x82447780 => {
    //   block [0x82447780..0x824477AC)
	// 82447780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244778C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82447790: 80831FCC  lwz r4, 0x1fcc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8140 as u32) ) } as u64;
	// 82447794: 48000C1D  bl 0x824483b0
	ctx.lr = 0x82447798;
	sub_824483B0(ctx, base);
	// 82447798: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244779C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824477A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824477A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824477A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824477B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824477B0 size=12
    let mut pc: u32 = 0x824477B0;
    'dispatch: loop {
        match pc {
            0x824477B0 => {
    //   block [0x824477B0..0x824477BC)
	// 824477B0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824477B4: 80831FCC  lwz r4, 0x1fcc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8140 as u32) ) } as u64;
	// 824477B8: 480012D0  b 0x82448a88
	sub_82448A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824477C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824477C0 size=20
    let mut pc: u32 = 0x824477C0;
    'dispatch: loop {
        match pc {
            0x824477C0 => {
    //   block [0x824477C0..0x824477D4)
	// 824477C0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824477C4: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824477C8: 80831FCC  lwz r4, 0x1fcc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8140 as u32) ) } as u64;
	// 824477CC: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824477D0: 480015C8  b 0x82448d98
	sub_82448D98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824477D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824477D8 size=12
    let mut pc: u32 = 0x824477D8;
    'dispatch: loop {
        match pc {
            0x824477D8 => {
    //   block [0x824477D8..0x824477E4)
	// 824477D8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824477DC: 60840501  ori r4, r4, 0x501
	ctx.r[4].u64 = ctx.r[4].u64 | 1281;
	// 824477E0: 48000128  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824477E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824477E8 size=16
    let mut pc: u32 = 0x824477E8;
    'dispatch: loop {
        match pc {
            0x824477E8 => {
    //   block [0x824477E8..0x824477F8)
	// 824477E8: 3964C780  addi r11, r4, -0x3880
	ctx.r[11].s64 = ctx.r[4].s64 + -14464;
	// 824477EC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824477F0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824477F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824477F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824477F8 size=16
    let mut pc: u32 = 0x824477F8;
    'dispatch: loop {
        match pc {
            0x824477F8 => {
    //   block [0x824477F8..0x82447808)
	// 824477F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824477FC: 91630204  stw r11, 0x204(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(516 as u32), ctx.r[11].u32 ) };
	// 82447800: 91630208  stw r11, 0x208(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(520 as u32), ctx.r[11].u32 ) };
	// 82447804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447808 size=24
    let mut pc: u32 = 0x82447808;
    'dispatch: loop {
        match pc {
            0x82447808 => {
    //   block [0x82447808..0x82447820)
	// 82447808: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244780C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82447810: 409A0008  bne cr6, 0x82447818
	if !ctx.cr[6].eq {
	pc = 0x82447818; continue 'dispatch;
	}
	// 82447814: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82447818: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8244781C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447820 size=12
    let mut pc: u32 = 0x82447820;
    'dispatch: loop {
        match pc {
            0x82447820 => {
    //   block [0x82447820..0x8244782C)
	// 82447820: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82447828: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244782C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244782C size=12
    let mut pc: u32 = 0x8244782C;
    'dispatch: loop {
        match pc {
            0x8244782C => {
    //   block [0x8244782C..0x82447838)
	// 8244782C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82447830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82447834: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447838 size=4
    let mut pc: u32 = 0x82447838;
    'dispatch: loop {
        match pc {
            0x82447838 => {
    //   block [0x82447838..0x8244783C)
	// 82447838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447840 size=16
    let mut pc: u32 = 0x82447840;
    'dispatch: loop {
        match pc {
            0x82447840 => {
    //   block [0x82447840..0x82447850)
	// 82447840: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82447844: 409A000C  bne cr6, 0x82447850
	if !ctx.cr[6].eq {
		sub_82447850(ctx, base);
		return;
	}
	// 82447848: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244784C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447850 size=28
    let mut pc: u32 = 0x82447850;
    'dispatch: loop {
        match pc {
            0x82447850 => {
    //   block [0x82447850..0x8244786C)
	// 82447850: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82447854: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82447858: 419AFFF0  beq cr6, 0x82447848
	if ctx.cr[6].eq {
		sub_82447840(ctx, base);
		return;
	}
	// 8244785C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82447860: 906B06D0  stw r3, 0x6d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1744 as u32), ctx.r[3].u32 ) };
	// 82447864: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447870 size=4
    let mut pc: u32 = 0x82447870;
    'dispatch: loop {
        match pc {
            0x82447870 => {
    //   block [0x82447870..0x82447874)
	// 82447870: 4BFDB4F8  b 0x82422d68
	sub_82422D68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447878 size=40
    let mut pc: u32 = 0x82447878;
    'dispatch: loop {
        match pc {
            0x82447878 => {
    //   block [0x82447878..0x824478A0)
	// 82447878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244787C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447884: 4BFF2825  bl 0x8243a0a8
	ctx.lr = 0x82447888;
	sub_8243A0A8(ctx, base);
	// 82447888: 480025B9  bl 0x82449e40
	ctx.lr = 0x8244788C;
	sub_82449E40(ctx, base);
	// 8244788C: 4BFFC8DD  bl 0x82444168
	ctx.lr = 0x82447890;
	sub_82444168(ctx, base);
	// 82447890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244789C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824478A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824478A0 size=60
    let mut pc: u32 = 0x824478A0;
    'dispatch: loop {
        match pc {
            0x824478A0 => {
    //   block [0x824478A0..0x824478DC)
	// 824478A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824478A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824478A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824478AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824478B0: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824478B4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 824478B8: 3BEBC8C4  addi r31, r11, -0x373c
	ctx.r[31].s64 = ctx.r[11].s64 + -14140;
	// 824478BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824478C0: 4BFE5599  bl 0x8242ce58
	ctx.lr = 0x824478C4;
	sub_8242CE58(ctx, base);
	// 824478C4: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 824478C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824478CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824478D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824478D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824478D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824478E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824478E0 size=12
    let mut pc: u32 = 0x824478E0;
    'dispatch: loop {
        match pc {
            0x824478E0 => {
    //   block [0x824478E0..0x824478EC)
	// 824478E0: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824478E4: 806BC8E8  lwz r3, -0x3718(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14104 as u32) ) } as u64;
	// 824478E8: 4BFE56E0  b 0x8242cfc8
	sub_8242CFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824478F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824478F0 size=12
    let mut pc: u32 = 0x824478F0;
    'dispatch: loop {
        match pc {
            0x824478F0 => {
    //   block [0x824478F0..0x824478FC)
	// 824478F0: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824478F4: 806BC8E8  lwz r3, -0x3718(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14104 as u32) ) } as u64;
	// 824478F8: 4BFE5768  b 0x8242d060
	sub_8242D060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447900 size=4
    let mut pc: u32 = 0x82447900;
    'dispatch: loop {
        match pc {
            0x82447900 => {
    //   block [0x82447900..0x82447904)
	// 82447900: 48008598  b 0x8244fe98
	sub_8244FE98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447908 size=132
    let mut pc: u32 = 0x82447908;
    'dispatch: loop {
        match pc {
            0x82447908 => {
    //   block [0x82447908..0x8244798C)
	// 82447908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82447914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244791C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82447920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82447924: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82447928: 409A000C  bne cr6, 0x82447934
	if !ctx.cr[6].eq {
	pc = 0x82447934; continue 'dispatch;
	}
	// 8244792C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447930: 48000044  b 0x82447974
	pc = 0x82447974; continue 'dispatch;
	// 82447934: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82447938: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244793C: 409A0018  bne cr6, 0x82447954
	if !ctx.cr[6].eq {
	pc = 0x82447954; continue 'dispatch;
	}
	// 82447940: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82447944: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 82447948: 386B019C  addi r3, r11, 0x19c
	ctx.r[3].s64 = ctx.r[11].s64 + 412;
	// 8244794C: 4BFFFEBD  bl 0x82447808
	ctx.lr = 0x82447950;
	sub_82447808(ctx, base);
	// 82447950: 48000020  b 0x82447970
	pc = 0x82447970; continue 'dispatch;
	// 82447954: 387F09F8  addi r3, r31, 0x9f8
	ctx.r[3].s64 = ctx.r[31].s64 + 2552;
	// 82447958: 4BFFFEB1  bl 0x82447808
	ctx.lr = 0x8244795C;
	sub_82447808(ctx, base);
	// 8244795C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82447960: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82447964: 4099000C  ble cr6, 0x82447970
	if !ctx.cr[6].gt {
	pc = 0x82447970; continue 'dispatch;
	}
	// 82447968: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8244796C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82447970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82447974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82447978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244797C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82447984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447990 size=116
    let mut pc: u32 = 0x82447990;
    'dispatch: loop {
        match pc {
            0x82447990 => {
    //   block [0x82447990..0x82447A04)
	// 82447990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244799C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824479A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824479A4: 409A0014  bne cr6, 0x824479b8
	if !ctx.cr[6].eq {
	pc = 0x824479B8; continue 'dispatch;
	}
	// 824479A8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824479AC: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 824479B0: 386B019C  addi r3, r11, 0x19c
	ctx.r[3].s64 = ctx.r[11].s64 + 412;
	// 824479B4: 48000038  b 0x824479ec
	pc = 0x824479EC; continue 'dispatch;
	// 824479B8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 824479BC: 4BFFFE85  bl 0x82447840
	ctx.lr = 0x824479C0;
	sub_82447840(ctx, base);
	// 824479C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824479C4: 419A0024  beq cr6, 0x824479e8
	if ctx.cr[6].eq {
	pc = 0x824479E8; continue 'dispatch;
	}
	// 824479C8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824479CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824479D0: 60840101  ori r4, r4, 0x101
	ctx.r[4].u64 = ctx.r[4].u64 | 257;
	// 824479D4: 4BFFFF35  bl 0x82447908
	ctx.lr = 0x824479D8;
	sub_82447908(ctx, base);
	// 824479D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824479DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824479E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824479E4: 4E800020  blr
	return;
	// 824479E8: 386A09F8  addi r3, r10, 0x9f8
	ctx.r[3].s64 = ctx.r[10].s64 + 2552;
	// 824479EC: 48003245  bl 0x8244ac30
	ctx.lr = 0x824479F0;
	sub_8244AC30(ctx, base);
	// 824479F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824479F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824479F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824479FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447A08 size=184
    let mut pc: u32 = 0x82447A08;
    'dispatch: loop {
        match pc {
            0x82447A08 => {
    //   block [0x82447A08..0x82447AC0)
	// 82447A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447A10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82447A14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447A18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447A1C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82447A20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82447A24: 3BEB04A0  addi r31, r11, 0x4a0
	ctx.r[31].s64 = ctx.r[11].s64 + 1184;
	// 82447A28: 38A0008B  li r5, 0x8b
	ctx.r[5].s64 = 139;
	// 82447A2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82447A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447A34: 480021BD  bl 0x82449bf0
	ctx.lr = 0x82447A38;
	sub_82449BF0(ctx, base);
	// 82447A38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82447A3C: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 82447A40: 388B5DB8  addi r4, r11, 0x5db8
	ctx.r[4].s64 = ctx.r[11].s64 + 23992;
	// 82447A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447A48: 480023F1  bl 0x82449e38
	ctx.lr = 0x82447A4C;
	sub_82449E38(ctx, base);
	// 82447A4C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82447A50: 387F019C  addi r3, r31, 0x19c
	ctx.r[3].s64 = ctx.r[31].s64 + 412;
	// 82447A54: F97F0190  std r11, 0x190(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u64 ) };
	// 82447A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447A5C: 917F0198  stw r11, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[11].u32 ) };
	// 82447A60: 4BFFFEA1  bl 0x82447900
	ctx.lr = 0x82447A64;
	sub_82447900(ctx, base);
	// 82447A64: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 82447A68: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82447A6C: 4BFF6A3D  bl 0x8243e4a8
	ctx.lr = 0x82447A70;
	sub_8243E4A8(ctx, base);
	// 82447A70: 387F01C4  addi r3, r31, 0x1c4
	ctx.r[3].s64 = ctx.r[31].s64 + 452;
	// 82447A74: 48000B05  bl 0x82448578
	ctx.lr = 0x82447A78;
	sub_82448578(ctx, base);
	// 82447A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447A7C: 4BFFFD7D  bl 0x824477f8
	ctx.lr = 0x82447A80;
	sub_824477F8(ctx, base);
	// 82447A80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82447A84: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82447A88: 397F020C  addi r11, r31, 0x20c
	ctx.r[11].s64 = ctx.r[31].s64 + 524;
	// 82447A8C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82447A90: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82447A94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82447A98: 4200FFF8  bdnz 0x82447a90
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82447A90; continue 'dispatch;
	}
	// 82447A9C: 387F01C8  addi r3, r31, 0x1c8
	ctx.r[3].s64 = ctx.r[31].s64 + 456;
	// 82447AA0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447AA4: 480014F5  bl 0x82448f98
	ctx.lr = 0x82447AA8;
	sub_82448F98(ctx, base);
	// 82447AA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82447AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447AB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82447AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447AC0 size=100
    let mut pc: u32 = 0x82447AC0;
    'dispatch: loop {
        match pc {
            0x82447AC0 => {
    //   block [0x82447AC0..0x82447B24)
	// 82447AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447AC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447ACC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447AD0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82447AD4: 3D008290  lis r8, -0x7d70
	ctx.r[8].s64 = -2104492032;
	// 82447AD8: 396B62D0  addi r11, r11, 0x62d0
	ctx.r[11].s64 = ctx.r[11].s64 + 25296;
	// 82447ADC: 3D208313  lis r9, -0x7ced
	ctx.r[9].s64 = -2095906816;
	// 82447AE0: 39403880  li r10, 0x3880
	ctx.r[10].s64 = 14464;
	// 82447AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82447AE8: 9168C8E4  stw r11, -0x371c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-14108 as u32), ctx.r[11].u32 ) };
	// 82447AEC: 914906CC  stw r10, 0x6cc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1740 as u32), ctx.r[10].u32 ) };
	// 82447AF0: 4BFFFD81  bl 0x82447870
	ctx.lr = 0x82447AF4;
	sub_82447870(ctx, base);
	// 82447AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447AF8: 4BFFFF11  bl 0x82447a08
	ctx.lr = 0x82447AFC;
	sub_82447A08(ctx, base);
	// 82447AFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82447B00: 409A0010  bne cr6, 0x82447b10
	if !ctx.cr[6].eq {
	pc = 0x82447B10; continue 'dispatch;
	}
	// 82447B04: 4BFFFD75  bl 0x82447878
	ctx.lr = 0x82447B08;
	sub_82447878(ctx, base);
	// 82447B08: 4BFFFD99  bl 0x824478a0
	ctx.lr = 0x82447B0C;
	sub_824478A0(ctx, base);
	// 82447B0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447B1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447B28 size=148
    let mut pc: u32 = 0x82447B28;
    'dispatch: loop {
        match pc {
            0x82447B28 => {
    //   block [0x82447B28..0x82447BBC)
	// 82447B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447B2C: 480ED591  bl 0x825350bc
	ctx.lr = 0x82447B30;
	sub_82535080(ctx, base);
	// 82447B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447B34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82447B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82447B3C: 3D608245  lis r11, -0x7dbb
	ctx.r[11].s64 = -2109407232;
	// 82447B40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82447B44: 388B3D48  addi r4, r11, 0x3d48
	ctx.r[4].s64 = ctx.r[11].s64 + 15688;
	// 82447B48: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82447B4C: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82447B50: 4BFF0239  bl 0x82437d88
	ctx.lr = 0x82447B54;
	sub_82437D88(ctx, base);
	// 82447B54: 3D608245  lis r11, -0x7dbb
	ctx.r[11].s64 = -2109407232;
	// 82447B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447B5C: 388B3070  addi r4, r11, 0x3070
	ctx.r[4].s64 = ctx.r[11].s64 + 12400;
	// 82447B60: 4BFF0231  bl 0x82437d90
	ctx.lr = 0x82447B64;
	sub_82437D90(ctx, base);
	// 82447B64: 3D608245  lis r11, -0x7dbb
	ctx.r[11].s64 = -2109407232;
	// 82447B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447B6C: 388B26A0  addi r4, r11, 0x26a0
	ctx.r[4].s64 = ctx.r[11].s64 + 9888;
	// 82447B70: 4BFF0229  bl 0x82437d98
	ctx.lr = 0x82447B74;
	sub_82437D98(ctx, base);
	// 82447B74: 3D608245  lis r11, -0x7dbb
	ctx.r[11].s64 = -2109407232;
	// 82447B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447B7C: 388B2AC0  addi r4, r11, 0x2ac0
	ctx.r[4].s64 = ctx.r[11].s64 + 10944;
	// 82447B80: 4BFF0229  bl 0x82437da8
	ctx.lr = 0x82447B84;
	sub_82437DA8(ctx, base);
	// 82447B84: 3D608245  lis r11, -0x7dbb
	ctx.r[11].s64 = -2109407232;
	// 82447B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447B8C: 388B28B8  addi r4, r11, 0x28b8
	ctx.r[4].s64 = ctx.r[11].s64 + 10424;
	// 82447B90: 4BFF0229  bl 0x82437db8
	ctx.lr = 0x82447B94;
	sub_82437DB8(ctx, base);
	// 82447B94: 3D608245  lis r11, -0x7dbb
	ctx.r[11].s64 = -2109407232;
	// 82447B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447B9C: 388B3330  addi r4, r11, 0x3330
	ctx.r[4].s64 = ctx.r[11].s64 + 13104;
	// 82447BA0: 4BFF0229  bl 0x82437dc8
	ctx.lr = 0x82447BA4;
	sub_82437DC8(ctx, base);
	// 82447BA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82447BA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82447BAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447BB0: 4BFF0709  bl 0x824382b8
	ctx.lr = 0x82447BB4;
	sub_824382B8(ctx, base);
	// 82447BB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82447BB8: 480ED554  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447BC0 size=20
    let mut pc: u32 = 0x82447BC0;
    'dispatch: loop {
        match pc {
            0x82447BC0 => {
    //   block [0x82447BC0..0x82447BD4)
	// 82447BC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82447BC4: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 82447BC8: 396B64B8  addi r11, r11, 0x64b8
	ctx.r[11].s64 = ctx.r[11].s64 + 25784;
	// 82447BCC: 916AD988  stw r11, -0x2678(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9848 as u32), ctx.r[11].u32 ) };
	// 82447BD0: 4800B7A0  b 0x82453370
	sub_82453370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447BD8 size=4
    let mut pc: u32 = 0x82447BD8;
    'dispatch: loop {
        match pc {
            0x82447BD8 => {
    //   block [0x82447BD8..0x82447BDC)
	// 82447BD8: 4BFF09A8  b 0x82438580
	sub_82438580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447BE0 size=68
    let mut pc: u32 = 0x82447BE0;
    'dispatch: loop {
        match pc {
            0x82447BE0 => {
    //   block [0x82447BE0..0x82447C24)
	// 82447BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447BEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447BF0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82447BF4: 38A00128  li r5, 0x128
	ctx.r[5].s64 = 296;
	// 82447BF8: 3BEB0360  addi r31, r11, 0x360
	ctx.r[31].s64 = ctx.r[11].s64 + 864;
	// 82447BFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82447C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82447C04: 480ED5CD  bl 0x825351d0
	ctx.lr = 0x82447C08;
	sub_825351D0(ctx, base);
	// 82447C08: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82447C0C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82447C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447C1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447C28 size=40
    let mut pc: u32 = 0x82447C28;
    'dispatch: loop {
        match pc {
            0x82447C28 => {
    //   block [0x82447C28..0x82447C50)
	// 82447C28: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82447C2C: 394B0360  addi r10, r11, 0x360
	ctx.r[10].s64 = ctx.r[11].s64 + 864;
	// 82447C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447C34: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82447C38: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82447C3C: 40990024  ble cr6, 0x82447c60
	if !ctx.cr[6].gt {
		sub_82447C50(ctx, base);
		return;
	}
	// 82447C40: 386A0008  addi r3, r10, 8
	ctx.r[3].s64 = ctx.r[10].s64 + 8;
	// 82447C44: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447C48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82447C4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447C50 size=24
    let mut pc: u32 = 0x82447C50;
    'dispatch: loop {
        match pc {
            0x82447C50 => {
    //   block [0x82447C50..0x82447C68)
	// 82447C50: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82447C54: 38630024  addi r3, r3, 0x24
	ctx.r[3].s64 = ctx.r[3].s64 + 36;
	// 82447C58: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82447C5C: 4198FFE8  blt cr6, 0x82447c44
	if ctx.cr[6].lt {
		sub_82447C28(ctx, base);
		return;
	}
	// 82447C60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447C68 size=56
    let mut pc: u32 = 0x82447C68;
    'dispatch: loop {
        match pc {
            0x82447C68 => {
    //   block [0x82447C68..0x82447CA0)
	// 82447C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447C6C: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 82447C70: 39200064  li r9, 0x64
	ctx.r[9].s64 = 100;
	// 82447C74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82447C78: 38E0007F  li r7, 0x7f
	ctx.r[7].s64 = 127;
	// 82447C7C: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 82447C80: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82447C84: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82447C88: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82447C8C: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82447C90: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82447C94: 98E30015  stb r7, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[7].u8 ) };
	// 82447C98: 98C30016  stb r6, 0x16(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[6].u8 ) };
	// 82447C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447CA0 size=8
    let mut pc: u32 = 0x82447CA0;
    'dispatch: loop {
        match pc {
            0x82447CA0 => {
    //   block [0x82447CA0..0x82447CA8)
	// 82447CA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82447CA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447CA8 size=28
    let mut pc: u32 = 0x82447CA8;
    'dispatch: loop {
        match pc {
            0x82447CA8 => {
    //   block [0x82447CA8..0x82447CC4)
	// 82447CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447CAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82447CB0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82447CB4: 814B0360  lwz r10, 0x360(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(864 as u32) ) } as u64;
	// 82447CB8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82447CBC: 914B0360  stw r10, 0x360(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(864 as u32), ctx.r[10].u32 ) };
	// 82447CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447CC8 size=84
    let mut pc: u32 = 0x82447CC8;
    'dispatch: loop {
        match pc {
            0x82447CC8 => {
    //   block [0x82447CC8..0x82447D1C)
	// 82447CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82447CDC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82447CE0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82447CE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82447CE8: 419A0018  beq cr6, 0x82447d00
	if ctx.cr[6].eq {
	pc = 0x82447D00; continue 'dispatch;
	}
	// 82447CEC: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82447CF0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82447CF4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82447CF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82447CFC: 4E800421  bctrl
	ctx.lr = 0x82447D00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82447D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447D04: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82447D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447D14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447D20 size=20
    let mut pc: u32 = 0x82447D20;
    'dispatch: loop {
        match pc {
            0x82447D20 => {
    //   block [0x82447D20..0x82447D34)
	// 82447D20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82447D24: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82447D28: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82447D2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82447D30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447D34 size=20
    let mut pc: u32 = 0x82447D34;
    'dispatch: loop {
        match pc {
            0x82447D34 => {
    //   block [0x82447D34..0x82447D48)
	// 82447D34: 88CB0016  lbz r6, 0x16(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(22 as u32) ) } as u64;
	// 82447D38: 88AB0015  lbz r5, 0x15(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82447D3C: 888B0014  lbz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82447D40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82447D44: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447D48 size=4
    let mut pc: u32 = 0x82447D48;
    'dispatch: loop {
        match pc {
            0x82447D48 => {
    //   block [0x82447D48..0x82447D4C)
	// 82447D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447D50 size=20
    let mut pc: u32 = 0x82447D50;
    'dispatch: loop {
        match pc {
            0x82447D50 => {
    //   block [0x82447D50..0x82447D64)
	// 82447D50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82447D54: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82447D58: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82447D5C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82447D60: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447D64 size=20
    let mut pc: u32 = 0x82447D64;
    'dispatch: loop {
        match pc {
            0x82447D64 => {
    //   block [0x82447D64..0x82447D78)
	// 82447D64: 88CB0016  lbz r6, 0x16(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(22 as u32) ) } as u64;
	// 82447D68: 88AB0015  lbz r5, 0x15(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82447D6C: 888B0014  lbz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82447D70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82447D74: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447D78 size=4
    let mut pc: u32 = 0x82447D78;
    'dispatch: loop {
        match pc {
            0x82447D78 => {
    //   block [0x82447D78..0x82447D7C)
	// 82447D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447D80 size=4
    let mut pc: u32 = 0x82447D80;
    'dispatch: loop {
        match pc {
            0x82447D80 => {
    //   block [0x82447D80..0x82447D84)
	// 82447D80: 4BFFFE60  b 0x82447be0
	sub_82447BE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447D88 size=84
    let mut pc: u32 = 0x82447D88;
    'dispatch: loop {
        match pc {
            0x82447D88 => {
    //   block [0x82447D88..0x82447DDC)
	// 82447D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447D94: 4BFFFE95  bl 0x82447c28
	ctx.lr = 0x82447D98;
	sub_82447C28(ctx, base);
	// 82447D98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82447D9C: 409A0014  bne cr6, 0x82447db0
	if !ctx.cr[6].eq {
	pc = 0x82447DB0; continue 'dispatch;
	}
	// 82447DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447DAC: 4E800020  blr
	return;
	// 82447DB0: 4BFFFEB9  bl 0x82447c68
	ctx.lr = 0x82447DB4;
	sub_82447C68(ctx, base);
	// 82447DB4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82447DB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82447DBC: 814B0360  lwz r10, 0x360(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(864 as u32) ) } as u64;
	// 82447DC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82447DC4: 914B0360  stw r10, 0x360(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(864 as u32), ctx.r[10].u32 ) };
	// 82447DC8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82447DCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447DE0 size=92
    let mut pc: u32 = 0x82447DE0;
    'dispatch: loop {
        match pc {
            0x82447DE0 => {
    //   block [0x82447DE0..0x82447E3C)
	// 82447DE0: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82447DE4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447DE8: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82447DEC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82447DF0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82447DF4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82447DF8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82447DFC: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82447E00: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82447E04: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82447E08: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82447E0C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82447E10: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82447E14: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82447E18: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82447E1C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82447E20: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82447E24: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82447E28: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82447E2C: 81440018  lwz r10, 0x18(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82447E30: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82447E34: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82447E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447E40 size=128
    let mut pc: u32 = 0x82447E40;
    'dispatch: loop {
        match pc {
            0x82447E40 => {
    //   block [0x82447E40..0x82447EC0)
	// 82447E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447E4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447E50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82447E54: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82447E58: 7C862851  subf. r4, r6, r5
	ctx.r[4].s64 = ctx.r[5].s64 - ctx.r[6].s64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82447E5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447E60: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82447E64: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82447E68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82447E6C: 41810014  bgt 0x82447e80
	if ctx.cr[0].gt {
	pc = 0x82447E80; continue 'dispatch;
	}
	// 82447E70: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82447E74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447E78: 6084040C  ori r4, r4, 0x40c
	ctx.r[4].u64 = ctx.r[4].u64 | 1036;
	// 82447E7C: 4800002C  b 0x82447ea8
	pc = 0x82447EA8; continue 'dispatch;
	// 82447E80: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82447E84: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82447E88: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82447E8C: 4BFDB5BD  bl 0x82423448
	ctx.lr = 0x82447E90;
	sub_82423448(ctx, base);
	// 82447E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82447E94: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82447E98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447E9C: 409A0010  bne cr6, 0x82447eac
	if !ctx.cr[6].eq {
	pc = 0x82447EAC; continue 'dispatch;
	}
	// 82447EA0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82447EA4: 6084040A  ori r4, r4, 0x40a
	ctx.r[4].u64 = ctx.r[4].u64 | 1034;
	// 82447EA8: 4BFFFA61  bl 0x82447908
	ctx.lr = 0x82447EAC;
	sub_82447908(ctx, base);
	// 82447EAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447EB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82447EC0 size=100
    let mut pc: u32 = 0x82447EC0;
    'dispatch: loop {
        match pc {
            0x82447EC0 => {
    //   block [0x82447EC0..0x82447F24)
	// 82447EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82447EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82447EC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82447ECC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82447ED0: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 82447ED4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82447ED8: 396B1398  addi r11, r11, 0x1398
	ctx.r[11].s64 = ctx.r[11].s64 + 5016;
	// 82447EDC: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82447EE0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447EE4: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 82447EE8: 409A0028  bne cr6, 0x82447f10
	if !ctx.cr[6].eq {
	pc = 0x82447F10; continue 'dispatch;
	}
	// 82447EEC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82447EF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82447EF4: 419A001C  beq cr6, 0x82447f10
	if ctx.cr[6].eq {
	pc = 0x82447F10; continue 'dispatch;
	}
	// 82447EF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447EFC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82447F00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82447F04: 4E800421  bctrl
	ctx.lr = 0x82447F08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82447F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447F0C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82447F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82447F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82447F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82447F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82447F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447F28 size=36
    let mut pc: u32 = 0x82447F28;
    'dispatch: loop {
        match pc {
            0x82447F28 => {
    //   block [0x82447F28..0x82447F4C)
	// 82447F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82447F2C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82447F30: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82447F34: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82447F38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82447F3C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82447F40: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82447F44: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82447F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447F50 size=56
    let mut pc: u32 = 0x82447F50;
    'dispatch: loop {
        match pc {
            0x82447F50 => {
    //   block [0x82447F50..0x82447F88)
	// 82447F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447F54: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82447F58: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82447F5C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82447F60: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82447F64: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82447F68: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82447F6C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82447F70: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82447F74: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82447F78: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82447F7C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82447F80: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82447F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447F88 size=20
    let mut pc: u32 = 0x82447F88;
    'dispatch: loop {
        match pc {
            0x82447F88 => {
    //   block [0x82447F88..0x82447F9C)
	// 82447F88: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82447F8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82447F90: 409A000C  bne cr6, 0x82447f9c
	if !ctx.cr[6].eq {
		sub_82447F9C(ctx, base);
		return;
	}
	// 82447F94: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82447F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447F9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447F9C size=52
    let mut pc: u32 = 0x82447F9C;
    'dispatch: loop {
        match pc {
            0x82447F9C => {
    //   block [0x82447F9C..0x82447FD0)
	// 82447F9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447FA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82447FA4: 409A002C  bne cr6, 0x82447fd0
	if !ctx.cr[6].eq {
		sub_82447FD0(ctx, base);
		return;
	}
	// 82447FA8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82447FAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82447FB0: 419AFFE4  beq cr6, 0x82447f94
	if ctx.cr[6].eq {
		sub_82447F88(ctx, base);
		return;
	}
	// 82447FB4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82447FB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82447FBC: 4099FFD8  ble cr6, 0x82447f94
	if !ctx.cr[6].gt {
		sub_82447F88(ctx, base);
		return;
	}
	// 82447FC0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82447FC4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82447FC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82447FCC: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447FD0 size=8
    let mut pc: u32 = 0x82447FD0;
    'dispatch: loop {
        match pc {
            0x82447FD0 => {
    //   block [0x82447FD0..0x82447FD8)
	// 82447FD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82447FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447FD8 size=16
    let mut pc: u32 = 0x82447FD8;
    'dispatch: loop {
        match pc {
            0x82447FD8 => {
    //   block [0x82447FD8..0x82447FE8)
	// 82447FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82447FDC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82447FE0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82447FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82447FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82447FE8 size=56
    let mut pc: u32 = 0x82447FE8;
    'dispatch: loop {
        match pc {
            0x82447FE8 => {
    //   block [0x82447FE8..0x82448020)
	// 82447FE8: 54AA2036  slwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82447FEC: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 82447FF0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82447FF4: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82447FF8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82447FFC: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 82448000: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82448004: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82448008: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8244800C: 81460008  lwz r10, 8(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82448010: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82448014: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82448018: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8244801C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448020 size=56
    let mut pc: u32 = 0x82448020;
    'dispatch: loop {
        match pc {
            0x82448020 => {
    //   block [0x82448020..0x82448058)
	// 82448020: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 82448024: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448028: 54AA2036  slwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244802C: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 82448030: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82448034: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448038: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244803C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82448040: 91460004  stw r10, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82448044: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82448048: 91460008  stw r10, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8244804C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82448050: 9166000C  stw r11, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82448054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448058 size=216
    let mut pc: u32 = 0x82448058;
    'dispatch: loop {
        match pc {
            0x82448058 => {
    //   block [0x82448058..0x82448130)
	// 82448058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244805C: 480ED059  bl 0x825350b4
	ctx.lr = 0x82448060;
	sub_82535080(ctx, base);
	// 82448060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448068: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244806C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82448070: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82448074: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448078: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8244807C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448080: 4E800421  bctrl
	ctx.lr = 0x82448084;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448084: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448088: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 8244808C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82448090: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82448094: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 82448098: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8244809C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824480A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824480A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824480A8: 4E800421  bctrl
	ctx.lr = 0x824480AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824480AC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824480B0: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 824480B4: 40980048  bge cr6, 0x824480fc
	if !ctx.cr[6].lt {
	pc = 0x824480FC; continue 'dispatch;
	}
	// 824480B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824480BC: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 824480C0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824480C4: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 824480C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824480CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824480D0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824480D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824480D8: 4E800421  bctrl
	ctx.lr = 0x824480DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824480DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824480E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824480E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824480E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824480EC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 824480F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824480F4: 4E800421  bctrl
	ctx.lr = 0x824480F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824480F8: 48000010  b 0x82448108
	pc = 0x82448108; continue 'dispatch;
	// 824480FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82448100: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82448104: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82448108: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244810C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82448110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82448114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448118: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8244811C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448120: 4E800421  bctrl
	ctx.lr = 0x82448124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448124: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82448128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244812C: 480ECFD8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448130 size=108
    let mut pc: u32 = 0x82448130;
    'dispatch: loop {
        match pc {
            0x82448130 => {
    //   block [0x82448130..0x8244819C)
	// 82448130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244813C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82448140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448148: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244814C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82448150: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448154: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82448158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244815C: 4E800421  bctrl
	ctx.lr = 0x82448160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448160: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448164: 7FCA0034  cntlzw r10, r30
	ctx.r[10].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 82448168: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8244816C: 5544DFFE  rlwinm r4, r10, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82448170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448174: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82448178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244817C: 4E800421  bctrl
	ctx.lr = 0x82448180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448180: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82448184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82448188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244818C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448190: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82448194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82448198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824481A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824481A0 size=68
    let mut pc: u32 = 0x824481A0;
    'dispatch: loop {
        match pc {
            0x824481A0 => {
    //   block [0x824481A0..0x824481E4)
	// 824481A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824481A4: 480ECF19  bl 0x825350bc
	ctx.lr = 0x824481A8;
	sub_82535080(ctx, base);
	// 824481A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824481AC: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 824481B0: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824481B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824481B8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824481BC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 824481C0: 4BFFF721  bl 0x824478e0
	ctx.lr = 0x824481C4;
	sub_824478E0(ctx, base);
	// 824481C4: 817F13C0  lwz r11, 0x13c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5056 as u32) ) } as u64;
	// 824481C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824481CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824481D0: 817F13C4  lwz r11, 0x13c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5060 as u32) ) } as u64;
	// 824481D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824481D8: 4BFFF719  bl 0x824478f0
	ctx.lr = 0x824481DC;
	sub_824478F0(ctx, base);
	// 824481DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824481E0: 480ECF2C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824481E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824481E8 size=60
    let mut pc: u32 = 0x824481E8;
    'dispatch: loop {
        match pc {
            0x824481E8 => {
    //   block [0x824481E8..0x82448224)
	// 824481E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824481EC: 480ECED1  bl 0x825350bc
	ctx.lr = 0x824481F0;
	sub_82535080(ctx, base);
	// 824481F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824481F4: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 824481F8: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824481FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82448200: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82448204: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82448208: 4BFFF6D9  bl 0x824478e0
	ctx.lr = 0x8244820C;
	sub_824478E0(ctx, base);
	// 8244820C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82448210: 93DF13C0  stw r30, 0x13c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5056 as u32), ctx.r[30].u32 ) };
	// 82448214: 93BF13C4  stw r29, 0x13c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5060 as u32), ctx.r[29].u32 ) };
	// 82448218: 4BFFF6D9  bl 0x824478f0
	ctx.lr = 0x8244821C;
	sub_824478F0(ctx, base);
	// 8244821C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82448220: 480ECEEC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448228 size=16
    let mut pc: u32 = 0x82448228;
    'dispatch: loop {
        match pc {
            0x82448228 => {
    //   block [0x82448228..0x82448238)
	// 82448228: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8244822C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448230: 806B13B4  lwz r3, 0x13b4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5044 as u32) ) } as u64;
	// 82448234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448238 size=16
    let mut pc: u32 = 0x82448238;
    'dispatch: loop {
        match pc {
            0x82448238 => {
    //   block [0x82448238..0x82448248)
	// 82448238: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8244823C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448240: 806B13CC  lwz r3, 0x13cc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5068 as u32) ) } as u64;
	// 82448244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448248 size=116
    let mut pc: u32 = 0x82448248;
    'dispatch: loop {
        match pc {
            0x82448248 => {
    //   block [0x82448248..0x824482BC)
	// 82448248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244824C: 480ECE71  bl 0x825350bc
	ctx.lr = 0x82448250;
	sub_82535080(ctx, base);
	// 82448250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448254: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 82448258: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8244825C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82448260: 4BFFF681  bl 0x824478e0
	ctx.lr = 0x82448264;
	sub_824478E0(ctx, base);
	// 82448264: 83DF13C8  lwz r30, 0x13c8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5064 as u32) ) } as u64;
	// 82448268: 83BF13CC  lwz r29, 0x13cc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5068 as u32) ) } as u64;
	// 8244826C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82448270: 409A002C  bne cr6, 0x8244829c
	if !ctx.cr[6].eq {
	pc = 0x8244829C; continue 'dispatch;
	}
	// 82448274: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82448278: 419A0030  beq cr6, 0x824482a8
	if ctx.cr[6].eq {
	pc = 0x824482A8; continue 'dispatch;
	}
	// 8244827C: 807F13AC  lwz r3, 0x13ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5036 as u32) ) } as u64;
	// 82448280: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82448284: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448288: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8244828C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448290: 4E800421  bctrl
	ctx.lr = 0x82448294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448294: 7FC3EA14  add r30, r3, r29
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 82448298: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8244829C: 4098000C  bge cr6, 0x824482a8
	if !ctx.cr[6].lt {
	pc = 0x824482A8; continue 'dispatch;
	}
	// 824482A0: 3FC07FFF  lis r30, 0x7fff
	ctx.r[30].s64 = 2147418112;
	// 824482A4: 63DEFFFF  ori r30, r30, 0xffff
	ctx.r[30].u64 = ctx.r[30].u64 | 65535;
	// 824482A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824482AC: 4BFFF645  bl 0x824478f0
	ctx.lr = 0x824482B0;
	sub_824478F0(ctx, base);
	// 824482B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824482B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824482B8: 480ECE54  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824482C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824482C0 size=40
    let mut pc: u32 = 0x824482C0;
    'dispatch: loop {
        match pc {
            0x824482C0 => {
    //   block [0x824482C0..0x824482E8)
	// 824482C0: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 824482C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824482C8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824482CC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824482D0: 814B139C  lwz r10, 0x139c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5020 as u32) ) } as u64;
	// 824482D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824482D8: 409A0010  bne cr6, 0x824482e8
	if !ctx.cr[6].eq {
		sub_824482E8(ctx, base);
		return;
	}
	// 824482DC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824482E0: 60840401  ori r4, r4, 0x401
	ctx.r[4].u64 = ctx.r[4].u64 | 1025;
	// 824482E4: 4BFFF624  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824482E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824482E8 size=16
    let mut pc: u32 = 0x824482E8;
    'dispatch: loop {
        match pc {
            0x824482E8 => {
    //   block [0x824482E8..0x824482F8)
	// 824482E8: 816B13AC  lwz r11, 0x13ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5036 as u32) ) } as u64;
	// 824482EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824482F0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824482F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824482F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824482F8 size=24
    let mut pc: u32 = 0x824482F8;
    'dispatch: loop {
        match pc {
            0x824482F8 => {
    //   block [0x824482F8..0x82448310)
	// 824482F8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 824482FC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448300: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 82448304: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82448308: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244830C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448310 size=12
    let mut pc: u32 = 0x82448310;
    'dispatch: loop {
        match pc {
            0x82448310 => {
    //   block [0x82448310..0x8244831C)
	// 82448310: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82448314: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82448318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448320 size=40
    let mut pc: u32 = 0x82448320;
    'dispatch: loop {
        match pc {
            0x82448320 => {
    //   block [0x82448320..0x82448348)
	// 82448320: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 82448324: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448328: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8244832C: 814B139C  lwz r10, 0x139c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5020 as u32) ) } as u64;
	// 82448330: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82448334: 409A0014  bne cr6, 0x82448348
	if !ctx.cr[6].eq {
		sub_82448348(ctx, base);
		return;
	}
	// 82448338: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8244833C: 808B13E4  lwz r4, 0x13e4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5092 as u32) ) } as u64;
	// 82448340: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82448344: 48000BB4  b 0x82448ef8
	sub_82448EF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448348 size=8
    let mut pc: u32 = 0x82448348;
    'dispatch: loop {
        match pc {
            0x82448348 => {
    //   block [0x82448348..0x82448350)
	// 82448348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244834C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448350 size=96
    let mut pc: u32 = 0x82448350;
    'dispatch: loop {
        match pc {
            0x82448350 => {
    //   block [0x82448350..0x824483B0)
	// 82448350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244835C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448364: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 82448368: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8244836C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82448370: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448374: 814B139C  lwz r10, 0x139c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5020 as u32) ) } as u64;
	// 82448378: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244837C: 409A0018  bne cr6, 0x82448394
	if !ctx.cr[6].eq {
	pc = 0x82448394; continue 'dispatch;
	}
	// 82448380: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82448384: 808B13E4  lwz r4, 0x13e4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5092 as u32) ) } as u64;
	// 82448388: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8244838C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448390: 48000B69  bl 0x82448ef8
	ctx.lr = 0x82448394;
	sub_82448EF8(ctx, base);
	// 82448394: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82448398: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8244839C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824483A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824483A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824483A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824483AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824483B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824483B0 size=8
    let mut pc: u32 = 0x824483B0;
    'dispatch: loop {
        match pc {
            0x824483B0 => {
    //   block [0x824483B0..0x824483B8)
	// 824483B0: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 824483B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824483B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824483B8 size=16
    let mut pc: u32 = 0x824483B8;
    'dispatch: loop {
        match pc {
            0x824483B8 => {
    //   block [0x824483B8..0x824483C8)
	// 824483B8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 824483BC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824483C0: 90AB13A0  stw r5, 0x13a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5024 as u32), ctx.r[5].u32 ) };
	// 824483C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824483C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824483C8 size=16
    let mut pc: u32 = 0x824483C8;
    'dispatch: loop {
        match pc {
            0x824483C8 => {
    //   block [0x824483C8..0x824483D8)
	// 824483C8: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 824483CC: 409A000C  bne cr6, 0x824483d8
	if !ctx.cr[6].eq {
		sub_824483D8(ctx, base);
		return;
	}
	// 824483D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824483D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824483D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824483D8 size=16
    let mut pc: u32 = 0x824483D8;
    'dispatch: loop {
        match pc {
            0x824483D8 => {
    //   block [0x824483D8..0x824483E8)
	// 824483D8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 824483DC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824483E0: 806B13A0  lwz r3, 0x13a0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5024 as u32) ) } as u64;
	// 824483E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824483E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824483E8 size=8
    let mut pc: u32 = 0x824483E8;
    'dispatch: loop {
        match pc {
            0x824483E8 => {
    //   block [0x824483E8..0x824483F0)
	// 824483E8: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 824483EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824483F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824483F0 size=16
    let mut pc: u32 = 0x824483F0;
    'dispatch: loop {
        match pc {
            0x824483F0 => {
    //   block [0x824483F0..0x82448400)
	// 824483F0: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 824483F4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824483F8: 90AB13A4  stw r5, 0x13a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5028 as u32), ctx.r[5].u32 ) };
	// 824483FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448400 size=16
    let mut pc: u32 = 0x82448400;
    'dispatch: loop {
        match pc {
            0x82448400 => {
    //   block [0x82448400..0x82448410)
	// 82448400: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 82448404: 409A000C  bne cr6, 0x82448410
	if !ctx.cr[6].eq {
		sub_82448410(ctx, base);
		return;
	}
	// 82448408: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244840C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448410 size=16
    let mut pc: u32 = 0x82448410;
    'dispatch: loop {
        match pc {
            0x82448410 => {
    //   block [0x82448410..0x82448420)
	// 82448410: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 82448414: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448418: 806B13A4  lwz r3, 0x13a4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5028 as u32) ) } as u64;
	// 8244841C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448420 size=168
    let mut pc: u32 = 0x82448420;
    'dispatch: loop {
        match pc {
            0x82448420 => {
    //   block [0x82448420..0x824484C8)
	// 82448420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244842C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82448430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82448438: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8244843C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82448440: 4BFDB009  bl 0x82423448
	ctx.lr = 0x82448444;
	sub_82423448(ctx, base);
	// 82448444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448448: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244844C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82448450: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448454: 4E800421  bctrl
	ctx.lr = 0x82448458;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448458: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 8244845C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448460: 3BCBD990  addi r30, r11, -0x2670
	ctx.r[30].s64 = ctx.r[11].s64 + -9840;
	// 82448464: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82448468: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8244846C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448474: 4E800421  bctrl
	ctx.lr = 0x82448478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448478: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8244847C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82448480: 4BFDD1B9  bl 0x82425638
	ctx.lr = 0x82448484;
	sub_82425638(ctx, base);
	// 82448484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448488: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244848C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82448490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448494: 4E800421  bctrl
	ctx.lr = 0x82448498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448498: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244849C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824484A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824484A4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824484A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824484AC: 4E800421  bctrl
	ctx.lr = 0x824484B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824484B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824484B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824484B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824484BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824484C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824484C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824484C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824484C8 size=64
    let mut pc: u32 = 0x824484C8;
    'dispatch: loop {
        match pc {
            0x824484C8 => {
    //   block [0x824484C8..0x82448508)
	// 824484C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824484CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824484D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824484D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824484D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824484DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824484E0: 4E800421  bctrl
	ctx.lr = 0x824484E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824484E4: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 824484E8: 816BD994  lwz r11, -0x266c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9836 as u32) ) } as u64;
	// 824484EC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 824484F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824484F4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824484F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824484FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448508 size=64
    let mut pc: u32 = 0x82448508;
    'dispatch: loop {
        match pc {
            0x82448508 => {
    //   block [0x82448508..0x82448548)
	// 82448508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244850C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448514: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448518: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244851C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448520: 4E800421  bctrl
	ctx.lr = 0x82448524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448524: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82448528: 816BD990  lwz r11, -0x2670(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9840 as u32) ) } as u64;
	// 8244852C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82448530: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82448534: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82448538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244853C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448548 size=44
    let mut pc: u32 = 0x82448548;
    'dispatch: loop {
        match pc {
            0x82448548 => {
    //   block [0x82448548..0x82448574)
	// 82448548: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244854C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82448550: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82448554: 41980008  blt cr6, 0x8244855c
	if ctx.cr[6].lt {
	pc = 0x8244855C; continue 'dispatch;
	}
	// 82448558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244855C: 786A07C4  rldicr r10, r3, 0, 0x1f
	ctx.r[10].u64 = (ctx.r[3].u64).rotate_left(0) & 0xFFFFFFFF00000000;
	// 82448560: 796B07C6  sldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(32);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 82448564: 78890020  clrldi r9, r4, 0x20
	ctx.r[9].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82448568: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244856C: 7D634B78  or r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82448570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448578 size=4
    let mut pc: u32 = 0x82448578;
    'dispatch: loop {
        match pc {
            0x82448578 => {
    //   block [0x82448578..0x8244857C)
	// 82448578: 4BFFFEA8  b 0x82448420
	sub_82448420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448580 size=72
    let mut pc: u32 = 0x82448580;
    'dispatch: loop {
        match pc {
            0x82448580 => {
    //   block [0x82448580..0x824485C8)
	// 82448580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244858C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448590: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82448594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448598: 4BFFF929  bl 0x82447ec0
	ctx.lr = 0x8244859C;
	sub_82447EC0(ctx, base);
	// 8244859C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824485A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824485A4: 4BFFF91D  bl 0x82447ec0
	ctx.lr = 0x824485A8;
	sub_82447EC0(ctx, base);
	// 824485A8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824485AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824485B0: 4BFFF911  bl 0x82447ec0
	ctx.lr = 0x824485B4;
	sub_82447EC0(ctx, base);
	// 824485B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824485B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824485BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824485C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824485C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824485C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824485C8 size=152
    let mut pc: u32 = 0x824485C8;
    'dispatch: loop {
        match pc {
            0x824485C8 => {
    //   block [0x824485C8..0x82448660)
	// 824485C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824485CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824485D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824485D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824485D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824485DC: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824485E0: 1D670074  mulli r11, r7, 0x74
	ctx.r[11].s64 = ctx.r[7].s64 * 116;
	// 824485E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824485E8: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824485EC: 7D68302E  lwzx r11, r8, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 824485F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824485F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824485F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824485FC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82448600: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82448604: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 82448608: 4BFFF921  bl 0x82447f28
	ctx.lr = 0x8244860C;
	sub_82447F28(ctx, base);
	// 8244860C: 7D68F02E  lwzx r11, r8, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82448610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82448614: 38FF1738  addi r7, r31, 0x1738
	ctx.r[7].s64 = ctx.r[31].s64 + 5944;
	// 82448618: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244861C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82448620: 7D08302E  lwzx r8, r8, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82448624: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82448628: 9149000C  stw r10, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8244862C: 90E90010  stw r7, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82448630: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82448634: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82448638: 7D48592E  stwx r10, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8244863C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 82448640: 2F0B0880  cmpwi cr6, r11, 0x880
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2176, &mut ctx.xer);
	// 82448644: 4198FFF0  blt cr6, 0x82448634
	if ctx.cr[6].lt {
	pc = 0x82448634; continue 'dispatch;
	}
	// 82448648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244864C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448654: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82448658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244865C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448660 size=132
    let mut pc: u32 = 0x82448660;
    'dispatch: loop {
        match pc {
            0x82448660 => {
    //   block [0x82448660..0x824486E4)
	// 82448660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244866C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82448670: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82448674: 1D660074  mulli r11, r6, 0x74
	ctx.r[11].s64 = ctx.r[6].s64 * 116;
	// 82448678: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8244867C: 7D69402E  lwzx r11, r9, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82448680: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 82448684: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82448688: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8244868C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82448690: 69650001  xori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 ^ 1;
	// 82448694: 4BFFF895  bl 0x82447f28
	ctx.lr = 0x82448698;
	sub_82447F28(ctx, base);
	// 82448698: 7D49382E  lwzx r10, r9, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8244869C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824486A0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824486A4: 7D49402E  lwzx r10, r9, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 824486A8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824486AC: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824486B0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824486B4: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824486B8: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824486BC: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824486C0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824486C4: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824486C8: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824486CC: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824486D0: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 824486D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824486D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824486DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824486E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824486E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824486E8 size=56
    let mut pc: u32 = 0x824486E8;
    'dispatch: loop {
        match pc {
            0x824486E8 => {
    //   block [0x824486E8..0x82448720)
	// 824486E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824486EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824486F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824486F4: 1D660074  mulli r11, r6, 0x74
	ctx.r[11].s64 = ctx.r[6].s64 * 116;
	// 824486F8: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824486FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82448700: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82448704: 4BFFF825  bl 0x82447f28
	ctx.lr = 0x82448708;
	sub_82447F28(ctx, base);
	// 82448708: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 8244870C: 4BFFF845  bl 0x82447f50
	ctx.lr = 0x82448710;
	sub_82447F50(ctx, base);
	// 82448710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82448714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244871C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448720 size=112
    let mut pc: u32 = 0x82448720;
    'dispatch: loop {
        match pc {
            0x82448720 => {
    //   block [0x82448720..0x82448790)
	// 82448720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448724: 480EC995  bl 0x825350b8
	ctx.lr = 0x82448728;
	sub_82535080(ctx, base);
	// 82448728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244872C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448730: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82448734: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82448738: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244873C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82448740: 4BFFF1A1  bl 0x824478e0
	ctx.lr = 0x82448744;
	sub_824478E0(ctx, base);
	// 82448744: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82448748: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8244874C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82448750: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82448754: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82448758: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244875C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82448760: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82448764: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82448768: 4200FFF0  bdnz 0x82448758
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82448758; continue 'dispatch;
	}
	// 8244876C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82448770: 4BFFF869  bl 0x82447fd8
	ctx.lr = 0x82448774;
	sub_82447FD8(ctx, base);
	// 82448774: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82448778: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8244877C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82448780: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82448784: 4BFFF16D  bl 0x824478f0
	ctx.lr = 0x82448788;
	sub_824478F0(ctx, base);
	// 82448788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244878C: 480EC97C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448790 size=160
    let mut pc: u32 = 0x82448790;
    'dispatch: loop {
        match pc {
            0x82448790 => {
    //   block [0x82448790..0x82448830)
	// 82448790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448798: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244879C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824487A0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824487A4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824487A8: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 824487AC: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824487B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824487B4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 824487B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824487BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824487C0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824487C4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824487C8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824487CC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824487D0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824487D4: 816A139C  lwz r11, 0x139c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5020 as u32) ) } as u64;
	// 824487D8: 806A13AC  lwz r3, 0x13ac(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5036 as u32) ) } as u64;
	// 824487DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824487E0: 419A0038  beq cr6, 0x82448818
	if ctx.cr[6].eq {
	pc = 0x82448818; continue 'dispatch;
	}
	// 824487E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824487E8: 419A0030  beq cr6, 0x82448818
	if ctx.cr[6].eq {
	pc = 0x82448818; continue 'dispatch;
	}
	// 824487EC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 824487F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824487F4: 4BFFF865  bl 0x82448058
	ctx.lr = 0x824487F8;
	sub_82448058(ctx, base);
	// 824487F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824487FC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82448800: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82448804: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82448808: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244880C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82448810: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82448814: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82448818: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244881C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82448820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448828: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244882C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448830 size=132
    let mut pc: u32 = 0x82448830;
    'dispatch: loop {
        match pc {
            0x82448830 => {
    //   block [0x82448830..0x824488B4)
	// 82448830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244883C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448844: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82448848: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8244884C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82448850: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82448854: 4BFFF805  bl 0x82448058
	ctx.lr = 0x82448858;
	sub_82448058(ctx, base);
	// 82448858: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8244885C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82448860: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82448864: 41980014  blt cr6, 0x82448878
	if ctx.cr[6].lt {
	pc = 0x82448878; continue 'dispatch;
	}
	// 82448868: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244886C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82448870: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82448874: 4198002C  blt cr6, 0x824488a0
	if ctx.cr[6].lt {
	pc = 0x824488A0; continue 'dispatch;
	}
	// 82448878: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8244887C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82448880: 41980014  blt cr6, 0x82448894
	if ctx.cr[6].lt {
	pc = 0x82448894; continue 'dispatch;
	}
	// 82448884: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82448888: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8244888C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82448890: 41980010  blt cr6, 0x824488a0
	if ctx.cr[6].lt {
	pc = 0x824488A0; continue 'dispatch;
	}
	// 82448894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82448898: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8244889C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 824488A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824488A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824488A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824488AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824488B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824488B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824488B8 size=180
    let mut pc: u32 = 0x824488B8;
    'dispatch: loop {
        match pc {
            0x824488B8 => {
    //   block [0x824488B8..0x8244896C)
	// 824488B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824488BC: 480EC801  bl 0x825350bc
	ctx.lr = 0x824488C0;
	sub_82535080(ctx, base);
	// 824488C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824488C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824488C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824488CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824488D0: 4BFFFBF9  bl 0x824484c8
	ctx.lr = 0x824488D4;
	sub_824484C8(ctx, base);
	// 824488D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824488D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824488DC: 419A0034  beq cr6, 0x82448910
	if ctx.cr[6].eq {
	pc = 0x82448910; continue 'dispatch;
	}
	// 824488E0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824488E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824488E8: 4BFDAAD9  bl 0x824233c0
	ctx.lr = 0x824488EC;
	sub_824233C0(ctx, base);
	// 824488EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824488F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824488F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824488F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824488FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82448900: 4BFDAAC1  bl 0x824233c0
	ctx.lr = 0x82448904;
	sub_824233C0(ctx, base);
	// 82448904: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82448908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244890C: 480EC800  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82448910: 4BFFFBF9  bl 0x82448508
	ctx.lr = 0x82448914;
	sub_82448508(ctx, base);
	// 82448914: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448918: 419A0040  beq cr6, 0x82448958
	if ctx.cr[6].eq {
	pc = 0x82448958; continue 'dispatch;
	}
	// 8244891C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448920: 4BFDCC99  bl 0x824255b8
	ctx.lr = 0x82448924;
	sub_824255B8(ctx, base);
	// 82448924: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82448928: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8244892C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448930: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82448934: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448938: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8244893C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448940: 4E800421  bctrl
	ctx.lr = 0x82448944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448944: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448948: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8244894C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82448950: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82448954: 480EC7B8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82448958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244895C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82448960: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82448964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82448968: 480EC7A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448970 size=144
    let mut pc: u32 = 0x82448970;
    'dispatch: loop {
        match pc {
            0x82448970 => {
    //   block [0x82448970..0x82448A00)
	// 82448970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448974: 480EC745  bl 0x825350b8
	ctx.lr = 0x82448978;
	sub_82535080(ctx, base);
	// 82448978: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244897C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82448980: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82448984: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 82448988: 7CAA282E  lwzx r5, r10, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8244898C: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448990: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82448994: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 82448998: 409A0010  bne cr6, 0x824489a8
	if !ctx.cr[6].eq {
	pc = 0x824489A8; continue 'dispatch;
	}
	// 8244899C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824489A0: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 824489A4: 48000040  b 0x824489e4
	pc = 0x824489E4; continue 'dispatch;
	// 824489A8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 824489AC: 7C8A202E  lwzx r4, r10, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824489B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824489B4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824489B8: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 824489BC: 4BFFF485  bl 0x82447e40
	ctx.lr = 0x824489C0;
	sub_82447E40(ctx, base);
	// 824489C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824489C4: 409A0034  bne cr6, 0x824489f8
	if !ctx.cr[6].eq {
	pc = 0x824489F8; continue 'dispatch;
	}
	// 824489C8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824489CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824489D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824489D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824489D8: 4BFFFD49  bl 0x82448720
	ctx.lr = 0x824489DC;
	sub_82448720(ctx, base);
	// 824489DC: 387C0028  addi r3, r28, 0x28
	ctx.r[3].s64 = ctx.r[28].s64 + 40;
	// 824489E0: 480074B9  bl 0x8244fe98
	ctx.lr = 0x824489E4;
	sub_8244FE98(ctx, base);
	// 824489E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824489E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824489EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824489F0: 4BFFF539  bl 0x82447f28
	ctx.lr = 0x824489F4;
	sub_82447F28(ctx, base);
	// 824489F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824489F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824489FC: 480EC70C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448A00 size=132
    let mut pc: u32 = 0x82448A00;
    'dispatch: loop {
        match pc {
            0x82448A00 => {
    //   block [0x82448A00..0x82448A84)
	// 82448A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82448A0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448A10: 1D650074  mulli r11, r5, 0x74
	ctx.r[11].s64 = ctx.r[5].s64 * 116;
	// 82448A14: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448A18: 38AB1398  addi r5, r11, 0x1398
	ctx.r[5].s64 = ctx.r[11].s64 + 5016;
	// 82448A1C: 3BE50010  addi r31, r5, 0x10
	ctx.r[31].s64 = ctx.r[5].s64 + 16;
	// 82448A20: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448A24: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82448A28: 419A0024  beq cr6, 0x82448a4c
	if ctx.cr[6].eq {
	pc = 0x82448A4C; continue 'dispatch;
	}
	// 82448A2C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82448A30: 60840409  ori r4, r4, 0x409
	ctx.r[4].u64 = ctx.r[4].u64 | 1033;
	// 82448A34: 4BFFEED5  bl 0x82447908
	ctx.lr = 0x82448A38;
	sub_82447908(ctx, base);
	// 82448A38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82448A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448A44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82448A48: 4E800020  blr
	return;
	// 82448A4C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82448A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448A54: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82448A58: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82448A5C: 69660001  xori r6, r11, 1
	ctx.r[6].u64 = ctx.r[11].u64 ^ 1;
	// 82448A60: 4BFFFCC1  bl 0x82448720
	ctx.lr = 0x82448A64;
	sub_82448720(ctx, base);
	// 82448A64: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82448A68: 48001DD9  bl 0x8244a840
	ctx.lr = 0x82448A6C;
	sub_8244A840(ctx, base);
	// 82448A6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82448A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448A7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82448A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448A88 size=8
    let mut pc: u32 = 0x82448A88;
    'dispatch: loop {
        match pc {
            0x82448A88 => {
    //   block [0x82448A88..0x82448A90)
	// 82448A88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82448A8C: 4BFFFD04  b 0x82448790
	sub_82448790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448A90 size=8
    let mut pc: u32 = 0x82448A90;
    'dispatch: loop {
        match pc {
            0x82448A90 => {
    //   block [0x82448A90..0x82448A98)
	// 82448A90: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82448A94: 4BFFFCFC  b 0x82448790
	sub_82448790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448A98 size=264
    let mut pc: u32 = 0x82448A98;
    'dispatch: loop {
        match pc {
            0x82448A98 => {
    //   block [0x82448A98..0x82448BA0)
	// 82448A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448A9C: 480EC60D  bl 0x825350a8
	ctx.lr = 0x82448AA0;
	sub_82535080(ctx, base);
	// 82448AA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448AA4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82448AA8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82448AAC: 1D7A0074  mulli r11, r26, 0x74
	ctx.r[11].s64 = ctx.r[26].s64 * 116;
	// 82448AB0: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82448AB4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82448AB8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82448ABC: 396B1398  addi r11, r11, 0x1398
	ctx.r[11].s64 = ctx.r[11].s64 + 5016;
	// 82448AC0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82448AC4: 3B8B0010  addi r28, r11, 0x10
	ctx.r[28].s64 = ctx.r[11].s64 + 16;
	// 82448AC8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82448ACC: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82448AD0: 419A00C4  beq cr6, 0x82448b94
	if ctx.cr[6].eq {
	pc = 0x82448B94; continue 'dispatch;
	}
	// 82448AD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82448AD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82448ADC: 419A00B8  beq cr6, 0x82448b94
	if ctx.cr[6].eq {
	pc = 0x82448B94; continue 'dispatch;
	}
	// 82448AE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82448AE4: 419A00B0  beq cr6, 0x82448b94
	if ctx.cr[6].eq {
	pc = 0x82448B94; continue 'dispatch;
	}
	// 82448AE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82448AEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448AF0: 4BFFF641  bl 0x82448130
	ctx.lr = 0x82448AF4;
	sub_82448130(ctx, base);
	// 82448AF4: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82448AF8: 40980034  bge cr6, 0x82448b2c
	if !ctx.cr[6].lt {
	pc = 0x82448B2C; continue 'dispatch;
	}
	// 82448AFC: 7F63F850  subf r27, r3, r31
	ctx.r[27].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82448B00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82448B04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448B08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82448B0C: 4BFFF625  bl 0x82448130
	ctx.lr = 0x82448B10;
	sub_82448130(ctx, base);
	// 82448B10: 7F03D800  cmpw cr6, r3, r27
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82448B14: 40980018  bge cr6, 0x82448b2c
	if !ctx.cr[6].lt {
	pc = 0x82448B2C; continue 'dispatch;
	}
	// 82448B18: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82448B1C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82448B20: 6084040B  ori r4, r4, 0x40b
	ctx.r[4].u64 = ctx.r[4].u64 | 1035;
	// 82448B24: 4BFFEDE5  bl 0x82447908
	ctx.lr = 0x82448B28;
	sub_82447908(ctx, base);
	// 82448B28: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82448B2C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82448B30: 409A003C  bne cr6, 0x82448b6c
	if !ctx.cr[6].eq {
	pc = 0x82448B6C; continue 'dispatch;
	}
	// 82448B34: 2F1A0001  cmpwi cr6, r26, 1
	ctx.cr[6].compare_i32(ctx.r[26].s32, 1, &mut ctx.xer);
	// 82448B38: 409A000C  bne cr6, 0x82448b44
	if !ctx.cr[6].eq {
	pc = 0x82448B44; continue 'dispatch;
	}
	// 82448B3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82448B40: 4BFFFCF1  bl 0x82448830
	ctx.lr = 0x82448B44;
	sub_82448830(ctx, base);
	// 82448B44: 817C0024  lwz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82448B48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82448B4C: 41980034  blt cr6, 0x82448b80
	if ctx.cr[6].lt {
	pc = 0x82448B80; continue 'dispatch;
	}
	// 82448B50: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82448B54: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82448B58: 917C0024  stw r11, 0x24(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82448B5C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82448B60: 91790044  stw r11, 0x44(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82448B64: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82448B68: 480EC590  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82448B6C: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82448B70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82448B74: 4198000C  blt cr6, 0x82448b80
	if ctx.cr[6].lt {
	pc = 0x82448B80; continue 'dispatch;
	}
	// 82448B78: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82448B7C: 917C0020  stw r11, 0x20(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82448B80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82448B84: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82448B88: 91790044  stw r11, 0x44(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82448B8C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82448B90: 480EC568  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82448B94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448B98: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82448B9C: 480EC55C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448BA0 size=48
    let mut pc: u32 = 0x82448BA0;
    'dispatch: loop {
        match pc {
            0x82448BA0 => {
    //   block [0x82448BA0..0x82448BD0)
	// 82448BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448BAC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82448BB0: 4BFFFEE1  bl 0x82448a90
	ctx.lr = 0x82448BB4;
	sub_82448A90(ctx, base);
	// 82448BB4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82448BB8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82448BBC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82448BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82448BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448BD0 size=276
    let mut pc: u32 = 0x82448BD0;
    'dispatch: loop {
        match pc {
            0x82448BD0 => {
    //   block [0x82448BD0..0x82448CE4)
	// 82448BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448BD4: 480EC4E9  bl 0x825350bc
	ctx.lr = 0x82448BD8;
	sub_82535080(ctx, base);
	// 82448BD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448BDC: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82448BE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82448BE4: 3BE90008  addi r31, r9, 8
	ctx.r[31].s64 = ctx.r[9].s64 + 8;
	// 82448BE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82448BEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82448BF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82448BF4: 80A90004  lwz r5, 4(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82448BF8: 4BFFF1E9  bl 0x82447de0
	ctx.lr = 0x82448BFC;
	sub_82447DE0(ctx, base);
	// 82448BFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448C00: 81490028  lwz r10, 0x28(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 82448C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82448C08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82448C0C: 7D2B53D6  divw r9, r11, r10
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82448C10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82448C14: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82448C18: 7CEA5850  subf r7, r10, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82448C1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448C20: 4BFFFD51  bl 0x82448970
	ctx.lr = 0x82448C24;
	sub_82448970(ctx, base);
	// 82448C24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448C28: 409A00B4  bne cr6, 0x82448cdc
	if !ctx.cr[6].eq {
	pc = 0x82448CDC; continue 'dispatch;
	}
	// 82448C2C: 38E00800  li r7, 0x800
	ctx.r[7].s64 = 2048;
	// 82448C30: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82448C34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82448C38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82448C3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448C40: 4BFFFD31  bl 0x82448970
	ctx.lr = 0x82448C44;
	sub_82448970(ctx, base);
	// 82448C44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448C48: 409A0094  bne cr6, 0x82448cdc
	if !ctx.cr[6].eq {
	pc = 0x82448CDC; continue 'dispatch;
	}
	// 82448C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82448C50: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82448C54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82448C58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82448C5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448C60: 4BFFFD11  bl 0x82448970
	ctx.lr = 0x82448C64;
	sub_82448970(ctx, base);
	// 82448C64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448C68: 409A0074  bne cr6, 0x82448cdc
	if !ctx.cr[6].eq {
	pc = 0x82448CDC; continue 'dispatch;
	}
	// 82448C6C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82448C70: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82448C74: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82448C78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82448C7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82448C80: 4BFFF949  bl 0x824485c8
	ctx.lr = 0x82448C84;
	sub_824485C8(ctx, base);
	// 82448C84: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82448C88: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82448C8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82448C90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448C94: 4BFFF9CD  bl 0x82448660
	ctx.lr = 0x82448C98;
	sub_82448660(ctx, base);
	// 82448C98: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82448C9C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82448CA0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82448CA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82448CA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82448CAC: 4BFFF91D  bl 0x824485c8
	ctx.lr = 0x82448CB0;
	sub_824485C8(ctx, base);
	// 82448CB0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82448CB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82448CB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82448CBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448CC0: 4BFFF9A1  bl 0x82448660
	ctx.lr = 0x82448CC4;
	sub_82448660(ctx, base);
	// 82448CC4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82448CC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82448CCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82448CD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448CD4: 4BFFFA15  bl 0x824486e8
	ctx.lr = 0x82448CD8;
	sub_824486E8(ctx, base);
	// 82448CD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448CDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82448CE0: 480EC42C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82448CE8 size=176
    let mut pc: u32 = 0x82448CE8;
    'dispatch: loop {
        match pc {
            0x82448CE8 => {
    //   block [0x82448CE8..0x82448D98)
	// 82448CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82448CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82448CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448CFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82448D00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448D04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82448D08: 4BFFF281  bl 0x82447f88
	ctx.lr = 0x82448D0C;
	sub_82447F88(ctx, base);
	// 82448D0C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448D14: 419A0014  beq cr6, 0x82448d28
	if ctx.cr[6].eq {
	pc = 0x82448D28; continue 'dispatch;
	}
	// 82448D18: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82448D1C: 60840408  ori r4, r4, 0x408
	ctx.r[4].u64 = ctx.r[4].u64 | 1032;
	// 82448D20: 4BFFEBE9  bl 0x82447908
	ctx.lr = 0x82448D24;
	sub_82447908(ctx, base);
	// 82448D24: 4800005C  b 0x82448d80
	pc = 0x82448D80; continue 'dispatch;
	// 82448D28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82448D2C: 4800024D  bl 0x82448f78
	ctx.lr = 0x82448D30;
	sub_82448F78(ctx, base);
	// 82448D30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448D34: 419A000C  beq cr6, 0x82448d40
	if ctx.cr[6].eq {
	pc = 0x82448D40; continue 'dispatch;
	}
	// 82448D38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82448D3C: 48000038  b 0x82448d74
	pc = 0x82448D74; continue 'dispatch;
	// 82448D40: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82448D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448D48: 48000231  bl 0x82448f78
	ctx.lr = 0x82448D4C;
	sub_82448F78(ctx, base);
	// 82448D4C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448D50: 419A000C  beq cr6, 0x82448d5c
	if ctx.cr[6].eq {
	pc = 0x82448D5C; continue 'dispatch;
	}
	// 82448D54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82448D58: 4800001C  b 0x82448d74
	pc = 0x82448D74; continue 'dispatch;
	// 82448D5C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82448D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448D64: 48000215  bl 0x82448f78
	ctx.lr = 0x82448D68;
	sub_82448F78(ctx, base);
	// 82448D68: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 82448D6C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82448D70: 556507BC  rlwinm r5, r11, 0, 0x1e, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82448D74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82448D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448D7C: 4BFFFC85  bl 0x82448a00
	ctx.lr = 0x82448D80;
	sub_82448A00(ctx, base);
	// 82448D80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82448D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448D8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82448D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82448D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448D98 size=8
    let mut pc: u32 = 0x82448D98;
    'dispatch: loop {
        match pc {
            0x82448D98 => {
    //   block [0x82448D98..0x82448DA0)
	// 82448D98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82448D9C: 4BFFFCFC  b 0x82448a98
	sub_82448A98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448DA0 size=8
    let mut pc: u32 = 0x82448DA0;
    'dispatch: loop {
        match pc {
            0x82448DA0 => {
    //   block [0x82448DA0..0x82448DA8)
	// 82448DA0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82448DA4: 4BFFFCF4  b 0x82448a98
	sub_82448A98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448DA8 size=104
    let mut pc: u32 = 0x82448DA8;
    'dispatch: loop {
        match pc {
            0x82448DA8 => {
    //   block [0x82448DA8..0x82448E10)
	// 82448DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448DAC: 480EC311  bl 0x825350bc
	ctx.lr = 0x82448DB0;
	sub_82535080(ctx, base);
	// 82448DB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448DB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448DB8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82448DBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448DC0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82448DC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448DC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82448DCC: 419A003C  beq cr6, 0x82448e08
	if ctx.cr[6].eq {
	pc = 0x82448E08; continue 'dispatch;
	}
	// 82448DD0: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82448DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82448DD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82448DDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82448DE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448DE4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82448DE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448DEC: 4E800421  bctrl
	ctx.lr = 0x82448DF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448DF0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448DF4: 409A0014  bne cr6, 0x82448e08
	if !ctx.cr[6].eq {
	pc = 0x82448E08; continue 'dispatch;
	}
	// 82448DF8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82448DFC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82448E00: 2F1E000F  cmpwi cr6, r30, 0xf
	ctx.cr[6].compare_i32(ctx.r[30].s32, 15, &mut ctx.xer);
	// 82448E04: 4198FFC0  blt cr6, 0x82448dc4
	if ctx.cr[6].lt {
	pc = 0x82448DC4; continue 'dispatch;
	}
	// 82448E08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82448E0C: 480EC300  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448E10 size=48
    let mut pc: u32 = 0x82448E10;
    'dispatch: loop {
        match pc {
            0x82448E10 => {
    //   block [0x82448E10..0x82448E40)
	// 82448E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82448E14: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82448E18: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82448E1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82448E20: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82448E24: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82448E28: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82448E2C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82448E30: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82448E34: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82448E38: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82448E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448E40 size=40
    let mut pc: u32 = 0x82448E40;
    'dispatch: loop {
        match pc {
            0x82448E40 => {
    //   block [0x82448E40..0x82448E68)
	// 82448E40: 1D660074  mulli r11, r6, 0x74
	ctx.r[11].s64 = ctx.r[6].s64 * 116;
	// 82448E44: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448E48: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82448E4C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82448E50: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82448E54: 396B07F3  addi r11, r11, 0x7f3
	ctx.r[11].s64 = ctx.r[11].s64 + 2035;
	// 82448E58: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82448E5C: 7CCB192E  stwx r6, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[6].u32) };
	// 82448E60: 908A13E4  stw r4, 0x13e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5092 as u32), ctx.r[4].u32 ) };
	// 82448E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448E68 size=28
    let mut pc: u32 = 0x82448E68;
    'dispatch: loop {
        match pc {
            0x82448E68 => {
    //   block [0x82448E68..0x82448E84)
	// 82448E68: 1D440074  mulli r10, r4, 0x74
	ctx.r[10].s64 = ctx.r[4].s64 * 116;
	// 82448E6C: 1D650044  mulli r11, r5, 0x44
	ctx.r[11].s64 = ctx.r[5].s64 * 68;
	// 82448E70: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82448E74: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448E78: 90AA13E8  stw r5, 0x13e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5096 as u32), ctx.r[5].u32 ) };
	// 82448E7C: 908B1FC8  stw r4, 0x1fc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8136 as u32), ctx.r[4].u32 ) };
	// 82448E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448E88 size=112
    let mut pc: u32 = 0x82448E88;
    'dispatch: loop {
        match pc {
            0x82448E88 => {
    //   block [0x82448E88..0x82448EF8)
	// 82448E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448E8C: 480EC22D  bl 0x825350b8
	ctx.lr = 0x82448E90;
	sub_82535080(ctx, base);
	// 82448E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82448E98: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82448E9C: 397F1FB8  addi r11, r31, 0x1fb8
	ctx.r[11].s64 = ctx.r[31].s64 + 8120;
	// 82448EA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448EA4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82448EA8: 3BCB000C  addi r30, r11, 0xc
	ctx.r[30].s64 = ctx.r[11].s64 + 12;
	// 82448EAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82448EB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82448EB4: 419A002C  beq cr6, 0x82448ee0
	if ctx.cr[6].eq {
	pc = 0x82448EE0; continue 'dispatch;
	}
	// 82448EB8: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82448EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82448EC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82448EC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82448EC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448ECC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82448ED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448ED4: 4E800421  bctrl
	ctx.lr = 0x82448ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82448ED8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82448EDC: 409A0014  bne cr6, 0x82448ef0
	if !ctx.cr[6].eq {
	pc = 0x82448EF0; continue 'dispatch;
	}
	// 82448EE0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82448EE4: 3BDE0044  addi r30, r30, 0x44
	ctx.r[30].s64 = ctx.r[30].s64 + 68;
	// 82448EE8: 2F1D0009  cmpwi cr6, r29, 9
	ctx.cr[6].compare_i32(ctx.r[29].s32, 9, &mut ctx.xer);
	// 82448EEC: 4198FFC0  blt cr6, 0x82448eac
	if ctx.cr[6].lt {
	pc = 0x82448EAC; continue 'dispatch;
	}
	// 82448EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82448EF4: 480EC214  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448EF8 size=40
    let mut pc: u32 = 0x82448EF8;
    'dispatch: loop {
        match pc {
            0x82448EF8 => {
    //   block [0x82448EF8..0x82448F20)
	// 82448EF8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82448EFC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82448F00: 1D6B0044  mulli r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 * 68;
	// 82448F04: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448F08: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82448F0C: 816B1FC4  lwz r11, 0x1fc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8132 as u32) ) } as u64;
	// 82448F10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82448F14: 409A000C  bne cr6, 0x82448f20
	if !ctx.cr[6].eq {
		sub_82448F20(ctx, base);
		return;
	}
	// 82448F18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82448F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448F20 size=24
    let mut pc: u32 = 0x82448F20;
    'dispatch: loop {
        match pc {
            0x82448F20 => {
    //   block [0x82448F20..0x82448F38)
	// 82448F20: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82448F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82448F28: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82448F2C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82448F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82448F34: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448F38 size=16
    let mut pc: u32 = 0x82448F38;
    'dispatch: loop {
        match pc {
            0x82448F38 => {
    //   block [0x82448F38..0x82448F48)
	// 82448F38: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 82448F3C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448F40: 90AB1FB8  stw r5, 0x1fb8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8120 as u32), ctx.r[5].u32 ) };
	// 82448F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448F48 size=16
    let mut pc: u32 = 0x82448F48;
    'dispatch: loop {
        match pc {
            0x82448F48 => {
    //   block [0x82448F48..0x82448F58)
	// 82448F48: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 82448F4C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448F50: 806B1FB8  lwz r3, 0x1fb8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8120 as u32) ) } as u64;
	// 82448F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448F58 size=16
    let mut pc: u32 = 0x82448F58;
    'dispatch: loop {
        match pc {
            0x82448F58 => {
    //   block [0x82448F58..0x82448F68)
	// 82448F58: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 82448F5C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448F60: 90AB1FBC  stw r5, 0x1fbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8124 as u32), ctx.r[5].u32 ) };
	// 82448F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448F68 size=16
    let mut pc: u32 = 0x82448F68;
    'dispatch: loop {
        match pc {
            0x82448F68 => {
    //   block [0x82448F68..0x82448F78)
	// 82448F68: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 82448F6C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448F70: 806B1FBC  lwz r3, 0x1fbc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8124 as u32) ) } as u64;
	// 82448F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448F78 size=28
    let mut pc: u32 = 0x82448F78;
    'dispatch: loop {
        match pc {
            0x82448F78 => {
    //   block [0x82448F78..0x82448F94)
	// 82448F78: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 82448F7C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82448F80: 816B1FC4  lwz r11, 0x1fc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8132 as u32) ) } as u64;
	// 82448F84: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82448F88: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82448F8C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82448F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82448F98 size=60
    let mut pc: u32 = 0x82448F98;
    'dispatch: loop {
        match pc {
            0x82448F98 => {
    //   block [0x82448F98..0x82448FD4)
	// 82448F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82448F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82448FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82448FA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82448FA8: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 82448FAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82448FB0: 480EBBA1  bl 0x82534b50
	ctx.lr = 0x82448FB4;
	sub_82534B50(ctx, base);
	// 82448FB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82448FB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82448FBC: 4BFFFDED  bl 0x82448da8
	ctx.lr = 0x82448FC0;
	sub_82448DA8(ctx, base);
	// 82448FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82448FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82448FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82448FCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82448FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448FD8 size=12
    let mut pc: u32 = 0x82448FD8;
    'dispatch: loop {
        match pc {
            0x82448FD8 => {
    //   block [0x82448FD8..0x82448FE4)
	// 82448FD8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82448FDC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82448FE0: 4BFFFE88  b 0x82448e68
	sub_82448E68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448FE8 size=12
    let mut pc: u32 = 0x82448FE8;
    'dispatch: loop {
        match pc {
            0x82448FE8 => {
    //   block [0x82448FE8..0x82448FF4)
	// 82448FE8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82448FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82448FF0: 4BFFFE50  b 0x82448e40
	sub_82448E40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82448FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82448FF8 size=12
    let mut pc: u32 = 0x82448FF8;
    'dispatch: loop {
        match pc {
            0x82448FF8 => {
    //   block [0x82448FF8..0x82449004)
	// 82448FF8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82448FFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82449000: 4BFFFE40  b 0x82448e40
	sub_82448E40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449008 size=12
    let mut pc: u32 = 0x82449008;
    'dispatch: loop {
        match pc {
            0x82449008 => {
    //   block [0x82449008..0x82449014)
	// 82449008: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8244900C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82449010: 4BFFFE30  b 0x82448e40
	sub_82448E40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449018 size=104
    let mut pc: u32 = 0x82449018;
    'dispatch: loop {
        match pc {
            0x82449018 => {
    //   block [0x82449018..0x82449080)
	// 82449018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244901C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449024: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82449028: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8244902C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82449030: 4BFFFE39  bl 0x82448e68
	ctx.lr = 0x82449034;
	sub_82448E68(ctx, base);
	// 82449034: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82449038: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8244903C: 4BFFFFAD  bl 0x82448fe8
	ctx.lr = 0x82449040;
	sub_82448FE8(ctx, base);
	// 82449040: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82449044: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82449048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244904C: 419A001C  beq cr6, 0x82449068
	if ctx.cr[6].eq {
	pc = 0x82449068; continue 'dispatch;
	}
	// 82449050: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82449054: 4BFFFE15  bl 0x82448e68
	ctx.lr = 0x82449058;
	sub_82448E68(ctx, base);
	// 82449058: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8244905C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82449060: 4BFFFF89  bl 0x82448fe8
	ctx.lr = 0x82449064;
	sub_82448FE8(ctx, base);
	// 82449064: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82449068: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8244906C: 4BFFFDFD  bl 0x82448e68
	ctx.lr = 0x82449070;
	sub_82448E68(ctx, base);
	// 82449070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244907C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449080 size=104
    let mut pc: u32 = 0x82449080;
    'dispatch: loop {
        match pc {
            0x82449080 => {
    //   block [0x82449080..0x824490E8)
	// 82449080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244908C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82449090: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82449094: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82449098: 4BFFFDD1  bl 0x82448e68
	ctx.lr = 0x8244909C;
	sub_82448E68(ctx, base);
	// 8244909C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 824490A0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 824490A4: 4BFFFF45  bl 0x82448fe8
	ctx.lr = 0x824490A8;
	sub_82448FE8(ctx, base);
	// 824490A8: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 824490AC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824490B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824490B4: 419A001C  beq cr6, 0x824490d0
	if ctx.cr[6].eq {
	pc = 0x824490D0; continue 'dispatch;
	}
	// 824490B8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 824490BC: 4BFFFDAD  bl 0x82448e68
	ctx.lr = 0x824490C0;
	sub_82448E68(ctx, base);
	// 824490C0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 824490C4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 824490C8: 4BFFFF21  bl 0x82448fe8
	ctx.lr = 0x824490CC;
	sub_82448FE8(ctx, base);
	// 824490CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824490D0: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 824490D4: 4BFFFD95  bl 0x82448e68
	ctx.lr = 0x824490D8;
	sub_82448E68(ctx, base);
	// 824490D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824490DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824490E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824490E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824490E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824490E8 size=184
    let mut pc: u32 = 0x824490E8;
    'dispatch: loop {
        match pc {
            0x824490E8 => {
    //   block [0x824490E8..0x824491A0)
	// 824490E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824490EC: 480EBFD1  bl 0x825350bc
	ctx.lr = 0x824490F0;
	sub_82535080(ctx, base);
	// 824490F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824490F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824490F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824490FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82449100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82449104: 4BFFFD65  bl 0x82448e68
	ctx.lr = 0x82449108;
	sub_82448E68(ctx, base);
	// 82449108: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244910C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82449110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82449114: 419A0018  beq cr6, 0x8244912c
	if ctx.cr[6].eq {
	pc = 0x8244912C; continue 'dispatch;
	}
	// 82449118: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8244911C: 4BFFFECD  bl 0x82448fe8
	ctx.lr = 0x82449120;
	sub_82448FE8(ctx, base);
	// 82449120: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82449124: 4BFFFEF5  bl 0x82449018
	ctx.lr = 0x82449128;
	sub_82449018(ctx, base);
	// 82449128: 48000014  b 0x8244913c
	pc = 0x8244913C; continue 'dispatch;
	// 8244912C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82449130: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82449134: 4BFF361D  bl 0x8243c750
	ctx.lr = 0x82449138;
	sub_8243C750(ctx, base);
	// 82449138: 93BF0BB0  stw r29, 0xbb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2992 as u32), ctx.r[29].u32 ) };
	// 8244913C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82449140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449144: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82449148: 419A001C  beq cr6, 0x82449164
	if ctx.cr[6].eq {
	pc = 0x82449164; continue 'dispatch;
	}
	// 8244914C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82449150: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82449154: 4BFFFEA5  bl 0x82448ff8
	ctx.lr = 0x82449158;
	sub_82448FF8(ctx, base);
	// 82449158: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244915C: 4BFFFF25  bl 0x82449080
	ctx.lr = 0x82449160;
	sub_82449080(ctx, base);
	// 82449160: 48000014  b 0x82449174
	pc = 0x82449174; continue 'dispatch;
	// 82449164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82449168: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8244916C: 4BFF35E5  bl 0x8243c750
	ctx.lr = 0x82449170;
	sub_8243C750(ctx, base);
	// 82449170: 93BF0BB4  stw r29, 0xbb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2996 as u32), ctx.r[29].u32 ) };
	// 82449174: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82449178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244917C: 419A001C  beq cr6, 0x82449198
	if ctx.cr[6].eq {
	pc = 0x82449198; continue 'dispatch;
	}
	// 82449180: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82449184: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82449188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244918C: 4BFFFE7D  bl 0x82449008
	ctx.lr = 0x82449190;
	sub_82449008(ctx, base);
	// 82449190: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82449194: 4BFFFE45  bl 0x82448fd8
	ctx.lr = 0x82449198;
	sub_82448FD8(ctx, base);
	// 82449198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244919C: 480EBF70  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824491A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824491A0 size=364
    let mut pc: u32 = 0x824491A0;
    'dispatch: loop {
        match pc {
            0x824491A0 => {
    //   block [0x824491A0..0x8244930C)
	// 824491A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824491A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824491A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824491AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824491B0: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 824491B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824491B8: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824491BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824491C0: 419A0030  beq cr6, 0x824491f0
	if ctx.cr[6].eq {
	pc = 0x824491F0; continue 'dispatch;
	}
	// 824491C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824491C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824491CC: 4BFFFE1D  bl 0x82448fe8
	ctx.lr = 0x824491D0;
	sub_82448FE8(ctx, base);
	// 824491D0: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 824491D4: 4BFFFF15  bl 0x824490e8
	ctx.lr = 0x824491D8;
	sub_824490E8(ctx, base);
	// 824491D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824491DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824491E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824491E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824491E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824491EC: 4E800020  blr
	return;
	// 824491F0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824491F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824491F8: 419A0048  beq cr6, 0x82449240
	if ctx.cr[6].eq {
	pc = 0x82449240; continue 'dispatch;
	}
	// 824491FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82449200: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82449204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449208: 4BFFFDE1  bl 0x82448fe8
	ctx.lr = 0x8244920C;
	sub_82448FE8(ctx, base);
	// 8244920C: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82449210: 4BFFFE09  bl 0x82449018
	ctx.lr = 0x82449214;
	sub_82449018(ctx, base);
	// 82449214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82449218: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8244921C: 4BFF3535  bl 0x8243c750
	ctx.lr = 0x82449220;
	sub_8243C750(ctx, base);
	// 82449220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82449224: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449228: 917F0BB4  stw r11, 0xbb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2996 as u32), ctx.r[11].u32 ) };
	// 8244922C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244923C: 4E800020  blr
	return;
	// 82449240: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82449244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82449248: 419A0048  beq cr6, 0x82449290
	if ctx.cr[6].eq {
	pc = 0x82449290; continue 'dispatch;
	}
	// 8244924C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82449250: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82449254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449258: 4BFFFD91  bl 0x82448fe8
	ctx.lr = 0x8244925C;
	sub_82448FE8(ctx, base);
	// 8244925C: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82449260: 4BFFFE21  bl 0x82449080
	ctx.lr = 0x82449264;
	sub_82449080(ctx, base);
	// 82449264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82449268: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8244926C: 4BFF34E5  bl 0x8243c750
	ctx.lr = 0x82449270;
	sub_8243C750(ctx, base);
	// 82449270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82449274: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449278: 917F0BB0  stw r11, 0xbb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2992 as u32), ctx.r[11].u32 ) };
	// 8244927C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449288: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244928C: 4E800020  blr
	return;
	// 82449290: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82449294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82449298: 419A005C  beq cr6, 0x824492f4
	if ctx.cr[6].eq {
	pc = 0x824492F4; continue 'dispatch;
	}
	// 8244929C: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 824492A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824492A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824492A8: 4BFFFD41  bl 0x82448fe8
	ctx.lr = 0x824492AC;
	sub_82448FE8(ctx, base);
	// 824492AC: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 824492B0: 4BFFFD29  bl 0x82448fd8
	ctx.lr = 0x824492B4;
	sub_82448FD8(ctx, base);
	// 824492B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824492B8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824492BC: 4BFF3495  bl 0x8243c750
	ctx.lr = 0x824492C0;
	sub_8243C750(ctx, base);
	// 824492C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824492C4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 824492C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824492CC: 4BFF3485  bl 0x8243c750
	ctx.lr = 0x824492D0;
	sub_8243C750(ctx, base);
	// 824492D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824492D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824492D8: 917F0BB0  stw r11, 0xbb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2992 as u32), ctx.r[11].u32 ) };
	// 824492DC: 917F0BB4  stw r11, 0xbb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2996 as u32), ctx.r[11].u32 ) };
	// 824492E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824492E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824492E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824492EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824492F0: 4E800020  blr
	return;
	// 824492F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824492F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824492FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449310 size=152
    let mut pc: u32 = 0x82449310;
    'dispatch: loop {
        match pc {
            0x82449310 => {
    //   block [0x82449310..0x824493A8)
	// 82449310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244931C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449320: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82449324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82449328: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8244932C: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82449330: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82449334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82449338: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8244933C: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82449340: 4BFFFAD1  bl 0x82448e10
	ctx.lr = 0x82449344;
	sub_82448E10(ctx, base);
	// 82449344: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82449348: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8244934C: 38630044  addi r3, r3, 0x44
	ctx.r[3].s64 = ctx.r[3].s64 + 68;
	// 82449350: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82449354: 409AFFE4  bne cr6, 0x82449338
	if !ctx.cr[6].eq {
	pc = 0x82449338; continue 'dispatch;
	}
	// 82449358: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8244935C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449360: 4BFFFE41  bl 0x824491a0
	ctx.lr = 0x82449364;
	sub_824491A0(ctx, base);
	// 82449364: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449368: 419A0028  beq cr6, 0x82449390
	if ctx.cr[6].eq {
	pc = 0x82449390; continue 'dispatch;
	}
	// 8244936C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82449370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449374: 60840302  ori r4, r4, 0x302
	ctx.r[4].u64 = ctx.r[4].u64 | 770;
	// 82449378: 4BFFE591  bl 0x82447908
	ctx.lr = 0x8244937C;
	sub_82447908(ctx, base);
	// 8244937C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244938C: 4E800020  blr
	return;
	// 82449390: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244939C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824493A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824493A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824493A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824493A8 size=8
    let mut pc: u32 = 0x824493A8;
    'dispatch: loop {
        match pc {
            0x824493A8 => {
    //   block [0x824493A8..0x824493B0)
	// 824493A8: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 824493AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824493B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824493B0 size=32
    let mut pc: u32 = 0x824493B0;
    'dispatch: loop {
        match pc {
            0x824493B0 => {
    //   block [0x824493B0..0x824493D0)
	// 824493B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824493B4: 3940FFFD  li r10, -3
	ctx.r[10].s64 = -3;
	// 824493B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824493BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824493C0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824493C4: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824493C8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824493CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824493D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824493D0 size=88
    let mut pc: u32 = 0x824493D0;
    'dispatch: loop {
        match pc {
            0x824493D0 => {
    //   block [0x824493D0..0x82449428)
	// 824493D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824493D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824493D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824493DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824493E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824493E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824493E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824493EC: 4BFFE455  bl 0x82447840
	ctx.lr = 0x824493F0;
	sub_82447840(ctx, base);
	// 824493F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824493F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824493F8: 419A0014  beq cr6, 0x8244940c
	if ctx.cr[6].eq {
	pc = 0x8244940C; continue 'dispatch;
	}
	// 824493FC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82449400: 60840151  ori r4, r4, 0x151
	ctx.r[4].u64 = ctx.r[4].u64 | 337;
	// 82449404: 4BFFE505  bl 0x82447908
	ctx.lr = 0x82449408;
	sub_82447908(ctx, base);
	// 82449408: 48000008  b 0x82449410
	pc = 0x82449410; continue 'dispatch;
	// 8244940C: 93DF2658  stw r30, 0x2658(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(9816 as u32), ctx.r[30].u32 ) };
	// 82449410: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244941C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82449420: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449428 size=12
    let mut pc: u32 = 0x82449428;
    'dispatch: loop {
        match pc {
            0x82449428 => {
    //   block [0x82449428..0x82449434)
	// 82449428: 81632658  lwz r11, 0x2658(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9816 as u32) ) } as u64;
	// 8244942C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82449430: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449434 size=28
    let mut pc: u32 = 0x82449434;
    'dispatch: loop {
        match pc {
            0x82449434 => {
    //   block [0x82449434..0x82449450)
	// 82449434: 814B0DB8  lwz r10, 0xdb8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3512 as u32) ) } as u64;
	// 82449438: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244943C: 40980008  bge cr6, 0x82449444
	if !ctx.cr[6].lt {
	pc = 0x82449444; continue 'dispatch;
	}
	// 82449440: 908B0DB8  stw r4, 0xdb8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3512 as u32), ctx.r[4].u32 ) };
	// 82449444: 814B0DBC  lwz r10, 0xdbc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3516 as u32) ) } as u64;
	// 82449448: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244944C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449450 size=8
    let mut pc: u32 = 0x82449450;
    'dispatch: loop {
        match pc {
            0x82449450 => {
    //   block [0x82449450..0x82449458)
	// 82449450: 90AB0DBC  stw r5, 0xdbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3516 as u32), ctx.r[5].u32 ) };
	// 82449454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449458 size=104
    let mut pc: u32 = 0x82449458;
    'dispatch: loop {
        match pc {
            0x82449458 => {
    //   block [0x82449458..0x824494C0)
	// 82449458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244945C: 480EBC61  bl 0x825350bc
	ctx.lr = 0x82449460;
	sub_82535080(ctx, base);
	// 82449460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449464: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82449468: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8244946C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82449470: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82449474: 4BFFFB05  bl 0x82448f78
	ctx.lr = 0x82449478;
	sub_82448F78(ctx, base);
	// 82449478: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244947C: 419A0034  beq cr6, 0x824494b0
	if ctx.cr[6].eq {
	pc = 0x824494B0; continue 'dispatch;
	}
	// 82449480: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82449484: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82449488: 4BFF3281  bl 0x8243c708
	ctx.lr = 0x8244948C;
	sub_8243C708(ctx, base);
	// 8244948C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82449490: 409A0020  bne cr6, 0x824494b0
	if !ctx.cr[6].eq {
	pc = 0x824494B0; continue 'dispatch;
	}
	// 82449494: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82449498: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244949C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824494A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824494A4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824494A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824494AC: 480EBC60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824494B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824494B4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824494B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824494BC: 480EBC50  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824494C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824494C0 size=104
    let mut pc: u32 = 0x824494C0;
    'dispatch: loop {
        match pc {
            0x824494C0 => {
    //   block [0x824494C0..0x82449528)
	// 824494C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824494C4: 480EBBF9  bl 0x825350bc
	ctx.lr = 0x824494C8;
	sub_82535080(ctx, base);
	// 824494C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824494CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824494D0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824494D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824494D8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824494DC: 4BFFFA9D  bl 0x82448f78
	ctx.lr = 0x824494E0;
	sub_82448F78(ctx, base);
	// 824494E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824494E4: 419A0034  beq cr6, 0x82449518
	if ctx.cr[6].eq {
	pc = 0x82449518; continue 'dispatch;
	}
	// 824494E8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 824494EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824494F0: 4BFF3219  bl 0x8243c708
	ctx.lr = 0x824494F4;
	sub_8243C708(ctx, base);
	// 824494F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824494F8: 409A0020  bne cr6, 0x82449518
	if !ctx.cr[6].eq {
	pc = 0x82449518; continue 'dispatch;
	}
	// 824494FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82449500: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82449504: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82449508: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8244950C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82449510: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449514: 480EBBF8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82449518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244951C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82449520: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449524: 480EBBE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449528 size=48
    let mut pc: u32 = 0x82449528;
    'dispatch: loop {
        match pc {
            0x82449528 => {
    //   block [0x82449528..0x82449558)
	// 82449528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244952C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449534: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82449538: 4BFFFA41  bl 0x82448f78
	ctx.lr = 0x8244953C;
	sub_82448F78(ctx, base);
	// 8244953C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82449540: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82449544: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82449548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244954C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449558 size=100
    let mut pc: u32 = 0x82449558;
    'dispatch: loop {
        match pc {
            0x82449558 => {
    //   block [0x82449558..0x824495BC)
	// 82449558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244955C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82449564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82449568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244956C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82449570: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82449574: 4BFFE2CD  bl 0x82447840
	ctx.lr = 0x82449578;
	sub_82447840(ctx, base);
	// 82449578: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244957C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449580: 419A0014  beq cr6, 0x82449594
	if ctx.cr[6].eq {
	pc = 0x82449594; continue 'dispatch;
	}
	// 82449584: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82449588: 6084015C  ori r4, r4, 0x15c
	ctx.r[4].u64 = ctx.r[4].u64 | 348;
	// 8244958C: 4BFFE37D  bl 0x82447908
	ctx.lr = 0x82449590;
	sub_82447908(ctx, base);
	// 82449590: 48000014  b 0x824495a4
	pc = 0x824495A4; continue 'dispatch;
	// 82449594: 817F2658  lwz r11, 0x2658(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82449598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244959C: 419A0008  beq cr6, 0x824495a4
	if ctx.cr[6].eq {
	pc = 0x824495A4; continue 'dispatch;
	}
	// 824495A0: 93CB0DD4  stw r30, 0xdd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3540 as u32), ctx.r[30].u32 ) };
	// 824495A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824495A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824495AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824495B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824495B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824495B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824495C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824495C0 size=40
    let mut pc: u32 = 0x824495C0;
    'dispatch: loop {
        match pc {
            0x824495C0 => {
    //   block [0x824495C0..0x824495E8)
	// 824495C0: 39631FB8  addi r11, r3, 0x1fb8
	ctx.r[11].s64 = ctx.r[3].s64 + 8120;
	// 824495C4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824495C8: 1D4A0074  mulli r10, r10, 0x74
	ctx.r[10].s64 = ctx.r[10].s64 * 116;
	// 824495CC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 824495D0: 814A13E8  lwz r10, 0x13e8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5096 as u32) ) } as u64;
	// 824495D4: 1D4A0044  mulli r10, r10, 0x44
	ctx.r[10].s64 = ctx.r[10].s64 * 68;
	// 824495D8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824495DC: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 824495E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824495E4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824495E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824495E8 size=8
    let mut pc: u32 = 0x824495E8;
    'dispatch: loop {
        match pc {
            0x824495E8 => {
    //   block [0x824495E8..0x824495F0)
	// 824495E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824495EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824495F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824495F0 size=168
    let mut pc: u32 = 0x824495F0;
    'dispatch: loop {
        match pc {
            0x824495F0 => {
    //   block [0x824495F0..0x82449698)
	// 824495F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824495F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824495F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824495FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449600: 83E32658  lwz r31, 0x2658(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82449604: 817F0DD0  lwz r11, 0xdd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3536 as u32) ) } as u64;
	// 82449608: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244960C: 41990074  bgt cr6, 0x82449680
	if ctx.cr[6].gt {
	pc = 0x82449680; continue 'dispatch;
	}
	// 82449610: 807F0DC4  lwz r3, 0xdc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3524 as u32) ) } as u64;
	// 82449614: 80BF0DC8  lwz r5, 0xdc8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3528 as u32) ) } as u64;
	// 82449618: 809F0DCC  lwz r4, 0xdcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3532 as u32) ) } as u64;
	// 8244961C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449620: 40990028  ble cr6, 0x82449648
	if !ctx.cr[6].gt {
	pc = 0x82449648; continue 'dispatch;
	}
	// 82449624: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82449628: 40990020  ble cr6, 0x82449648
	if !ctx.cr[6].gt {
	pc = 0x82449648; continue 'dispatch;
	}
	// 8244962C: 4BFFAF4D  bl 0x82444578
	ctx.lr = 0x82449630;
	sub_82444578(ctx, base);
	// 82449630: 907F0DA8  stw r3, 0xda8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3496 as u32), ctx.r[3].u32 ) };
	// 82449634: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244963C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449644: 4E800020  blr
	return;
	// 82449648: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244964C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82449650: 41990030  bgt cr6, 0x82449680
	if ctx.cr[6].gt {
	pc = 0x82449680; continue 'dispatch;
	}
	// 82449654: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449658: 41990008  bgt cr6, 0x82449660
	if ctx.cr[6].gt {
	pc = 0x82449660; continue 'dispatch;
	}
	// 8244965C: 807F0DAC  lwz r3, 0xdac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3500 as u32) ) } as u64;
	// 82449660: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82449664: 4199000C  bgt cr6, 0x82449670
	if ctx.cr[6].gt {
	pc = 0x82449670; continue 'dispatch;
	}
	// 82449668: 80BF0DB0  lwz r5, 0xdb0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3504 as u32) ) } as u64;
	// 8244966C: 809F0DB4  lwz r4, 0xdb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3508 as u32) ) } as u64;
	// 82449670: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449674: 4099000C  ble cr6, 0x82449680
	if !ctx.cr[6].gt {
	pc = 0x82449680; continue 'dispatch;
	}
	// 82449678: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8244967C: 4199FFB0  bgt cr6, 0x8244962c
	if ctx.cr[6].gt {
	pc = 0x8244962C; continue 'dispatch;
	}
	// 82449680: 917F0DA8  stw r11, 0xda8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3496 as u32), ctx.r[11].u32 ) };
	// 82449684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244968C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449698 size=320
    let mut pc: u32 = 0x82449698;
    'dispatch: loop {
        match pc {
            0x82449698 => {
    //   block [0x82449698..0x824497D8)
	// 82449698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244969C: 480EBA19  bl 0x825350b4
	ctx.lr = 0x824496A0;
	sub_82535080(ctx, base);
	// 824496A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824496A4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824496A8: 83FC2658  lwz r31, 0x2658(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(9816 as u32) ) } as u64;
	// 824496AC: 3BDF0AD0  addi r30, r31, 0xad0
	ctx.r[30].s64 = ctx.r[31].s64 + 2768;
	// 824496B0: 3BBF0D0C  addi r29, r31, 0xd0c
	ctx.r[29].s64 = ctx.r[31].s64 + 3340;
	// 824496B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824496B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824496BC: 409A0114  bne cr6, 0x824497d0
	if !ctx.cr[6].eq {
	pc = 0x824497D0; continue 'dispatch;
	}
	// 824496C0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 824496C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824496C8: 4BFFFD91  bl 0x82449458
	ctx.lr = 0x824496CC;
	sub_82449458(ctx, base);
	// 824496CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824496D0: 409A0100  bne cr6, 0x824497d0
	if !ctx.cr[6].eq {
	pc = 0x824497D0; continue 'dispatch;
	}
	// 824496D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824496D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824496DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824496E0: 4BFFFDE1  bl 0x824494c0
	ctx.lr = 0x824496E4;
	sub_824494C0(ctx, base);
	// 824496E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824496E8: 409A00E8  bne cr6, 0x824497d0
	if !ctx.cr[6].eq {
	pc = 0x824497D0; continue 'dispatch;
	}
	// 824496EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824496F0: 4BFFFE39  bl 0x82449528
	ctx.lr = 0x824496F4;
	sub_82449528(ctx, base);
	// 824496F4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 824496F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824496FC: 419A0094  beq cr6, 0x82449790
	if ctx.cr[6].eq {
	pc = 0x82449790; continue 'dispatch;
	}
	// 82449700: 937F08A0  stw r27, 0x8a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2208 as u32), ctx.r[27].u32 ) };
	// 82449704: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82449708: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244970C: 419A0078  beq cr6, 0x82449784
	if ctx.cr[6].eq {
	pc = 0x82449784; continue 'dispatch;
	}
	// 82449710: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82449714: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82449718: 40990030  ble cr6, 0x82449748
	if !ctx.cr[6].gt {
	pc = 0x82449748; continue 'dispatch;
	}
	// 8244971C: 807F0DC4  lwz r3, 0xdc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3524 as u32) ) } as u64;
	// 82449720: 80BF0040  lwz r5, 0x40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82449724: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449728: 40990060  ble cr6, 0x82449788
	if !ctx.cr[6].gt {
	pc = 0x82449788; continue 'dispatch;
	}
	// 8244972C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82449730: 40990058  ble cr6, 0x82449788
	if !ctx.cr[6].gt {
	pc = 0x82449788; continue 'dispatch;
	}
	// 82449734: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 82449738: 4BFFAE41  bl 0x82444578
	ctx.lr = 0x8244973C;
	sub_82444578(ctx, base);
	// 8244973C: 815F08A8  lwz r10, 0x8a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) } as u64;
	// 82449740: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82449744: 48000078  b 0x824497bc
	pc = 0x824497BC; continue 'dispatch;
	// 82449748: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244974C: 48000955  bl 0x8244a0a0
	ctx.lr = 0x82449750;
	sub_8244A0A0(ctx, base);
	// 82449750: 2F03006C  cmpwi cr6, r3, 0x6c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 108, &mut ctx.xer);
	// 82449754: 40980030  bge cr6, 0x82449784
	if !ctx.cr[6].lt {
	pc = 0x82449784; continue 'dispatch;
	}
	// 82449758: 817F08A4  lwz r11, 0x8a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2212 as u32) ) } as u64;
	// 8244975C: 3D4081E7  lis r10, -0x7e19
	ctx.r[10].s64 = -2115567616;
	// 82449760: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82449764: 614A22C3  ori r10, r10, 0x22c3
	ctx.r[10].u64 = ctx.r[10].u64 | 8899;
	// 82449768: 7D4B5096  mulhw r10, r11, r10
	ctx.r[10].s64 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 8244976C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82449770: 7D6B5670  srawi r11, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 10) as i64;
	// 82449774: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82449778: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244977C: 815F08A8  lwz r10, 0x8a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) } as u64;
	// 82449780: 4800003C  b 0x824497bc
	pc = 0x824497BC; continue 'dispatch;
	// 82449784: 817F08A4  lwz r11, 0x8a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2212 as u32) ) } as u64;
	// 82449788: 815F08A8  lwz r10, 0x8a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2216 as u32) ) } as u64;
	// 8244978C: 48000030  b 0x824497bc
	pc = 0x824497BC; continue 'dispatch;
	// 82449790: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82449794: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82449798: 419A0010  beq cr6, 0x824497a8
	if ctx.cr[6].eq {
	pc = 0x824497A8; continue 'dispatch;
	}
	// 8244979C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824497A0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824497A4: 48000018  b 0x824497bc
	pc = 0x824497BC; continue 'dispatch;
	// 824497A8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824497AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824497B0: 419A0020  beq cr6, 0x824497d0
	if ctx.cr[6].eq {
	pc = 0x824497D0; continue 'dispatch;
	}
	// 824497B4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824497B8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824497BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824497C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824497C4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824497C8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 824497CC: 4BFFFE25  bl 0x824495f0
	ctx.lr = 0x824497D0;
	sub_824495F0(ctx, base);
	// 824497D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824497D4: 480EB930  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824497D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824497D8 size=112
    let mut pc: u32 = 0x824497D8;
    'dispatch: loop {
        match pc {
            0x824497D8 => {
    //   block [0x824497D8..0x82449848)
	// 824497D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824497DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824497E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824497E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824497E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824497EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824497F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824497F4: 4BFFE04D  bl 0x82447840
	ctx.lr = 0x824497F8;
	sub_82447840(ctx, base);
	// 824497F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824497FC: 419A0018  beq cr6, 0x82449814
	if ctx.cr[6].eq {
	pc = 0x82449814; continue 'dispatch;
	}
	// 82449800: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82449804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449808: 60840159  ori r4, r4, 0x159
	ctx.r[4].u64 = ctx.r[4].u64 | 345;
	// 8244980C: 4BFFE0FD  bl 0x82447908
	ctx.lr = 0x82449810;
	sub_82447908(ctx, base);
	// 82449810: 48000020  b 0x82449830
	pc = 0x82449830; continue 'dispatch;
	// 82449814: 817F2658  lwz r11, 0x2658(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82449818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244981C: 419A0010  beq cr6, 0x8244982c
	if ctx.cr[6].eq {
	pc = 0x8244982C; continue 'dispatch;
	}
	// 82449820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449824: 93CB0DC4  stw r30, 0xdc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3524 as u32), ctx.r[30].u32 ) };
	// 82449828: 4BFFFDC9  bl 0x824495f0
	ctx.lr = 0x8244982C;
	sub_824495F0(ctx, base);
	// 8244982C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244983C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82449840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449848 size=100
    let mut pc: u32 = 0x82449848;
    'dispatch: loop {
        match pc {
            0x82449848 => {
    //   block [0x82449848..0x824498AC)
	// 82449848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244984C: 480EB871  bl 0x825350bc
	ctx.lr = 0x82449850;
	sub_82535080(ctx, base);
	// 82449850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82449858: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244985C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82449860: 4BFFDFE1  bl 0x82447840
	ctx.lr = 0x82449864;
	sub_82447840(ctx, base);
	// 82449864: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449868: 419A001C  beq cr6, 0x82449884
	if ctx.cr[6].eq {
	pc = 0x82449884; continue 'dispatch;
	}
	// 8244986C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82449870: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449874: 6084015A  ori r4, r4, 0x15a
	ctx.r[4].u64 = ctx.r[4].u64 | 346;
	// 82449878: 4BFFE091  bl 0x82447908
	ctx.lr = 0x8244987C;
	sub_82447908(ctx, base);
	// 8244987C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449880: 480EB88C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82449884: 817F2658  lwz r11, 0x2658(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82449888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244988C: 419A0014  beq cr6, 0x824498a0
	if ctx.cr[6].eq {
	pc = 0x824498A0; continue 'dispatch;
	}
	// 82449890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449894: 93CB0DC8  stw r30, 0xdc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3528 as u32), ctx.r[30].u32 ) };
	// 82449898: 93AB0DCC  stw r29, 0xdcc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3532 as u32), ctx.r[29].u32 ) };
	// 8244989C: 4BFFFD55  bl 0x824495f0
	ctx.lr = 0x824498A0;
	sub_824495F0(ctx, base);
	// 824498A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824498A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824498A8: 480EB864  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824498B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824498B0 size=112
    let mut pc: u32 = 0x824498B0;
    'dispatch: loop {
        match pc {
            0x824498B0 => {
    //   block [0x824498B0..0x82449920)
	// 824498B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824498B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824498B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824498BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824498C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824498C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824498C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824498CC: 4BFFDF75  bl 0x82447840
	ctx.lr = 0x824498D0;
	sub_82447840(ctx, base);
	// 824498D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824498D4: 419A0018  beq cr6, 0x824498ec
	if ctx.cr[6].eq {
	pc = 0x824498EC; continue 'dispatch;
	}
	// 824498D8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824498DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824498E0: 6084015B  ori r4, r4, 0x15b
	ctx.r[4].u64 = ctx.r[4].u64 | 347;
	// 824498E4: 4BFFE025  bl 0x82447908
	ctx.lr = 0x824498E8;
	sub_82447908(ctx, base);
	// 824498E8: 48000020  b 0x82449908
	pc = 0x82449908; continue 'dispatch;
	// 824498EC: 817F2658  lwz r11, 0x2658(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 824498F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824498F4: 419A0010  beq cr6, 0x82449904
	if ctx.cr[6].eq {
	pc = 0x82449904; continue 'dispatch;
	}
	// 824498F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824498FC: 93CB0DD0  stw r30, 0xdd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3536 as u32), ctx.r[30].u32 ) };
	// 82449900: 4BFFFCF1  bl 0x824495f0
	ctx.lr = 0x82449904;
	sub_824495F0(ctx, base);
	// 82449904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244990C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449914: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82449918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244991C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449920 size=196
    let mut pc: u32 = 0x82449920;
    'dispatch: loop {
        match pc {
            0x82449920 => {
    //   block [0x82449920..0x824499E4)
	// 82449920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244992C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82449930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449934: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82449938: 83FE2658  lwz r31, 0x2658(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(9816 as u32) ) } as u64;
	// 8244993C: 4BFFA995  bl 0x824442d0
	ctx.lr = 0x82449940;
	sub_824442D0(ctx, base);
	// 82449940: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449944: 409A0088  bne cr6, 0x824499cc
	if !ctx.cr[6].eq {
	pc = 0x824499CC; continue 'dispatch;
	}
	// 82449948: 817F0DAC  lwz r11, 0xdac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3500 as u32) ) } as u64;
	// 8244994C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82449950: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82449954: 41990040  bgt cr6, 0x82449994
	if ctx.cr[6].gt {
	pc = 0x82449994; continue 'dispatch;
	}
	// 82449958: 817E2660  lwz r11, 0x2660(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(9824 as u32) ) } as u64;
	// 8244995C: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 82449960: 409A000C  bne cr6, 0x8244996c
	if !ctx.cr[6].eq {
	pc = 0x8244996C; continue 'dispatch;
	}
	// 82449964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82449968: 48000010  b 0x82449978
	pc = 0x82449978; continue 'dispatch;
	// 8244996C: 813F0DD4  lwz r9, 0xdd4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3540 as u32) ) } as u64;
	// 82449970: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82449974: 41980020  blt cr6, 0x82449994
	if ctx.cr[6].lt {
	pc = 0x82449994; continue 'dispatch;
	}
	// 82449978: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244997C: 4BFFFC45  bl 0x824495c0
	ctx.lr = 0x82449980;
	sub_824495C0(ctx, base);
	// 82449980: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82449984: 419A0010  beq cr6, 0x82449994
	if ctx.cr[6].eq {
	pc = 0x82449994; continue 'dispatch;
	}
	// 82449988: 7D634A14  add r11, r3, r9
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[9].u64;
	// 8244998C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82449990: 917F0DAC  stw r11, 0xdac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3500 as u32), ctx.r[11].u32 ) };
	// 82449994: 817F0DB0  lwz r11, 0xdb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3504 as u32) ) } as u64;
	// 82449998: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244999C: 41990020  bgt cr6, 0x824499bc
	if ctx.cr[6].gt {
	pc = 0x824499BC; continue 'dispatch;
	}
	// 824499A0: 817E0E40  lwz r11, 0xe40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3648 as u32) ) } as u64;
	// 824499A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824499A8: 40990014  ble cr6, 0x824499bc
	if !ctx.cr[6].gt {
	pc = 0x824499BC; continue 'dispatch;
	}
	// 824499AC: 917F0DB0  stw r11, 0xdb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3504 as u32), ctx.r[11].u32 ) };
	// 824499B0: 817E0E44  lwz r11, 0xe44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3652 as u32) ) } as u64;
	// 824499B4: 917F0DB4  stw r11, 0xdb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3508 as u32), ctx.r[11].u32 ) };
	// 824499B8: 4800000C  b 0x824499c4
	pc = 0x824499C4; continue 'dispatch;
	// 824499BC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824499C0: 419A000C  beq cr6, 0x824499cc
	if ctx.cr[6].eq {
	pc = 0x824499CC; continue 'dispatch;
	}
	// 824499C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824499C8: 4BFFFC29  bl 0x824495f0
	ctx.lr = 0x824499CC;
	sub_824495F0(ctx, base);
	// 824499CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824499D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824499D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824499D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824499DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824499E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824499E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824499E8 size=64
    let mut pc: u32 = 0x824499E8;
    'dispatch: loop {
        match pc {
            0x824499E8 => {
    //   block [0x824499E8..0x82449A28)
	// 824499E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824499EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824499F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824499F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824499F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824499FC: 817F2658  lwz r11, 0x2658(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82449A00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82449A04: 419A0010  beq cr6, 0x82449a14
	if ctx.cr[6].eq {
	pc = 0x82449A14; continue 'dispatch;
	}
	// 82449A08: 4BFFFC91  bl 0x82449698
	ctx.lr = 0x82449A0C;
	sub_82449698(ctx, base);
	// 82449A0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449A10: 4BFFFF11  bl 0x82449920
	ctx.lr = 0x82449A14;
	sub_82449920(ctx, base);
	// 82449A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449A28 size=92
    let mut pc: u32 = 0x82449A28;
    'dispatch: loop {
        match pc {
            0x82449A28 => {
    //   block [0x82449A28..0x82449A84)
	// 82449A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449A34: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449A38: 816B034C  lwz r11, 0x34c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(844 as u32) ) } as u64;
	// 82449A3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82449A40: 40990030  ble cr6, 0x82449a70
	if !ctx.cr[6].gt {
	pc = 0x82449A70; continue 'dispatch;
	}
	// 82449A44: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449A48: 816B0348  lwz r11, 0x348(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(840 as u32) ) } as u64;
	// 82449A4C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82449A50: 419A0020  beq cr6, 0x82449a70
	if ctx.cr[6].eq {
	pc = 0x82449A70; continue 'dispatch;
	}
	// 82449A54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82449A58: 4BF80821  bl 0x823ca278
	ctx.lr = 0x82449A5C;
	sub_823CA278(ctx, base);
	// 82449A5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449A60: 419A0010  beq cr6, 0x82449a70
	if ctx.cr[6].eq {
	pc = 0x82449A70; continue 'dispatch;
	}
	// 82449A64: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82449A68: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 82449A6C: 409A0008  bne cr6, 0x82449a74
	if !ctx.cr[6].eq {
	pc = 0x82449A74; continue 'dispatch;
	}
	// 82449A70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449A88 size=32
    let mut pc: u32 = 0x82449A88;
    'dispatch: loop {
        match pc {
            0x82449A88 => {
    //   block [0x82449A88..0x82449AA8)
	// 82449A88: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449A8C: 814B034C  lwz r10, 0x34c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(844 as u32) ) } as u64;
	// 82449A90: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82449A94: 914B034C  stw r10, 0x34c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(844 as u32), ctx.r[10].u32 ) };
	// 82449A98: 814B034C  lwz r10, 0x34c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(844 as u32) ) } as u64;
	// 82449A9C: 814B034C  lwz r10, 0x34c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(844 as u32) ) } as u64;
	// 82449AA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82449AA4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449AA8 size=12
    let mut pc: u32 = 0x82449AA8;
    'dispatch: loop {
        match pc {
            0x82449AA8 => {
    //   block [0x82449AA8..0x82449AB4)
	// 82449AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82449AAC: 914B034C  stw r10, 0x34c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(844 as u32), ctx.r[10].u32 ) };
	// 82449AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449AB8 size=28
    let mut pc: u32 = 0x82449AB8;
    'dispatch: loop {
        match pc {
            0x82449AB8 => {
    //   block [0x82449AB8..0x82449AD4)
	// 82449AB8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449ABC: E96B0340  ld r11, 0x340(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(832 as u32) ) };
	// 82449AC0: 2F2B0001  cmpdi cr6, r11, 1
	ctx.cr[6].compare_i64(ctx.r[11].s64, 1, &mut ctx.xer);
	// 82449AC4: 419A0010  beq cr6, 0x82449ad4
	if ctx.cr[6].eq {
		sub_82449AD4(ctx, base);
		return;
	}
	// 82449AC8: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82449ACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449AD0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449AD4 size=8
    let mut pc: u32 = 0x82449AD4;
    'dispatch: loop {
        match pc {
            0x82449AD4 => {
    //   block [0x82449AD4..0x82449ADC)
	// 82449AD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82449AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449AE0 size=12
    let mut pc: u32 = 0x82449AE0;
    'dispatch: loop {
        match pc {
            0x82449AE0 => {
    //   block [0x82449AE0..0x82449AEC)
	// 82449AE0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449AE4: E86B0340  ld r3, 0x340(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(832 as u32) ) };
	// 82449AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449AF0 size=12
    let mut pc: u32 = 0x82449AF0;
    'dispatch: loop {
        match pc {
            0x82449AF0 => {
    //   block [0x82449AF0..0x82449AFC)
	// 82449AF0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449AF4: F86B0340  std r3, 0x340(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(832 as u32), ctx.r[3].u64 ) };
	// 82449AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449B00 size=188
    let mut pc: u32 = 0x82449B00;
    'dispatch: loop {
        match pc {
            0x82449B00 => {
    //   block [0x82449B00..0x82449BBC)
	// 82449B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449B08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82449B0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449B10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82449B14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82449B18: 386B6508  addi r3, r11, 0x6508
	ctx.r[3].s64 = ctx.r[11].s64 + 25864;
	// 82449B1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82449B20: 4815C2A1  bl 0x825a5dc0
	ctx.lr = 0x82449B24;
	sub_825A5DC0(ctx, base);
	// 82449B24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449B28: 409A0008  bne cr6, 0x82449b30
	if !ctx.cr[6].eq {
	pc = 0x82449B30; continue 'dispatch;
	}
	// 82449B2C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82449B30: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449B34: 814B034C  lwz r10, 0x34c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(844 as u32) ) } as u64;
	// 82449B38: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82449B3C: 914B034C  stw r10, 0x34c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(844 as u32), ctx.r[10].u32 ) };
	// 82449B40: 816B034C  lwz r11, 0x34c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(844 as u32) ) } as u64;
	// 82449B44: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82449B48: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449B4C: 40990010  ble cr6, 0x82449b5c
	if !ctx.cr[6].gt {
	pc = 0x82449B5C; continue 'dispatch;
	}
	// 82449B50: 814B0348  lwz r10, 0x348(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(840 as u32) ) } as u64;
	// 82449B54: 7F0AF800  cmpw cr6, r10, r31
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82449B58: 419A0050  beq cr6, 0x82449ba8
	if ctx.cr[6].eq {
	pc = 0x82449BA8; continue 'dispatch;
	}
	// 82449B5C: 93EB0348  stw r31, 0x348(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(840 as u32), ctx.r[31].u32 ) };
	// 82449B60: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82449B64: 419A003C  beq cr6, 0x82449ba0
	if ctx.cr[6].eq {
	pc = 0x82449BA0; continue 'dispatch;
	}
	// 82449B68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82449B6C: 480FC4E5  bl 0x82546050
	ctx.lr = 0x82449B70;
	sub_82546050(ctx, base);
	// 82449B70: 3D6002FA  lis r11, 0x2fa
	ctx.r[11].s64 = 49938432;
	// 82449B74: 616AF080  ori r10, r11, 0xf080
	ctx.r[10].u64 = ctx.r[11].u64 | 61568;
	// 82449B78: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82449B7C: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 82449B80: 409A000C  bne cr6, 0x82449b8c
	if !ctx.cr[6].eq {
	pc = 0x82449B8C; continue 'dispatch;
	}
	// 82449B84: 3D6002F9  lis r11, 0x2f9
	ctx.r[11].s64 = 49872896;
	// 82449B88: 616B0838  ori r11, r11, 0x838
	ctx.r[11].u64 = ctx.r[11].u64 | 2104;
	// 82449B8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449B90: 419A0010  beq cr6, 0x82449ba0
	if ctx.cr[6].eq {
	pc = 0x82449BA0; continue 'dispatch;
	}
	// 82449B94: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82449B98: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82449B9C: 409A0008  bne cr6, 0x82449ba4
	if !ctx.cr[6].eq {
	pc = 0x82449BA4; continue 'dispatch;
	}
	// 82449BA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82449BA4: 4BFFFF4D  bl 0x82449af0
	ctx.lr = 0x82449BA8;
	sub_82449AF0(ctx, base);
	// 82449BA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449BC0 size=36
    let mut pc: u32 = 0x82449BC0;
    'dispatch: loop {
        match pc {
            0x82449BC0 => {
    //   block [0x82449BC0..0x82449BE4)
	// 82449BC0: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 82449BC4: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 82449BC8: 7C6907B4  extsw r9, r3
	ctx.r[9].s64 = ctx.r[3].s32 as i64;
	// 82449BCC: 7CC807B4  extsw r8, r6
	ctx.r[8].s64 = ctx.r[6].s32 as i64;
	// 82449BD0: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 82449BD4: 7D4941D2  mulld r10, r9, r8
	ctx.r[10].s64 = ctx.r[9].s64 * ctx.r[8].s64;
	// 82449BD8: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 82449BDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82449BE0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449BE4 size=8
    let mut pc: u32 = 0x82449BE4;
    'dispatch: loop {
        match pc {
            0x82449BE4 => {
    //   block [0x82449BE4..0x82449BEC)
	// 82449BE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449BF0 size=52
    let mut pc: u32 = 0x82449BF0;
    'dispatch: loop {
        match pc {
            0x82449BF0 => {
    //   block [0x82449BF0..0x82449C24)
	// 82449BF0: 54AA073E  clrlwi r10, r5, 0x1c
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 82449BF4: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82449BF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82449BFC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82449C00: 419A0018  beq cr6, 0x82449c18
	if ctx.cr[6].eq {
	pc = 0x82449C18; continue 'dispatch;
	}
	// 82449C04: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C08: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82449C0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82449C10: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C14: 409AFFF0  bne cr6, 0x82449c04
	if !ctx.cr[6].eq {
	pc = 0x82449C04; continue 'dispatch;
	}
	// 82449C18: 54AAE13E  srwi r10, r5, 4
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82449C1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82449C20: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449C24 size=144
    let mut pc: u32 = 0x82449C24;
    'dispatch: loop {
        match pc {
            0x82449C24 => {
    //   block [0x82449C24..0x82449CB4)
	// 82449C24: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C28: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82449C2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82449C30: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C34: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C38: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C3C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C40: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C44: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C48: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C4C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C50: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C54: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C58: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C5C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C60: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C64: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C68: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C6C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C70: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C74: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C78: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C7C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C80: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C84: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C88: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C8C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C90: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C94: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449C98: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449C9C: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449CA0: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449CA4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82449CA8: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82449CAC: 409AFF78  bne cr6, 0x82449c24
	if !ctx.cr[6].eq {
	pc = 0x82449C24; continue 'dispatch;
	}
	// 82449CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449CB8 size=212
    let mut pc: u32 = 0x82449CB8;
    'dispatch: loop {
        match pc {
            0x82449CB8 => {
    //   block [0x82449CB8..0x82449D8C)
	// 82449CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449CC4: 4BFFFDF5  bl 0x82449ab8
	ctx.lr = 0x82449CC8;
	sub_82449AB8(ctx, base);
	// 82449CC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449CCC: 409A0024  bne cr6, 0x82449cf0
	if !ctx.cr[6].eq {
	pc = 0x82449CF0; continue 'dispatch;
	}
	// 82449CD0: 4BFFFE11  bl 0x82449ae0
	ctx.lr = 0x82449CD4;
	sub_82449AE0(ctx, base);
	// 82449CD4: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82449CD8: F86BD998  std r3, -0x2668(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(-9832 as u32), ctx.r[3].u64 ) };
	// 82449CDC: 4BFFFD4D  bl 0x82449a28
	ctx.lr = 0x82449CE0;
	sub_82449A28(ctx, base);
	// 82449CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449CEC: 4E800020  blr
	return;
	// 82449CF0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449CF4: 816B06D0  lwz r11, 0x6d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1744 as u32) ) } as u64;
	// 82449CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82449CFC: 419A0058  beq cr6, 0x82449d54
	if ctx.cr[6].eq {
	pc = 0x82449D54; continue 'dispatch;
	}
	// 82449D00: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82449D04: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82449D08: 419A004C  beq cr6, 0x82449d54
	if ctx.cr[6].eq {
	pc = 0x82449D54; continue 'dispatch;
	}
	// 82449D0C: 814B106C  lwz r10, 0x106c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4204 as u32) ) } as u64;
	// 82449D10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82449D14: 419A0040  beq cr6, 0x82449d54
	if ctx.cr[6].eq {
	pc = 0x82449D54; continue 'dispatch;
	}
	// 82449D18: 806B1080  lwz r3, 0x1080(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4224 as u32) ) } as u64;
	// 82449D1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82449D20: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82449D24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82449D28: 4E800421  bctrl
	ctx.lr = 0x82449D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82449D2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82449D30: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 82449D34: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82449D38: F96AD998  std r11, -0x2668(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(-9832 as u32), ctx.r[11].u64 ) };
	// 82449D3C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82449D40: 7D6307B4  extsw r3, r11
	ctx.r[3].s64 = ctx.r[11].s32 as i64;
	// 82449D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449D50: 4E800020  blr
	return;
	// 82449D54: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82449D58: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 82449D5C: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 82449D60: 814B01BC  lwz r10, 0x1bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 82449D64: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82449D68: F949D998  std r10, -0x2668(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(-9832 as u32), ctx.r[10].u64 ) };
	// 82449D6C: 814B01B0  lwz r10, 0x1b0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(432 as u32) ) } as u64;
	// 82449D70: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82449D74: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82449D78: 7D6307B4  extsw r3, r11
	ctx.r[3].s64 = ctx.r[11].s32 as i64;
	// 82449D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449D90 size=60
    let mut pc: u32 = 0x82449D90;
    'dispatch: loop {
        match pc {
            0x82449D90 => {
    //   block [0x82449D90..0x82449DCC)
	// 82449D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82449D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449DA0: 3FE08290  lis r31, -0x7d70
	ctx.r[31].s64 = -2104492032;
	// 82449DA4: E87FD998  ld r3, -0x2668(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(-9832 as u32) ) };
	// 82449DA8: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 82449DAC: 409A000C  bne cr6, 0x82449db8
	if !ctx.cr[6].eq {
	pc = 0x82449DB8; continue 'dispatch;
	}
	// 82449DB0: 4BFFFF09  bl 0x82449cb8
	ctx.lr = 0x82449DB4;
	sub_82449CB8(ctx, base);
	// 82449DB4: E87FD998  ld r3, -0x2668(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(-9832 as u32) ) };
	// 82449DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449DD0 size=32
    let mut pc: u32 = 0x82449DD0;
    'dispatch: loop {
        match pc {
            0x82449DD0 => {
    //   block [0x82449DD0..0x82449DF0)
	// 82449DD0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82449DD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82449DD8: 794A0040  clrldi r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 82449DDC: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82449DE0: F9430008  std r10, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82449DE4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82449DE8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82449DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449DF0 size=72
    let mut pc: u32 = 0x82449DF0;
    'dispatch: loop {
        match pc {
            0x82449DF0 => {
    //   block [0x82449DF0..0x82449E38)
	// 82449DF0: E9430000  ld r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82449DF4: E9630008  ld r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 82449DF8: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82449DFC: 7F245800  cmpd cr6, r4, r11
	ctx.cr[6].compare_i64(ctx.r[4].s64, ctx.r[11].s64, &mut ctx.xer);
	// 82449E00: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82449E04: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82449E08: 41980008  blt cr6, 0x82449e10
	if ctx.cr[6].lt {
	pc = 0x82449E10; continue 'dispatch;
	}
	// 82449E0C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82449E10: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82449E14: F9430008  std r10, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82449E18: 7F245800  cmpd cr6, r4, r11
	ctx.cr[6].compare_i64(ctx.r[4].s64, ctx.r[11].s64, &mut ctx.xer);
	// 82449E1C: 40990008  ble cr6, 0x82449e24
	if !ctx.cr[6].gt {
	pc = 0x82449E24; continue 'dispatch;
	}
	// 82449E20: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82449E24: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82449E28: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82449E2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82449E30: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82449E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449E38 size=4
    let mut pc: u32 = 0x82449E38;
    'dispatch: loop {
        match pc {
            0x82449E38 => {
    //   block [0x82449E38..0x82449E3C)
	// 82449E38: 480EAD18  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449E40 size=16
    let mut pc: u32 = 0x82449E40;
    'dispatch: loop {
        match pc {
            0x82449E40 => {
    //   block [0x82449E40..0x82449E50)
	// 82449E40: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 82449E44: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82449E48: 388BD9A0  addi r4, r11, -0x2660
	ctx.r[4].s64 = ctx.r[11].s64 + -9824;
	// 82449E4C: 4BFFB4DC  b 0x82445328
	sub_82445328(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449E50 size=28
    let mut pc: u32 = 0x82449E50;
    'dispatch: loop {
        match pc {
            0x82449E50 => {
    //   block [0x82449E50..0x82449E6C)
	// 82449E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82449E54: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82449E58: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82449E5C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82449E60: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82449E64: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82449E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449E70 size=20
    let mut pc: u32 = 0x82449E70;
    'dispatch: loop {
        match pc {
            0x82449E70 => {
    //   block [0x82449E70..0x82449E84)
	// 82449E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82449E74: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82449E78: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82449E7C: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82449E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449E88 size=48
    let mut pc: u32 = 0x82449E88;
    'dispatch: loop {
        match pc {
            0x82449E88 => {
    //   block [0x82449E88..0x82449EB8)
	// 82449E88: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82449E8C: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82449E90: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82449E94: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82449E98: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82449E9C: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82449EA0: 896B0002  lbz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82449EA4: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82449EA8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82449EAC: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82449EB0: 7D435B78  or r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82449EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449EB8 size=112
    let mut pc: u32 = 0x82449EB8;
    'dispatch: loop {
        match pc {
            0x82449EB8 => {
    //   block [0x82449EB8..0x82449F28)
	// 82449EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82449EC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449EC8: 4BFFB4C1  bl 0x82445388
	ctx.lr = 0x82449ECC;
	sub_82445388(ctx, base);
	// 82449ECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82449ED0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82449ED4: 409A0018  bne cr6, 0x82449eec
	if !ctx.cr[6].eq {
	pc = 0x82449EEC; continue 'dispatch;
	}
	// 82449ED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449EE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449EE8: 4E800020  blr
	return;
	// 82449EEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82449EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449EF4: 4BFFB565  bl 0x82445458
	ctx.lr = 0x82449EF8;
	sub_82445458(ctx, base);
	// 82449EF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449EFC: 409A000C  bne cr6, 0x82449f08
	if !ctx.cr[6].eq {
	pc = 0x82449F08; continue 'dispatch;
	}
	// 82449F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82449F04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82449F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449F0C: 4BFFB515  bl 0x82445420
	ctx.lr = 0x82449F10;
	sub_82445420(ctx, base);
	// 82449F10: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82449F14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82449F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449F20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82449F28 size=8
    let mut pc: u32 = 0x82449F28;
    'dispatch: loop {
        match pc {
            0x82449F28 => {
    //   block [0x82449F28..0x82449F30)
	// 82449F28: 38A00894  li r5, 0x894
	ctx.r[5].s64 = 2196;
	// 82449F2C: 480EAC24  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449F30 size=100
    let mut pc: u32 = 0x82449F30;
    'dispatch: loop {
        match pc {
            0x82449F30 => {
    //   block [0x82449F30..0x82449F94)
	// 82449F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82449F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82449F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82449F44: 817F2658  lwz r11, 0x2658(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82449F48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82449F4C: 409A001C  bne cr6, 0x82449f68
	if !ctx.cr[6].eq {
	pc = 0x82449F68; continue 'dispatch;
	}
	// 82449F50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449F60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449F64: 4E800020  blr
	return;
	// 82449F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449F6C: 4BFF345D  bl 0x8243d3c8
	ctx.lr = 0x82449F70;
	sub_8243D3C8(ctx, base);
	// 82449F70: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449F74: 4199FFDC  bgt cr6, 0x82449f50
	if ctx.cr[6].gt {
	pc = 0x82449F50; continue 'dispatch;
	}
	// 82449F78: 817F2658  lwz r11, 0x2658(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82449F7C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82449F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82449F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82449F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82449F8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82449F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82449F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82449F98 size=104
    let mut pc: u32 = 0x82449F98;
    'dispatch: loop {
        match pc {
            0x82449F98 => {
    //   block [0x82449F98..0x8244A000)
	// 82449F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82449F9C: 480EB121  bl 0x825350bc
	ctx.lr = 0x82449FA0;
	sub_82535080(ctx, base);
	// 82449FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82449FA4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82449FA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82449FAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82449FB0: 7F04F000  cmpw cr6, r4, r30
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82449FB4: 41990034  bgt cr6, 0x82449fe8
	if ctx.cr[6].gt {
	pc = 0x82449FE8; continue 'dispatch;
	}
	// 82449FB8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82449FBC: 57E4063E  clrlwi r4, r31, 0x18
	ctx.r[4].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82449FC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82449FC4: 4BFFB61D  bl 0x824455e0
	ctx.lr = 0x82449FC8;
	sub_824455E0(ctx, base);
	// 82449FC8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82449FCC: 419A0010  beq cr6, 0x82449fdc
	if ctx.cr[6].eq {
	pc = 0x82449FDC; continue 'dispatch;
	}
	// 82449FD0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82449FD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82449FD8: 409A001C  bne cr6, 0x82449ff4
	if !ctx.cr[6].eq {
	pc = 0x82449FF4; continue 'dispatch;
	}
	// 82449FDC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82449FE0: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82449FE4: 4099FFD4  ble cr6, 0x82449fb8
	if !ctx.cr[6].gt {
	pc = 0x82449FB8; continue 'dispatch;
	}
	// 82449FE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82449FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82449FF0: 480EB11C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82449FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82449FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82449FFC: 480EB110  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A000 size=60
    let mut pc: u32 = 0x8244A000;
    'dispatch: loop {
        match pc {
            0x8244A000 => {
    //   block [0x8244A000..0x8244A03C)
	// 8244A000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A00C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8244A010: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8244A014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244A018: 4E800421  bctrl
	ctx.lr = 0x8244A01C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8244A01C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A020: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244A024: 419A0008  beq cr6, 0x8244a02c
	if ctx.cr[6].eq {
	pc = 0x8244A02C; continue 'dispatch;
	}
	// 8244A028: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244A02C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244A030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A040 size=64
    let mut pc: u32 = 0x8244A040;
    'dispatch: loop {
        match pc {
            0x8244A040 => {
    //   block [0x8244A040..0x8244A080)
	// 8244A040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A04C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8244A050: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8244A054: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8244A058: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244A05C: 4E800421  bctrl
	ctx.lr = 0x8244A060;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8244A060: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A064: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244A068: 419A0008  beq cr6, 0x8244a070
	if ctx.cr[6].eq {
	pc = 0x8244A070; continue 'dispatch;
	}
	// 8244A06C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244A070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244A074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A080 size=28
    let mut pc: u32 = 0x8244A080;
    'dispatch: loop {
        match pc {
            0x8244A080 => {
    //   block [0x8244A080..0x8244A09C)
	// 8244A080: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8244A084: 91630930  stw r11, 0x930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2352 as u32), ctx.r[11].u32 ) };
	// 8244A088: 816300A0  lwz r11, 0xa0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 8244A08C: 91630934  stw r11, 0x934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2356 as u32), ctx.r[11].u32 ) };
	// 8244A090: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8244A094: 91630938  stw r11, 0x938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2360 as u32), ctx.r[11].u32 ) };
	// 8244A098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A0A0 size=20
    let mut pc: u32 = 0x8244A0A0;
    'dispatch: loop {
        match pc {
            0x8244A0A0 => {
    //   block [0x8244A0A0..0x8244A0B4)
	// 8244A0A0: 81630078  lwz r11, 0x78(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 8244A0A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244A0A8: 409A000C  bne cr6, 0x8244a0b4
	if !ctx.cr[6].eq {
		sub_8244A0B4(ctx, base);
		return;
	}
	// 8244A0AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244A0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A0B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A0B4 size=20
    let mut pc: u32 = 0x8244A0B4;
    'dispatch: loop {
        match pc {
            0x8244A0B4 => {
    //   block [0x8244A0B4..0x8244A0C8)
	// 8244A0B4: 8163007C  lwz r11, 0x7c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 8244A0B8: 81430080  lwz r10, 0x80(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 8244A0BC: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 8244A0C0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244A0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A0C8 size=20
    let mut pc: u32 = 0x8244A0C8;
    'dispatch: loop {
        match pc {
            0x8244A0C8 => {
    //   block [0x8244A0C8..0x8244A0DC)
	// 8244A0C8: 39630078  addi r11, r3, 0x78
	ctx.r[11].s64 = ctx.r[3].s64 + 120;
	// 8244A0CC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244A0D0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244A0D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244A0D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A0DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A0DC size=12
    let mut pc: u32 = 0x8244A0DC;
    'dispatch: loop {
        match pc {
            0x8244A0DC => {
    //   block [0x8244A0DC..0x8244A0E8)
	// 8244A0DC: 814B0070  lwz r10, 0x70(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8244A0E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244A0E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A0E8 size=8
    let mut pc: u32 = 0x8244A0E8;
    'dispatch: loop {
        match pc {
            0x8244A0E8 => {
    //   block [0x8244A0E8..0x8244A0F0)
	// 8244A0E8: 806B0074  lwz r3, 0x74(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 8244A0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A0F0 size=44
    let mut pc: u32 = 0x8244A0F0;
    'dispatch: loop {
        match pc {
            0x8244A0F0 => {
    //   block [0x8244A0F0..0x8244A11C)
	// 8244A0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A0F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A0FC: 4BFFFD8D  bl 0x82449e88
	ctx.lr = 0x8244A100;
	sub_82449E88(ctx, base);
	// 8244A100: 3963FE41  addi r11, r3, -0x1bf
	ctx.r[11].s64 = ctx.r[3].s64 + -447;
	// 8244A104: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8244A108: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8244A10C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244A110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A120 size=60
    let mut pc: u32 = 0x8244A120;
    'dispatch: loop {
        match pc {
            0x8244A120 => {
    //   block [0x8244A120..0x8244A15C)
	// 8244A120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244A12C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244A134: 4BFFFDFD  bl 0x82449f30
	ctx.lr = 0x8244A138;
	sub_82449F30(ctx, base);
	// 8244A138: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8244A13C: 419A000C  beq cr6, 0x8244a148
	if ctx.cr[6].eq {
	pc = 0x8244A148; continue 'dispatch;
	}
	// 8244A140: 389F0078  addi r4, r31, 0x78
	ctx.r[4].s64 = ctx.r[31].s64 + 120;
	// 8244A144: 4BFFFDE5  bl 0x82449f28
	ctx.lr = 0x8244A148;
	sub_82449F28(ctx, base);
	// 8244A148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244A14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244A158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A160 size=152
    let mut pc: u32 = 0x8244A160;
    'dispatch: loop {
        match pc {
            0x8244A160 => {
    //   block [0x8244A160..0x8244A1F8)
	// 8244A160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244A16C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244A170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A174: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A178: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8244A17C: 388B5658  addi r4, r11, 0x5658
	ctx.r[4].s64 = ctx.r[11].s64 + 22104;
	// 8244A180: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244A184: 4BFFFE7D  bl 0x8244a000
	ctx.lr = 0x8244A188;
	sub_8244A000(ctx, base);
	// 8244A188: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A18C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A194: 388B5680  addi r4, r11, 0x5680
	ctx.r[4].s64 = ctx.r[11].s64 + 22144;
	// 8244A198: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8244A19C: 4BFFFE65  bl 0x8244a000
	ctx.lr = 0x8244A1A0;
	sub_8244A000(ctx, base);
	// 8244A1A0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A1A4: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A1A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A1AC: 388B56A8  addi r4, r11, 0x56a8
	ctx.r[4].s64 = ctx.r[11].s64 + 22184;
	// 8244A1B0: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8244A1B4: 4BFFFE4D  bl 0x8244a000
	ctx.lr = 0x8244A1B8;
	sub_8244A000(ctx, base);
	// 8244A1B8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8244A1BC: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8244A1C0: 409A000C  bne cr6, 0x8244a1cc
	if !ctx.cr[6].eq {
	pc = 0x8244A1CC; continue 'dispatch;
	}
	// 8244A1C4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8244A1C8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8244A1CC: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A1D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A1D4: 388B56D0  addi r4, r11, 0x56d0
	ctx.r[4].s64 = ctx.r[11].s64 + 22224;
	// 8244A1D8: 4BFFFE29  bl 0x8244a000
	ctx.lr = 0x8244A1DC;
	sub_8244A000(ctx, base);
	// 8244A1DC: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8244A1E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244A1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A1EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244A1F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244A1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A1F8 size=212
    let mut pc: u32 = 0x8244A1F8;
    'dispatch: loop {
        match pc {
            0x8244A1F8 => {
    //   block [0x8244A1F8..0x8244A2CC)
	// 8244A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244A204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244A208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A20C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A210: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244A214: 388B56F8  addi r4, r11, 0x56f8
	ctx.r[4].s64 = ctx.r[11].s64 + 22264;
	// 8244A218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244A21C: 4BFFFDE5  bl 0x8244a000
	ctx.lr = 0x8244A220;
	sub_8244A000(ctx, base);
	// 8244A220: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A224: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A22C: 388B5720  addi r4, r11, 0x5720
	ctx.r[4].s64 = ctx.r[11].s64 + 22304;
	// 8244A230: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244A234: 4BFFFDCD  bl 0x8244a000
	ctx.lr = 0x8244A238;
	sub_8244A000(ctx, base);
	// 8244A238: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A23C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A244: 388B5748  addi r4, r11, 0x5748
	ctx.r[4].s64 = ctx.r[11].s64 + 22344;
	// 8244A248: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8244A24C: 4BFFFDB5  bl 0x8244a000
	ctx.lr = 0x8244A250;
	sub_8244A000(ctx, base);
	// 8244A250: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A254: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A25C: 388B5770  addi r4, r11, 0x5770
	ctx.r[4].s64 = ctx.r[11].s64 + 22384;
	// 8244A260: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8244A264: 4BFFFD9D  bl 0x8244a000
	ctx.lr = 0x8244A268;
	sub_8244A000(ctx, base);
	// 8244A268: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A26C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A274: 388B57C0  addi r4, r11, 0x57c0
	ctx.r[4].s64 = ctx.r[11].s64 + 22464;
	// 8244A278: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8244A27C: 4BFFFD85  bl 0x8244a000
	ctx.lr = 0x8244A280;
	sub_8244A000(ctx, base);
	// 8244A280: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A284: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A28C: 388B57E8  addi r4, r11, 0x57e8
	ctx.r[4].s64 = ctx.r[11].s64 + 22504;
	// 8244A290: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8244A294: 4BFFFD6D  bl 0x8244a000
	ctx.lr = 0x8244A298;
	sub_8244A000(ctx, base);
	// 8244A298: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A29C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A2A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A2A4: 388B5810  addi r4, r11, 0x5810
	ctx.r[4].s64 = ctx.r[11].s64 + 22544;
	// 8244A2A8: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8244A2AC: 4BFFFD55  bl 0x8244a000
	ctx.lr = 0x8244A2B0;
	sub_8244A000(ctx, base);
	// 8244A2B0: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8244A2B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244A2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A2C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244A2C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244A2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A2D0 size=140
    let mut pc: u32 = 0x8244A2D0;
    'dispatch: loop {
        match pc {
            0x8244A2D0 => {
    //   block [0x8244A2D0..0x8244A35C)
	// 8244A2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A2D4: 480EADE9  bl 0x825350bc
	ctx.lr = 0x8244A2D8;
	sub_82535080(ctx, base);
	// 8244A2D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A2DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8244A2E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244A2E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244A2E8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8244A2EC: 419A0068  beq cr6, 0x8244a354
	if ctx.cr[6].eq {
	pc = 0x8244A354; continue 'dispatch;
	}
	// 8244A2F0: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A2F4: 38AB5860  addi r5, r11, 0x5860
	ctx.r[5].s64 = ctx.r[11].s64 + 22624;
	// 8244A2F8: 4BFFFD49  bl 0x8244a040
	ctx.lr = 0x8244A2FC;
	sub_8244A040(ctx, base);
	// 8244A2FC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A300: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A304: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244A308: 38AB5888  addi r5, r11, 0x5888
	ctx.r[5].s64 = ctx.r[11].s64 + 22664;
	// 8244A30C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A310: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244A314: 4BFFFD2D  bl 0x8244a040
	ctx.lr = 0x8244A318;
	sub_8244A040(ctx, base);
	// 8244A318: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A31C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A320: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244A324: 38AB58B0  addi r5, r11, 0x58b0
	ctx.r[5].s64 = ctx.r[11].s64 + 22704;
	// 8244A328: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A32C: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8244A330: 4BFFFD11  bl 0x8244a040
	ctx.lr = 0x8244A334;
	sub_8244A040(ctx, base);
	// 8244A334: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A338: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A33C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244A340: 38AB58D8  addi r5, r11, 0x58d8
	ctx.r[5].s64 = ctx.r[11].s64 + 22744;
	// 8244A344: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A348: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8244A34C: 4BFFFCF5  bl 0x8244a040
	ctx.lr = 0x8244A350;
	sub_8244A040(ctx, base);
	// 8244A350: 907D000C  stw r3, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8244A354: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244A358: 480EADB4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A360 size=416
    let mut pc: u32 = 0x8244A360;
    'dispatch: loop {
        match pc {
            0x8244A360 => {
    //   block [0x8244A360..0x8244A500)
	// 8244A360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A364: 480EAD4D  bl 0x825350b0
	ctx.lr = 0x8244A368;
	sub_82535080(ctx, base);
	// 8244A368: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A36C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A370: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8244A374: 38AB5900  addi r5, r11, 0x5900
	ctx.r[5].s64 = ctx.r[11].s64 + 22784;
	// 8244A378: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244A37C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8244A380: 4BFFFCC1  bl 0x8244a040
	ctx.lr = 0x8244A384;
	sub_8244A040(ctx, base);
	// 8244A384: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A388: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A38C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A390: 38AB5928  addi r5, r11, 0x5928
	ctx.r[5].s64 = ctx.r[11].s64 + 22824;
	// 8244A394: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A398: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244A39C: 4BFFFCA5  bl 0x8244a040
	ctx.lr = 0x8244A3A0;
	sub_8244A040(ctx, base);
	// 8244A3A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A3A4: 3B9F000C  addi r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 + 12;
	// 8244A3A8: 3B7F0008  addi r27, r31, 8
	ctx.r[27].s64 = ctx.r[31].s64 + 8;
	// 8244A3AC: 57BA063E  clrlwi r26, r29, 0x18
	ctx.r[26].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8244A3B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A3B4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8244A3B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8244A3BC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8244A3C0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8244A3C4: 4BFFB58D  bl 0x82445950
	ctx.lr = 0x8244A3C8;
	sub_82445950(ctx, base);
	// 8244A3C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A3CC: 409A0010  bne cr6, 0x8244a3dc
	if !ctx.cr[6].eq {
	pc = 0x8244A3DC; continue 'dispatch;
	}
	// 8244A3D0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8244A3D4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244A3D8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244A3DC: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A3E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A3E4: 38AB5978  addi r5, r11, 0x5978
	ctx.r[5].s64 = ctx.r[11].s64 + 22904;
	// 8244A3E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A3EC: 4BFFFC55  bl 0x8244a040
	ctx.lr = 0x8244A3F0;
	sub_8244A040(ctx, base);
	// 8244A3F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A3F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8244A3F8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8244A3FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A400: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8244A404: 4BFFB205  bl 0x82445608
	ctx.lr = 0x8244A408;
	sub_82445608(ctx, base);
	// 8244A408: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A40C: 409A0010  bne cr6, 0x8244a41c
	if !ctx.cr[6].eq {
	pc = 0x8244A41C; continue 'dispatch;
	}
	// 8244A410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244A414: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8244A418: 48000008  b 0x8244a420
	pc = 0x8244A420; continue 'dispatch;
	// 8244A41C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244A420: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8244A424: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244A428: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8244A42C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8244A430: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8244A434: 419A00C4  beq cr6, 0x8244a4f8
	if ctx.cr[6].eq {
	pc = 0x8244A4F8; continue 'dispatch;
	}
	// 8244A438: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A43C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A440: 38AB59A0  addi r5, r11, 0x59a0
	ctx.r[5].s64 = ctx.r[11].s64 + 22944;
	// 8244A444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A448: 4BFFFBF9  bl 0x8244a040
	ctx.lr = 0x8244A44C;
	sub_8244A040(ctx, base);
	// 8244A44C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A450: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A454: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A458: 38AB59C8  addi r5, r11, 0x59c8
	ctx.r[5].s64 = ctx.r[11].s64 + 22984;
	// 8244A45C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A460: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8244A464: 4BFFFBDD  bl 0x8244a040
	ctx.lr = 0x8244A468;
	sub_8244A040(ctx, base);
	// 8244A468: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A46C: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A470: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A474: 38AB59F0  addi r5, r11, 0x59f0
	ctx.r[5].s64 = ctx.r[11].s64 + 23024;
	// 8244A478: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A47C: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8244A480: 4BFFFBC1  bl 0x8244a040
	ctx.lr = 0x8244A484;
	sub_8244A040(ctx, base);
	// 8244A484: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A488: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A48C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A490: 38AB5A18  addi r5, r11, 0x5a18
	ctx.r[5].s64 = ctx.r[11].s64 + 23064;
	// 8244A494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A498: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8244A49C: 4BFFFBA5  bl 0x8244a040
	ctx.lr = 0x8244A4A0;
	sub_8244A040(ctx, base);
	// 8244A4A0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A4A4: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A4A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A4AC: 38AB5A40  addi r5, r11, 0x5a40
	ctx.r[5].s64 = ctx.r[11].s64 + 23104;
	// 8244A4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A4B4: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8244A4B8: 4BFFFB89  bl 0x8244a040
	ctx.lr = 0x8244A4BC;
	sub_8244A040(ctx, base);
	// 8244A4BC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A4C0: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A4C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A4C8: 38AB5A68  addi r5, r11, 0x5a68
	ctx.r[5].s64 = ctx.r[11].s64 + 23144;
	// 8244A4CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A4D0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8244A4D4: 4BFFFB6D  bl 0x8244a040
	ctx.lr = 0x8244A4D8;
	sub_8244A040(ctx, base);
	// 8244A4D8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244A4DC: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8244A4E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244A4E4: 38AB5A90  addi r5, r11, 0x5a90
	ctx.r[5].s64 = ctx.r[11].s64 + 23184;
	// 8244A4E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A4EC: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8244A4F0: 4BFFFB51  bl 0x8244a040
	ctx.lr = 0x8244A4F4;
	sub_8244A040(ctx, base);
	// 8244A4F4: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 8244A4F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8244A4FC: 480EAC04  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A500 size=340
    let mut pc: u32 = 0x8244A500;
    'dispatch: loop {
        match pc {
            0x8244A500 => {
    //   block [0x8244A500..0x8244A654)
	// 8244A500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A504: 480EABB9  bl 0x825350bc
	ctx.lr = 0x8244A508;
	sub_82535080(ctx, base);
	// 8244A508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A50C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8244A510: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 8244A514: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244A518: 4BFFAF41  bl 0x82445458
	ctx.lr = 0x8244A51C;
	sub_82445458(ctx, base);
	// 8244A51C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A520: 419A012C  beq cr6, 0x8244a64c
	if ctx.cr[6].eq {
	pc = 0x8244A64C; continue 'dispatch;
	}
	// 8244A524: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8244A528: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244A52C: 419A0120  beq cr6, 0x8244a64c
	if ctx.cr[6].eq {
	pc = 0x8244A64C; continue 'dispatch;
	}
	// 8244A530: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8244A534: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8244A538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A53C: 4BFFB0F5  bl 0x82445630
	ctx.lr = 0x8244A540;
	sub_82445630(ctx, base);
	// 8244A540: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A544: 409A0018  bne cr6, 0x8244a55c
	if !ctx.cr[6].eq {
	pc = 0x8244A55C; continue 'dispatch;
	}
	// 8244A548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244A54C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244A550: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8244A554: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8244A558: 4800000C  b 0x8244a564
	pc = 0x8244A564; continue 'dispatch;
	// 8244A55C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244A560: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8244A564: 1D2A0064  mulli r9, r10, 0x64
	ctx.r[9].s64 = ctx.r[10].s64 * 100;
	// 8244A568: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8244A56C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8244A570: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244A574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A578: 7FA95A14  add r29, r9, r11
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8244A57C: 4BFFB21D  bl 0x82445798
	ctx.lr = 0x8244A580;
	sub_82445798(ctx, base);
	// 8244A580: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A584: 409A0010  bne cr6, 0x8244a594
	if !ctx.cr[6].eq {
	pc = 0x8244A594; continue 'dispatch;
	}
	// 8244A588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244A58C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8244A590: 48000008  b 0x8244a598
	pc = 0x8244A598; continue 'dispatch;
	// 8244A594: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244A598: 2F1D006E  cmpwi cr6, r29, 0x6e
	ctx.cr[6].compare_i32(ctx.r[29].s32, 110, &mut ctx.xer);
	// 8244A59C: 4098000C  bge cr6, 0x8244a5a8
	if !ctx.cr[6].lt {
	pc = 0x8244A5A8; continue 'dispatch;
	}
	// 8244A5A0: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8244A5A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8244A5A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244A5AC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8244A5B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A5B4: 4BFFFBAD  bl 0x8244a160
	ctx.lr = 0x8244A5B8;
	sub_8244A160(ctx, base);
	// 8244A5B8: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8244A5BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A5C0: 4BFFFC39  bl 0x8244a1f8
	ctx.lr = 0x8244A5C4;
	sub_8244A1F8(ctx, base);
	// 8244A5C4: 38A000BD  li r5, 0xbd
	ctx.r[5].s64 = 189;
	// 8244A5C8: 388000BD  li r4, 0xbd
	ctx.r[4].s64 = 189;
	// 8244A5CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A5D0: 4BFFF9C9  bl 0x82449f98
	ctx.lr = 0x8244A5D4;
	sub_82449F98(ctx, base);
	// 8244A5D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A5D8: 38A000BF  li r5, 0xbf
	ctx.r[5].s64 = 191;
	// 8244A5DC: 388000BF  li r4, 0xbf
	ctx.r[4].s64 = 191;
	// 8244A5E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A5E4: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8244A5E8: 4BFFF9B1  bl 0x82449f98
	ctx.lr = 0x8244A5EC;
	sub_82449F98(ctx, base);
	// 8244A5EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A5F0: 38A000DF  li r5, 0xdf
	ctx.r[5].s64 = 223;
	// 8244A5F4: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 8244A5F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A5FC: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8244A600: 4BFFF999  bl 0x82449f98
	ctx.lr = 0x8244A604;
	sub_82449F98(ctx, base);
	// 8244A604: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A608: 38A000EF  li r5, 0xef
	ctx.r[5].s64 = 239;
	// 8244A60C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 8244A610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A614: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8244A618: 4BFFF981  bl 0x82449f98
	ctx.lr = 0x8244A61C;
	sub_82449F98(ctx, base);
	// 8244A61C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A620: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8244A624: 38BF004C  addi r5, r31, 0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + 76;
	// 8244A628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A62C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8244A630: 4BFFFCA1  bl 0x8244a2d0
	ctx.lr = 0x8244A634;
	sub_8244A2D0(ctx, base);
	// 8244A634: 38BF005C  addi r5, r31, 0x5c
	ctx.r[5].s64 = ctx.r[31].s64 + 92;
	// 8244A638: 809F0048  lwz r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8244A63C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A640: 4BFFFD21  bl 0x8244a360
	ctx.lr = 0x8244A644;
	sub_8244A360(ctx, base);
	// 8244A644: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8244A648: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244A64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244A650: 480EAABC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A658 size=88
    let mut pc: u32 = 0x8244A658;
    'dispatch: loop {
        match pc {
            0x8244A658 => {
    //   block [0x8244A658..0x8244A6B0)
	// 8244A658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244A664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244A668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A66C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244A670: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 8244A674: 809F0090  lwz r4, 0x90(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8244A678: 4BFFAD11  bl 0x82445388
	ctx.lr = 0x8244A67C;
	sub_82445388(ctx, base);
	// 8244A67C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244A680: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8244A684: 419A0014  beq cr6, 0x8244a698
	if ctx.cr[6].eq {
	pc = 0x8244A698; continue 'dispatch;
	}
	// 8244A688: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244A68C: 4BFFFE75  bl 0x8244a500
	ctx.lr = 0x8244A690;
	sub_8244A500(ctx, base);
	// 8244A690: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A694: 4BFFAD8D  bl 0x82445420
	ctx.lr = 0x8244A698;
	sub_82445420(ctx, base);
	// 8244A698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244A69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A6A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244A6A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244A6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A6B0 size=140
    let mut pc: u32 = 0x8244A6B0;
    'dispatch: loop {
        match pc {
            0x8244A6B0 => {
    //   block [0x8244A6B0..0x8244A73C)
	// 8244A6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A6B4: 480EAA05  bl 0x825350b8
	ctx.lr = 0x8244A6B8;
	sub_82535080(ctx, base);
	// 8244A6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A6BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244A6C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8244A6C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244A6C8: 817F0D2C  lwz r11, 0xd2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3372 as u32) ) } as u64;
	// 8244A6CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244A6D0: 419A0010  beq cr6, 0x8244a6e0
	if ctx.cr[6].eq {
	pc = 0x8244A6E0; continue 'dispatch;
	}
	// 8244A6D4: 807F0D30  lwz r3, 0xd30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3376 as u32) ) } as u64;
	// 8244A6D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244A6DC: 4E800421  bctrl
	ctx.lr = 0x8244A6E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8244A6E0: 3BDF0078  addi r30, r31, 0x78
	ctx.r[30].s64 = ctx.r[31].s64 + 120;
	// 8244A6E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244A6E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244A6EC: 419A0010  beq cr6, 0x8244a6fc
	if ctx.cr[6].eq {
	pc = 0x8244A6FC; continue 'dispatch;
	}
	// 8244A6F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244A6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244A6F8: 480EAA10  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8244A6FC: 2F1D0800  cmpwi cr6, r29, 0x800
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2048, &mut ctx.xer);
	// 8244A700: 41980008  blt cr6, 0x8244a708
	if ctx.cr[6].lt {
	pc = 0x8244A708; continue 'dispatch;
	}
	// 8244A704: 3BA00800  li r29, 0x800
	ctx.r[29].s64 = 2048;
	// 8244A708: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8244A70C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8244A710: 387E0094  addi r3, r30, 0x94
	ctx.r[3].s64 = ctx.r[30].s64 + 148;
	// 8244A714: 4BFFF725  bl 0x82449e38
	ctx.lr = 0x8244A718;
	sub_82449E38(ctx, base);
	// 8244A718: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A71C: 93BE0090  stw r29, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[29].u32 ) };
	// 8244A720: 4BFFFF39  bl 0x8244a658
	ctx.lr = 0x8244A724;
	sub_8244A658(ctx, base);
	// 8244A724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A728: 4BFFF959  bl 0x8244a080
	ctx.lr = 0x8244A72C;
	sub_8244A080(ctx, base);
	// 8244A72C: 4BFFF9F5  bl 0x8244a120
	ctx.lr = 0x8244A730;
	sub_8244A120(ctx, base);
	// 8244A730: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244A734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244A738: 480EA9D0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A740 size=92
    let mut pc: u32 = 0x8244A740;
    'dispatch: loop {
        match pc {
            0x8244A740 => {
    //   block [0x8244A740..0x8244A79C)
	// 8244A740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244A748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244A74C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244A750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244A758: 4BFFF7D9  bl 0x82449f30
	ctx.lr = 0x8244A75C;
	sub_82449F30(ctx, base);
	// 8244A75C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8244A760: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8244A764: 419A0020  beq cr6, 0x8244a784
	if ctx.cr[6].eq {
	pc = 0x8244A784; continue 'dispatch;
	}
	// 8244A768: 3BDF0078  addi r30, r31, 0x78
	ctx.r[30].s64 = ctx.r[31].s64 + 120;
	// 8244A76C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A770: 4BFFF7B9  bl 0x82449f28
	ctx.lr = 0x8244A774;
	sub_82449F28(ctx, base);
	// 8244A774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A778: 4BFFFEE1  bl 0x8244a658
	ctx.lr = 0x8244A77C;
	sub_8244A658(ctx, base);
	// 8244A77C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A780: 4BFFF901  bl 0x8244a080
	ctx.lr = 0x8244A784;
	sub_8244A080(ctx, base);
	// 8244A784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244A788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244A78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244A790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244A794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244A798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244A7A0 size=160
    let mut pc: u32 = 0x8244A7A0;
    'dispatch: loop {
        match pc {
            0x8244A7A0 => {
    //   block [0x8244A7A0..0x8244A840)
	// 8244A7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A7A4: 480EA915  bl 0x825350b8
	ctx.lr = 0x8244A7A8;
	sub_82535080(ctx, base);
	// 8244A7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A7AC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8244A7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244A7B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8244A7B8: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8244A7BC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244A7C0: 419A0010  beq cr6, 0x8244a7d0
	if ctx.cr[6].eq {
	pc = 0x8244A7D0; continue 'dispatch;
	}
	// 8244A7C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244A7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244A7CC: 480EA93C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8244A7D0: 3905FFFA  addi r8, r5, -6
	ctx.r[8].s64 = ctx.r[5].s64 + -6;
	// 8244A7D4: 38E60006  addi r7, r6, 6
	ctx.r[7].s64 = ctx.r[6].s64 + 6;
	// 8244A7D8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8244A7DC: 4BFFF915  bl 0x8244a0f0
	ctx.lr = 0x8244A7E0;
	sub_8244A0F0(ctx, base);
	// 8244A7E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A7E4: 409A001C  bne cr6, 0x8244a800
	if !ctx.cr[6].eq {
	pc = 0x8244A800; continue 'dispatch;
	}
	// 8244A7E8: 3908FFFE  addi r8, r8, -2
	ctx.r[8].s64 = ctx.r[8].s64 + -2;
	// 8244A7EC: 38E70002  addi r7, r7, 2
	ctx.r[7].s64 = ctx.r[7].s64 + 2;
	// 8244A7F0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8244A7F4: 4BFFF8FD  bl 0x8244a0f0
	ctx.lr = 0x8244A7F8;
	sub_8244A0F0(ctx, base);
	// 8244A7F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A7FC: 419AFFC8  beq cr6, 0x8244a7c4
	if ctx.cr[6].eq {
	pc = 0x8244A7C4; continue 'dispatch;
	}
	// 8244A800: 3BE8FFF4  addi r31, r8, -0xc
	ctx.r[31].s64 = ctx.r[8].s64 + -12;
	// 8244A804: 3BC7000C  addi r30, r7, 0xc
	ctx.r[30].s64 = ctx.r[7].s64 + 12;
	// 8244A808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244A80C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244A810: 4BFFF6A9  bl 0x82449eb8
	ctx.lr = 0x8244A814;
	sub_82449EB8(ctx, base);
	// 8244A814: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244A818: 419AFFAC  beq cr6, 0x8244a7c4
	if ctx.cr[6].eq {
	pc = 0x8244A7C4; continue 'dispatch;
	}
	// 8244A81C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8244A820: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244A824: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8244A828: 4BFFFE89  bl 0x8244a6b0
	ctx.lr = 0x8244A82C;
	sub_8244A6B0(ctx, base);
	// 8244A82C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A830: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244A834: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244A838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244A83C: 480EA8CC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A840 size=20
    let mut pc: u32 = 0x8244A840;
    'dispatch: loop {
        match pc {
            0x8244A840 => {
    //   block [0x8244A840..0x8244A854)
	// 8244A840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244A844: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244A848: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8244A84C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8244A850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8244A858 size=92
    let mut pc: u32 = 0x8244A858;
    'dispatch: loop {
        match pc {
            0x8244A858 => {
    //   block [0x8244A858..0x8244A8B4)
	// 8244A858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A85C: 480EA861  bl 0x825350bc
	ctx.lr = 0x8244A860;
	sub_82535080(ctx, base);
	// 8244A860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244A864: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8244A868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244A86C: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 8244A870: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8244A874: 555E0038  rlwinm r30, r10, 0, 0, 0x1c
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8244A878: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8244A87C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244A880: 7FAB2A14  add r29, r11, r5
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8244A884: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8244A888: 480EA949  bl 0x825351d0
	ctx.lr = 0x8244A88C;
	sub_825351D0(ctx, base);
	// 8244A88C: 7FAB2670  srawi r11, r29, 4
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 4) as i64;
	// 8244A890: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8244A894: 7D4B0194  addze r10, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8244A898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244A89C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8244A8A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244A8A4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8244A8A8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8244A8AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244A8B0: 480EA85C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A8B8 size=32
    let mut pc: u32 = 0x8244A8B8;
    'dispatch: loop {
        match pc {
            0x8244A8B8 => {
    //   block [0x8244A8B8..0x8244A8D8)
	// 8244A8B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244A8BC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244A8C0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8244A8C4: 409A0014  bne cr6, 0x8244a8d8
	if !ctx.cr[6].eq {
		sub_8244A8D8(ctx, base);
		return;
	}
	// 8244A8C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8244A8CC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244A8D0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244A8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A8D8 size=100
    let mut pc: u32 = 0x8244A8D8;
    'dispatch: loop {
        match pc {
            0x8244A8D8 => {
    //   block [0x8244A8D8..0x8244A93C)
	// 8244A8D8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244A8DC: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244A8E0: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244A8E4: E8E40000  ld r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8244A8E8: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 8244A8EC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8244A8F0: F8EA0000  std r7, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8244A8F4: E9040008  ld r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8244A8F8: F90A0008  std r8, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 8244A8FC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244A900: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8244A904: 4198000C  blt cr6, 0x8244a910
	if ctx.cr[6].lt {
	pc = 0x8244A910; continue 'dispatch;
	}
	// 8244A908: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8244A90C: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 8244A910: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244A914: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8244A918: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8244A91C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8244A920: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244A924: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8244A928: 40980008  bge cr6, 0x8244a930
	if !ctx.cr[6].lt {
	pc = 0x8244A930; continue 'dispatch;
	}
	// 8244A92C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244A930: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244A934: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244A938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A940 size=184
    let mut pc: u32 = 0x8244A940;
    'dispatch: loop {
        match pc {
            0x8244A940 => {
    //   block [0x8244A940..0x8244A9F8)
	// 8244A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244A944: 480EA775  bl 0x825350b8
	ctx.lr = 0x8244A948;
	sub_82535080(ctx, base);
	// 8244A948: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244A94C: 7FE53214  add r31, r5, r6
	ctx.r[31].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 8244A950: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244A954: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244A958: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244A95C: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244A960: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8244A964: 4099008C  ble cr6, 0x8244a9f0
	if !ctx.cr[6].gt {
	pc = 0x8244A9F0; continue 'dispatch;
	}
	// 8244A968: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244A96C: 55092036  slwi r9, r8, 4
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244A970: 54FD2036  slwi r29, r7, 4
	ctx.r[29].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8244A974: 7D69F214  add r11, r9, r30
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 8244A978: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244A97C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244A980: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244A984: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8244A988: 41990010  bgt cr6, 0x8244a998
	if ctx.cr[6].gt {
	pc = 0x8244A998; continue 'dispatch;
	}
	// 8244A98C: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8244A990: 4199002C  bgt cr6, 0x8244a9bc
	if ctx.cr[6].gt {
	pc = 0x8244A9BC; continue 'dispatch;
	}
	// 8244A994: 48000020  b 0x8244a9b4
	pc = 0x8244A9B4; continue 'dispatch;
	// 8244A998: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8244A99C: 4199000C  bgt cr6, 0x8244a9a8
	if ctx.cr[6].gt {
	pc = 0x8244A9A8; continue 'dispatch;
	}
	// 8244A9A0: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8244A9A4: 41980050  blt cr6, 0x8244a9f4
	if ctx.cr[6].lt {
	pc = 0x8244A9F4; continue 'dispatch;
	}
	// 8244A9A8: 7F052040  cmplw cr6, r5, r4
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8244A9AC: 41990010  bgt cr6, 0x8244a9bc
	if ctx.cr[6].gt {
	pc = 0x8244A9BC; continue 'dispatch;
	}
	// 8244A9B0: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 8244A9B4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8244A9B8: 4198003C  blt cr6, 0x8244a9f4
	if ctx.cr[6].lt {
	pc = 0x8244A9F4; continue 'dispatch;
	}
	// 8244A9BC: 39680001  addi r11, r8, 1
	ctx.r[11].s64 = ctx.r[8].s64 + 1;
	// 8244A9C0: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8244A9C4: 40980010  bge cr6, 0x8244a9d4
	if !ctx.cr[6].lt {
	pc = 0x8244A9D4; continue 'dispatch;
	}
	// 8244A9C8: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244A9CC: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 8244A9D0: 48000014  b 0x8244a9e4
	pc = 0x8244A9E4; continue 'dispatch;
	// 8244A9D4: 7D7D4850  subf r11, r29, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[29].s64;
	// 8244A9D8: 7D474050  subf r10, r7, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 8244A9DC: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 8244A9E0: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 8244A9E4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8244A9E8: 7F03E000  cmpw cr6, r3, r28
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8244A9EC: 4198FF88  blt cr6, 0x8244a974
	if ctx.cr[6].lt {
	pc = 0x8244A974; continue 'dispatch;
	}
	// 8244A9F0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244A9F4: 480EA714  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244A9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244A9F8 size=40
    let mut pc: u32 = 0x8244A9F8;
    'dispatch: loop {
        match pc {
            0x8244A9F8 => {
    //   block [0x8244A9F8..0x8244AA20)
	// 8244A9F8: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8244A9FC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8244AA00: 814B13D0  lwz r10, 0x13d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5072 as u32) ) } as u64;
	// 8244AA04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8244AA08: 419A0018  beq cr6, 0x8244aa20
	if ctx.cr[6].eq {
		sub_8244AA20(ctx, base);
		return;
	}
	// 8244AA0C: 814B13D8  lwz r10, 0x13d8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5080 as u32) ) } as u64;
	// 8244AA10: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244AA14: 816B13D4  lwz r11, 0x13d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5076 as u32) ) } as u64;
	// 8244AA18: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8244AA1C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AA20 size=8
    let mut pc: u32 = 0x8244AA20;
    'dispatch: loop {
        match pc {
            0x8244AA20 => {
    //   block [0x8244AA20..0x8244AA28)
	// 8244AA20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244AA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AA28 size=104
    let mut pc: u32 = 0x8244AA28;
    'dispatch: loop {
        match pc {
            0x8244AA28 => {
    //   block [0x8244AA28..0x8244AA90)
	// 8244AA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AA2C: 480EA691  bl 0x825350bc
	ctx.lr = 0x8244AA30;
	sub_82535080(ctx, base);
	// 8244AA30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AA34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8244AA38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244AA3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244AA40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8244AA44: 419A0040  beq cr6, 0x8244aa84
	if ctx.cr[6].eq {
	pc = 0x8244AA84; continue 'dispatch;
	}
	// 8244AA48: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8244AA4C: 40990038  ble cr6, 0x8244aa84
	if !ctx.cr[6].gt {
	pc = 0x8244AA84; continue 'dispatch;
	}
	// 8244AA50: 4BFFCDF1  bl 0x82447840
	ctx.lr = 0x8244AA54;
	sub_82447840(ctx, base);
	// 8244AA54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244AA58: 419A001C  beq cr6, 0x8244aa74
	if ctx.cr[6].eq {
	pc = 0x8244AA74; continue 'dispatch;
	}
	// 8244AA5C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8244AA60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244AA64: 60840165  ori r4, r4, 0x165
	ctx.r[4].u64 = ctx.r[4].u64 | 357;
	// 8244AA68: 4BFFCEA1  bl 0x82447908
	ctx.lr = 0x8244AA6C;
	sub_82447908(ctx, base);
	// 8244AA6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244AA70: 480EA69C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8244AA74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8244AA78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244AA7C: 387E1444  addi r3, r30, 0x1444
	ctx.r[3].s64 = ctx.r[30].s64 + 5188;
	// 8244AA80: 4BFFFDD9  bl 0x8244a858
	ctx.lr = 0x8244AA84;
	sub_8244A858(ctx, base);
	// 8244AA84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244AA88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244AA8C: 480EA680  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AA90 size=148
    let mut pc: u32 = 0x8244AA90;
    'dispatch: loop {
        match pc {
            0x8244AA90 => {
    //   block [0x8244AA90..0x8244AB24)
	// 8244AA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244AA98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244AA9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AAA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244AAA4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8244AAA8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8244AAAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244AAB0: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244AAB4: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8244AAB8: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 8244AABC: 41980050  blt cr6, 0x8244ab0c
	if ctx.cr[6].lt {
	pc = 0x8244AB0C; continue 'dispatch;
	}
	// 8244AAC0: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 8244AAC4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8244AAC8: 386B13D0  addi r3, r11, 0x13d0
	ctx.r[3].s64 = ctx.r[11].s64 + 5072;
	// 8244AACC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244AAD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244AAD4: 419A0038  beq cr6, 0x8244ab0c
	if ctx.cr[6].eq {
	pc = 0x8244AB0C; continue 'dispatch;
	}
	// 8244AAD8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8244AADC: 4BFFFDDD  bl 0x8244a8b8
	ctx.lr = 0x8244AAE0;
	sub_8244A8B8(ctx, base);
	// 8244AAE0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8244AAE4: 409A0028  bne cr6, 0x8244ab0c
	if !ctx.cr[6].eq {
	pc = 0x8244AB0C; continue 'dispatch;
	}
	// 8244AAE8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8244AAEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244AAF0: 60840421  ori r4, r4, 0x421
	ctx.r[4].u64 = ctx.r[4].u64 | 1057;
	// 8244AAF4: 4BFFCE15  bl 0x82447908
	ctx.lr = 0x8244AAF8;
	sub_82447908(ctx, base);
	// 8244AAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AB04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244AB08: 4E800020  blr
	return;
	// 8244AB0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244AB10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AB1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244AB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AB28 size=128
    let mut pc: u32 = 0x8244AB28;
    'dispatch: loop {
        match pc {
            0x8244AB28 => {
    //   block [0x8244AB28..0x8244ABA8)
	// 8244AB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AB2C: 480EA591  bl 0x825350bc
	ctx.lr = 0x8244AB30;
	sub_82535080(ctx, base);
	// 8244AB30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AB34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244AB38: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8244AB3C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8244AB40: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244AB44: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8244AB48: 419A0058  beq cr6, 0x8244aba0
	if ctx.cr[6].eq {
	pc = 0x8244ABA0; continue 'dispatch;
	}
	// 8244AB4C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8244AB50: 4BFFFDF1  bl 0x8244a940
	ctx.lr = 0x8244AB54;
	sub_8244A940(ctx, base);
	// 8244AB54: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8244AB58: 419A0048  beq cr6, 0x8244aba0
	if ctx.cr[6].eq {
	pc = 0x8244ABA0; continue 'dispatch;
	}
	// 8244AB5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244AB60: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244AB64: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8244AB68: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8244AB6C: 4198000C  blt cr6, 0x8244ab78
	if ctx.cr[6].lt {
	pc = 0x8244AB78; continue 'dispatch;
	}
	// 8244AB70: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8244AB74: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8244AB78: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244AB7C: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244AB80: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8244AB84: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8244AB88: 7D23E850  subf r9, r3, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 8244AB8C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8244AB90: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8244AB94: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8244AB98: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8244AB9C: F97E0008  std r11, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8244ABA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244ABA4: 480EA568  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244ABA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244ABA8 size=108
    let mut pc: u32 = 0x8244ABA8;
    'dispatch: loop {
        match pc {
            0x8244ABA8 => {
    //   block [0x8244ABA8..0x8244AC14)
	// 8244ABA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244ABAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244ABB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244ABB4: 1D640074  mulli r11, r4, 0x74
	ctx.r[11].s64 = ctx.r[4].s64 * 116;
	// 8244ABB8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8244ABBC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8244ABC0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8244ABC4: 396B13A8  addi r11, r11, 0x13a8
	ctx.r[11].s64 = ctx.r[11].s64 + 5032;
	// 8244ABC8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8244ABCC: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8244ABD0: F9250000  std r9, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8244ABD4: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244ABD8: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244ABDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244ABE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244ABE4: 419A001C  beq cr6, 0x8244ac00
	if ctx.cr[6].eq {
	pc = 0x8244AC00; continue 'dispatch;
	}
	// 8244ABE8: 7D673214  add r11, r7, r6
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 8244ABEC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8244ABF0: 41980008  blt cr6, 0x8244abf8
	if ctx.cr[6].lt {
	pc = 0x8244ABF8; continue 'dispatch;
	}
	// 8244ABF4: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 8244ABF8: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8244ABFC: 4BFFFF2D  bl 0x8244ab28
	ctx.lr = 0x8244AC00;
	sub_8244AB28(ctx, base);
	// 8244AC00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244AC04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AC18 size=20
    let mut pc: u32 = 0x8244AC18;
    'dispatch: loop {
        match pc {
            0x8244AC18 => {
    //   block [0x8244AC18..0x8244AC2C)
	// 8244AC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244AC1C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244AC20: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8244AC24: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244AC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AC30 size=12
    let mut pc: u32 = 0x8244AC30;
    'dispatch: loop {
        match pc {
            0x8244AC30 => {
    //   block [0x8244AC30..0x8244AC3C)
	// 8244AC30: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8244AC34: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8244AC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AC40 size=24
    let mut pc: u32 = 0x8244AC40;
    'dispatch: loop {
        match pc {
            0x8244AC40 => {
    //   block [0x8244AC40..0x8244AC58)
	// 8244AC40: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 8244AC44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8244AC48: 906BDCA4  stw r3, -0x235c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-9052 as u32), ctx.r[3].u32 ) };
	// 8244AC4C: 409A000C  bne cr6, 0x8244ac58
	if !ctx.cr[6].eq {
		sub_8244AC58(ctx, base);
		return;
	}
	// 8244AC50: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244AC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AC58 size=24
    let mut pc: u32 = 0x8244AC58;
    'dispatch: loop {
        match pc {
            0x8244AC58 => {
    //   block [0x8244AC58..0x8244AC70)
	// 8244AC58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244AC5C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8244AC60: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8244AC64: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8244AC68: 7C6B00D0  neg r3, r11
	ctx.r[3].s64 = -ctx.r[11].s64;
	// 8244AC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AC70 size=40
    let mut pc: u32 = 0x8244AC70;
    'dispatch: loop {
        match pc {
            0x8244AC70 => {
    //   block [0x8244AC70..0x8244AC98)
	// 8244AC70: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8244AC74: 816B033C  lwz r11, 0x33c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(828 as u32) ) } as u64;
	// 8244AC78: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8244AC7C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244AC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244AC84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244AC88: 40990020  ble cr6, 0x8244aca8
	if !ctx.cr[6].gt {
		sub_8244AC98(ctx, base);
		return;
	}
	// 8244AC8C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244AC90: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8244AC94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AC98 size=24
    let mut pc: u32 = 0x8244AC98;
    'dispatch: loop {
        match pc {
            0x8244AC98 => {
    //   block [0x8244AC98..0x8244ACB0)
	// 8244AC98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8244AC9C: 38630100  addi r3, r3, 0x100
	ctx.r[3].s64 = ctx.r[3].s64 + 256;
	// 8244ACA0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8244ACA4: 4198FFE8  blt cr6, 0x8244ac8c
	if ctx.cr[6].lt {
		sub_8244AC70(ctx, base);
		return;
	}
	// 8244ACA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244ACAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244ACB0 size=20
    let mut pc: u32 = 0x8244ACB0;
    'dispatch: loop {
        match pc {
            0x8244ACB0 => {
    //   block [0x8244ACB0..0x8244ACC4)
	// 8244ACB0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8244ACB4: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8244ACB8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244ACBC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8244ACC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244ACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244ACC8 size=40
    let mut pc: u32 = 0x8244ACC8;
    'dispatch: loop {
        match pc {
            0x8244ACC8 => {
    //   block [0x8244ACC8..0x8244ACF0)
	// 8244ACC8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8244ACCC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244ACD0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8244ACD4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244ACD8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8244ACDC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8244ACE0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8244ACE4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8244ACE8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8244ACEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244ACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244ACF0 size=40
    let mut pc: u32 = 0x8244ACF0;
    'dispatch: loop {
        match pc {
            0x8244ACF0 => {
    //   block [0x8244ACF0..0x8244AD18)
	// 8244ACF0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8244ACF4: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8244ACF8: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8244ACFC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8244AD00: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8244AD04: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8244AD08: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8244AD0C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8244AD10: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8244AD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244AD18 size=4
    let mut pc: u32 = 0x8244AD18;
    'dispatch: loop {
        match pc {
            0x8244AD18 => {
    //   block [0x8244AD18..0x8244AD1C)
	// 8244AD18: 4BFFFF00  b 0x8244ac18
	sub_8244AC18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AD20 size=72
    let mut pc: u32 = 0x8244AD20;
    'dispatch: loop {
        match pc {
            0x8244AD20 => {
    //   block [0x8244AD20..0x8244AD68)
	// 8244AD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244AD28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244AD2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AD30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8244AD34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8244AD38: 409A0010  bne cr6, 0x8244ad48
	if !ctx.cr[6].eq {
	pc = 0x8244AD48; continue 'dispatch;
	}
	// 8244AD3C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8244AD40: 806B033C  lwz r3, 0x33c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(828 as u32) ) } as u64;
	// 8244AD44: 48000008  b 0x8244ad4c
	pc = 0x8244AD4C; continue 'dispatch;
	// 8244AD48: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8244AD4C: 4800516D  bl 0x8244feb8
	ctx.lr = 0x8244AD50;
	sub_8244FEB8(ctx, base);
	// 8244AD50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244AD54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AD60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244AD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AD68 size=112
    let mut pc: u32 = 0x8244AD68;
    'dispatch: loop {
        match pc {
            0x8244AD68 => {
    //   block [0x8244AD68..0x8244ADD8)
	// 8244AD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244AD70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AD74: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244AD78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8244AD7C: 409A0010  bne cr6, 0x8244ad8c
	if !ctx.cr[6].eq {
	pc = 0x8244AD8C; continue 'dispatch;
	}
	// 8244AD80: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8244AD84: 806B033C  lwz r3, 0x33c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(828 as u32) ) } as u64;
	// 8244AD88: 48000038  b 0x8244adc0
	pc = 0x8244ADC0; continue 'dispatch;
	// 8244AD8C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8244AD90: 4BFFFEB1  bl 0x8244ac40
	ctx.lr = 0x8244AD94;
	sub_8244AC40(ctx, base);
	// 8244AD94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244AD98: 419A0024  beq cr6, 0x8244adbc
	if ctx.cr[6].eq {
	pc = 0x8244ADBC; continue 'dispatch;
	}
	// 8244AD9C: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8244ADA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244ADA4: 60840101  ori r4, r4, 0x101
	ctx.r[4].u64 = ctx.r[4].u64 | 257;
	// 8244ADA8: 4BFFFF79  bl 0x8244ad20
	ctx.lr = 0x8244ADAC;
	sub_8244AD20(ctx, base);
	// 8244ADAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244ADB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244ADB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244ADB8: 4E800020  blr
	return;
	// 8244ADBC: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 8244ADC0: 4BFFFE71  bl 0x8244ac30
	ctx.lr = 0x8244ADC4;
	sub_8244AC30(ctx, base);
	// 8244ADC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244ADC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244ADCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244ADD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244ADD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244ADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244ADD8 size=164
    let mut pc: u32 = 0x8244ADD8;
    'dispatch: loop {
        match pc {
            0x8244ADD8 => {
    //   block [0x8244ADD8..0x8244AE7C)
	// 8244ADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244ADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244ADE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244ADE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244ADE8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8244ADEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8244ADF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244ADF4: 4BFFEDFD  bl 0x82449bf0
	ctx.lr = 0x8244ADF8;
	sub_82449BF0(ctx, base);
	// 8244ADF8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8244ADFC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8244AE00: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244AE04: 4BFFFE15  bl 0x8244ac18
	ctx.lr = 0x8244AE08;
	sub_8244AC18(ctx, base);
	// 8244AE08: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8244AE0C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8244AE10: 4BFFFEA1  bl 0x8244acb0
	ctx.lr = 0x8244AE14;
	sub_8244ACB0(ctx, base);
	// 8244AE14: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8244AE18: 4BFFFEB1  bl 0x8244acc8
	ctx.lr = 0x8244AE1C;
	sub_8244ACC8(ctx, base);
	// 8244AE1C: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 8244AE20: 4BFFFEA9  bl 0x8244acc8
	ctx.lr = 0x8244AE24;
	sub_8244ACC8(ctx, base);
	// 8244AE24: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8244AE28: 4BFFFEA1  bl 0x8244acc8
	ctx.lr = 0x8244AE2C;
	sub_8244ACC8(ctx, base);
	// 8244AE2C: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 8244AE30: 4BFFFE99  bl 0x8244acc8
	ctx.lr = 0x8244AE34;
	sub_8244ACC8(ctx, base);
	// 8244AE34: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 8244AE38: 4BFFFEB9  bl 0x8244acf0
	ctx.lr = 0x8244AE3C;
	sub_8244ACF0(ctx, base);
	// 8244AE3C: 3D408245  lis r10, -0x7dbb
	ctx.r[10].s64 = -2109407232;
	// 8244AE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244AE44: 394AC220  addi r10, r10, -0x3de0
	ctx.r[10].s64 = ctx.r[10].s64 + -15840;
	// 8244AE48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244AE4C: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 8244AE50: 915F00D4  stw r10, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 8244AE54: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 8244AE58: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 8244AE5C: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8244AE60: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8244AE64: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 8244AE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AE74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244AE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AE80 size=84
    let mut pc: u32 = 0x8244AE80;
    'dispatch: loop {
        match pc {
            0x8244AE80 => {
    //   block [0x8244AE80..0x8244AED4)
	// 8244AE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244AE88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AE8C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8244AE90: 4BFFFDB1  bl 0x8244ac40
	ctx.lr = 0x8244AE94;
	sub_8244AC40(ctx, base);
	// 8244AE94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244AE98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244AE9C: 419A0020  beq cr6, 0x8244aebc
	if ctx.cr[6].eq {
	pc = 0x8244AEBC; continue 'dispatch;
	}
	// 8244AEA0: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8244AEA4: 60840103  ori r4, r4, 0x103
	ctx.r[4].u64 = ctx.r[4].u64 | 259;
	// 8244AEA8: 4BFFFE79  bl 0x8244ad20
	ctx.lr = 0x8244AEAC;
	sub_8244AD20(ctx, base);
	// 8244AEAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AEB8: 4E800020  blr
	return;
	// 8244AEBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8244AEC0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244AEC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AED8 size=144
    let mut pc: u32 = 0x8244AED8;
    'dispatch: loop {
        match pc {
            0x8244AED8 => {
    //   block [0x8244AED8..0x8244AF68)
	// 8244AED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244AEE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244AEE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244AEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AEEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244AEF0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8244AEF4: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8244AEF8: 3FE08313  lis r31, -0x7ced
	ctx.r[31].s64 = -2095906816;
	// 8244AEFC: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244AF00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8244AF04: 396B0110  addi r11, r11, 0x110
	ctx.r[11].s64 = ctx.r[11].s64 + 272;
	// 8244AF08: 5565F0BE  srwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8244AF0C: 907F033C  stw r3, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[3].u32 ) };
	// 8244AF10: 4BFFECE1  bl 0x82449bf0
	ctx.lr = 0x8244AF14;
	sub_82449BF0(ctx, base);
	// 8244AF14: 807F033C  lwz r3, 0x33c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 8244AF18: 4BFFFE01  bl 0x8244ad18
	ctx.lr = 0x8244AF1C;
	sub_8244AD18(ctx, base);
	// 8244AF1C: 817F033C  lwz r11, 0x33c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 8244AF20: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8244AF24: 93CB000C  stw r30, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8244AF28: 817F033C  lwz r11, 0x33c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 8244AF2C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 8244AF30: 4099001C  ble cr6, 0x8244af4c
	if !ctx.cr[6].gt {
	pc = 0x8244AF4C; continue 'dispatch;
	}
	// 8244AF34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8244AF38: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8244AF3C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244AF40: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 8244AF44: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8244AF48: 409AFFF0  bne cr6, 0x8244af38
	if !ctx.cr[6].eq {
	pc = 0x8244AF38; continue 'dispatch;
	}
	// 8244AF4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244AF50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244AF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AF5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244AF60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244AF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AF68 size=112
    let mut pc: u32 = 0x8244AF68;
    'dispatch: loop {
        match pc {
            0x8244AF68 => {
    //   block [0x8244AF68..0x8244AFD8)
	// 8244AF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244AF70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244AF74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244AF78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AF7C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8244AF80: 816B033C  lwz r11, 0x33c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(828 as u32) ) } as u64;
	// 8244AF84: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8244AF88: 83CB000C  lwz r30, 0xc(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244AF8C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8244AF90: 40990028  ble cr6, 0x8244afb8
	if !ctx.cr[6].gt {
	pc = 0x8244AFB8; continue 'dispatch;
	}
	// 8244AF94: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244AF98: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8244AF9C: 419A000C  beq cr6, 0x8244afa8
	if ctx.cr[6].eq {
	pc = 0x8244AFA8; continue 'dispatch;
	}
	// 8244AFA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244AFA4: 4BFFFEDD  bl 0x8244ae80
	ctx.lr = 0x8244AFA8;
	sub_8244AE80(ctx, base);
	// 8244AFA8: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8244AFAC: 3BFF0100  addi r31, r31, 0x100
	ctx.r[31].s64 = ctx.r[31].s64 + 256;
	// 8244AFB0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8244AFB4: 409AFFE0  bne cr6, 0x8244af94
	if !ctx.cr[6].eq {
	pc = 0x8244AF94; continue 'dispatch;
	}
	// 8244AFB8: 4BFE3F99  bl 0x8242ef50
	ctx.lr = 0x8244AFBC;
	sub_8242EF50(ctx, base);
	// 8244AFBC: 4BFE3F95  bl 0x8242ef50
	ctx.lr = 0x8244AFC0;
	sub_8242EF50(ctx, base);
	// 8244AFC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244AFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AFCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244AFD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244AFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244AFD8 size=60
    let mut pc: u32 = 0x8244AFD8;
    'dispatch: loop {
        match pc {
            0x8244AFD8 => {
    //   block [0x8244AFD8..0x8244B014)
	// 8244AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244AFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244AFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244AFE4: 4BFFFC8D  bl 0x8244ac70
	ctx.lr = 0x8244AFE8;
	sub_8244AC70(ctx, base);
	// 8244AFE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8244AFEC: 409A0014  bne cr6, 0x8244b000
	if !ctx.cr[6].eq {
	pc = 0x8244B000; continue 'dispatch;
	}
	// 8244AFF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244AFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244AFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244AFFC: 4E800020  blr
	return;
	// 8244B000: 4BFFFDD9  bl 0x8244add8
	ctx.lr = 0x8244B004;
	sub_8244ADD8(ctx, base);
	// 8244B004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244B008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244B00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244B010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244B018 size=68
    let mut pc: u32 = 0x8244B018;
    'dispatch: loop {
        match pc {
            0x8244B018 => {
    //   block [0x8244B018..0x8244B05C)
	// 8244B018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244B01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244B020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244B024: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8244B028: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 8244B02C: 396B6510  addi r11, r11, 0x6510
	ctx.r[11].s64 = ctx.r[11].s64 + 25872;
	// 8244B030: 916ADCA8  stw r11, -0x2358(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9048 as u32), ctx.r[11].u32 ) };
	// 8244B034: 4BFFFEA5  bl 0x8244aed8
	ctx.lr = 0x8244B038;
	sub_8244AED8(ctx, base);
	// 8244B038: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244B03C: 409A0010  bne cr6, 0x8244b04c
	if !ctx.cr[6].eq {
	pc = 0x8244B04C; continue 'dispatch;
	}
	// 8244B040: 4BFE3F11  bl 0x8242ef50
	ctx.lr = 0x8244B044;
	sub_8242EF50(ctx, base);
	// 8244B044: 4BFE3F0D  bl 0x8242ef50
	ctx.lr = 0x8244B048;
	sub_8242EF50(ctx, base);
	// 8244B048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244B04C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244B050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244B054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244B058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244B060 size=52
    let mut pc: u32 = 0x8244B060;
    'dispatch: loop {
        match pc {
            0x8244B060 => {
    //   block [0x8244B060..0x8244B094)
	// 8244B060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244B064: 480EA059  bl 0x825350bc
	ctx.lr = 0x8244B068;
	sub_82535080(ctx, base);
	// 8244B068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244B06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244B070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244B074: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244B078: 4BFFFBC9  bl 0x8244ac40
	ctx.lr = 0x8244B07C;
	sub_8244AC40(ctx, base);
	// 8244B07C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244B080: 409A000C  bne cr6, 0x8244b08c
	if !ctx.cr[6].eq {
	pc = 0x8244B08C; continue 'dispatch;
	}
	// 8244B084: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 8244B088: 93BF00E8  stw r29, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[29].u32 ) };
	// 8244B08C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244B090: 480EA07C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244B098 size=52
    let mut pc: u32 = 0x8244B098;
    'dispatch: loop {
        match pc {
            0x8244B098 => {
    //   block [0x8244B098..0x8244B0CC)
	// 8244B098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244B09C: 480EA021  bl 0x825350bc
	ctx.lr = 0x8244B0A0;
	sub_82535080(ctx, base);
	// 8244B0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244B0A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244B0A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244B0AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244B0B0: 4BFFFB91  bl 0x8244ac40
	ctx.lr = 0x8244B0B4;
	sub_8244AC40(ctx, base);
	// 8244B0B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244B0B8: 409A000C  bne cr6, 0x8244b0c4
	if !ctx.cr[6].eq {
	pc = 0x8244B0C4; continue 'dispatch;
	}
	// 8244B0BC: 93DF00EC  stw r30, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u32 ) };
	// 8244B0C0: 93BF00F0  stw r29, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[29].u32 ) };
	// 8244B0C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244B0C8: 480EA044  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244B0D0 size=52
    let mut pc: u32 = 0x8244B0D0;
    'dispatch: loop {
        match pc {
            0x8244B0D0 => {
    //   block [0x8244B0D0..0x8244B104)
	// 8244B0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244B0D4: 480E9FE9  bl 0x825350bc
	ctx.lr = 0x8244B0D8;
	sub_82535080(ctx, base);
	// 8244B0D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244B0DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244B0E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244B0E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244B0E8: 4BFFFB59  bl 0x8244ac40
	ctx.lr = 0x8244B0EC;
	sub_8244AC40(ctx, base);
	// 8244B0EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244B0F0: 409A000C  bne cr6, 0x8244b0fc
	if !ctx.cr[6].eq {
	pc = 0x8244B0FC; continue 'dispatch;
	}
	// 8244B0F4: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 8244B0F8: 93BF00F8  stw r29, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[29].u32 ) };
	// 8244B0FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244B100: 480EA00C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244B108 size=120
    let mut pc: u32 = 0x8244B108;
    'dispatch: loop {
        match pc {
            0x8244B108 => {
    //   block [0x8244B108..0x8244B180)
	// 8244B108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244B10C: 480E9FA9  bl 0x825350b4
	ctx.lr = 0x8244B110;
	sub_82535080(ctx, base);
	// 8244B110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244B114: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244B118: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8244B11C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8244B120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244B124: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8244B128: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8244B12C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244B130: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244B134: 4BFFFB0D  bl 0x8244ac40
	ctx.lr = 0x8244B138;
	sub_8244AC40(ctx, base);
	// 8244B138: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244B13C: 419A001C  beq cr6, 0x8244b158
	if ctx.cr[6].eq {
	pc = 0x8244B158; continue 'dispatch;
	}
	// 8244B140: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8244B144: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244B148: 60840301  ori r4, r4, 0x301
	ctx.r[4].u64 = ctx.r[4].u64 | 769;
	// 8244B14C: 4BFFFBD5  bl 0x8244ad20
	ctx.lr = 0x8244B150;
	sub_8244AD20(ctx, base);
	// 8244B150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244B154: 480E9FB0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8244B158: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8244B15C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8244B160: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8244B164: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8244B168: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8244B16C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244B170: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244B174: 4E800421  bctrl
	ctx.lr = 0x8244B178;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8244B178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244B17C: 480E9F88  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244B180 size=652
    let mut pc: u32 = 0x8244B180;
    'dispatch: loop {
        match pc {
            0x8244B180 => {
    //   block [0x8244B180..0x8244B40C)
	// 8244B180: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8244B184: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8244B188: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 8244B18C: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8244B190: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8244B194: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8244B198: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B19C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8244B1A0: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 8244B1A4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B1A8: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B1AC: 2F0A001E  cmpwi cr6, r10, 0x1e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 30, &mut ctx.xer);
	// 8244B1B0: 7D095030  slw r9, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B1B4: 41980038  blt cr6, 0x8244b1ec
	if ctx.cr[6].lt {
	pc = 0x8244B1EC; continue 'dispatch;
	}
	// 8244B1B8: 390AFFE2  addi r8, r10, -0x1e
	ctx.r[8].s64 = ctx.r[10].s64 + -30;
	// 8244B1BC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8244B1C0: 419A001C  beq cr6, 0x8244b1dc
	if ctx.cr[6].eq {
	pc = 0x8244B1DC; continue 'dispatch;
	}
	// 8244B1C4: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B1C8: 7D494B78  or r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8244B1CC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B1D0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B1D4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B1D8: 4800001C  b 0x8244b1f4
	pc = 0x8244B1F4; continue 'dispatch;
	// 8244B1DC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B1E0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B1E4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B1E8: 4800000C  b 0x8244b1f4
	pc = 0x8244B1F4; continue 'dispatch;
	// 8244B1EC: 390A0002  addi r8, r10, 2
	ctx.r[8].s64 = ctx.r[10].s64 + 2;
	// 8244B1F0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B1F4: 553E17BE  srwi r30, r9, 0x1e
	ctx.r[30].u32 = ctx.r[9].u32.wrapping_shr(30);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8244B1F8: 39280002  addi r9, r8, 2
	ctx.r[9].s64 = ctx.r[8].s64 + 2;
	// 8244B1FC: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244B200: 41980018  blt cr6, 0x8244b218
	if ctx.cr[6].lt {
	pc = 0x8244B218; continue 'dispatch;
	}
	// 8244B204: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244B208: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B20C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B210: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B214: 48000008  b 0x8244b21c
	pc = 0x8244B21C; continue 'dispatch;
	// 8244B218: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B21C: 2F09001D  cmpwi cr6, r9, 0x1d
	ctx.cr[6].compare_i32(ctx.r[9].s32, 29, &mut ctx.xer);
	// 8244B220: 41980044  blt cr6, 0x8244b264
	if ctx.cr[6].lt {
	pc = 0x8244B264; continue 'dispatch;
	}
	// 8244B224: 3929FFE3  addi r9, r9, -0x1d
	ctx.r[9].s64 = ctx.r[9].s64 + -29;
	// 8244B228: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B22C: 419A0024  beq cr6, 0x8244b250
	if ctx.cr[6].eq {
	pc = 0x8244B250; continue 'dispatch;
	}
	// 8244B230: 21090003  subfic r8, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[9].s64;
	// 8244B234: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B238: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B23C: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B240: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B244: 551F1F7E  srwi r31, r8, 0x1d
	ctx.r[31].u32 = ctx.r[8].u32.wrapping_shr(29);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8244B248: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B24C: 48000024  b 0x8244b270
	pc = 0x8244B270; continue 'dispatch;
	// 8244B250: 555F1F7E  srwi r31, r10, 0x1d
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8244B254: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B258: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B25C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B260: 48000010  b 0x8244b270
	pc = 0x8244B270; continue 'dispatch;
	// 8244B264: 555F1F7E  srwi r31, r10, 0x1d
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8244B268: 39290003  addi r9, r9, 3
	ctx.r[9].s64 = ctx.r[9].s64 + 3;
	// 8244B26C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B270: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8244B274: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244B278: 41980018  blt cr6, 0x8244b290
	if ctx.cr[6].lt {
	pc = 0x8244B290; continue 'dispatch;
	}
	// 8244B27C: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244B280: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B284: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B288: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B28C: 48000008  b 0x8244b294
	pc = 0x8244B294; continue 'dispatch;
	// 8244B290: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B294: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8244B298: 41980044  blt cr6, 0x8244b2dc
	if ctx.cr[6].lt {
	pc = 0x8244B2DC; continue 'dispatch;
	}
	// 8244B29C: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8244B2A0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B2A4: 419A0024  beq cr6, 0x8244b2c8
	if ctx.cr[6].eq {
	pc = 0x8244B2C8; continue 'dispatch;
	}
	// 8244B2A8: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8244B2AC: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B2B0: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B2B4: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B2B8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B2BC: 55047C7E  srwi r4, r8, 0x11
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shr(17);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8244B2C0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B2C4: 48000024  b 0x8244b2e8
	pc = 0x8244B2E8; continue 'dispatch;
	// 8244B2C8: 55447C7E  srwi r4, r10, 0x11
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8244B2CC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B2D0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B2D4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B2D8: 48000010  b 0x8244b2e8
	pc = 0x8244B2E8; continue 'dispatch;
	// 8244B2DC: 55447C7E  srwi r4, r10, 0x11
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8244B2E0: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8244B2E4: 554A7820  slwi r10, r10, 0xf
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B2E8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8244B2EC: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244B2F0: 41980018  blt cr6, 0x8244b308
	if ctx.cr[6].lt {
	pc = 0x8244B308; continue 'dispatch;
	}
	// 8244B2F4: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244B2F8: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B2FC: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B300: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B304: 48000008  b 0x8244b30c
	pc = 0x8244B30C; continue 'dispatch;
	// 8244B308: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B30C: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8244B310: 4198003C  blt cr6, 0x8244b34c
	if ctx.cr[6].lt {
	pc = 0x8244B34C; continue 'dispatch;
	}
	// 8244B314: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8244B318: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B31C: 419A0020  beq cr6, 0x8244b33c
	if ctx.cr[6].eq {
	pc = 0x8244B33C; continue 'dispatch;
	}
	// 8244B320: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8244B324: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B328: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B32C: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B330: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B334: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B338: 4800001C  b 0x8244b354
	pc = 0x8244B354; continue 'dispatch;
	// 8244B33C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244B340: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B344: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B348: 4800000C  b 0x8244b354
	pc = 0x8244B354; continue 'dispatch;
	// 8244B34C: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8244B350: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B354: 55467C7E  srwi r6, r10, 0x11
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244B358: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8244B35C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244B360: 41980018  blt cr6, 0x8244b378
	if ctx.cr[6].lt {
	pc = 0x8244B378; continue 'dispatch;
	}
	// 8244B364: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244B368: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B36C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B370: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B374: 48000008  b 0x8244b37c
	pc = 0x8244B37C; continue 'dispatch;
	// 8244B378: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244B37C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244B380: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244B384: 41980014  blt cr6, 0x8244b398
	if ctx.cr[6].lt {
	pc = 0x8244B398; continue 'dispatch;
	}
	// 8244B388: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244B38C: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B390: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B394: 48000008  b 0x8244b39c
	pc = 0x8244B39C; continue 'dispatch;
	// 8244B398: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244B39C: 2F0A000A  cmpwi cr6, r10, 0xa
	ctx.cr[6].compare_i32(ctx.r[10].s32, 10, &mut ctx.xer);
	// 8244B3A0: 41980024  blt cr6, 0x8244b3c4
	if ctx.cr[6].lt {
	pc = 0x8244B3C4; continue 'dispatch;
	}
	// 8244B3A4: 394AFFF6  addi r10, r10, -0xa
	ctx.r[10].s64 = ctx.r[10].s64 + -10;
	// 8244B3A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244B3AC: 419A0018  beq cr6, 0x8244b3c4
	if ctx.cr[6].eq {
	pc = 0x8244B3C4; continue 'dispatch;
	}
	// 8244B3B0: 214A0016  subfic r10, r10, 0x16
	ctx.xer.ca = ctx.r[10].u32 <= 22 as u32;
	ctx.r[10].s64 = (22 as i64) - ctx.r[10].s64;
	// 8244B3B4: 7D6B5430  srw r11, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B3B8: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8244B3BC: 556BB2BE  srwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244B3C0: 48000008  b 0x8244b3c8
	pc = 0x8244B3C8; continue 'dispatch;
	// 8244B3C4: 552BB2BE  srwi r11, r9, 0xa
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244B3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244B3CC: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8244B3D0: 78890020  clrldi r9, r4, 0x20
	ctx.r[9].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 8244B3D4: 7BEA7C4C  rldimi r10, r31, 0xf, 0x11
	ctx.r[10].u64 = ((ctx.r[31].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[10].u64 & 0xFFFF800000007FFF);
	// 8244B3D8: 78C80020  clrldi r8, r6, 0x20
	ctx.r[8].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	// 8244B3DC: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8244B3E0: 7FC90034  cntlzw r9, r30
	ctx.r[9].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 8244B3E4: 794A7C24  sldi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(15);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8244B3E8: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8244B3EC: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8244B3F0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8244B3F4: 91230020  stw r9, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 8244B3F8: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 8244B3FC: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8244B400: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244B404: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8244B408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244B410 size=1348
    let mut pc: u32 = 0x8244B410;
    'dispatch: loop {
        match pc {
            0x8244B410 => {
    //   block [0x8244B410..0x8244B954)
	// 8244B410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244B414: 480E9C99  bl 0x825350ac
	ctx.lr = 0x8244B418;
	sub_82535080(ctx, base);
	// 8244B418: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244B41C: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 8244B420: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8244B424: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8244B428: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8244B42C: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8244B430: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8244B434: 3BBB0028  addi r29, r27, 0x28
	ctx.r[29].s64 = ctx.r[27].s64 + 40;
	// 8244B438: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B43C: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 8244B440: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 8244B444: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B448: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B44C: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 8244B450: 7D095030  slw r9, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B454: 4198004C  blt cr6, 0x8244b4a0
	if ctx.cr[6].lt {
	pc = 0x8244B4A0; continue 'dispatch;
	}
	// 8244B458: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 8244B45C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244B460: 419A0028  beq cr6, 0x8244b488
	if ctx.cr[6].eq {
	pc = 0x8244B488; continue 'dispatch;
	}
	// 8244B464: 210A0010  subfic r8, r10, 0x10
	ctx.xer.ca = ctx.r[10].u32 <= 16 as u32;
	ctx.r[8].s64 = (16 as i64) - ctx.r[10].s64;
	// 8244B468: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B46C: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8244B470: 7D685030  slw r8, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B474: 552B843E  srwi r11, r9, 0x10
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244B478: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244B47C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B480: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B484: 4800002C  b 0x8244b4b0
	pc = 0x8244B4B0; continue 'dispatch;
	// 8244B488: 5529843E  srwi r9, r9, 0x10
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244B48C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244B490: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8244B494: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B498: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B49C: 48000014  b 0x8244b4b0
	pc = 0x8244B4B0; continue 'dispatch;
	// 8244B4A0: 5526843E  srwi r6, r9, 0x10
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shr(16);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244B4A4: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8244B4A8: 5528801E  slwi r8, r9, 0x10
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B4AC: 90DD0000  stw r6, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8244B4B0: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8244B4B4: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244B4B8: 41980018  blt cr6, 0x8244b4d0
	if ctx.cr[6].lt {
	pc = 0x8244B4D0; continue 'dispatch;
	}
	// 8244B4BC: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244B4C0: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B4C4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B4C8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B4CC: 48000008  b 0x8244b4d4
	pc = 0x8244B4D4; continue 'dispatch;
	// 8244B4D0: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B4D4: 2F09000A  cmpwi cr6, r9, 0xa
	ctx.cr[6].compare_i32(ctx.r[9].s32, 10, &mut ctx.xer);
	// 8244B4D8: 4198004C  blt cr6, 0x8244b524
	if ctx.cr[6].lt {
	pc = 0x8244B524; continue 'dispatch;
	}
	// 8244B4DC: 3929FFF6  addi r9, r9, -0xa
	ctx.r[9].s64 = ctx.r[9].s64 + -10;
	// 8244B4E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B4E4: 419A0028  beq cr6, 0x8244b50c
	if ctx.cr[6].eq {
	pc = 0x8244B50C; continue 'dispatch;
	}
	// 8244B4E8: 21090016  subfic r8, r9, 0x16
	ctx.xer.ca = ctx.r[9].u32 <= 22 as u32;
	ctx.r[8].s64 = (22 as i64) - ctx.r[9].s64;
	// 8244B4EC: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B4F0: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B4F4: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B4F8: 550BB2BE  srwi r11, r8, 0xa
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244B4FC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8244B500: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B504: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B508: 4800002C  b 0x8244b534
	pc = 0x8244B534; continue 'dispatch;
	// 8244B50C: 5548B2BE  srwi r8, r10, 0xa
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B510: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B514: 911D0004  stw r8, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8244B518: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B51C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B520: 48000014  b 0x8244b534
	pc = 0x8244B534; continue 'dispatch;
	// 8244B524: 5548B2BE  srwi r8, r10, 0xa
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(10);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B528: 39290016  addi r9, r9, 0x16
	ctx.r[9].s64 = ctx.r[9].s64 + 22;
	// 8244B52C: 554AB012  slwi r10, r10, 0x16
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(22);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B530: 911D0004  stw r8, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8244B534: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8244B538: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244B53C: 41980018  blt cr6, 0x8244b554
	if ctx.cr[6].lt {
	pc = 0x8244B554; continue 'dispatch;
	}
	// 8244B540: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244B544: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B548: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B54C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B550: 48000008  b 0x8244b558
	pc = 0x8244B558; continue 'dispatch;
	// 8244B554: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B558: 2F09001A  cmpwi cr6, r9, 0x1a
	ctx.cr[6].compare_i32(ctx.r[9].s32, 26, &mut ctx.xer);
	// 8244B55C: 4198004C  blt cr6, 0x8244b5a8
	if ctx.cr[6].lt {
	pc = 0x8244B5A8; continue 'dispatch;
	}
	// 8244B560: 3929FFE6  addi r9, r9, -0x1a
	ctx.r[9].s64 = ctx.r[9].s64 + -26;
	// 8244B564: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B568: 419A0028  beq cr6, 0x8244b590
	if ctx.cr[6].eq {
	pc = 0x8244B590; continue 'dispatch;
	}
	// 8244B56C: 21090006  subfic r8, r9, 6
	ctx.xer.ca = ctx.r[9].u32 <= 6 as u32;
	ctx.r[8].s64 = (6 as i64) - ctx.r[9].s64;
	// 8244B570: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B574: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B578: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B57C: 550B36BE  srwi r11, r8, 0x1a
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(26);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244B580: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244B584: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B588: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B58C: 4800002C  b 0x8244b5b8
	pc = 0x8244B5B8; continue 'dispatch;
	// 8244B590: 554836BE  srwi r8, r10, 0x1a
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(26);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B594: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B598: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8244B59C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B5A0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B5A4: 48000014  b 0x8244b5b8
	pc = 0x8244B5B8; continue 'dispatch;
	// 8244B5A8: 554836BE  srwi r8, r10, 0x1a
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(26);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B5AC: 39290006  addi r9, r9, 6
	ctx.r[9].s64 = ctx.r[9].s64 + 6;
	// 8244B5B0: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B5B4: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8244B5B8: 7D480034  cntlzw r8, r10
	ctx.r[8].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8244B5BC: 2F09001F  cmpwi cr6, r9, 0x1f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 31, &mut ctx.xer);
	// 8244B5C0: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8244B5C4: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8244B5C8: 911D0010  stw r8, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8244B5CC: 409A0018  bne cr6, 0x8244b5e4
	if !ctx.cr[6].eq {
	pc = 0x8244B5E4; continue 'dispatch;
	}
	// 8244B5D0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244B5D4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B5D8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8244B5DC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B5E0: 4800000C  b 0x8244b5ec
	pc = 0x8244B5EC; continue 'dispatch;
	// 8244B5E4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8244B5E8: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B5EC: 7D0A0034  cntlzw r10, r8
	ctx.r[10].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8244B5F0: 2F09001F  cmpwi cr6, r9, 0x1f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 31, &mut ctx.xer);
	// 8244B5F4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8244B5F8: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8244B5FC: 915D0014  stw r10, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8244B600: 409A0018  bne cr6, 0x8244b618
	if !ctx.cr[6].eq {
	pc = 0x8244B618; continue 'dispatch;
	}
	// 8244B604: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8244B608: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244B610: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B614: 4800000C  b 0x8244b620
	pc = 0x8244B620; continue 'dispatch;
	// 8244B618: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8244B61C: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244B620: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8244B624: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8244B628: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8244B62C: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8244B630: 911D0018  stw r8, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 8244B634: 409A0018  bne cr6, 0x8244b64c
	if !ctx.cr[6].eq {
	pc = 0x8244B64C; continue 'dispatch;
	}
	// 8244B638: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8244B63C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244B644: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B648: 4800000C  b 0x8244b654
	pc = 0x8244B654; continue 'dispatch;
	// 8244B64C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244B650: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244B654: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8244B658: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8244B65C: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8244B660: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8244B664: 911D001C  stw r8, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8244B668: 409A0018  bne cr6, 0x8244b680
	if !ctx.cr[6].eq {
	pc = 0x8244B680; continue 'dispatch;
	}
	// 8244B66C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244B670: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244B678: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B67C: 4800000C  b 0x8244b688
	pc = 0x8244B688; continue 'dispatch;
	// 8244B680: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244B684: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B688: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8244B68C: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244B690: 41980018  blt cr6, 0x8244b6a8
	if ctx.cr[6].lt {
	pc = 0x8244B6A8; continue 'dispatch;
	}
	// 8244B694: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244B698: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B69C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B6A0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B6A4: 48000008  b 0x8244b6ac
	pc = 0x8244B6AC; continue 'dispatch;
	// 8244B6A8: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B6AC: 2F09001B  cmpwi cr6, r9, 0x1b
	ctx.cr[6].compare_i32(ctx.r[9].s32, 27, &mut ctx.xer);
	// 8244B6B0: 4198004C  blt cr6, 0x8244b6fc
	if ctx.cr[6].lt {
	pc = 0x8244B6FC; continue 'dispatch;
	}
	// 8244B6B4: 3929FFE5  addi r9, r9, -0x1b
	ctx.r[9].s64 = ctx.r[9].s64 + -27;
	// 8244B6B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B6BC: 419A0028  beq cr6, 0x8244b6e4
	if ctx.cr[6].eq {
	pc = 0x8244B6E4; continue 'dispatch;
	}
	// 8244B6C0: 21090005  subfic r8, r9, 5
	ctx.xer.ca = ctx.r[9].u32 <= 5 as u32;
	ctx.r[8].s64 = (5 as i64) - ctx.r[9].s64;
	// 8244B6C4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B6C8: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B6CC: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B6D0: 550B2EFE  srwi r11, r8, 0x1b
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(27);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244B6D4: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8244B6D8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B6DC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B6E0: 4800002C  b 0x8244b70c
	pc = 0x8244B70C; continue 'dispatch;
	// 8244B6E4: 55482EFE  srwi r8, r10, 0x1b
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(27);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B6E8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B6EC: 911D000C  stw r8, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8244B6F0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B6F4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B6F8: 48000014  b 0x8244b70c
	pc = 0x8244B70C; continue 'dispatch;
	// 8244B6FC: 55482EFE  srwi r8, r10, 0x1b
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(27);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B700: 39290005  addi r9, r9, 5
	ctx.r[9].s64 = ctx.r[9].s64 + 5;
	// 8244B704: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B708: 911D000C  stw r8, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8244B70C: 2F090018  cmpwi cr6, r9, 0x18
	ctx.cr[6].compare_i32(ctx.r[9].s32, 24, &mut ctx.xer);
	// 8244B710: 41980044  blt cr6, 0x8244b754
	if ctx.cr[6].lt {
	pc = 0x8244B754; continue 'dispatch;
	}
	// 8244B714: 3929FFE8  addi r9, r9, -0x18
	ctx.r[9].s64 = ctx.r[9].s64 + -24;
	// 8244B718: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B71C: 419A0024  beq cr6, 0x8244b740
	if ctx.cr[6].eq {
	pc = 0x8244B740; continue 'dispatch;
	}
	// 8244B720: 21090008  subfic r8, r9, 8
	ctx.xer.ca = ctx.r[9].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[9].s64;
	// 8244B724: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B728: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B72C: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B730: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B734: 551E463E  srwi r30, r8, 0x18
	ctx.r[30].u32 = ctx.r[8].u32.wrapping_shr(24);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8244B738: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B73C: 48000024  b 0x8244b760
	pc = 0x8244B760; continue 'dispatch;
	// 8244B740: 555E463E  srwi r30, r10, 0x18
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8244B744: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B748: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B74C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B750: 48000010  b 0x8244b760
	pc = 0x8244B760; continue 'dispatch;
	// 8244B754: 555E463E  srwi r30, r10, 0x18
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8244B758: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8244B75C: 554A402E  slwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B760: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244B764: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244B768: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8244B76C: 409800E4  bge cr6, 0x8244b850
	if !ctx.cr[6].lt {
	pc = 0x8244B850; continue 'dispatch;
	}
	// 8244B770: 2F090018  cmpwi cr6, r9, 0x18
	ctx.cr[6].compare_i32(ctx.r[9].s32, 24, &mut ctx.xer);
	// 8244B774: 4198003C  blt cr6, 0x8244b7b0
	if ctx.cr[6].lt {
	pc = 0x8244B7B0; continue 'dispatch;
	}
	// 8244B778: 3929FFE8  addi r9, r9, -0x18
	ctx.r[9].s64 = ctx.r[9].s64 + -24;
	// 8244B77C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B780: 419A0020  beq cr6, 0x8244b7a0
	if ctx.cr[6].eq {
	pc = 0x8244B7A0; continue 'dispatch;
	}
	// 8244B784: 21090008  subfic r8, r9, 8
	ctx.xer.ca = ctx.r[9].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[9].s64;
	// 8244B788: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B78C: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B790: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B794: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B798: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B79C: 4800001C  b 0x8244b7b8
	pc = 0x8244B7B8; continue 'dispatch;
	// 8244B7A0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244B7A4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B7A8: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B7AC: 4800000C  b 0x8244b7b8
	pc = 0x8244B7B8; continue 'dispatch;
	// 8244B7B0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8244B7B4: 5548402E  slwi r8, r10, 8
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B7B8: 5545463E  srwi r5, r10, 0x18
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shr(24);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8244B7BC: 39490002  addi r10, r9, 2
	ctx.r[10].s64 = ctx.r[9].s64 + 2;
	// 8244B7C0: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244B7C4: 41980018  blt cr6, 0x8244b7dc
	if ctx.cr[6].lt {
	pc = 0x8244B7DC; continue 'dispatch;
	}
	// 8244B7C8: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244B7CC: 7D685030  slw r8, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B7D0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B7D4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B7D8: 48000008  b 0x8244b7e0
	pc = 0x8244B7E0; continue 'dispatch;
	// 8244B7DC: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B7E0: 7D090034  cntlzw r9, r8
	ctx.r[9].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8244B7E4: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8244B7E8: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8244B7EC: 5526DFFE  rlwinm r6, r9, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8244B7F0: 409A0114  bne cr6, 0x8244b904
	if !ctx.cr[6].eq {
	pc = 0x8244B904; continue 'dispatch;
	}
	// 8244B7F4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B7F8: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8244B800: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B804: 55486CFE  srwi r8, r10, 0x13
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B808: 3929000D  addi r9, r9, 0xd
	ctx.r[9].s64 = ctx.r[9].s64 + 13;
	// 8244B80C: 554A6824  slwi r10, r10, 0xd
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(13);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B810: 5463103A  slwi r3, r3, 2
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8244B814: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 8244B818: 3B410061  addi r26, r1, 0x61
	ctx.r[26].s64 = ctx.r[1].s64 + 97;
	// 8244B81C: 3B210062  addi r25, r1, 0x62
	ctx.r[25].s64 = ctx.r[1].s64 + 98;
	// 8244B820: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244B824: 7CA3F9AE  stbx r5, r3, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u8) };
	// 8244B828: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8244B82C: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8244B830: 7CC5D1AE  stbx r6, r5, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[26].u32), ctx.r[6].u8) };
	// 8244B834: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8244B838: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244B83C: 7D06CB2E  sthx r8, r6, r25
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[25].u32), ctx.r[8].u16) };
	// 8244B840: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8244B844: 38680001  addi r3, r8, 1
	ctx.r[3].s64 = ctx.r[8].s64 + 1;
	// 8244B848: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8244B84C: 4198FF24  blt cr6, 0x8244b770
	if ctx.cr[6].lt {
	pc = 0x8244B770; continue 'dispatch;
	}
	// 8244B850: 39690007  addi r11, r9, 7
	ctx.r[11].s64 = ctx.r[9].s64 + 7;
	// 8244B854: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 8244B858: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 8244B85C: 3BEBFFF8  addi r31, r11, -8
	ctx.r[31].s64 = ctx.r[11].s64 + -8;
	// 8244B860: 7D64F850  subf r11, r4, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[4].s64;
	// 8244B864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244B868: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244B86C: 48000C7D  bl 0x8244c4e8
	ctx.lr = 0x8244B870;
	sub_8244C4E8(ctx, base);
	// 8244B870: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244B874: 409A0024  bne cr6, 0x8244b898
	if !ctx.cr[6].eq {
	pc = 0x8244B898; continue 'dispatch;
	}
	// 8244B878: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 8244B87C: 48000C6D  bl 0x8244c4e8
	ctx.lr = 0x8244B880;
	sub_8244C4E8(ctx, base);
	// 8244B880: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 8244B884: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8244B888: 409A0010  bne cr6, 0x8244b898
	if !ctx.cr[6].eq {
	pc = 0x8244B898; continue 'dispatch;
	}
	// 8244B88C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B890: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8244B894: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244B898: 817B00E4  lwz r11, 0xe4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(228 as u32) ) } as u64;
	// 8244B89C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244B8A0: 419A005C  beq cr6, 0x8244b8fc
	if ctx.cr[6].eq {
	pc = 0x8244B8FC; continue 'dispatch;
	}
	// 8244B8A4: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244B8A8: 57C8CFFE  rlwinm r8, r30, 0x19, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	// 8244B8AC: 813D0010  lwz r9, 0x10(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244B8B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8244B8B4: 807B00E8  lwz r3, 0xe8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(232 as u32) ) } as u64;
	// 8244B8B8: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 8244B8BC: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8244B8C0: 9901005A  stb r8, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[8].u8 ) };
	// 8244B8C4: 811D0014  lwz r8, 0x14(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8244B8C8: 99210055  stb r9, 0x55(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(85 as u32), ctx.r[9].u8 ) };
	// 8244B8CC: 813D001C  lwz r9, 0x1c(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8244B8D0: 99410057  stb r10, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[10].u8 ) };
	// 8244B8D4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244B8D8: 99010056  stb r8, 0x56(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[8].u8 ) };
	// 8244B8DC: 811D000C  lwz r8, 0xc(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244B8E0: 99210058  stb r9, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u8 ) };
	// 8244B8E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8244B8E8: 57CA067E  clrlwi r10, r30, 0x19
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x0000007Fu64;
	// 8244B8EC: 99010059  stb r8, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[8].u8 ) };
	// 8244B8F0: 9941005B  stb r10, 0x5b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(91 as u32), ctx.r[10].u8 ) };
	// 8244B8F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244B8F8: 4E800421  bctrl
	ctx.lr = 0x8244B8FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8244B8FC: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8244B900: 480E97FC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8244B904: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8244B908: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B90C: 2F090013  cmpwi cr6, r9, 0x13
	ctx.cr[6].compare_i32(ctx.r[9].s32, 19, &mut ctx.xer);
	// 8244B910: 4198FEF4  blt cr6, 0x8244b804
	if ctx.cr[6].lt {
	pc = 0x8244B804; continue 'dispatch;
	}
	// 8244B914: 3929FFED  addi r9, r9, -0x13
	ctx.r[9].s64 = ctx.r[9].s64 + -19;
	// 8244B918: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244B91C: 419A0024  beq cr6, 0x8244b940
	if ctx.cr[6].eq {
	pc = 0x8244B940; continue 'dispatch;
	}
	// 8244B920: 2109000D  subfic r8, r9, 0xd
	ctx.xer.ca = ctx.r[9].u32 <= 13 as u32;
	ctx.r[8].s64 = (13 as i64) - ctx.r[9].s64;
	// 8244B924: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B928: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244B92C: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B930: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B934: 55086CFE  srwi r8, r8, 0x13
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B938: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B93C: 4BFFFED4  b 0x8244b810
	pc = 0x8244B810; continue 'dispatch;
	// 8244B940: 55486CFE  srwi r8, r10, 0x13
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244B944: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244B948: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B94C: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8244B950: 4BFFFEC0  b 0x8244b810
	pc = 0x8244B810; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244B958 size=2040
    let mut pc: u32 = 0x8244B958;
    'dispatch: loop {
        match pc {
            0x8244B958 => {
    //   block [0x8244B958..0x8244C150)
	// 8244B958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244B95C: 480E9761  bl 0x825350bc
	ctx.lr = 0x8244B960;
	sub_82535080(ctx, base);
	// 8244B960: 39640003  addi r11, r4, 3
	ctx.r[11].s64 = ctx.r[4].s64 + 3;
	// 8244B964: 3BC300A8  addi r30, r3, 0xa8
	ctx.r[30].s64 = ctx.r[3].s64 + 168;
	// 8244B968: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8244B96C: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8244B970: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 8244B974: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B978: 396A0003  addi r11, r10, 3
	ctx.r[11].s64 = ctx.r[10].s64 + 3;
	// 8244B97C: 3BE90004  addi r31, r9, 4
	ctx.r[31].s64 = ctx.r[9].s64 + 4;
	// 8244B980: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244B984: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B988: 2F0A0018  cmpwi cr6, r10, 0x18
	ctx.cr[6].compare_i32(ctx.r[10].s32, 24, &mut ctx.xer);
	// 8244B98C: 7D095030  slw r9, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B990: 4198003C  blt cr6, 0x8244b9cc
	if ctx.cr[6].lt {
	pc = 0x8244B9CC; continue 'dispatch;
	}
	// 8244B994: 394AFFE8  addi r10, r10, -0x18
	ctx.r[10].s64 = ctx.r[10].s64 + -24;
	// 8244B998: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244B99C: 419A0020  beq cr6, 0x8244b9bc
	if ctx.cr[6].eq {
	pc = 0x8244B9BC; continue 'dispatch;
	}
	// 8244B9A0: 210A0008  subfic r8, r10, 8
	ctx.xer.ca = ctx.r[10].u32 <= 8 as u32;
	ctx.r[8].s64 = (8 as i64) - ctx.r[10].s64;
	// 8244B9A4: 7D675030  slw r7, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B9A8: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244B9AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B9B0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244B9B4: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8244B9B8: 4800001C  b 0x8244b9d4
	pc = 0x8244B9D4; continue 'dispatch;
	// 8244B9BC: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 8244B9C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244B9C4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244B9C8: 4800000C  b 0x8244b9d4
	pc = 0x8244B9D4; continue 'dispatch;
	// 8244B9CC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8244B9D0: 5527402E  slwi r7, r9, 8
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8244B9D4: 5523463E  srwi r3, r9, 0x18
	ctx.r[3].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8244B9D8: 3903FF20  addi r8, r3, -0xe0
	ctx.r[8].s64 = ctx.r[3].s64 + -224;
	// 8244B9DC: 2B08000F  cmplwi cr6, r8, 0xf
	ctx.cr[6].compare_u32(ctx.r[8].u32, 15 as u32, &mut ctx.xer);
	// 8244B9E0: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8244B9E4: 4199000C  bgt cr6, 0x8244b9f0
	if ctx.cr[6].gt {
	pc = 0x8244B9F0; continue 'dispatch;
	}
	// 8244B9E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8244B9EC: 48000054  b 0x8244ba40
	pc = 0x8244BA40; continue 'dispatch;
	// 8244B9F0: 3903FF40  addi r8, r3, -0xc0
	ctx.r[8].s64 = ctx.r[3].s64 + -192;
	// 8244B9F4: 2B08001F  cmplwi cr6, r8, 0x1f
	ctx.cr[6].compare_u32(ctx.r[8].u32, 31 as u32, &mut ctx.xer);
	// 8244B9F8: 4199000C  bgt cr6, 0x8244ba04
	if ctx.cr[6].gt {
	pc = 0x8244BA04; continue 'dispatch;
	}
	// 8244B9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8244BA00: 48000040  b 0x8244ba40
	pc = 0x8244BA40; continue 'dispatch;
	// 8244BA04: 2F0300BD  cmpwi cr6, r3, 0xbd
	ctx.cr[6].compare_i32(ctx.r[3].s32, 189, &mut ctx.xer);
	// 8244BA08: 409A0010  bne cr6, 0x8244ba18
	if !ctx.cr[6].eq {
	pc = 0x8244BA18; continue 'dispatch;
	}
	// 8244BA0C: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8244BA10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8244BA14: 4800002C  b 0x8244ba40
	pc = 0x8244BA40; continue 'dispatch;
	// 8244BA18: 2F0300BF  cmpwi cr6, r3, 0xbf
	ctx.cr[6].compare_i32(ctx.r[3].s32, 191, &mut ctx.xer);
	// 8244BA1C: 409A0010  bne cr6, 0x8244ba2c
	if !ctx.cr[6].eq {
	pc = 0x8244BA2C; continue 'dispatch;
	}
	// 8244BA20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8244BA24: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8244BA28: 48000018  b 0x8244ba40
	pc = 0x8244BA40; continue 'dispatch;
	// 8244BA2C: 2F0300BE  cmpwi cr6, r3, 0xbe
	ctx.cr[6].compare_i32(ctx.r[3].s32, 190, &mut ctx.xer);
	// 8244BA30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8244BA34: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8244BA38: 419A0008  beq cr6, 0x8244ba40
	if ctx.cr[6].eq {
	pc = 0x8244BA40; continue 'dispatch;
	}
	// 8244BA3C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8244BA40: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 8244BA44: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8244BA48: 911E0018  stw r8, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 8244BA4C: 409A0074  bne cr6, 0x8244bac0
	if !ctx.cr[6].eq {
	pc = 0x8244BAC0; continue 'dispatch;
	}
	// 8244BA50: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 8244BA54: 41980054  blt cr6, 0x8244baa8
	if ctx.cr[6].lt {
	pc = 0x8244BAA8; continue 'dispatch;
	}
	// 8244BA58: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 8244BA5C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244BA60: 419A002C  beq cr6, 0x8244ba8c
	if ctx.cr[6].eq {
	pc = 0x8244BA8C; continue 'dispatch;
	}
	// 8244BA64: 212A0010  subfic r9, r10, 0x10
	ctx.xer.ca = ctx.r[10].u32 <= 16 as u32;
	ctx.r[9].s64 = (16 as i64) - ctx.r[10].s64;
	// 8244BA68: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 8244BA6C: 7D694C30  srw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BA70: 7D283B78  or r8, r9, r7
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 8244BA74: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BA78: 550B843E  srwi r11, r8, 0x10
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244BA7C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8244BA80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BA84: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BA88: 4800006C  b 0x8244baf4
	pc = 0x8244BAF4; continue 'dispatch;
	// 8244BA8C: 54E8843E  srwi r8, r7, 0x10
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BA90: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8244BA94: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 8244BA98: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8244BA9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BAA0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BAA4: 48000050  b 0x8244baf4
	pc = 0x8244BAF4; continue 'dispatch;
	// 8244BAA8: 54E8843E  srwi r8, r7, 0x10
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shr(16);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BAAC: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8244BAB0: 54E9801E  slwi r9, r7, 0x10
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BAB4: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 8244BAB8: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8244BABC: 48000038  b 0x8244baf4
	pc = 0x8244BAF4; continue 'dispatch;
	// 8244BAC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244BAC4: 419A001C  beq cr6, 0x8244bae0
	if ctx.cr[6].eq {
	pc = 0x8244BAE0; continue 'dispatch;
	}
	// 8244BAC8: 212A0020  subfic r9, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[9].s64 = (32 as i64) - ctx.r[10].s64;
	// 8244BACC: 7D694C30  srw r9, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BAD0: 7D283B78  or r8, r9, r7
	ctx.r[8].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 8244BAD4: 911E001C  stw r8, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 8244BAD8: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BADC: 4800000C  b 0x8244bae8
	pc = 0x8244BAE8; continue 'dispatch;
	// 8244BAE0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8244BAE4: 90FE001C  stw r7, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 8244BAE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BAEC: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 8244BAF0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BAF4: 2F0300BF  cmpwi cr6, r3, 0xbf
	ctx.cr[6].compare_i32(ctx.r[3].s32, 191, &mut ctx.xer);
	// 8244BAF8: 419A0648  beq cr6, 0x8244c140
	if ctx.cr[6].eq {
	pc = 0x8244C140; continue 'dispatch;
	}
	// 8244BAFC: 2F0300BE  cmpwi cr6, r3, 0xbe
	ctx.cr[6].compare_i32(ctx.r[3].s32, 190, &mut ctx.xer);
	// 8244BB00: 419A0640  beq cr6, 0x8244c140
	if ctx.cr[6].eq {
	pc = 0x8244C140; continue 'dispatch;
	}
	// 8244BB04: 5528463E  srwi r8, r9, 0x18
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BB08: 2F0A0018  cmpwi cr6, r10, 0x18
	ctx.cr[6].compare_i32(ctx.r[10].s32, 24, &mut ctx.xer);
	// 8244BB0C: 40990010  ble cr6, 0x8244bb1c
	if !ctx.cr[6].gt {
	pc = 0x8244BB1C; continue 'dispatch;
	}
	// 8244BB10: 20EA0038  subfic r7, r10, 0x38
	ctx.xer.ca = ctx.r[10].u32 <= 56 as u32;
	ctx.r[7].s64 = (56 as i64) - ctx.r[10].s64;
	// 8244BB14: 7D673C30  srw r7, r11, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BB18: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8244BB1C: 2B0800FF  cmplwi cr6, r8, 0xff
	ctx.cr[6].compare_u32(ctx.r[8].u32, 255 as u32, &mut ctx.xer);
	// 8244BB20: 409A002C  bne cr6, 0x8244bb4c
	if !ctx.cr[6].eq {
	pc = 0x8244BB4C; continue 'dispatch;
	}
	// 8244BB24: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8244BB28: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244BB2C: 41980018  blt cr6, 0x8244bb44
	if ctx.cr[6].lt {
	pc = 0x8244BB44; continue 'dispatch;
	}
	// 8244BB30: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244BB34: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BB38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BB3C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BB40: 4BFFFFC4  b 0x8244bb04
	pc = 0x8244BB04; continue 'dispatch;
	// 8244BB44: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BB48: 4BFFFFBC  b 0x8244bb04
	pc = 0x8244BB04; continue 'dispatch;
	// 8244BB4C: 552817BE  srwi r8, r9, 0x1e
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(30);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BB50: 2F0A001E  cmpwi cr6, r10, 0x1e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 30, &mut ctx.xer);
	// 8244BB54: 40990010  ble cr6, 0x8244bb64
	if !ctx.cr[6].gt {
	pc = 0x8244BB64; continue 'dispatch;
	}
	// 8244BB58: 20EA003E  subfic r7, r10, 0x3e
	ctx.xer.ca = ctx.r[10].u32 <= 62 as u32;
	ctx.r[7].s64 = (62 as i64) - ctx.r[10].s64;
	// 8244BB5C: 7D673C30  srw r7, r11, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BB60: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8244BB64: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 8244BB68: 409A006C  bne cr6, 0x8244bbd4
	if !ctx.cr[6].eq {
	pc = 0x8244BBD4; continue 'dispatch;
	}
	// 8244BB6C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8244BB70: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244BB74: 41980018  blt cr6, 0x8244bb8c
	if ctx.cr[6].lt {
	pc = 0x8244BB8C; continue 'dispatch;
	}
	// 8244BB78: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244BB7C: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BB80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BB84: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BB88: 48000008  b 0x8244bb90
	pc = 0x8244BB90; continue 'dispatch;
	// 8244BB8C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BB90: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8244BB94: 2F0A001F  cmpwi cr6, r10, 0x1f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 31, &mut ctx.xer);
	// 8244BB98: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 8244BB9C: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8244BBA0: 409A0074  bne cr6, 0x8244bc14
	if !ctx.cr[6].eq {
	pc = 0x8244BC14; continue 'dispatch;
	}
	// 8244BBA4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8244BBA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BBAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244BBB0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BBB4: 55286CFE  srwi r8, r9, 0x13
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BBB8: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 8244BBBC: 55296824  slwi r9, r9, 0xd
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(13);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BBC0: 55083830  slwi r8, r8, 7
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BBC4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8244BBC8: 419A0008  beq cr6, 0x8244bbd0
	if ctx.cr[6].eq {
	pc = 0x8244BBD0; continue 'dispatch;
	}
	// 8244BBCC: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BBD0: 911E0020  stw r8, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 8244BBD4: 5528273E  srwi r8, r9, 0x1c
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(28);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BBD8: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 8244BBDC: 40990010  ble cr6, 0x8244bbec
	if !ctx.cr[6].gt {
	pc = 0x8244BBEC; continue 'dispatch;
	}
	// 8244BBE0: 20EA003C  subfic r7, r10, 0x3c
	ctx.xer.ca = ctx.r[10].u32 <= 60 as u32;
	ctx.r[7].s64 = (60 as i64) - ctx.r[10].s64;
	// 8244BBE4: 7D673C30  srw r7, r11, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[11].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BBE8: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 8244BBEC: 2B080002  cmplwi cr6, r8, 2
	ctx.cr[6].compare_u32(ctx.r[8].u32, 2 as u32, &mut ctx.xer);
	// 8244BBF0: 409A01DC  bne cr6, 0x8244bdcc
	if !ctx.cr[6].eq {
	pc = 0x8244BDCC; continue 'dispatch;
	}
	// 8244BBF4: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 8244BBF8: 2F080020  cmpwi cr6, r8, 0x20
	ctx.cr[6].compare_i32(ctx.r[8].s32, 32, &mut ctx.xer);
	// 8244BBFC: 41980068  blt cr6, 0x8244bc64
	if ctx.cr[6].lt {
	pc = 0x8244BC64; continue 'dispatch;
	}
	// 8244BC00: 3908FFE0  addi r8, r8, -0x20
	ctx.r[8].s64 = ctx.r[8].s64 + -32;
	// 8244BC04: 7D6A4030  slw r10, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BC08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BC0C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BC10: 48000058  b 0x8244bc68
	pc = 0x8244BC68; continue 'dispatch;
	// 8244BC14: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244BC18: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BC1C: 2F0A0013  cmpwi cr6, r10, 0x13
	ctx.cr[6].compare_i32(ctx.r[10].s32, 19, &mut ctx.xer);
	// 8244BC20: 4198FF94  blt cr6, 0x8244bbb4
	if ctx.cr[6].lt {
	pc = 0x8244BBB4; continue 'dispatch;
	}
	// 8244BC24: 394AFFED  addi r10, r10, -0x13
	ctx.r[10].s64 = ctx.r[10].s64 + -19;
	// 8244BC28: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244BC2C: 419A0024  beq cr6, 0x8244bc50
	if ctx.cr[6].eq {
	pc = 0x8244BC50; continue 'dispatch;
	}
	// 8244BC30: 210A000D  subfic r8, r10, 0xd
	ctx.xer.ca = ctx.r[10].u32 <= 13 as u32;
	ctx.r[8].s64 = (13 as i64) - ctx.r[10].s64;
	// 8244BC34: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BC38: 7D084B78  or r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8244BC3C: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BC40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BC44: 55086CFE  srwi r8, r8, 0x13
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BC48: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BC4C: 4BFFFF74  b 0x8244bbc0
	pc = 0x8244BBC0; continue 'dispatch;
	// 8244BC50: 55286CFE  srwi r8, r9, 0x13
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(19);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BC54: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8244BC58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BC5C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BC60: 4BFFFF60  b 0x8244bbc0
	pc = 0x8244BBC0; continue 'dispatch;
	// 8244BC64: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BC68: 2F08001D  cmpwi cr6, r8, 0x1d
	ctx.cr[6].compare_i32(ctx.r[8].s32, 29, &mut ctx.xer);
	// 8244BC6C: 41980044  blt cr6, 0x8244bcb0
	if ctx.cr[6].lt {
	pc = 0x8244BCB0; continue 'dispatch;
	}
	// 8244BC70: 3928FFE3  addi r9, r8, -0x1d
	ctx.r[9].s64 = ctx.r[8].s64 + -29;
	// 8244BC74: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244BC78: 419A0024  beq cr6, 0x8244bc9c
	if ctx.cr[6].eq {
	pc = 0x8244BC9C; continue 'dispatch;
	}
	// 8244BC7C: 21090003  subfic r8, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[9].s64;
	// 8244BC80: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BC84: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244BC88: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BC8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BC90: 55061F7E  srwi r6, r8, 0x1d
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244BC94: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BC98: 48000024  b 0x8244bcbc
	pc = 0x8244BCBC; continue 'dispatch;
	// 8244BC9C: 55461F7E  srwi r6, r10, 0x1d
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244BCA0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244BCA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BCA8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BCAC: 48000010  b 0x8244bcbc
	pc = 0x8244BCBC; continue 'dispatch;
	// 8244BCB0: 55461F7E  srwi r6, r10, 0x1d
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244BCB4: 39280003  addi r9, r8, 3
	ctx.r[9].s64 = ctx.r[8].s64 + 3;
	// 8244BCB8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BCBC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8244BCC0: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244BCC4: 41980018  blt cr6, 0x8244bcdc
	if ctx.cr[6].lt {
	pc = 0x8244BCDC; continue 'dispatch;
	}
	// 8244BCC8: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244BCCC: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BCD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BCD4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BCD8: 48000008  b 0x8244bce0
	pc = 0x8244BCE0; continue 'dispatch;
	// 8244BCDC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BCE0: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8244BCE4: 4198003C  blt cr6, 0x8244bd20
	if ctx.cr[6].lt {
	pc = 0x8244BD20; continue 'dispatch;
	}
	// 8244BCE8: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8244BCEC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244BCF0: 419A0020  beq cr6, 0x8244bd10
	if ctx.cr[6].eq {
	pc = 0x8244BD10; continue 'dispatch;
	}
	// 8244BCF4: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8244BCF8: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BCFC: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244BD00: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BD04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BD08: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BD0C: 4800001C  b 0x8244bd28
	pc = 0x8244BD28; continue 'dispatch;
	// 8244BD10: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244BD14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BD18: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BD1C: 4800000C  b 0x8244bd28
	pc = 0x8244BD28; continue 'dispatch;
	// 8244BD20: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8244BD24: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BD28: 55477C7E  srwi r7, r10, 0x11
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8244BD2C: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8244BD30: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244BD34: 41980018  blt cr6, 0x8244bd4c
	if ctx.cr[6].lt {
	pc = 0x8244BD4C; continue 'dispatch;
	}
	// 8244BD38: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244BD3C: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BD40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BD44: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BD48: 48000008  b 0x8244bd50
	pc = 0x8244BD50; continue 'dispatch;
	// 8244BD4C: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BD50: 2F0A0011  cmpwi cr6, r10, 0x11
	ctx.cr[6].compare_i32(ctx.r[10].s32, 17, &mut ctx.xer);
	// 8244BD54: 41980034  blt cr6, 0x8244bd88
	if ctx.cr[6].lt {
	pc = 0x8244BD88; continue 'dispatch;
	}
	// 8244BD58: 394AFFEF  addi r10, r10, -0x11
	ctx.r[10].s64 = ctx.r[10].s64 + -17;
	// 8244BD5C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244BD60: 419A001C  beq cr6, 0x8244bd7c
	if ctx.cr[6].eq {
	pc = 0x8244BD7C; continue 'dispatch;
	}
	// 8244BD64: 210A000F  subfic r8, r10, 0xf
	ctx.xer.ca = ctx.r[10].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[10].s64;
	// 8244BD68: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BD6C: 7D6B4430  srw r11, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BD70: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8244BD74: 55697C7E  srwi r9, r11, 0x11
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BD78: 48000018  b 0x8244bd90
	pc = 0x8244BD90; continue 'dispatch;
	// 8244BD7C: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BD80: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BD84: 4800000C  b 0x8244bd90
	pc = 0x8244BD90; continue 'dispatch;
	// 8244BD88: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 8244BD8C: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BD90: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8244BD94: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8244BD98: 4198000C  blt cr6, 0x8244bda4
	if ctx.cr[6].lt {
	pc = 0x8244BDA4; continue 'dispatch;
	}
	// 8244BD9C: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8244BDA0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BDA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244BDA8: 78E80020  clrldi r8, r7, 0x20
	ctx.r[8].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 8244BDAC: 78CA7C4C  rldimi r10, r6, 0xf, 0x11
	ctx.r[10].u64 = ((ctx.r[6].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[10].u64 & 0xFFFF800000007FFF);
	// 8244BDB0: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 8244BDB4: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8244BDB8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8244BDBC: 794A7C24  sldi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(15);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8244BDC0: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8244BDC4: F91E0008  std r8, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 8244BDC8: 48000348  b 0x8244c110
	pc = 0x8244C110; continue 'dispatch;
	// 8244BDCC: 2B080003  cmplwi cr6, r8, 3
	ctx.cr[6].compare_u32(ctx.r[8].u32, 3 as u32, &mut ctx.xer);
	// 8244BDD0: 409A0324  bne cr6, 0x8244c0f4
	if !ctx.cr[6].eq {
	pc = 0x8244C0F4; continue 'dispatch;
	}
	// 8244BDD4: 390A0004  addi r8, r10, 4
	ctx.r[8].s64 = ctx.r[10].s64 + 4;
	// 8244BDD8: 2F080020  cmpwi cr6, r8, 0x20
	ctx.cr[6].compare_i32(ctx.r[8].s32, 32, &mut ctx.xer);
	// 8244BDDC: 41980018  blt cr6, 0x8244bdf4
	if ctx.cr[6].lt {
	pc = 0x8244BDF4; continue 'dispatch;
	}
	// 8244BDE0: 3908FFE0  addi r8, r8, -0x20
	ctx.r[8].s64 = ctx.r[8].s64 + -32;
	// 8244BDE4: 7D6A4030  slw r10, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BDE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BDEC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BDF0: 48000008  b 0x8244bdf8
	pc = 0x8244BDF8; continue 'dispatch;
	// 8244BDF4: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BDF8: 2F08001D  cmpwi cr6, r8, 0x1d
	ctx.cr[6].compare_i32(ctx.r[8].s32, 29, &mut ctx.xer);
	// 8244BDFC: 41980044  blt cr6, 0x8244be40
	if ctx.cr[6].lt {
	pc = 0x8244BE40; continue 'dispatch;
	}
	// 8244BE00: 3928FFE3  addi r9, r8, -0x1d
	ctx.r[9].s64 = ctx.r[8].s64 + -29;
	// 8244BE04: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244BE08: 419A0024  beq cr6, 0x8244be2c
	if ctx.cr[6].eq {
	pc = 0x8244BE2C; continue 'dispatch;
	}
	// 8244BE0C: 21090003  subfic r8, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[9].s64;
	// 8244BE10: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BE14: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244BE18: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BE1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BE20: 55031F7E  srwi r3, r8, 0x1d
	ctx.r[3].u32 = ctx.r[8].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8244BE24: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BE28: 48000024  b 0x8244be4c
	pc = 0x8244BE4C; continue 'dispatch;
	// 8244BE2C: 55431F7E  srwi r3, r10, 0x1d
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8244BE30: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244BE34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BE38: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BE3C: 48000010  b 0x8244be4c
	pc = 0x8244BE4C; continue 'dispatch;
	// 8244BE40: 55431F7E  srwi r3, r10, 0x1d
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8244BE44: 39280003  addi r9, r8, 3
	ctx.r[9].s64 = ctx.r[8].s64 + 3;
	// 8244BE48: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BE4C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8244BE50: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244BE54: 41980018  blt cr6, 0x8244be6c
	if ctx.cr[6].lt {
	pc = 0x8244BE6C; continue 'dispatch;
	}
	// 8244BE58: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244BE5C: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BE60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BE64: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BE68: 48000008  b 0x8244be70
	pc = 0x8244BE70; continue 'dispatch;
	// 8244BE6C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BE70: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8244BE74: 41980044  blt cr6, 0x8244beb8
	if ctx.cr[6].lt {
	pc = 0x8244BEB8; continue 'dispatch;
	}
	// 8244BE78: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8244BE7C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244BE80: 419A0024  beq cr6, 0x8244bea4
	if ctx.cr[6].eq {
	pc = 0x8244BEA4; continue 'dispatch;
	}
	// 8244BE84: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8244BE88: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BE8C: 7D085378  or r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244BE90: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BE94: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BE98: 55067C7E  srwi r6, r8, 0x11
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244BE9C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BEA0: 48000024  b 0x8244bec4
	pc = 0x8244BEC4; continue 'dispatch;
	// 8244BEA4: 55467C7E  srwi r6, r10, 0x11
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244BEA8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8244BEAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BEB0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BEB4: 48000010  b 0x8244bec4
	pc = 0x8244BEC4; continue 'dispatch;
	// 8244BEB8: 55467C7E  srwi r6, r10, 0x11
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244BEBC: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8244BEC0: 554A7820  slwi r10, r10, 0xf
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BEC4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8244BEC8: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244BECC: 41980018  blt cr6, 0x8244bee4
	if ctx.cr[6].lt {
	pc = 0x8244BEE4; continue 'dispatch;
	}
	// 8244BED0: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244BED4: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BED8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BEDC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BEE0: 48000008  b 0x8244bee8
	pc = 0x8244BEE8; continue 'dispatch;
	// 8244BEE4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244BEE8: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8244BEEC: 4198003C  blt cr6, 0x8244bf28
	if ctx.cr[6].lt {
	pc = 0x8244BF28; continue 'dispatch;
	}
	// 8244BEF0: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8244BEF4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244BEF8: 419A0020  beq cr6, 0x8244bf18
	if ctx.cr[6].eq {
	pc = 0x8244BF18; continue 'dispatch;
	}
	// 8244BEFC: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8244BF00: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BF04: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244BF08: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BF0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BF10: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BF14: 4800001C  b 0x8244bf30
	pc = 0x8244BF30; continue 'dispatch;
	// 8244BF18: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244BF1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BF20: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BF24: 4800000C  b 0x8244bf30
	pc = 0x8244BF30; continue 'dispatch;
	// 8244BF28: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8244BF2C: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BF30: 55477C7E  srwi r7, r10, 0x11
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8244BF34: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8244BF38: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244BF3C: 41980018  blt cr6, 0x8244bf54
	if ctx.cr[6].lt {
	pc = 0x8244BF54; continue 'dispatch;
	}
	// 8244BF40: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244BF44: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BF48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BF4C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BF50: 48000008  b 0x8244bf58
	pc = 0x8244BF58; continue 'dispatch;
	// 8244BF54: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BF58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8244BF5C: 78C60020  clrldi r6, r6, 0x20
	ctx.r[6].u64 = ctx.r[6].u64 & 0x00000000FFFFFFFFu64;
	// 8244BF60: 78687C4C  rldimi r8, r3, 0xf, 0x11
	ctx.r[8].u64 = ((ctx.r[3].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[8].u64 & 0xFFFF800000007FFF);
	// 8244BF64: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 8244BF68: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 8244BF6C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8244BF70: 79087C24  sldi r8, r8, 0xf
	ctx.r[8].u64 = ctx.r[8].u64.wrapping_shl(15);
	ctx.r[8].u32 = ctx.r[8].u64 as u32;
	// 8244BF74: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244BF78: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 8244BF7C: F91E0000  std r8, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8244BF80: 41980018  blt cr6, 0x8244bf98
	if ctx.cr[6].lt {
	pc = 0x8244BF98; continue 'dispatch;
	}
	// 8244BF84: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244BF88: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BF8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BF90: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BF94: 48000008  b 0x8244bf9c
	pc = 0x8244BF9C; continue 'dispatch;
	// 8244BF98: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244BF9C: 2F0A001D  cmpwi cr6, r10, 0x1d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 29, &mut ctx.xer);
	// 8244BFA0: 4198003C  blt cr6, 0x8244bfdc
	if ctx.cr[6].lt {
	pc = 0x8244BFDC; continue 'dispatch;
	}
	// 8244BFA4: 394AFFE3  addi r10, r10, -0x1d
	ctx.r[10].s64 = ctx.r[10].s64 + -29;
	// 8244BFA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244BFAC: 419A0020  beq cr6, 0x8244bfcc
	if ctx.cr[6].eq {
	pc = 0x8244BFCC; continue 'dispatch;
	}
	// 8244BFB0: 210A0003  subfic r8, r10, 3
	ctx.xer.ca = ctx.r[10].u32 <= 3 as u32;
	ctx.r[8].s64 = (3 as i64) - ctx.r[10].s64;
	// 8244BFB4: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BFB8: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 8244BFBC: 7D685030  slw r8, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BFC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BFC4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BFC8: 4800001C  b 0x8244bfe4
	pc = 0x8244BFE4; continue 'dispatch;
	// 8244BFCC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244BFD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244BFD4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244BFD8: 4800000C  b 0x8244bfe4
	pc = 0x8244BFE4; continue 'dispatch;
	// 8244BFDC: 394A0003  addi r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 3;
	// 8244BFE0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244BFE4: 55261F7E  srwi r6, r9, 0x1d
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shr(29);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8244BFE8: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8244BFEC: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8244BFF0: 41980018  blt cr6, 0x8244c008
	if ctx.cr[6].lt {
	pc = 0x8244C008; continue 'dispatch;
	}
	// 8244BFF4: 3929FFE0  addi r9, r9, -0x20
	ctx.r[9].s64 = ctx.r[9].s64 + -32;
	// 8244BFF8: 7D6A4830  slw r10, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244BFFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C000: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C004: 48000008  b 0x8244c00c
	pc = 0x8244C00C; continue 'dispatch;
	// 8244C008: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244C00C: 2F090011  cmpwi cr6, r9, 0x11
	ctx.cr[6].compare_i32(ctx.r[9].s32, 17, &mut ctx.xer);
	// 8244C010: 4198003C  blt cr6, 0x8244c04c
	if ctx.cr[6].lt {
	pc = 0x8244C04C; continue 'dispatch;
	}
	// 8244C014: 3929FFEF  addi r9, r9, -0x11
	ctx.r[9].s64 = ctx.r[9].s64 + -17;
	// 8244C018: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8244C01C: 419A0020  beq cr6, 0x8244c03c
	if ctx.cr[6].eq {
	pc = 0x8244C03C; continue 'dispatch;
	}
	// 8244C020: 2109000F  subfic r8, r9, 0xf
	ctx.xer.ca = ctx.r[9].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[9].s64;
	// 8244C024: 7D684430  srw r8, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244C028: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 8244C02C: 7D684830  slw r8, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 8244C030: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C034: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C038: 4800001C  b 0x8244c054
	pc = 0x8244C054; continue 'dispatch;
	// 8244C03C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8244C040: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C044: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C048: 4800000C  b 0x8244c054
	pc = 0x8244C054; continue 'dispatch;
	// 8244C04C: 3929000F  addi r9, r9, 0xf
	ctx.r[9].s64 = ctx.r[9].s64 + 15;
	// 8244C050: 55487820  slwi r8, r10, 0xf
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8244C054: 55477C7E  srwi r7, r10, 0x11
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(17);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8244C058: 39490001  addi r10, r9, 1
	ctx.r[10].s64 = ctx.r[9].s64 + 1;
	// 8244C05C: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 8244C060: 41980018  blt cr6, 0x8244c078
	if ctx.cr[6].lt {
	pc = 0x8244C078; continue 'dispatch;
	}
	// 8244C064: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 8244C068: 7D695030  slw r9, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8244C06C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C070: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C074: 48000008  b 0x8244c07c
	pc = 0x8244C07C; continue 'dispatch;
	// 8244C078: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244C07C: 2F0A0011  cmpwi cr6, r10, 0x11
	ctx.cr[6].compare_i32(ctx.r[10].s32, 17, &mut ctx.xer);
	// 8244C080: 41980034  blt cr6, 0x8244c0b4
	if ctx.cr[6].lt {
	pc = 0x8244C0B4; continue 'dispatch;
	}
	// 8244C084: 394AFFEF  addi r10, r10, -0x11
	ctx.r[10].s64 = ctx.r[10].s64 + -17;
	// 8244C088: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244C08C: 419A001C  beq cr6, 0x8244c0a8
	if ctx.cr[6].eq {
	pc = 0x8244C0A8; continue 'dispatch;
	}
	// 8244C090: 210A000F  subfic r8, r10, 0xf
	ctx.xer.ca = ctx.r[10].u32 <= 15 as u32;
	ctx.r[8].s64 = (15 as i64) - ctx.r[10].s64;
	// 8244C094: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C098: 7D6B4430  srw r11, r11, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8244C09C: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 8244C0A0: 55697C7E  srwi r9, r11, 0x11
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244C0A4: 48000018  b 0x8244c0bc
	pc = 0x8244C0BC; continue 'dispatch;
	// 8244C0A8: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244C0AC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C0B0: 4800000C  b 0x8244c0bc
	pc = 0x8244C0BC; continue 'dispatch;
	// 8244C0B4: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 8244C0B8: 55297C7E  srwi r9, r9, 0x11
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(17);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8244C0BC: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8244C0C0: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8244C0C4: 4198000C  blt cr6, 0x8244c0d0
	if ctx.cr[6].lt {
	pc = 0x8244C0D0; continue 'dispatch;
	}
	// 8244C0C8: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8244C0CC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244C0D4: 78E80020  clrldi r8, r7, 0x20
	ctx.r[8].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 8244C0D8: 78CA7C4C  rldimi r10, r6, 0xf, 0x11
	ctx.r[10].u64 = ((ctx.r[6].u64).rotate_left(15) & 0x00007FFFFFFF8000) | (ctx.r[10].u64 & 0xFFFF800000007FFF);
	// 8244C0DC: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 8244C0E0: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 8244C0E4: 794A7C24  sldi r10, r10, 0xf
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(15);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 8244C0E8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8244C0EC: F95E0008  std r10, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8244C0F0: 48000024  b 0x8244c114
	pc = 0x8244C114; continue 'dispatch;
	// 8244C0F4: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 8244C0F8: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8244C0FC: 4198000C  blt cr6, 0x8244c108
	if ctx.cr[6].lt {
	pc = 0x8244C108; continue 'dispatch;
	}
	// 8244C100: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 8244C104: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8244C108: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8244C10C: F95E0008  std r10, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8244C110: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8244C114: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 8244C118: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 8244C11C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 8244C120: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8244C124: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 8244C128: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244C12C: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8244C130: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8244C134: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8244C138: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8244C13C: 480E8FD0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8244C140: 93A50000  stw r29, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8244C144: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8244C148: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8244C14C: 480E8FC0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244C150 size=208
    let mut pc: u32 = 0x8244C150;
    'dispatch: loop {
        match pc {
            0x8244C150 => {
    //   block [0x8244C150..0x8244C220)
	// 8244C150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244C154: 480E8F61  bl 0x825350b4
	ctx.lr = 0x8244C158;
	sub_82535080(ctx, base);
	// 8244C158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244C15C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8244C160: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8244C164: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244C168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244C16C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244C170: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8244C174: 937D0000  stw r27, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8244C178: 48000371  bl 0x8244c4e8
	ctx.lr = 0x8244C17C;
	sub_8244C4E8(ctx, base);
	// 8244C17C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8244C180: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8244C184: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8244C188: 419A007C  beq cr6, 0x8244c204
	if ctx.cr[6].eq {
	pc = 0x8244C204; continue 'dispatch;
	}
	// 8244C18C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8244C190: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8244C194: 419A0054  beq cr6, 0x8244c1e8
	if ctx.cr[6].eq {
	pc = 0x8244C1E8; continue 'dispatch;
	}
	// 8244C198: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 8244C19C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8244C1A0: 409A003C  bne cr6, 0x8244c1dc
	if !ctx.cr[6].eq {
	pc = 0x8244C1DC; continue 'dispatch;
	}
	// 8244C1A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8244C1A8: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8244C1AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244C1B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244C1B4: 4BFFF7A5  bl 0x8244b958
	ctx.lr = 0x8244C1B8;
	sub_8244B958(ctx, base);
	// 8244C1B8: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8244C1BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244C1C0: 419A001C  beq cr6, 0x8244c1dc
	if ctx.cr[6].eq {
	pc = 0x8244C1DC; continue 'dispatch;
	}
	// 8244C1C4: 815F00B8  lwz r10, 0xb8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 8244C1C8: E8BF00A8  ld r5, 0xa8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	// 8244C1CC: 807F00F8  lwz r3, 0xf8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 8244C1D0: 5544063E  clrlwi r4, r10, 0x18
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8244C1D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244C1D8: 4E800421  bctrl
	ctx.lr = 0x8244C1DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8244C1DC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8244C1E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244C1E4: 480E8F20  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8244C1E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8244C1EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244C1F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244C1F4: 4BFFF21D  bl 0x8244b410
	ctx.lr = 0x8244C1F8;
	sub_8244B410(ctx, base);
	// 8244C1F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244C1FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244C200: 480E8F04  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8244C204: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8244C208: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244C20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244C210: 4BFFEF71  bl 0x8244b180
	ctx.lr = 0x8244C214;
	sub_8244B180(ctx, base);
	// 8244C214: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244C218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244C21C: 480E8EE8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244C220 size=228
    let mut pc: u32 = 0x8244C220;
    'dispatch: loop {
        match pc {
            0x8244C220 => {
    //   block [0x8244C220..0x8244C304)
	// 8244C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244C224: 480E8E91  bl 0x825350b4
	ctx.lr = 0x8244C228;
	sub_82535080(ctx, base);
	// 8244C228: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244C22C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8244C230: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8244C234: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244C238: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8244C23C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8244C240: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8244C244: 41980054  blt cr6, 0x8244c298
	if ctx.cr[6].lt {
	pc = 0x8244C298; continue 'dispatch;
	}
	// 8244C248: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8244C24C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8244C250: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8244C254: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8244C258: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8244C25C: 4BFFFEF5  bl 0x8244c150
	ctx.lr = 0x8244C260;
	sub_8244C150(ctx, base);
	// 8244C260: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C264: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244C268: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244C26C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8244C270: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8244C274: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8244C278: 7FEBF850  subf r31, r11, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8244C27C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8244C280: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C284: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8244C288: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244C28C: 419A000C  beq cr6, 0x8244c298
	if ctx.cr[6].eq {
	pc = 0x8244C298; continue 'dispatch;
	}
	// 8244C290: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8244C294: 4098FFB4  bge cr6, 0x8244c248
	if !ctx.cr[6].lt {
	pc = 0x8244C248; continue 'dispatch;
	}
	// 8244C298: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C29C: 556B039C  rlwinm r11, r11, 0, 0xe, 0xe
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8244C2A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244C2A4: 419A0054  beq cr6, 0x8244c2f8
	if ctx.cr[6].eq {
	pc = 0x8244C2F8; continue 'dispatch;
	}
	// 8244C2A8: 397B0028  addi r11, r27, 0x28
	ctx.r[11].s64 = ctx.r[27].s64 + 40;
	// 8244C2AC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8244C2B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244C2B4: 419A000C  beq cr6, 0x8244c2c0
	if ctx.cr[6].eq {
	pc = 0x8244C2C0; continue 'dispatch;
	}
	// 8244C2B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244C2BC: 48000014  b 0x8244c2d0
	pc = 0x8244C2D0; continue 'dispatch;
	// 8244C2C0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244C2C4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8244C2C8: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8244C2CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244C2D0: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244C2D4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8244C2D8: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 8244C2DC: 394A0048  addi r10, r10, 0x48
	ctx.r[10].s64 = ctx.r[10].s64 + 72;
	// 8244C2E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8244C2E4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C2E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8244C2EC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8244C2F0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8244C2F4: 4200FFF0  bdnz 0x8244c2e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8244C2E4; continue 'dispatch;
	}
	// 8244C2F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C2FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8244C300: 480E8E04  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244C308 size=104
    let mut pc: u32 = 0x8244C308;
    'dispatch: loop {
        match pc {
            0x8244C308 => {
    //   block [0x8244C308..0x8244C370)
	// 8244C308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244C30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244C310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244C314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244C318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244C31C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8244C320: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8244C324: 4BFFE91D  bl 0x8244ac40
	ctx.lr = 0x8244C328;
	sub_8244AC40(ctx, base);
	// 8244C328: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244C32C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C330: 419A0014  beq cr6, 0x8244c344
	if ctx.cr[6].eq {
	pc = 0x8244C344; continue 'dispatch;
	}
	// 8244C334: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8244C338: 60840201  ori r4, r4, 0x201
	ctx.r[4].u64 = ctx.r[4].u64 | 513;
	// 8244C33C: 4BFFE9E5  bl 0x8244ad20
	ctx.lr = 0x8244C340;
	sub_8244AD20(ctx, base);
	// 8244C340: 48000018  b 0x8244c358
	pc = 0x8244C358; continue 'dispatch;
	// 8244C344: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 8244C348: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8244C34C: F95F0000  std r10, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8244C350: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8244C354: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8244C358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244C35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244C360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244C364: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244C368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244C36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244C370 size=116
    let mut pc: u32 = 0x8244C370;
    'dispatch: loop {
        match pc {
            0x8244C370 => {
    //   block [0x8244C370..0x8244C3E4)
	// 8244C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244C374: 480E8D49  bl 0x825350bc
	ctx.lr = 0x8244C378;
	sub_82535080(ctx, base);
	// 8244C378: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244C37C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244C380: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244C384: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8244C388: 4BFFE8B9  bl 0x8244ac40
	ctx.lr = 0x8244C38C;
	sub_8244AC40(ctx, base);
	// 8244C38C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244C390: 419A001C  beq cr6, 0x8244c3ac
	if ctx.cr[6].eq {
	pc = 0x8244C3AC; continue 'dispatch;
	}
	// 8244C394: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8244C398: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C39C: 60840202  ori r4, r4, 0x202
	ctx.r[4].u64 = ctx.r[4].u64 | 514;
	// 8244C3A0: 4BFFE981  bl 0x8244ad20
	ctx.lr = 0x8244C3A4;
	sub_8244AD20(ctx, base);
	// 8244C3A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244C3A8: 480E8D64  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8244C3AC: 57AB2834  slwi r11, r29, 5
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8244C3B0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8244C3B4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8244C3B8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8244C3BC: 396B0048  addi r11, r11, 0x48
	ctx.r[11].s64 = ctx.r[11].s64 + 72;
	// 8244C3C0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8244C3C4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C3C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8244C3CC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8244C3D0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8244C3D4: 4200FFF0  bdnz 0x8244c3c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8244C3C4; continue 'dispatch;
	}
	// 8244C3D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C3DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244C3E0: 480E8D2C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244C3E8 size=124
    let mut pc: u32 = 0x8244C3E8;
    'dispatch: loop {
        match pc {
            0x8244C3E8 => {
    //   block [0x8244C3E8..0x8244C464)
	// 8244C3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244C3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244C3F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244C3F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244C3F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244C3FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244C400: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244C404: 4BFFE83D  bl 0x8244ac40
	ctx.lr = 0x8244C408;
	sub_8244AC40(ctx, base);
	// 8244C408: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244C40C: 419A0018  beq cr6, 0x8244c424
	if ctx.cr[6].eq {
	pc = 0x8244C424; continue 'dispatch;
	}
	// 8244C410: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8244C414: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C418: 60840202  ori r4, r4, 0x202
	ctx.r[4].u64 = ctx.r[4].u64 | 514;
	// 8244C41C: 4BFFE905  bl 0x8244ad20
	ctx.lr = 0x8244C420;
	sub_8244AD20(ctx, base);
	// 8244C420: 4800002C  b 0x8244c44c
	pc = 0x8244C44C; continue 'dispatch;
	// 8244C424: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 8244C428: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8244C42C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8244C430: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8244C434: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C438: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8244C43C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8244C440: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8244C444: 4200FFF0  bdnz 0x8244c434
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8244C434; continue 'dispatch;
	}
	// 8244C448: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C44C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244C450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244C454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244C458: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244C45C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244C460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244C468 size=124
    let mut pc: u32 = 0x8244C468;
    'dispatch: loop {
        match pc {
            0x8244C468 => {
    //   block [0x8244C468..0x8244C4E4)
	// 8244C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244C470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244C474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244C478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244C47C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244C480: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244C484: 4BFFE7BD  bl 0x8244ac40
	ctx.lr = 0x8244C488;
	sub_8244AC40(ctx, base);
	// 8244C488: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244C48C: 419A0018  beq cr6, 0x8244c4a4
	if ctx.cr[6].eq {
	pc = 0x8244C4A4; continue 'dispatch;
	}
	// 8244C490: 3C80FF02  lis r4, -0xfe
	ctx.r[4].s64 = -16646144;
	// 8244C494: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C498: 60840203  ori r4, r4, 0x203
	ctx.r[4].u64 = ctx.r[4].u64 | 515;
	// 8244C49C: 4BFFE885  bl 0x8244ad20
	ctx.lr = 0x8244C4A0;
	sub_8244AD20(ctx, base);
	// 8244C4A0: 4800002C  b 0x8244c4cc
	pc = 0x8244C4CC; continue 'dispatch;
	// 8244C4A4: 397F00A8  addi r11, r31, 0xa8
	ctx.r[11].s64 = ctx.r[31].s64 + 168;
	// 8244C4A8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8244C4AC: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8244C4B0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8244C4B4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8244C4B8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8244C4BC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8244C4C0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8244C4C4: 4200FFF0  bdnz 0x8244c4b4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8244C4B4; continue 'dispatch;
	}
	// 8244C4C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C4CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244C4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244C4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244C4D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244C4DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244C4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244C4E8 size=80
    let mut pc: u32 = 0x8244C4E8;
    'dispatch: loop {
        match pc {
            0x8244C4E8 => {
    //   block [0x8244C4E8..0x8244C538)
	// 8244C4E8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244C4EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244C4F0: 409A0060  bne cr6, 0x8244c550
	if !ctx.cr[6].eq {
		sub_8244C550(ctx, base);
		return;
	}
	// 8244C4F4: 89630001  lbz r11, 1(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1 as u32) ) } as u64;
	// 8244C4F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244C4FC: 409A0054  bne cr6, 0x8244c550
	if !ctx.cr[6].eq {
		sub_8244C550(ctx, base);
		return;
	}
	// 8244C500: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 8244C504: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8244C508: 409A0048  bne cr6, 0x8244c550
	if !ctx.cr[6].eq {
		sub_8244C550(ctx, base);
		return;
	}
	// 8244C50C: 89630003  lbz r11, 3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 8244C510: 2F0B00B9  cmpwi cr6, r11, 0xb9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 185, &mut ctx.xer);
	// 8244C514: 419A0034  beq cr6, 0x8244c548
	if ctx.cr[6].eq {
		sub_8244C548(ctx, base);
		return;
	}
	// 8244C518: 2F0B00BA  cmpwi cr6, r11, 0xba
	ctx.cr[6].compare_i32(ctx.r[11].s32, 186, &mut ctx.xer);
	// 8244C51C: 419A0024  beq cr6, 0x8244c540
	if ctx.cr[6].eq {
		sub_8244C540(ctx, base);
		return;
	}
	// 8244C520: 2F0B00BB  cmpwi cr6, r11, 0xbb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 187, &mut ctx.xer);
	// 8244C524: 419A0014  beq cr6, 0x8244c538
	if ctx.cr[6].eq {
		sub_8244C538(ctx, base);
		return;
	}
	// 8244C528: 2B0B00BC  cmplwi cr6, r11, 0xbc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 188 as u32, &mut ctx.xer);
	// 8244C52C: 41980024  blt cr6, 0x8244c550
	if ctx.cr[6].lt {
		sub_8244C550(ctx, base);
		return;
	}
	// 8244C530: 3C600004  lis r3, 4
	ctx.r[3].s64 = 262144;
	// 8244C534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244C538 size=8
    let mut pc: u32 = 0x8244C538;
    'dispatch: loop {
        match pc {
            0x8244C538 => {
    //   block [0x8244C538..0x8244C540)
	// 8244C538: 3C600002  lis r3, 2
	ctx.r[3].s64 = 131072;
	// 8244C53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244C540 size=8
    let mut pc: u32 = 0x8244C540;
    'dispatch: loop {
        match pc {
            0x8244C540 => {
    //   block [0x8244C540..0x8244C548)
	// 8244C540: 3C600001  lis r3, 1
	ctx.r[3].s64 = 65536;
	// 8244C544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244C548 size=8
    let mut pc: u32 = 0x8244C548;
    'dispatch: loop {
        match pc {
            0x8244C548 => {
    //   block [0x8244C548..0x8244C550)
	// 8244C548: 3C600008  lis r3, 8
	ctx.r[3].s64 = 524288;
	// 8244C54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244C550 size=8
    let mut pc: u32 = 0x8244C550;
    'dispatch: loop {
        match pc {
            0x8244C550 => {
    //   block [0x8244C550..0x8244C558)
	// 8244C550: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8244C558 size=116
    let mut pc: u32 = 0x8244C558;
    'dispatch: loop {
        match pc {
            0x8244C558 => {
    //   block [0x8244C558..0x8244C5CC)
	// 8244C558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244C55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8244C560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244C564: 4BFEB1A5  bl 0x82437708
	ctx.lr = 0x8244C568;
	sub_82437708(ctx, base);
	// 8244C568: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244C56C: 419A0024  beq cr6, 0x8244c590
	if ctx.cr[6].eq {
	pc = 0x8244C590; continue 'dispatch;
	}
	// 8244C570: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 8244C574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C578: 6084FF03  ori r4, r4, 0xff03
	ctx.r[4].u64 = ctx.r[4].u64 | 65283;
	// 8244C57C: 48003975  bl 0x8244fef0
	ctx.lr = 0x8244C580;
	sub_8244FEF0(ctx, base);
	// 8244C580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244C584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244C588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244C58C: 4E800020  blr
	return;
	// 8244C590: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8244C594: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 8244C598: 386B6600  addi r3, r11, 0x6600
	ctx.r[3].s64 = ctx.r[11].s64 + 26112;
	// 8244C59C: 38801530  li r4, 0x1530
	ctx.r[4].s64 = 5424;
	// 8244C5A0: 48007839  bl 0x82453dd8
	ctx.lr = 0x8244C5A4;
	sub_82453DD8(ctx, base);
	// 8244C5A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244C5A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244C5AC: 419A0010  beq cr6, 0x8244c5bc
	if ctx.cr[6].eq {
	pc = 0x8244C5BC; continue 'dispatch;
	}
	// 8244C5B0: 3C80FF03  lis r4, -0xfd
	ctx.r[4].s64 = -16580608;
	// 8244C5B4: 6084FF07  ori r4, r4, 0xff07
	ctx.r[4].u64 = ctx.r[4].u64 | 65287;
	// 8244C5B8: 48003939  bl 0x8244fef0
	ctx.lr = 0x8244C5BC;
	sub_8244FEF0(ctx, base);
	// 8244C5BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8244C5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244C5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244C5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244C5D0 size=108
    let mut pc: u32 = 0x8244C5D0;
    'dispatch: loop {
        match pc {
            0x8244C5D0 => {
    //   block [0x8244C5D0..0x8244C63C)
	// 8244C5D0: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 8244C5D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8244C5D8: 390BDCB0  addi r8, r11, -0x2350
	ctx.r[8].s64 = ctx.r[11].s64 + -9040;
	// 8244C5DC: 39400180  li r10, 0x180
	ctx.r[10].s64 = 384;
	// 8244C5E0: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8244C5E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8244C5E8: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8244C5EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8244C5F0: 4200FFF8  bdnz 0x8244c5e8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8244C5E8; continue 'dispatch;
	}
	// 8244C5F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8244C5F8: 39680180  addi r11, r8, 0x180
	ctx.r[11].s64 = ctx.r[8].s64 + 384;
	// 8244C5FC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8244C600: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244C604: 2F0A0100  cmpwi cr6, r10, 0x100
	ctx.cr[6].compare_i32(ctx.r[10].s32, 256, &mut ctx.xer);
	// 8244C608: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8244C60C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8244C610: 4198FFEC  blt cr6, 0x8244c5fc
	if ctx.cr[6].lt {
	pc = 0x8244C5FC; continue 'dispatch;
	}
	// 8244C614: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 8244C618: 39400180  li r10, 0x180
	ctx.r[10].s64 = 384;
	// 8244C61C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8244C620: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8244C624: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8244C628: 4200FFF8  bdnz 0x8244c620
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8244C620; continue 'dispatch;
	}
	// 8244C62C: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8244C630: 39680180  addi r11, r8, 0x180
	ctx.r[11].s64 = ctx.r[8].s64 + 384;
	// 8244C634: 916A0338  stw r11, 0x338(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(824 as u32), ctx.r[11].u32 ) };
	// 8244C638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


